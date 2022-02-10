// The MIT License (MIT)
//
// Copyright © 2022 <Brandon Lewis>
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Fork this project to create your own MIT license that you can
// always link to.
use std::collections::HashMap;
use core::hash::Hash;
use crate::{Types};


/**
 * Now we're working on ch 5 of FPGR.
 *
 * This module is a toy graph re-writing system (GRS).
 */


/**
 * ADT for canonical form GRS
 */
enum CanonicalTerm<ID: Copy, V> {
    Id(ID),
    Constructor(V),
}


/**
 * A node in a canonical graph.
 */
struct CanonicalNode<ID: Copy + Eq + Hash, V> {
    id: ID, // XXX: do we need this?
    function: V,
    rest: Vec<CanonicalTerm<ID, V>>
}


/**
 * A canonical graph is an ordered set of nodes.
 */
struct CanonicalGraph<ID: Copy + Eq + Hash, V> {
    ordering: Vec<ID>,
    mapping: HashMap<ID, CanonicalNode<ID, V>>
}

impl<ID: Copy + Eq + Hash, V> CanonicalGraph<ID, V> {
    pub fn new(nodes: Vec<CanonicalNode<ID, V>>) -> Self {
        let mut ordering = Vec::new();
        let mut mapping = HashMap::new();

        for node in nodes {
            ordering.push(node.id);
            mapping.insert(node.id, node);
        }

        CanonicalGraph { ordering, mapping }
    }

    pub fn root(&self) -> &CanonicalNode<ID, V> {
        &self.mapping[&self.ordering[0]]
    }

    pub fn get(&self, id: ID) -> Option<&CanonicalNode<ID, V>> {
        self.mapping.get(&id)
    }
}

/**
 * A rewrite rule in canonical form.
 *
 * Notice here we preserve the ID type parameter, since client code
 * might wish control over node id type.
 *
 * We could add an Id field to types, but this requires every
 * implementation define this type, even if they don't use it.
 *
 * We also introduce the additional bound that T::Sym be Copy, since
 * it's used for node ids. We don't want to take it by move.
 */
struct CanonicalRule<T: Types> where T::Sym: Copy + Eq + Hash {
    redex: CanonicalGraph<T::Sym, T::Val>,
    contractum: CanonicalGraph<T::Sym, T::Val>,
    redirection: (T::Sym, T::Sym)
}


/**
 * A Graph Rewriting System (GRS) is an ordered set of rules.
 */
struct CanonicalGRS<T: Types>(Vec<CanonicalRule<T>>) where T::Sym: Copy + Eq + Hash;


type DataGraph<ID, T: Types> = CanonicalGraph<ID, T::Val>;


/**
 * Given a pattern and a subgraph rooted at data_root, find find an
 * assignment of the pattern variables that satisfies the pattern, if
 * possible.
 */
fn matches<ID: Copy + Eq + Hash, T: Types>(
    pattern: &CanonicalRule<T>,
    pattern_root: T::Sym,
    data: &DataGraph<ID, T>,
    data_root: ID,
    mapping: Option<Vec<(T::Sym, ID)>>
) -> Option<Vec<(T::Sym, ID)>> where T:: Sym: Copy + Eq + Hash, T::Val: PartialEq {
    let mut mapping = mapping.unwrap_or(Vec::new());
    use CanonicalNode as N;
    use CanonicalTerm::*;

    match (pattern.redex.get(pattern_root)?, data.get(data_root)?) {
        (N {function: x, id: var, rest: sp},
         N {function: y, id: id,  rest: sd}) if x == y => {
            mapping.push((*var, *id));
            // loop over the rest of the pattern
            for (pat, node) in sp.iter().zip(sd.iter()) { match (pat, node) {
                (Id(v), Id(i)) => {
                    // bind the pattern var to the corresponding id
                    // XXX: duplicates would be an error of the IDs don't match!
                    mapping.push((*v, *i));
                    // ensure the sub-terms also match
                    mapping = matches(pattern, *v, data, *i, Some(mapping))?;
                },

                // Ensure two constructors are the same.
                (Constructor(x), Constructor(y)) if x == y => (),

                _ => {return None}
                //mapping = matches_term(pat, node, mapping)?;
            } }
            Some(mapping)
        }, _ => None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SigmaRules, Types};

    // We can get away with a limited set of identifiers for
    // tests. Thes are lower-case to match the literature. Variables
    // in rules are typically lower case, while constants are
    // CamelCase or just a capital letter.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    enum Symbols {a, b, c, d, m, n, o, x, y, z}

    // We can get away with a limited set of "constant" values as
    // well.
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Values {Start, Add, If, True, False, Int(i8), Zero, Succ, Hd, Cons}

    // We can punt on sigma rules for now.
    impl SigmaRules for Values {
        type Error = ();
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct TestTypes;

    impl Types for TestTypes {
        type Sym = Symbols;
        type Val = Values;
    }

    type TestGrs = CanonicalGRS<TestTypes>;

    #[test]
    fn test_rules() {
        use Symbols::*;
        use Values::*;
        use CanonicalTerm::*;

        // Fully expanded example from the book:
        //
        // x: Add y z
        // z: Zero    -> x := z
        //
        // x: Add y z,
        // y: Succ a  -> m: Succ n, n: Add a z, x := m
        //
        // x: Start   -> m: Add n o, n: Succ o, o: Zero, x := m
        let trs: TestGrs = CanonicalGRS(vec![
            CanonicalRule {
                redex: CanonicalGraph::new(vec![
                    CanonicalNode {
                        id: x,
                        function: Add,
                        rest: vec![Id(y), Id(z)],
                    }, CanonicalNode {
                        id: y,
                        function: Zero,
                        rest: vec![]
                    },
                ]),
                contractum: CanonicalGraph::new(vec![]),
                redirection: (x, z)
            }, CanonicalRule {
                redex: CanonicalGraph::new(vec![
                    CanonicalNode {
                        id: x,
                        function: Add,
                        rest: vec![Id(y), Id(z)],
                    }, CanonicalNode {
                        id: y,
                        function: Succ,
                        rest: vec![Id(a)],
                    }
                ]),
                contractum: CanonicalGraph::new(vec![
                    CanonicalNode {
                        id: m,
                        function: Succ,
                        rest: vec![Id(n)],
                    }, CanonicalNode {
                        id: n,
                        function: Add,
                        rest: vec![Id(a), Id(z)],
                    }
                ]),
                redirection: (x, m)
            },CanonicalRule {
                redex: CanonicalGraph::new(vec![
                    CanonicalNode {
                        id: x,
                        function: Start,
                        rest: vec![]
                    },
                ]),
                contractum: CanonicalGraph::new(vec![
                    CanonicalNode {
                        id: m,
                        function: Add,
                        rest: vec![Id(n), Id(o)],
                    }, CanonicalNode {
                        id: n,
                        function: Succ,
                        rest: vec![Id(o)],
                    }, CanonicalNode {
                        id: o,
                        function: Zero,
                        rest: vec![]
                    }
                ]),
                redirection: (x, m)
            },
        ]);

        assert!(true);
    }
}
