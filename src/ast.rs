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

/**
 * This module provides an abstract representation for constructing
 * concrete GRSs.
 */


/**
 * This module is the abstract syntax of a canonical GRS.
 *
 * Shorthand syntax gets flattened to this, and then used to construct
 * concrete implementations.
 */
pub mod canonical {
    use crate::grs::Types;
    pub struct Node <NodeId, Val>(NodeId, Val, Vec<NodeId>);
    pub struct Graph<NodeId, Val>(Vec<Node<NodeId, Val>>);
    pub type DataGraph<T: Types> = Graph<T::Id,  T::Val>;
    pub type Pattern  <T: Types> = Graph<T::Var, T::Val>;

    pub struct Rule<T: Types> {
        redex: Pattern<T>,
        contractum: Pattern<T>,
        redirection: (T::Var, T::Var)
    }

    pub struct GRS<T: Types>(Vec<Rule<T>>);
}


/**
 * This type is the abstract syntax for the "shorthand" GRS notation.
 *
 * It elides node IDs when they are clear from context, and allows to
 * nesting of nodes, as in a TRS. A simple algorithm flattens this to
 * canonical form.
 *
 * The main distinction between ShorthandForm and a TRS is that nodes
 * can be named when desired, which allows for cyclical patterns and
 * data.
 *
 * In the literature, the shorthand form is mainly intended for
 * patterns, but I can't justify going out of my way to restrict it to
 * patterns.
 */
mod shorthand {
    use crate::grs::Types;

    pub enum Node<NodeId, Val> {
        Empty,
        Anon(Vec<Arg<NodeId, Val>>),
        Labeled(NodeId, Vec<Arg<NodeId, Val>>)
    }

    pub enum Arg<NodeId, Val> {
        Ref(NodeId),
        // To make Rust happy and not be stuck with an obnoxious
        // PhantomData<Val> in this enum, I added this variant, even
        // though canonical form as I defined it doesen't support
        // "constructor position". This will be flattened as:
        // `Arg::SubTerm(Some(id), box![Node::Anon(val)])`
        Label(NodeId, Val),
        // or make the above Ref(Option<NodeId>, Option<Symbol>)
        SubTerm(Option<NodeId>, Box<Node<NodeId, Val>>)
    }

    pub struct Graph<NodeId, Val>(Vec<Node<NodeId, Val>>);
    pub type DataGraph<T: Types> = Graph<T::Id,  T::Val>;
    pub type Pattern  <T: Types> = Graph<T::Var, T::Val>;

    enum Rule<T: Types> {
        Reduce  (Pattern<T>, Pattern<T>),
        Redirect(Pattern<T>, (T::Var, T::Var)),
        ReduceAndRedirect(Pattern<T>, Pattern<T>, (T::Var, T::Var))
    }

    pub struct GRS<T: Types>(Vec<Rule<T>>);

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

}
