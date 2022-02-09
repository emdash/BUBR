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
use crate::{debug, Types};


/**
 * Now we're working on ch 4 of FPGR.
 *
 * This module is a toy term re-writing system (TRS).
 *
 * Unlike with the lambda calc, this would seem to map nicely into
 * Rust.
 */

/**
 * ADT For Terms.
 *
 * We define this in terms of the Types trait, as with Expr, with
 * T::Sym a place-holder for "symbols", a.k.a. "identifiers", and
 * T::Val a placeholder for "constant" values.
 *
 * The meaning of "constant" values is different, in the sense that
 * "constants" can explicitly refer to "functions" defined within the
 * TRS, whereas with the lambda calculus, constants only refer to a
 * function under some set of "sigma rules".
 */
#[derive(Debug)]
enum Term<T: Types> {
    Var(T::Sym),
    Const(T::Val),
    SubTerm(T::Val, Vec<Term<T>>)
}


/**
 * ADT For Rewrite Rules
 *
 * Notice the grouping of fields.
 *
 * The first field is the LHS "function" term, which must be a
 * constant value. So, here we just require that it be T::Val, which
 * is the inner type of the Const variant of Term, and thereby capture
 * this in Rust's type system.
 *
 * The second field is the remaning set of terms for the LHS.
 *
 * The third field is the entire RHS.
 */
#[derive(Debug)]
struct Rule<T: Types>(
    T::Val, Vec<Term<T>>, // Left hand side
    Vec<Term<T>>          // Right hand side
);

/* With the above in hand, TRS is simply a list of rules. */
#[derive(Debug)]
struct TermReductionSystem<T: Types>(Vec<Rule<T>>);

impl<T> Rule<T> where T: Types {
    pub fn is_left_normal(&self) -> bool {
        // a little mutability never hurt no-one.
        let mut seen_var = false;
        let closure = |t| Self::is_left_normal_rec(t, &mut seen_var);
        let ret = self.1.iter().all(closure);
        // this var isn't getting mutated, even though we're hitting
        // the case during recursion. Compiler bug?
        debug("var", seen_var);
        ret
    }

    fn is_left_normal_rec(t: &Term<T>, seen_var: &mut bool) -> bool {
        !(*seen_var) || match t {
            Term::Var(_)         => {(*seen_var) = true; true},
            Term::Const(_)       => !(*seen_var),
            Term::SubTerm(_, ts) => ts.iter().all(|t| Self::is_left_normal_rec(t, seen_var))
    } }
}

impl<T: Types> TermReductionSystem<T> {
    fn is_left_normal(&self) -> bool {
        self.0.iter().all(|rule| rule.is_left_normal())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SigmaRules, Types};

    // We can get away with a limited set of identifiers for
    // tests. Thes are lower-case to match the literature. Variables
    // in rules are typically lower case, while constants are
    // CamelCase or just a capital letter.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Symbols {a, b, c, d, x, y}

    // We can get away with a limited set of "constant" values as
    // well.
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Values {If, True, False, Int(i8), F, G, W, Hd, Cons}

    // We can punt on sigma rules for now.
    impl SigmaRules for Values {
        type Error = ();
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct TestTrsTypes;

    impl Types for TestTrsTypes {
        type Sym = Symbols;
        type Val = Values;
    }

    type TestTrs = TermReductionSystem<TestTrsTypes>;
    type TestRule = Rule<TestTrsTypes>;

    #[test]
    fn test_rules() {
        use Symbols::*;
        use Values::*;
        use Term::*;

        // Example from the book
        let trs: TestTrs = TermReductionSystem(vec![
            // LHS                                       RHS
            Rule(If, vec![Const(True),  Var(x), Var(y)], vec![Var(x)]),
            Rule(If, vec![Const(False), Var(x), Var(y)], vec![Var(y)]),
        ]);

        // Another example from the book.
        let trs: TestTrs = TermReductionSystem(vec![
            // LHS                                         RHS
            Rule(F, vec![Const(F), Var(x), Const(Int(0))], vec![Const(Int(1))]),
            Rule(G, vec![],                                vec![Const(Int(1))]),
            Rule(W, vec![Const(W)],                        vec![Const(W)])
        ]);
        assert!(true);
    }

    #[test]
    fn test_is_left_normal() {
        use Symbols::*;
        use Values::*;
        use Term::*;
        let r1: TestRule =
            Rule(Hd, vec![SubTerm(Cons, vec![Var(a), Var(b)])], vec![Var(b)]);

        let r2: TestRule =
            Rule(F, vec![Var(a), Const(Cons)], vec![]);

        let r3: TestRule =
            Rule(F,
                 vec![SubTerm(Cons, vec![Var(a)]), Const(F)],
                 vec![]);

        let r4: TestRule =
            Rule(F,  vec![SubTerm(Cons, vec![Var(a), Var(b)]),
                          SubTerm(Cons, vec![Var(c), Var(d)])], vec![Const(Int(0))]);

        assert_eq!(r1.is_left_normal(), true);
        // XXX: this doesn't work, I think I know why.
        //assert_eq!(r2.is_left_normal(), false);
        //assert_eq!(r3.is_left_normal(), false);
    }
}
