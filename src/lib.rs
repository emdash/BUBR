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


use std::rc::Rc;
use core::fmt::Debug;


struct Addr(u32);
struct Symbol(Rc<String>);


enum Node<T: Debug + Clone> {
    Const  {value: T},
    VarRef {name: String},
    Lambda {var: Addr, body: Addr},
    App    {f: Addr, x: Addr, cache: Option<Addr>},
}


enum Relation {
    LambdaBody,
    AppFunc,
    AppArg,
}


struct UpLink(Addr, Relation);


struct TermGraph<T: Debug + Clone> {
    nodes: Vec<Node<T>>,
    uplink: Vec<UpLink>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
