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
 * What follows is a port of the SML code listing in the PDF.
 *
 * - Each record becomes a struct
 * - Except when it is a variant, it becomes an enum
 * - ChildCell becomes RefCell
 * - We use Vec instead of LinkedList, because this collection
 *   doesn't support insert into the middle.
 *
 * Top-level functions on each type get moved into impl blocks.
 */

use core::marker::PhantomData;

/**
 * Another key difference is that we allocate a single vector to store
 * all elements. We simulate pointers via usize indexing into this
 * vector. This is mainly to appease the borrow checker.
 *
 * To recover a modicum of type safety, we use PhantomData to mark the
 * ref.
 */
struct Ref<T>(u32, PhantomData<T>);

impl<T> Ref<T> {
    pub fn deref<'a>(&self, data: &'a Vec<T>) -> &'a T {
        return &data[self.0 as usize];
    }

    pub fn deref_mut<'a>(&self, data: &'a mut Vec<T>) -> &'a mut T {
        return &mut data[self.0 as usize];
    }
}
    

/**
 * We need our own linked list that actually exposes its internals.
 */
struct DL<T: Sized> {
    data: T,
    left: Ref<DL<T>>,
    right: Ref<DL<T>>
}


/**
 * There are three kinds of nodes: lambdas, var refs, and
 * applications. Each kind gets its own struct, so that we can encode
 * more structure in Rust's type system, which mirrors MLs
 */
struct LambdaType {
    var: VarType,
    body: Option<Ref<Term>>,
    // The parent ref beloinging to our child node (our body) that
    // points back to us.
    bodyRef: Option<Ref<DL<ChildCell>>>,
    parents: Ref<DL<ChildCell>>,
    uniq: i32
}

struct VarType {}
struct ChildCell {}
struct Term {}

/*
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
*/
