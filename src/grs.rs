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
use core::fmt::Debug;
use crate::{debug, Types};


/**
 * Now we're working on ch 5 of FPGR.
 *
 * This module is a toy graph re-writing system (GRS).
 */


/**
 * ADT for canonical form GRS
 */
enum CanonicalTerm<ID, V> {
    Id(ID),
    Constructor(V),
}


/**
 * A node in a canonical graph.
 */
struct CanonicalNode<ID, V> {
    id: ID,
    function: V,
    rest: Vec<CanonicalTerm<ID, V>>
}


/**
 * A canonical graph is an ordered set of nodes.
 */
struct CanonicalGraph<ID, V>(Vec<CanonicalNode<ID, V>>);


/**
 * A rewrite rule in canonical form.
 *
 * Notice here we preserve the ID type parameter, since client code
 * might wish control over node id type.
 *
 * We could add an Id field to types, but this requires every
 * implementation define this type, even if they don't use it.
 */
struct CanonicalRule<T: Types> {
    redex: CanonicalGraph<T::Sym, T::Val>,
    contractum: CanonicalGraph<T::Sym, T::Val>,
    redirection: (T::Sym, T::Sym)
}


/**
 * A Graph Rewriting System (GRS) is an ordered set of rules.
 */
struct CanonicalGRS<T: Types>(Vec<CanonicalRule<T>>);


/**
 * ADT for the data on which a GRS operates.
 */
struct DataGraph<ID, T: Types>(CanonicalGraph<ID, T::Val>);


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SigmaRules, Types};

    // We can get away with a limited set of identifiers for
    // tests. Thes are lower-case to match the literature. Variables
    // in rules are typically lower case, while constants are
    // CamelCase or just a capital letter.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialEq)]
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
                redex: CanonicalGraph(vec![
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
                contractum: CanonicalGraph(vec![]),
                redirection: (x, z)
            }, CanonicalRule {
                redex: CanonicalGraph(vec![
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
                contractum: CanonicalGraph(vec![
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
                redex: CanonicalGraph(vec![
                    CanonicalNode {
                        id: x,
                        function: Start,
                        rest: vec![]
                    },
                ]),
                contractum: CanonicalGraph(vec![
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
