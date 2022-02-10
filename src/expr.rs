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

use core::iter::Iterator;
use core::fmt::Debug;
use crate::{Token, Types, SigmaRules};


/**
 * This ADT abstracts over classic lambda expression trees.
 *
 * In theory, all of computing fits into this datatype.
 *
 * What we're going for here is a correct-by-construction reference
 * implementation, against which we can compare the behavior of other
 * approaches.
 *
 * It's not supposed to be efficient, because LC generally isn't
 * efficient. But it *is* supposed to be *general*. I.e. it meant to
 * be "optimal for a given choice of `Val` and `Sym`, types for "sigma
 * terms" and "variable names", respectively.
 *
 * To go further than that, we'd need to abstract over memory
 * management as well. I'm still not sure how to do that.
 */
#[derive(Clone, Debug, PartialEq)]
pub enum Expr<T: Types> {
    Lambda(T::Sym, Box<Expr<T>>),
    Val(T::Val),
    Var(T::Sym),
    App(Box<Expr<T>>, Box<Expr<T>>)
}

#[derive(Debug)]
pub enum ParseError<T: Types> {
    Unexpected(Token<T>),
    Mismatched,
    Underflow,
    NotAVar,
    EOF
}

#[derive(Debug)]
pub enum ReduceError<T: Types> {
    NameCollision,
    NotApplicable,
    NotBetaReducible,
    NotSigmaReducible(<T::Val as SigmaRules>::Error)
}


type ParseResult<T> = core::result::Result<Box<Expr<T>>, ParseError<T>>;
type ReduceResult<T> = core::result::Result<Box<Expr<T>>, ReduceError<T>>;


impl<'a, T: 'a> Expr<T> where T: Types + Clone {

    /* These are just wrapers around constructors that take a
     * (possibly) borrowed value and implicitly copy (as required) and
     * Box::new() their argument.
     */
    pub fn val<B>(v: B) -> Box<Self>
    where B: Into<T::Val> {
        Box::new(Expr::Val(v.into()))
    }

    pub fn lambda<B>(arg: B, body: Box<Self>) -> Box<Self>
    where B: Into<T::Sym> {
        Box::new(Expr::Lambda(arg.into(), body))
    }

    pub fn var<B>(name: B) -> Box<Self>
    where B: Into<T::Sym> {
        Box::new(Expr::Var(name.into()))
    }

    pub fn apply(func: Box<Self>, arg: Box<Self>) -> Box<Self> {
        Box::new(Expr::App(func, arg))
    }

    /* Reduce an expression tree
     *
     * This performs one reduction pass over the tree. The result
     * itself might still be reducible (i.e., in the presence of
     * recursion).
     */
    pub fn reduce(self) -> ReduceResult<T> {
        match self {
            // We distinguish between beta and sigma reduction by
            // inspecting the function term. A lambda implies beta
            // reduction, while a value implies sigma reduction.
            Self::App(f, x) => match *f {
                Self::Lambda(a, b) => Ok(b.beta_reduce(a, x)?),
                Self::Val(v)       => Self::sigma_reduce(v, x),
                _                  => Err(ReduceError::NotApplicable)
            },
            _ => Err(ReduceError::NotBetaReducible)
        }
    }

    // Perform the substitution implied by the beta reduction.
    fn beta_reduce(self, var: T::Sym, exp: Box<Self>) -> ReduceResult<T> {
        match self {
            Self::Var(v)       if v == var => Ok(exp.clone()),
            Self::Lambda(a, _) if a == var => Err(ReduceError::NameCollision),
            Self::Lambda(a, b)             => Ok(Self::lambda(a, b.beta_reduce(var, exp)?)),
            Self::App(f, x)                => Ok(Self::apply(
                f.beta_reduce(var.clone(), exp.clone())?,
                x.beta_reduce(var, exp)?)),
            x                              => Ok(Box::new(x))
        }
    }

    // Sigma reduction is delegated to the external value type, T::Val
    fn sigma_reduce(func: T::Val, arg: Box<Self>) -> ReduceResult<T> {
        match *arg {
            Self::Val(x) => T::Val::apply(func, x)
                .map_or_else(
                    |e| Err(ReduceError::NotSigmaReducible(e)),
                    |v| Ok(Self::val(v))
                ),
            _ => {panic!("omg, multiple args! panic!");}
        }
    }


    pub fn parse(
        input: impl Iterator<Item = &'a Token<T>>
    ) -> ParseResult<T> {
        let mut stack: Vec<Box<Self>> = Vec::new();

        for token in input { match token {
            // XXX: suspicious use of clone.
            Token::Val(v) => stack.push(Self::val(v.clone())),
            Token::Id(s)  => stack.push(Expr::var(s.clone())),
            Token::Lambda => {
                let body = stack.pop().ok_or(ParseError::Underflow)?;
                let arg = stack.pop().ok_or(ParseError::Underflow)?;
                // XXX: suspicious suspicious move.
                if let Expr::Var(s) = *arg {
                    stack.push(Expr::lambda(s, body));
                } else {
                    return Err(ParseError::NotAVar);
                }
            },
            Token::Apply  => {
                let arg = stack.pop().unwrap();
                let func = stack.pop().unwrap();
                stack.push(Expr::apply(func, arg));
            }
        } }

        if stack.len() == 1 {
            Ok(stack.pop().ok_or(ParseError::Underflow)?)
        } else {
            // If we got here and there's not exactly one value on the
            // stack, the program is incomplete
            Err(ParseError::EOF)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /* This shows how to implement Types for this crate */
    #[derive(Clone, Debug, PartialEq)]
    struct MyTypes;

    impl Types for MyTypes {
        type Val = i32;
        type Sym = String;
    }

    impl SigmaRules for i32 {
        type Error = ();

        // applying one int to another is nonsense, so we leave the
        // default impls here.
    }

    type Tok = Token<MyTypes>;
    type Exp = Expr<MyTypes>;

    #[test]
    fn test_parse_simple0() {
        let got = Expr::parse(vec![
            Tok::id("x"),
            Tok::id("y"),
            Tok::Apply
        ].iter()).unwrap();

        let expected = Expr::apply(Expr::var("x"), Expr::var("y"));
        assert_eq!(got, expected);
    }

    #[test]
    fn test_parse_simple1() {
        let got = Expr::parse(vec![
            Tok::id("x"),
            Tok::id("y"),
            Tok::Lambda,
        ].iter()).unwrap();

        let expected = Expr::lambda("x", Expr::var("y"));
        assert_eq!(got, expected);
    }

    #[test]
    fn test_parse_simple2() {
        let got = Expr::parse(vec![
            Tok::id("x"),
            Tok::id("y"),
            Tok::Lambda,
            Tok::id("z"),
            Tok::Apply,
        ].iter()).unwrap();

        let expected = Expr::apply(
            Expr::lambda(
                "x".to_string(),
                Expr::var("y".to_string())
            ),
            Expr::var("z".to_string())
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn test_beta_reduction() {
        type E = Exp;

        // (\x.x) 0 -b-> 0
        assert_eq!(
            E::apply(E::lambda("x", E::var("x")), E::val(0)).reduce().unwrap(),
            E::val(0)
        );

        // (\x.(\y.x)) 0 -b-> (\y.0)
        assert_eq!(
            E::apply(
                E::lambda("x",
                          E::lambda("y",
                                    E::var("x"))),
                E::val(0)).reduce().unwrap(),
            E::lambda("y", E::val(0))
        );

        // (\f.f 0) (\x.x) -b-> (\x.x) 0 -b-> 0
        assert_eq!(
            E::apply(
                E::lambda("f", E::apply(E::var("f"), E::val(0))),
                E::lambda("x", E::var("x")))
                .reduce()
                .unwrap()
                .reduce()
                .unwrap(),
            E::val(0)
        )
    }

    /**
     * This section demonstrates extending the pure lambda calc with sigma rules.
     */
    #[derive(Clone, Debug, PartialEq)]
    struct SigmaTestTypes;

    #[derive(Clone, Debug, PartialEq)]
    enum BinOp {
        And,
        Or,
        Xor
    }

    // SigmaRules needs to form a category, where every operation maps
    // back into the category (modulo errors, if the result would be
    // nonsense).
    //
    // Note how the enum contains values, operations, and partial
    // operations.
    //
    // What's the mathematical term for this? Totality?
    //
    // One way to make this a little less bonkers would be to
    // distinguish between sigma functions and sigma values in the
    // Expr ADT.
    #[derive(Clone, Debug, PartialEq)]
    enum SigmaTestVal {
        // negation is the only unary operator here
        Not,
        // primative value
        Prim(bool),
        Binary(BinOp),
        // paritially applied binary operator
        Partial(BinOp, bool)
    }

    impl SigmaTestVal {
        fn binary(op: BinOp, x: bool, y: bool) -> Self {
            match op {
                BinOp::And => Self::Prim(x && y),
                BinOp::Or  => Self::Prim(x || y),
                BinOp::Xor => Self::Prim(x ^  y),
            }
        }
    }

    // Meaningful errors are optional, but you will thank yourself
    // when things go bananas. Extra credit if you can figure out how
    // to track source locations somehow.
    #[derive(Debug)]
    enum SigmaTestError {
        NotImplemented,
        NotABool,
        NotAnOperator,
        Arity
    }

    // Pick a reasonable default error.
    impl Default for SigmaTestError {
        fn default() -> Self { Self::NotImplemented }
    }

    // Implement Types trait on our enum
    impl Types for SigmaTestTypes {
        type Val = SigmaTestVal;
        type Sym = String;
    }

    // Implement sigma rules for our enum
    impl SigmaRules for SigmaTestVal {
        type Error = SigmaTestError;

        fn apply(f: Self, x: Self) -> Result<Self, Self::Error> {
            use SigmaTestVal::*;
            use SigmaTestError::*;
            // This is the key thing, binary operations return a
            // partial with their inner argument.
            // Only when we apply a partial to its second value do
            // we get a result.
            match (f, x) {
                (Not,                 Prim(x)) => Ok(Prim(!x)),
                (Not,                 _)       => Err(NotABool),
                (Binary(o),           Prim(x)) => Ok(Partial(o, x)),
                (Partial(o, v),       Prim(x)) => Ok(Self::binary(o, v, x)),
                (Prim(_),          _      )    => Err(NotAnOperator),
                _ => Err(NotImplemented),
            }
        }
    }

    #[test]
    fn test_sigma_reduction() {
        type E = Expr<SigmaTestTypes>;
        use SigmaTestVal::*;
        use BinOp::*;

        assert_eq!(
            E::apply(E::val(Not), E::val(Prim(true))).reduce().unwrap(),
            E::val(Prim(false))
        );

        assert_eq!(
            E::apply(E::val(Binary(And)), E::val(Prim(true))).reduce().unwrap(),
            E::val(Partial(And, true))
        );

        assert_eq!(
            E::apply(E::val(Partial(And, true)), E::val(Prim(true))).reduce().unwrap(),
            E::val(Prim(true))
        );

        /* This case is failing, because something isn't quite right
         * with the recursion.
         */
        assert_eq!(
            E::apply(
                E::apply(E::val(Binary(Xor)),
                         E::val(Prim(true))).reduce().unwrap(),
                E::val(Prim(true))).reduce().unwrap(),
            E::val(Prim(false))
        );

    }
}
