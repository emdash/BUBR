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


// Graphs are self-referential structures.
//
// We use this newtype to represent the internal references.
#[derive(Copy, Clone, Debug)]
pub struct Ref(usize);

pub trait Value: Sized + Clone {}

#[derive(Copy, Clone, Debug)]
enum Relation {LambdaBody, AppFunc, AppArg}

#[derive(Copy, Clone, Debug)]
pub struct UpLink(Ref, Relation);

pub enum NodeType<T: Value> {
    Const  {value: T},
    VarRef {name: String},
    Lambda {var: Ref, body: Ref},
    App    {f: Ref, x: Ref}
}

pub struct Node<T: Value> {
    ty: NodeType<T>,
    cache: Option<Ref>,
    uplinks: Vec<UpLink>
}

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

    fn get_mut(&mut self, n: Ref, callback: impl FnOnce(&mut Node<T>) -> ()) {
        callback(&mut self.nodes[n.0]);
    }

    fn get(&self, n: Ref, callback: impl FnOnce(&Node<T>) -> ()) {
        callback(&self.nodes[n.0]);
    }
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
