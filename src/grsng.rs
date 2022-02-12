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
 * This is the "type context" for the data-structures and algorithms
 * in this crate.
 *
 * Using this to factor out a ton of repeated trait bounds and where
 * clauses into one place.
 */
trait Types {
    // The data contained within a node.
    type Val: Debug + Copy + PartialEq + SigmaRules;
    // Id of nodes in a data graph
    type Id: Debug + Copy + PartialEq;
    // Id of nodes in a pattern.
    type Var: Debug + Copy + PartialEq;
}


/**
 * This trait maps vars to IDs for rule rewriting.
 */
trait Mapping<T: Types>: Debug {
    fn get(&self, var: T::Var) -> T::Id;
    fn merge(self, other: Self) -> Self;
    fn bind(var: T::Var, id: T::Id) -> Self;
}


/**
 * Abstract over mutable runtime data representations.
 */
trait DataGraph<T: Types> {
    type It: Iterator<Item=T::Id>;
    fn value(&self, id: T::Id) -> T::Val;
    fn args(&self, id: T::Id) -> Self::It;
    fn alloc(&mut self, func: T::Val) -> T::Id;
    fn append_arg(&mut self, id: T::Id, arg: T::Id);
    fn redirect(&mut self, src: T::Id, dst: T::Id);
    fn root(&self) -> T::Id;
    fn gc(&mut self) -> Self;
}


/**
 * Abstract over immutable runtime pattern representations.
 */
trait Pattern<T: Types> {
    type It: Iterator<Item=T::Var>;
    type Mp: Mapping<T>;

    fn contains(&self, id: T::Var) -> bool;
    fn value<'a>(&self, id: T::Var) -> T::Val;
    fn args(&self, id: T::Var) -> Self::It;
    fn root(&self) -> T::Var;

    fn matches(
        &self,
        redex: T::Var,
        data: &impl DataGraph<T>,
        node: T::Id,
    ) -> Option<Self::Mp> {
        println!("enter: {:?}, {:?}", redex, node);

        let redex_value = self.value(redex);
        let node_value = data.value(node);

        if redex_value == node_value {
            println!("bind: {:?} -> {:?}", redex, node);
            let mut mapping = Self::Mp::bind(redex, node);
            let iter = self.args(redex).zip(data.args(node));
            for (var, id) in iter {
                println!("bind-rec: {:?}, {:?}", var, id);
                if self.contains(var) {
                    mapping = mapping.merge(self.matches(var, data, id)?);
                } else {
                    mapping = Self::Mp::bind(var, id);
                }
                println!("recurse-done {:?}", mapping);
            }
            println!("success: {:?}", mapping);
            Some(mapping)
        } else {
            println!("fail: {:?} != {:?}", redex_value, node_value);
            None
        }
    }

    fn rewrite(
        &self,
        contractum: T::Var,
        data: &mut impl DataGraph<T>,
        mapping: &Self::Mp
    ) -> T::Id {
        let id = data.alloc(self.value(contractum));
        for var in self.args(contractum) {
            if self.contains(var) {
                let arg_id = self.rewrite(var, data, mapping);
                data.append_arg(id, arg_id);
            } else {
                data.append_arg(id, mapping.get(var))
            }
        }
        id
    }
}


/**
 * A rewrite rule in canonical form.
 */
struct CanonicalRule<T: Types, P: Pattern<T>> {
    redex: P,
    contractum: P,
    redirection: (T::Var, T::Var)
}

impl<T: Types, P: Pattern<T>> CanonicalRule<T, P> {
    /**
     * If a rule matches the subgraph rooted at `node`, return the
     * mapping of variables to node ideas..
     */
    pub fn matches(&self, data: &impl DataGraph<T>) -> Option<P::Mp> {
        self.redex.matches(self.redex.root(), data, data.root())
    }

    pub fn rewrite(&self, data: &mut impl DataGraph<T>) -> bool {
        if let Some(mapping) = self.matches(data) {
            self.contractum.rewrite(self.contractum.root(), data, &mapping);
            // XXX: this is an extra step, which ideally we could
            // avoid by directly writing into the redirection node.
            //
            // XXX: not clear we even need redirections given a
            // functional strategy.
            data.redirect(
                mapping.get(self.redirection.0),
                mapping.get(self.redirection.1)
            );
            true
        } else {
            false
        }
    }
}

/*
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
*/


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

    impl DataGraph<TestTypes> for ([(Values, Vec<u8>); 256], u8) {
        type It = std::slice::Iter<&'_, u8>;
        fn value(&self, id: u8) -> Values { self.0[id as usize].0 }
        fn args(&self, id: u8)  -> Self::It { self.0[id as usize].1.iter() }
        fn alloc(&mut self, func: Values) -> u8 {
            let ret = self.1;
            self.1 +=1;
            if self.1 == 0 {
                panic!("graph store full!");
            }
            ret
        }

        fn append_arg(&mut self, id: u8, arg: u8) {
            self.0[id as usize].1.push(arg);
        }

        fn redirect(&mut self, src: u8, dst: u8) {
            self.swap(src as usize, dst as usize)
        }

        fn root(&self) -> u8 { 0 }

        fn gc(self) { println!("Gc, ... right..."); }
    }

}
    /*
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

*/
