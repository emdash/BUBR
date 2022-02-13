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


/**
 * Trait for operations external to pure lambda calculus.
 *
 * See tests for an examples of how this is used.
 */
pub trait SigmaRules: Sized {
    type Error: Sized + Debug + Default;

    fn apply(_f: Self, _x: Self) -> Result<Self, Self::Error> {
        Err(Self::Error::default())
    }
}


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
    fn new() -> Self;
    fn get(&self, var: T::Var) -> T::Id;
    fn bind(&mut self, var: T::Var, id: T::Id);
}


/**
 * Abstract over mutable runtime data representations.
 */
// XXX: I would have liked to just do:
// // fn args(&self, id: T::Id) -> impl Iterator<Item=T::Id>;
// but impl Trait is not allowed within a trait.
//
// The next thing I tried was this.
// // type It: impl Iterator<Item=T::Id>;
// // fn args(&self, id: T::Id) -> Self::It;
//
// This made the trait unimplementable, since there was no way to
// introduce the lifetime <'a> in the impl, since merely using it to
// specify the associated item It: core::slice::Iter<&'a, T> doesn't
// count...
//
// I googled around, and found this wierd pattern using HRTB:
// https://stackoverflow.com/questions/60459609/
//
// Shockingly this seems to type-check. I don't know if this pattern
// is sound, or idiomatic, or what. But it seems to solve the problem.
//
// I need to learn more about how HRTB work and see if this can be
// made simpler, or else abstracted into a hack.
//
// Another, simpler way of doing this would involve GATs, which are
// not yet stabilized, but should be soon.
//
// Still another interesting feature would be type-alias-impl-trait.
trait DataGraph<T: Types>: for <'a> DataGraphBody<'a, T> {}
trait DataGraphBody<'a, T: Types> {
    type It: Iterator<Item = T::Id>;
    fn new() -> Self;
    fn args(&'a self, id: T::Id) -> Self::It;
    fn value(&'a self, id: T::Id) -> T::Val;
    fn alloc(&'a mut self, func: T::Val) -> T::Id;
    fn append_arg(&'a mut self, id: T::Id, arg: T::Id);
    fn redirect(&'a mut self, src: T::Id, dst: T::Id);
    fn root(&'a self) -> T::Id;
    fn gc(&'a mut self) {}
}


/**
 * Abstract over immutable runtime pattern representations.
 */
trait Pattern<T: Types>: for <'a> PatternBody<'a, T> {}
trait PatternBody<'a, T: Types> {
    type It: Iterator<Item=T::Var>;

    fn contains(&'a self, id: T::Var) -> bool;
    fn value(&'a self, id: T::Var) -> T::Val;
    fn args(&'a self, id: T::Var) -> Self::It;
    fn root(&'a self) -> T::Var;

    fn matches(
        &'a self,
        redex: T::Var,
        data: &impl DataGraph<T>,
        node: T::Id,
        mapping: &mut impl Mapping<T>,
    ) -> Option<()> {
        println!("enter: {:?}, {:?}", redex, node);

        let redex_value = self.value(redex);
        let node_value = data.value(node);

        if redex_value == node_value {
            println!("bind: {:?} -> {:?}", redex, node);
            mapping.bind(redex, node);
            let iter = self.args(redex).zip(data.args(node));
            for (var, id) in iter {
                println!("bind-rec: {:?}, {:?}", var, id);
                if self.contains(var) {
                    self.matches(var, data, id, mapping)?;
                } else {
                    mapping.bind(var, id);
                }
                 println!("recurse-done {:?}", mapping);
            }
            println!("success: {:?}", mapping);
            Some(())
        } else {
            println!("fail: {:?} != {:?}", redex_value, node_value);
            None
        }
    }

    fn rewrite(
        &'a self,
        contractum: T::Var,
        data: &mut impl DataGraph<T>,
        mapping: &impl Mapping<T>
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
     * mapping of variables to node ids.
     */
    pub fn matches<M: Mapping<T>>(&self, data: &impl DataGraph<T>) -> Option<M> {
        let mut m = M::new();
        if let Some(()) = self.redex.matches(
            self.redex.root(),
            data,
            data.root(),
            &mut m
        ) {
            Some(m)
        } else {
            None
        }
    }

    pub fn rewrite<M: Mapping<T>>(&self, data: &mut impl DataGraph<T>) -> bool {
        let map: Option<M> = self.matches(data);
        if let Some(mapping) = map {
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


type CanonicalForm<Id, Val> = (Id, Val, [Id]);


// Leaving this here for now, we'll get to it soon enough.
/*
macro_rules! node {
    ($id:expr ; $func:expr) => {($id, $func, [])};
    ($id:expr ; $func:expr, $( $rest:expr ),+ ) => {
        ($id, $func, vec![$($rest),*])
    }
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
}*/


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Demonstration of BYOT (Bring Your Own Types)
    //
    // We can get away with a limited set of identifiers for
    // tests. Thes are lower-case to match the literature, where
    // pattern variables typically lower case, while constants are
    // CamelCase or just a single capital letter.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    enum Symbol {a, b, c, d, m, n, o, x, y, z}

    // We can get away with a limited set of "constant" values as
    // well.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum Value {Start, Add, If, True, False, Int(i8), Zero, Succ, Hd, Cons}

    impl SigmaRules for Value {
        type Error = ();
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct TestTypes;

    impl Types for TestTypes {
        type Var = Symbol;
        type Val = Value;
        type Id  = u8;
    }

    impl<'a> DataGraphBody<'a, TestTypes> for Vec<(Value, Vec<u8>)> {
        type It = core::iter::Copied<core::slice::Iter<'a, u8>>;

        fn new() -> Self { Vec::new() }

        fn value(&'a self, id: u8) -> Value {
            self[id as usize].0
        }

        fn args(&'a self, id: u8) -> Self::It {
            self[id as usize].1.iter().copied()
        }
        fn alloc(&'a mut self, func: Value) -> u8 {
            self.push((func, Vec::new()));
            let len = self.len();
            if len == 256 {
                panic!("storage exhausted");
            }
            (len - 1) as u8
        }

        fn append_arg(&'a mut self, id: u8, arg: u8) {
            self[id as usize].1.push(arg);
        }

        fn redirect(&'a mut self, src: u8, dst: u8) {
            self.swap(src as usize, dst as usize)
        }

        fn root(&'a self) -> u8 { 0 }
    }

    impl Mapping<TestTypes> for HashMap<Symbol, u8> {
        fn new() -> Self { HashMap::new() }
        fn get(&self, var: Symbol) -> u8 { self[&var] }
        fn bind(&mut self, var: Symbol, id: u8) {
            self.insert(var, id);
        }
    }

    impl<'a> PatternBody<'a, TestTypes>
        for (HashMap<Symbol, (Value, Vec<Symbol>)>, Symbol)
    {
        type It = core::iter::Copied<core::slice::Iter<'a, Symbol>>;

        fn contains(&'a self, id: Symbol) -> bool {
            self.0.contains_key(&id)
        }

        fn value(&'a self, id: Symbol) -> Value {
            self.0[&id].0
        }

        fn args(&'a self, id: Symbol) -> Self::It {
            self.0[&id].1.iter().copied()
        }

        fn root(&'a self) -> Symbol { self.1 }
    }

    #[test]
    fn test_grs() {
    }
}
