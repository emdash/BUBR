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
 * What follows has really deviated significantly from the original
 * intent of this side-quest.
 */

use core::fmt::Debug;


/**
 * Should this be its own crate? Or a macro?
 */
pub fn debug<T: Debug>(prefix: &str, value: T) -> T {
    println!("{}: {:?}", prefix, value);
    value
}


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
 * A container for various trait bounds.
 *
 * This gives us some parametricity without having where clauses
 * proliferate everywhere.
 */
pub trait Types {
    // A type which represents a "constant" value in the lambda calc.
    type Val: Debug + Clone + SigmaRules;
    // A type which represents a "symbol" in the lambda calc, usually
    // String. But if you want to replace this with an integer, or a
    // custom type, you can.
    type Sym: Debug + Clone + PartialEq;
}


/**
 * This is the abstract I/O format: Flat token sequences which
 * represent a postfix encoding lambda calculus. Postfix is used here
 * for the usual reasons: it is unambiguous, compact, and trivial to
 * evaluate.
 *
 * V is the value type, for constant values. S is the "symbol" type,
 * for identifiers.
 *
 * Example: `\x.x` becomes the sequence `[Id("x"), Id("x"), Lambda]`,
 * if S is `&'static str`.
 */
#[derive(Clone, Debug)]
pub enum Token<T: Types> {
    Id(T::Sym),
    Val(T::Val),
    Lambda,
    Apply,
}

impl<T: Types> Token<T> {
    pub fn id<B>(name: B) -> Token<T> where B: Into<T::Sym> {
        Token::Id(name.into())
    }
}


/**
 * Just to get oriented, we start with a simple lambda expression
 * parser and evaluator.
 */
mod expr;
mod trs;
mod grs;
mod grsng;
