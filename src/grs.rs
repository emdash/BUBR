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
use core::fmt::Debug;
use crate::SigmaRules;


/**
 * Now we're working on ch 5 of FPGR.
 *
 * This module is a toy graph re-writing system (GRS).
 */


/**
 * ADT for canonical form GRS
 */
#[derive(Debug)]
enum CanonicalTerm<ID: Copy + Debug, V: Debug> {
    Id(ID),
    Constructor(V),
}


/**
 * A node in a canonical graph.
 */
#[derive(Debug)]
struct CanonicalNode<ID: Copy + Eq + Hash + Debug, V: Debug> {
    id: ID,
    function: V,
    rest: Vec<CanonicalTerm<ID, V>>
}


/**
 * A canonical graph is an ordered set of nodes.
 */
#[derive(Debug)]
struct CanonicalGraph<ID: Copy + Eq + Hash + Debug, V: Debug> {
    ordering: Vec<ID>,
    mapping: HashMap<ID, CanonicalNode<ID, V>>
}


/**
 * This is like the one in lib.rs, but we add some extra bounds, and
 * one extra associated type.
 */
trait Types {
    type Val: Debug + Clone + SigmaRules + PartialEq + Eq;
    type Id: Debug + Clone + Eq + Copy + Hash + PartialEq;
    type Var: Debug + Clone + Eq + Copy + Hash;
}


struct NodeRef <'a, ID, T: Types>(&'a CanonicalGraph<ID, T::Val>, ID)
where ID: Debug + Clone + Eq + Copy + Hash + PartialEq;


/**
 * Some helpful type aliases to keep function signatures clean.
 *
 * We deliberately distinguish between pattern nodes and data nodes
 * at the type level.
 */
type DataNode   <    T: Types> = CanonicalNode <    T::Id,  T::Val>;
type DataRef    <'a, T: Types> = NodeRef       <'a, T::Id, T>;
type DataGraph  <    T: Types> = CanonicalGraph<    T::Id,  T::Val>;
type PatternNode<    T: Types> = CanonicalNode <    T::Var, T::Val>;
type PatternRef <'a, T: Types> = NodeRef       <'a, T::Var, T>;
type Pattern    <    T: Types> = CanonicalGraph<    T::Var, T::Val>;
type Mapping    <    T: Types> = HashMap       <    T::Var, T::Id>;


trait DataNodeTrait<T: Types> {
    fn id(&self) -> T::Id;
    fn redirect(&mut self, redex: T::Id, contractum: T::Id);
    fn insert(&mut self, new: DataNode<T>);
}

/*

trait PatternNodeTrait<T: Types> {
    fn matches<'a>(&'a self, data: DataRef<'a, T>) -> Option<Mapping<T>>;
    fn rewrite(&self, mapping: Mapping<T>) -> DataNode<T>;
}*/


impl<ID: Copy + Eq + Hash + Debug, V: Debug> CanonicalGraph<ID, V> {
    pub fn new(nodes: Vec<CanonicalNode<ID, V>>) -> Self {
        let mut ordering = Vec::new();
        let mut mapping = HashMap::new();

        for node in nodes {
            ordering.push(node.id);
            mapping.insert(node.id, node);
        }

        CanonicalGraph { ordering, mapping }
    }

    pub fn root(&self) -> ID { self.ordering[0] }

    pub fn get(&self, id: ID) -> Option<&CanonicalNode<ID, V>> {
        self.mapping.get(&id)
    }
}


/**
 * A rewrite rule in canonical form.
 */
struct CanonicalRule<T: Types> {
    redex: CanonicalGraph<T::Var, T::Val>,
    contractum: CanonicalGraph<T::Var, T::Val>,
    redirection: (T::Var, T::Var)
}


impl<T: Types> CanonicalRule<T> {
    /**
     * If a rule matches the subgraph rooted at `node`, return the
     * mapping of variables to node ideas..
     */
    pub fn matches(&self, data: &DataGraph<T>, data_root: T::Id) -> Option<Mapping<T>> {
        self.matches_rec(self.redex.root(), data, data_root, HashMap::new())
    }

    // Recursive implementation of above, to avoid forcing client code
    // to explicitly pass the mapping.
    fn matches_rec(
        &self,
        pattern_root: T::Var,
        data: &DataGraph<T>,
        data_root: T::Id,
        mapping: Mapping<T>
    ) -> Option<Mapping<T>> {
        use CanonicalNode as N;
        use CanonicalTerm::*;

        println!("enter: {:?}, {:?}", pattern_root, data_root);

        let mut mapping = mapping;
        let N {function: x, id: var, rest: sp} = self.redex.get(pattern_root)?;
        let N {function: y, id: id,  rest: sd} = data.get(data_root)?;

        println!("check: {:?}, {:?}", x, y);

        if x == y {
            println!("bind: {:?}, {:?}", *var, *id);
            mapping.insert(*var, *id);
            // loop over the rest of the pattern
            for (pat, node) in sp.iter().zip(sd.iter()) { match (pat, node) {
                (Id(v), Id(i)) => {
                    println!("bind-rec: {:?}, {:?}", *v, *i);

                    if let Some(_) = self.redex.get(*v) {
                        // ensure non-empty sub-terms also match.
                        // don't bind before-hand, matches will do this for us.
                        mapping = self.matches_rec(*v, data, *i, mapping)?;
                    } else {
                        // just bind the id in the mapping.
                        mapping.insert(*v, *i);
                    }

                    println!("recurse-done {:?}", mapping);
                },

                // Ensure two constructors are the same.
                (Constructor(x), Constructor(y)) if x == y => (),

                // Early return if any subterms fail to match
                (x, y) => {
                    println!("fail: {:?} != {:?}", x, y);
                    return None
                }
                //mapping = matches_term(pat, node, mapping)?;
            } }

            println!("success: {:?}", mapping);
            Some(mapping)
        } else {
            println!("fail: {:?} != {:?}", x, y);
            None
        }
    }

    /*
    pub fn rewrite(&self, data: DataGraph<T>) -> DataGraph<T> {
        if let Some(mapping) = self.matches((&data, data.root())) {
            let rewritten = self.rewrite_rec(data, data.root(), &mapping);
            data.set(data.root()) 
        } else {
            data
        }
    }*/
}


struct CanonicalGRS<T: Types>(Vec<CanonicalRule<T>>);


macro_rules! node {
    ($id:expr ; $func:expr) => {
        CanonicalNode {
            id: $id,
            function: $func,
            rest: vec![]
        }
    };

    ($id:expr ; $func:expr, $( $rest:expr ),+ ) => {
        CanonicalNode {
            id: $id,
            function: $func,
            rest: vec![$($rest),*]
        }
    };
}

macro_rules! graph {
    ($($nodes:expr),*) => {CanonicalGraph::new(vec![$($nodes),*])};
}

macro_rules! rule {
    ($redex:expr => $contractum:expr ; $red:expr => $con:expr) => {
        CanonicalRule {
            redex: $redex,
            contractum: $contractum,
            redirection: ($red, $con)
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    // We can get away with a limited set of identifiers for
    // tests. Thes are lower-case to match the literature. Variables
    // in rules are typically lower case, while constants are
    // CamelCase or just a capital letter.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    enum Symbols {a, b, c, d, m, n, o, x, y, z}

    // We can get away with a limited set of "constant" values as
    // well.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum Values {Start, Add, If, True, False, Int(i8), Zero, Succ, Hd, Cons}

    // We can punt on sigma rules for now.
    impl SigmaRules for Values {
        type Error = ();
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct TestTypes;

    impl Types for TestTypes {
        type Var = Symbols;
        type Val = Values;
        type Id  = u8;
    }

    type TestGrs = CanonicalGRS<TestTypes>;

    #[test]
    fn test_rules() {
        use Symbols::*;
        use Values::*;
        use CanonicalTerm::*;

        // example from the book:
        //
        // x: Add y z
        // z: Zero    -> x := z
        //
        // x: Add y z,
        // y: Succ a  -> m: Succ n, n: Add a z, x := m
        //
        // x: Start   -> m: Add n o, n: Succ o, o: Zero, x := m
        let grs: TestGrs = CanonicalGRS(vec![
            rule! {
                graph! {
                    node! {x; Add, Id(y), Id(z)},
                    node! {z; Zero}
                } => graph! {}; x => z
            }, rule! {
                graph! {
                    node! {x; Add,  Id(y), Id(z)},
                    node! {y; Succ, Id(a)}
                } => graph! {
                    node! {m; Succ, Id(n)},
                    node! {n; Add,  Id(a), Id(z)}
                }; x => m
            }, rule!{
                graph! {
                    node!(x; Start)
                } => graph!{
                    node!(m; Add,  Id(n), Id(o)),
                    node!(n; Succ, Id(o)),
                    node!(o; Zero)
                }; x => m
            },
        ]);


        let data = graph![
            node!(0; Add, Id(1), Id(1) ),
            node!(1; Zero)
        ];

        assert_eq!(
            grs.0[0].matches(&data, data.root()),
            Some(HashMap::from([
                (x, 0),
                (y, 1),
                (z, 1)
            ]))
        );

        assert_eq!(
            grs.0[1].matches(&data, data.root()),
            None
        );
    }
}
