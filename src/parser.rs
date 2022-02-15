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


/**
 * This module implements a simple recursive-descent parser for the
 * short-hand AST notation.
 */
pub fn parse_graph<Id, Val>(impl Iterator<Item=Token<C, I>>) -> Graph<Id, Val> {
    panic!("I'm not implemented");
}
