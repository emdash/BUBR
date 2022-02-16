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

use crate::debug;
use crate::ast::shorthand::*;
use crate::grs::Types;

/**
 * This module implements a simple recursive-descent parser for the
 * short-hand AST notation. The grammar here is based on that
 * presented in:
 *
 * https://clean.cs.ru.nl/Functional_Programming_and_Parallel_Graph_Rewriting
 *
 * Which grammar (suitably adjusted) is reproduced here, for our
 * mutual benefit.
 *
 * Rule              = RedexPattern -> ContractumPattern [",", Redirection]
 *                   | RedexPattern -> Redirection;
 * RedexPattern      = Graph;
 * Contractumpattern = Graph;
 * Graph             = [Nodeid ':'] Node {',' NodeDef};
 * Nodeid            = ... // See note below.
 * Node              = Symbol {Arg}   | EmptyNode;
 * Symbol            = Constant;
 * Arg               = Nodeid
 *                   | [Nodeid ':'] Symbol
 *                   | [Nodeid ':'] '(' Node ')';
 * EmptyNode         = 'nil';
 * NodeDef           = Nodeid ':' Node;
 * Redirection       = Nodeid ':=' Nodeid | Nodeid;
 *
 * Notes: the grammar here is certainly not what you will find below,
 * for two reasons: (1) This crate is making a type-level distinction
 * between patterns and data, and I want to factor things such that
 * the common machinery can be re-used for both cases; (2) It doesn't
 * seem quite right to me. Either I'm mis-understanding it, or some
 * truly wierd cases are allowed, like `Nodeid : Nodeid : Nodeid :
 * Nodeid ....`, and that doesn't seem like something I want to allow.
 *
 * P.S. All of the above was written before any of the actual
 * implementation, so it could be way off.
 */

pub enum Token<Id, Val> {
    ArrowShaft,
    ArrowTip,
    Comma,
    Colon,
    Open,
    Close,
    NodeId(Id),
    Symbol(Val),
    Empty,
    Redirect
}

// XXX: when the last rule is implemented and tested, delete this
// comment.
//
// My usual rule with parsers is to start with the simplest
// productions, and build upwards. But usually I am also writing the
// grammar as I go, whereas in this case the grammar is given.
//
// So instead I'm going to stub out the whole grammar, and then fill
// in the terms in whatever order ends up being the easiest.

// As an aside, one thing I am slowly learning about Rust: the right
// order of code is to start from the outside (API surface), and work
// inwards towards implementation. There is one pitfall with this
// approach, but it's a whopper: You still need to write tests. But
// not for the reasons you might think. The tests aren't so much about
// the code you run, but rather the code you can't run.
//
// You see, while Rust aims to provide fail-fast typechecking, if you
// don't exercise the code paths, you may end up defining an API whose
// traits are "valid" in the sense that they parse, yet are
// *unimplementable* in practice, especially once lifetimes start
// spreading through the code. The compiler can't catch this, at least
// not yet. Or maybe I should start using clippy?
//
// This means you have to write some trivial functions to just *call*
// the code you're writing, at which point the compiler often crushes
// your beautiful vision. You want this to happen as early as possible.
//
// At least it's better than a 3AM call.


pub fn parse_grs<T: Types>(input: impl Iterator<Item=Token<T::Var, T::Val>>) -> GRS<T> {
    // it gets boring writing "NotImplemented" over and over, so I'm
    // inserting Ralf Wiggum quotes.
    panic!("I'm unpossible!");
}

pub fn parse_data<T: Types>(input: impl Iterator<Item=Token<T::Id, T::Val>>) -> DataGraph<T> {
    panic!("foobar");
}

pub fn parse_rule<T: Types>(input: impl Iterator) -> GRS<T> {
    panic!("I'm happy *AND ANGRY!*!");
}

pub fn parse_pattern<T: Types>(input: impl Iterator) -> Pattern<T> {
    panic!("It tastes like burning!"); // ralphs quotes are dark :/
}

pub fn parse_graph<Id, Val>(input: impl Iterator<Item=Token<Id, Val>>) -> Graph<Id, Val> {
    panic!("I can do a summersault!");
}

pub fn parse_node<Id, Val>(input: impl Iterator<Item=Token<Id, Val>>) -> Node<Id, Val> {
    panic!("");
}

pub fn parse_arg<Id, Val>(input: impl Iterator<Item=Token<Id, Val>>) -> Arg<Id, Val> {
    panic!("");
}

pub fn parse_node_def<Id, Val>(input: impl Iterator<Item=Token<Id, Val>>) -> Node<Id, Val> {
    panic!("");
}

pub fn parse_redirection<T: Types>(input: impl Iterator<Item=Token<T::Var, T::Val>>) -> Rule<T> {
    panic!("");
}

// Terminals

pub fn parse_node_id<Id, Val>(input: impl Iterator<Item=Token<Id, Val>>) -> Id {
    panic!("Running out of ralph wiggum quotes");
}

pub fn parse_empty_node<Id, Val>(input: impl Iterator<Item=Token<Id, Val>>) -> Id {
    panic!("");
}


mod lexer {
    use super::Token;
    use core::marker::PhantomData;

    enum State {
        Start,
        Symbol(String),
        NodeId(String),
    }

    enum Action<Id, Val> {
        Next(State),
        EmitOne(Token<Id, Val>, State),
        // When an operator ends a word.
        EmitTwo(Token<Id, Val>, Token<Id, Val>),
        Unexpected(char)
    }

    enum CharType<Id, Val> {
        Whitespace,
        Operator(Token<Id, Val>),
        SymbolStart,
        SymbolChar
    }

    pub struct SimpleLexer<Id, Val, I>(
        I,
        State,
        PhantomData<(Id, Val)>
    ) where I: Iterator<Item=char>;

    impl<Id, Val, I> SimpleLexer<Id, Val, I>
    where Id: From<String>,
          Val: From<String>,
          I: Iterator<Item=char>
    {
        pub fn new(input: I) -> Self {
            SimpleLexer(input, State::Start, PhantomData)
        }

        fn push(s: String, c: char) -> String {
            let mut s = s;
            s.push(c);
            s
        }

        fn sym(s: String) -> Token<Id, Val> {
            Token::Symbol(Val::from(s))
        }

        fn id(s: String) -> Token<Id, Val> {
            Token::NodeId(Id::from(s))
        }

        fn pushs(s: String, c: char) -> State {
            State::Symbol(Self::push(s, c))
        }

        fn pushi(s: String, c: char) -> State {
            State::NodeId(Self::push(s, c))
        }

        fn classify(c: char) -> CharType<Id, Val> { match c {
            ' '                     => CharType::Whitespace,
            '\n'                    => CharType::Whitespace,
            '\r'                    => CharType::Whitespace,
            '\t'                    => CharType::Whitespace,
            '-'                     => CharType::Operator(Token::ArrowShaft),
            '>'                     => CharType::Operator(Token::ArrowTip),
            '('                     => CharType::Operator(Token::Open),
            ')'                     => CharType::Operator(Token::Close),
            ':'                     => CharType::Operator(Token::Colon),
            '_'                     => CharType::Operator(Token::Empty),
            '='                     => CharType::Operator(Token::Redirect),
            'x' if c.is_uppercase() => CharType::SymbolStart,
             _                      => CharType::SymbolChar
        } }

        fn lex(&mut self, c: char) -> Action<Id, Val> {
            use Action::*;
            use CharType::*;
            use State::*;
            use core::mem::replace;
            // use Self::*;
            match (replace(&mut self.1, State::Start), Self::classify(c)) {
                (Start,     Whitespace)    => Next(                      Start),
                (Start,     Operator(tok)) => EmitOne(tok,               Start),
                (Start,     SymbolStart)   => Next(      Symbol(String::new())),
                (Start,     SymbolChar)    => Next(      NodeId(String::new())),

                (Symbol(k), Whitespace)    => EmitOne(Self::sym(k),      Start),
                (Symbol(k), Operator(tok)) => EmitTwo(Self::sym(k), tok),
                (Symbol(k), SymbolStart)   => Next(   Symbol(Self::push(k, c))),
                (Symbol(k), SymbolChar)    => Next(   Symbol(Self::push(k, c))),

                (NodeId(k), Whitespace)    => EmitOne(Self::id(k),       Start),
                (NodeId(k), Operator(tok)) => EmitTwo(Self::id(k), tok),
                (NodeId(k), SymbolStart)   => Next(   NodeId(Self::push(k, c))),
                (NodeId(k), SymbolChar)    => Next(   NodeId(Self::push(k, c))),
            }
        }
    }

    impl<Id, Val, I> Iterator for SimpleLexer<Id, Val, I>
    where Id: From<String>,
          Val: From<String>,
          I: Iterator<Item=char>
    {
        type Item=Token<Id, Val>;

        fn next(&mut self) -> Option<Self::Item> {
            /*
            if let State::Pending(tok) = self.1 {
                self.1 = State::start;
                return Some(tok);
            }*/

            while let Some(character) = self.0.next() {
                match self.lex(character) {
                    Action::Next(s)         => {self.1 = s;},
                    Action::EmitOne(t, s)   => {self.1 = s; return Some(t);},
                    Action::EmitTwo(t1, t2) => {self.1 = State::Start /* Pending(t2)*/; return Some(t1);},
                    Action::Unexpected(c)   => {panic!("unexpected input {:?}");}
                }
            }
            return None
        }
    }
}
