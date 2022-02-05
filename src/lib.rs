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
    Accept(Box<TermTree>),
    SubExpr(Box<ParseState>, Box<ParseState>),
    Unexpected(Token),
}


impl TermTree {
    pub fn lambda(arg: &str, body: Box<TermTree>) -> Box<TermTree> {
        Box::new(TermTree::Lambda {arg: arg.to_string(), body})
    }

    pub fn var(name: &str) -> Box<TermTree> {
        Box::new(TermTree::Var {name: name.to_string()})
    }

    pub fn app(func: Box<TermTree>, arg: Box<TermTree>) -> Box<TermTree> {
        Box::new(TermTree::App {func, arg})
    }

    pub fn parse(input: &[Token]) -> Option<Box<TermTree>> {
        type S = ParseState;

        let mut state = S::Start;

        for token in input {
            match Self::dispatch(state, &token) {
                S::Accept(tt) => {return Some(tt);},
                s             => {state = s;}
            }
        }

        None
    }

    pub fn dispatch(state: ParseState, token: &Token) -> ParseState {
        type S = ParseState;
        type T  = Token;

        match (state, token) {
            (S::Start,          T::Id(v))  => S::AppFunc(Self::var(v)),

            (S::AppFunc(f),     T::Id(v))  => S::Accept(Self::app(f, Self::var(v))),

            (S::LambdaArg,      T::Id(v))  => S::LambdaBody(v.to_string()),

            (S::LambdaBody(a),  T::Id(v))  => S::Accept(Self::lambda(&a, Self::var(v))),
            (s,  T::Lambda)                => S::SubExpr(Box::new(s), Box::new(S::LambdaArg)),

            // Catch-all for unexpected cases.
            (_, t) => S::Unexpected(t.clone()),
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


impl TermGraph {
    pub fn new() -> Self {
        TermGraph {
            nodes: HashMap::new(),
            relations: HashMap::new(),
            vars: HashMap::new()
        }
    }
}


/*
#[cfg(test)]
mod tests {
    use super::*;

    impl Value for i32 {}
    
    #[test]
    fn alloc() {
        let x = TermGraph::<i32>::new();
    }
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
