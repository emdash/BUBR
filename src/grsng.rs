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
    type Val: Value;     // Node data
    type Id: NodeId;     // A way of referring to a data node. Must be copy.
    type Var: Variable;  // A way of referring to a pattern node. Must be copy.
}


// Helper traits. This lets us re-use these bounds in the few places
// where we can't refer to T::Foo directly.
trait Value: Debug + Copy + SigmaRules {}
trait NodeId: Debug + Copy + PartialEq {}
trait Variable: Debug + Copy + PartialEq {}


/**
 * This trait maps vars to IDs for rule rewriting.
 */
trait Mapping<T: Types> {
    fn get(&self, var: T::Var) -> T::Val;
    fn merge(self, other: Self) -> Self;
    fn bind(var: T::Var, id: T::Id) -> Self;
}


/**
 * A term in canonical form is either a variable or a constant.
 */
#[derive(Debug, Copy, Clone)]
enum CanonicalTerm<ID: Copy, T: Types> {
    Var(ID),
    Const(T::Val),
}


/**
 * Abstract over mutable runtime data representations.
 */
trait DataGraph<T: Types> {
    type It: Iterator<Item=CanonicalTerm<T::Id, T>>;
    fn value<'a>(&self, id: T::Id) -> &'a T::Val;
    fn children(&self, id: T::Id) -> Self::It;
    fn insert(self, id: T::Id, func: T::Val, rest: Self::It) -> Self;
    fn root(&self) -> T::Id;
    fn gc(self) -> Self;
}


/**
 * Abstract over immutable runtime pattern representations.
 */
trait Pattern<T: Types> {
    type It: Iterator<Item=CanonicalTerm<T::Var, T>>;
    type Mp: Mapping<T>;
    fn value<'a>(&self, id: T::Var) -> &'a T::Val;
    fn children(&self, id: T::Var) -> Self::It;
    fn root(&self) -> T::Id;
    fn matches(&self, data: &impl DataGraph<T>) -> Option<Self::Mp> {
        None
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

/*
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

*/
