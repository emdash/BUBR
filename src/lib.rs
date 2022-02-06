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
use std::collections::{HashMap};


/**
 * What follows is a hairbrained re-inrerpretation of both the SML
 * code listing and the formal algorithm presented in the PDF. 
 *
 * The algorithm, as presented, would be easier to implement in C/C++
 * or Java, or the unsafe subset of Rust.
 *
 * I gave up trying to circumvent the borrow checker. Instead I take a
 * more graph-theoretic approach.
 */


/**
 * This ADT conforms to the "conventional" lambda expression tree.
 */
#[derive(Clone, Debug)]
enum TermTree {
    Lambda {
        arg: String,
        body: Box<TermTree>,
    },

    Var {
        name: String,
    },

    App {
        func: Box<TermTree>,
        arg: Box<TermTree>
    }
}


/**
 * This is the flat format we produce and consume
 */
#[derive(Clone, Debug)]
enum Token {
    Id(String),
    Lambda,
    /*
    Open,
    Close,*/
}


#[derive(Clone, Debug)]
enum ParseState {
    Start,
    LambdaArg,
    LambdaBody(String),
    AppFunc(Box<TermTree>),
    Unexpected(Token),
}


impl TermTree {
    pub fn lambda(arg: String, body: Box<TermTree>) -> Box<TermTree> {
        Box::new(TermTree::Lambda {arg: arg, body})
    }

    pub fn var(name: String) -> Box<TermTree> {
        Box::new(TermTree::Var {name: name})
    }

    pub fn app(func: Box<TermTree>, arg: Box<TermTree>) -> Box<TermTree> {
        Box::new(TermTree::App {func, arg})
    }

    pub fn parse(input: impl Iterator<Item = Token>) -> Option<Box<TermTree>> {
        Self::next(ParseState::Start, input)
    }

    fn next(state: ParseState, mut input: impl Iterator<Item = Token>) -> Option<Box<TermTree>> {
        type S = ParseState;
        type T = Token;
        match (state, input.next()?) {
            (S::Start,         T::Id(v))  => Self::next(S::AppFunc(Self::var(v)), input),
            (S::Start,         T::Lambda) => Self::next(S::LambdaArg, input),
            (S::AppFunc(f),    T::Id(v))  => Some(Self::app(f, Self::var(v))),
            (S::AppFunc(f),    T::Lambda) => Some(Self::app(f, Self::next(S::LambdaArg, input)?)),
            (S::LambdaArg,     T::Id(v))  => Self::next(S::LambdaBody(v), input),
            (S::LambdaBody(a), T::Id(v))  => Some(Self::lambda(a, Self::var(v))),
            (S::LambdaBody(a), T::Lambda) => Some(Self::lambda(a, Self::next(S::LambdaArg, input)?)),
            _ => None
        }
    }
}


/**
 * There are three kinds of nodes: lambdas, var refs, and
 * applications. 
 */
enum Term {
    Lambda,
    VarRef(String),
    App
}


/**
 * These are the types of edges which can exist between two nodes.
 */
enum Relation {
    AppFunc,
    AppArg ,
    LambdaBody
}


/**
 * So the whole graph is a set of nodes, and a set of edges, plus some
 * extra book-keeping.
 */
struct TermGraph {
    nodes: HashMap<u32, Term>,
    relations: HashMap<(u32, u32), Relation>,
    vars: HashMap<String, u32>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_tree() {
        println!("{:?}", TermTree::parse(vec![
            Token::Id("x")
        ].into_iter()));
    }
}
