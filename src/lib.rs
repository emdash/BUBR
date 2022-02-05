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
use core::marker::PhantomData;


/**
 * What follows is a straight port of the SML code listing in the
 * PDF. Not attempting to take full advantage of Rust until better I
 * understand how this works.
 *
 * - Each record becomes a struct
 * - Except when it is a variant, it becomes an enum
 * - ChildCell becomes RefCell
 * - We use Vec instead of LinkedList, because this collection
 *   doesn't support insert into the middle.
 *
 * Top-level functions on each type get moved into impl blocks.
 */

/**
 * One key difference is that we use vectors for storage, and simulate
 * pointers via indexing. This is mainly to appease the borrow
 * checker, because we are working with doubly-linked lists.
 *
 * To recover a modicum of type safety, we use PhantomData to tag the
 * ref with its expected type.
 */
#[derive(Copy, Clone, Debug)]
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
 * We need our own linked list that actually exposes its internals,
 * since that's how the SML is written.
 */
#[derive(Clone, Debug)]
struct DL<T: Sized> {
    data: Ref<T>,
    left: Ref<DL<T>>,
    right: Ref<DL<T>>
}


/**
 * There are three kinds of nodes: lambdas, var refs, and
 * applications. Each kind gets its own struct, so that we can encode
 * more structure in Rust's type system, which mirrors MLs
 */
#[derive(Clone, Debug)]
struct LambdaType {
    var: VarType,
    body: Option<Ref<Term>>,
    // The parent ref beloinging to our child node (our body) that
    // points back to us.
    bodyRef: Option<Ref<DL<ChildCell>>>,
    parents: Ref<DL<ChildCell>>,
    uniq: i32
}

/**
 * funcRef and argRef are similar to bodyRef above.
 */
#[derive(Clone, Debug)]
struct AppType {
    func:    Option<Ref<Term>>,
    arg:     Option<Ref<Term>>,
    funcRef: Option<Ref<DL<ChildCell>>>,
    argRef:  Option<Ref<DL<ChildCell>>>,
    copy:    Option<Ref<AppType>>,
    parents: Ref<DL<ChildCell>>,
    uniq:    i32
}

#[derive(Clone, Debug)]
struct VarType {
    name: String,
    parents: Ref<DL<ChildCell>>,
    uniq: i32
}

/**
 * Type of general LC node.
 */
#[derive(Clone, Debug)]
enum Term {
    LambdaT(LambdaType),
    AppT(AppType),
    VarT(VarType)
}


/**
 * This tells us what our relation to our parent is.
 */
#[derive(Clone, Debug)]
enum ChildCell {
    AppFunc(AppType),
    AppArg(AppType),
    LambdaBody(LambdaType)
}


/**
 * This holds all our storage, and it's the type on which we implement
 * our top-level functions.
 */
struct TG {
    terms: Vec<Term>,
    child_cells: Vec<ChildCell>,
    dl_child_cell: Vec<DL<ChildCell>>
}


impl TG {
    // XXX: this helper could be elimiminated with a different type
    // structure.
    fn termParRef(term: &Term) -> Ref<DL<ChildCell>> {
        match term {
            Term::LambdaT(lt) => lt.parents.clone(),
            Term::AppT(at)    => at.parents.clone(),
            Term::VarT(vt)    => vt.parents.clone(),
        }
    }
}



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
