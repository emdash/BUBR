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
use crate::{Types};


/**
 * Now we're working on ch 4 of FPGR.
 *
 * This module is a toy term re-writing system (TRS).
 *
 * Unlike with the lambda calc, this would seem to map nicely into
 * Rust.
 */

/* ADT For Terms */
enum Term<T: Types> {
    Var(T::Sym),
    Const(T::Val),
    SubTerm(T::Val, Vec<Term<T>>)
}

/* ADT For rewrite rules */
struct Rule<T: Types>(
    T::Val, Vec<Term<T>>, // Left hand side
    Vec<Term<T>>          // Right hand side
);

/* With the above in hand, TRS is simply a list of rules. */
struct TermReductionSystem<T: Types>(Vec<Rule<T>>);


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SigmaRules, Types};

    // We can get away with a limited alphabet of identifiers for
    // tests. Thes are lower-case to match the literature. Variables
    // in rules are typically lower case, while constants are
    // CamelCase or just a capital letter.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Symbols {x, y}

    // We can get away with a limited set of "constant" values as
    // well.
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Values {If, True, False, Int(i8), F, G, W}

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
}
