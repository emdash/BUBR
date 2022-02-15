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
    Arrow,
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
// your beautiful vision, forcing you to start refactoring
// ... unfortunately Rust is not always forgiving with refactoring,
// owing to the various asymmetries in the language.
//
// Anyway, it's better than a 3AM call.

pub fn parse_grs

pub fn parse_graph<Id, Val>(impl Iterator<Item=Token<C, I>>) -> Graph<Id, Val> {
    panic!("I'm not implemented");
}

pub fn 
