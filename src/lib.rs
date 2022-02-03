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


// Graphs are self-referential structures.
//
// We use this newtype to represent the internal references.
pub struct Ref(u32);


pub trait Value: Sized + Clone {}


pub enum Node<T: Value> {
    Const  {value: T},
    VarRef {name: String},
    Lambda {var: Ref, body: Ref},
    App    {f: Ref, x: Ref, cache: Option<Ref>},
}


enum Relation {
    LambdaBody,
    AppFunc,
    AppArg,
}


pub struct UpLink(Ref, Relation);


pub struct TermGraph<T: Value> {
    nodes: Vec<Node<T>>,
    uplink: Vec<UpLink>,
}


impl<T: Value> TermGraph<T> {
    pub fn new() -> Self {
        TermGraph {
            nodes: Vec::new(),
            uplink: Vec::new()
        }
    }

    pub fn fromTree
}


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
