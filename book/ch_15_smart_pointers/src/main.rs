/* -------------------------------------------------------------------------- */
/* Smart pointers                                                             */
/* -------------------------------------------------------------------------- */
// - design pattern
// - data structures that act like pointers but contain metadata + additional
//   functionality
// - may own data they point to
// - usually implemented using structs implementing Deref and Drop traits
//   - Deref -> allow it to behave like and be interchangeable with references
//   - Drop -> allow customisation of code to be run when smart pointer instance
//             gets out of scope

/* -------------------------------------------------------------------------- */
/* Box<T>                                                                     */
/* -------------------------------------------------------------------------- */
// - SINGLE ownership
// - IMMUTABLE or MUTABLE borrows checked at COMPILE time
// - has known size, so good for indirection
// - data stored in heap, reference stored in stack

/* -------------------------------------------------------------------------- */
/* Cons list: A recursive type that needs indirection                         */
/* -------------------------------------------------------------------------- */
// - has infinite size
// - insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle

use crate::{
    MyBoxList::{BoxCons, BoxNil},
    MyRcList::{RcCons, RcNil},
    RcRefCellList::{RcRefCellCons, RcRefCellNil},
};

#[derive(Debug)]
enum MyBoxList {
    BoxCons(i32, Box<MyBoxList>), // Cons variant owns the value they hold
    BoxNil,
}

/* -------------------------------------------------------------------------- */
/* Deref/DerefMut trait: Smart pointers as regular references                 */
/* -------------------------------------------------------------------------- */
// - implementing Deref trait lets you customise the behaviour of the *
//   (dereference) operator
// - can be implemented in such a way on smart pointers so that code
//   operating on references can also be used on smart pointers

// - deref coercion done for 3 cases:
//   (1) &T -> &U, when T: Deref<Target=U>
//   (2) &mut T -> &mut U, when T: DerefMut<Target=U>
//   (3) &mut T -> &U, when T: Deref<Target=U>

// why immutable references cannot be coerced into mutable references?
// - because of borrowing rules, where there can be any number of immutable
//   references to a value, but only one mutable reference to a value at any time
// - converting an immutable ref to a mutable ref would require the initial
//   immutable ref to be the only immutable ref to that data, but that's not
//   guaranteed

use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // define an associated type for Deref trait to use
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/* -------------------------------------------------------------------------- */
/* Drop trait                                                                 */
/* -------------------------------------------------------------------------- */
// - implementing Drop trait lets you customise what happens when a value is
//   about to go out of scope

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

/* -------------------------------------------------------------------------- */
/* Rc<T>: Reference-counted smart pointer                                     */
/* -------------------------------------------------------------------------- */
// - problem: single value might have multiple owners
// - example: in a graph, a node should be cleaned up only if it doesn't have
//   any edges pointing towards it and therefore if it has zero owners/references to it
// - solution: Rc<T>
//   - MULTIPLE ownership
//   - IMMUTABLE borrows checked at COMPILE time
//   - keeps track of # of refs to a value, which determines whether value is
//     still in use

// - strong references -> to express shared ownership of Rc<T> instance
//   - can call Rc::clone and passing it reference to an Rc<T> value,
//     which increases the count of STRONG references to the data within the Rc<T>
//   - Rc<T> instance only cleaned up if its STRONG reference count is 0

// - weak references -> don't express ownership
//   - weak count has no effect on when an Rc<T> is cleaned up, as in it doesn't
//     need to be 0 for Rc<T> instance to be cleaned up
//   - can call Rc::downgrade and passing it reference to an Rc<T> value,
//     which creats a smart pointer of type Weak<T>, increasing the WEAK count
//   - can call upgrade method on Weak<T> instance to get an Option<Rc<T>> ->
//     returns Some if Rc<T> value has not been dropped, and None if it has

// - point: use Rc<T> when we want to allocate heap data to be READ by multiple parts
//   of program, but can't determine at compile time which one will finish using
//   the data last

// - conditions:
//   - only for single-threaded contexts
//   - can only have immutable references (therefore read shared data, not
//     write) -> solution: interior mutability = RefCell<T> in conjunction with Rc<T>

// Example:
// - Cons variant now holds a value and a Rc pointing to a MyRcList<T>
// - so creating b from a will not move ownership of a, but will use a clone of
//   of the Rc<MyRcList> that a is holding, increasing the number of references to
//   the now shared data in that Rc<MyRcList>.
enum MyRcList {
    RcCons(i32, Rc<MyRcList>),
    RcNil,
}

/* -------------------------------------------------------------------------- */
/* Interior mutability                                                        */
/* -------------------------------------------------------------------------- */
// - design pattern to allow mutating data even when there are immutable
//   references to that data
// - to mutate data, pattern uses unsafe code inside a data structure that
//   indicates to Rust compiler that we're checking the mutation and borrowing
//   rules manually instead of letting the compiler do it for us
// - BUT STILL MUST ensure that the borrowing rules are followed at runtime,
//   wrapping this unsafe code in a safe API, where the outer type is still
//   immutable

/* -------------------------------------------------------------------------- */
/* RefCell<T>                                                                 */
/* -------------------------------------------------------------------------- */
// - follows interior mutability pattern -> mutate value inside immutable value
// - for situations where it is useful for a value to mutate itself
//   in its methods but appear immutable to other code
// - example:

// - represents SINGLE ownership over data it holds
// - IMMUTABLE or MUTABLE borrows checked at RUNTIME
//   - borrowing rule invariants are enforced at runtime so though
//     the program compiles, breaking them causes program to panic and exit
// - conditions:
//   - only used when you're sure that your code follows borrowing rules
//   - only used in single-threaded contexts

/* -------------------------------------------------------------------------- */
/* RefCell<T> with Rc<T>: Allowing multiple owners of mutable data            */
/* -------------------------------------------------------------------------- */
// - by having an Rc<T> that holds a RefCell<T>

// - limitations:
//   - can cause memory leak through creating REFERENCE CYCLES
//     - creating references where items refer to each other in a cycle
//     - causing the reference count of each item in cycle to never reach 0
//     - so values will never be dropped -> memory leak

#[derive(Debug)]
enum RcRefCellList {
    RcRefCellCons(Rc<RefCell<i32>>, Rc<RcRefCellList>),
    RcRefCellNil,
}

/* -------------------------------------------------------------------------- */
/* Weak<T>: Preventing reference cycles                                       */
/* -------------------------------------------------------------------------- */
// recall problem: can cause memory leak by creating reference cycles
// - causable by creating references that point to each other when using
//   RefCell<T> that contain Rc<T> values
// - or within in similar nested combos of types with interior mutability and
//   reference counting

// possible solutions:
// - minimised by automated tests, code reviews, and other SDE practices
// - reorganise data structures so that some refs express ownership, and some
//   res don't -> even if cycles exist to create some ownership and
//   non-ownership relationships,  only the ownership relationships affect
//   whether the value can be dropped

// solution: Weak<T>
// - can call Rc::downgrade and pass it a reference to Rc<T> -> get smart pointer
//   Weak<T> -> increase weak count of Rc<T> instance by 1
// - can call upgrade on Weak<T> instance to return Option<Rc<T>> -> Some if Rc<T>
//   hasn't been dropped, None if it has
// - weak count doesn't need to be 0 for value within it to be dropped
// - so make sure the value that Weak<T> references still exists before doing
//   anything with them

// how it doesn't cause reference cycles:
// - won't cause memory leak by reference cycle as any cycle involving weak
//   references will be broken once strong reference count of values becomes 0

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    /* ---------------------------------------------------------------------- */
    /* Box<T>                                                                 */
    /* ---------------------------------------------------------------------- */

    let b = Box::new(5);
    println!("b = {b}");

    /* ---------------------------------------------------------------------- */
    /* Enabling recursive types with Box<T>                                   */
    /* ---------------------------------------------------------------------- */
    // - value of recursive type can have another value of same type as part of itself
    // - space value needs is unknown at compile time

    /* Box<T> as indirection:                                                 */
    // - Instead of storing a value directly, change the data structure to store
    //   the value indirectly by storing a pointer to the value instead

    /* Box<T> for heap allocation:                                            */
    // - Box<T> is a pointer and has defined size, so Box<T> will point to the
    //   next List value that will be on heap rather than inside the Cons variant
    let list = BoxCons(
        1,
        Box::new(BoxCons(2, Box::new(BoxCons(3, Box::new(BoxNil))))),
    );
    println!("{list:?}");

    /* ---------------------------------------------------------------------- */
    /* Deref coercion                                                         */
    /* ---------------------------------------------------------------------- */
    // - converts reference to a type that implements Deref into reference to
    //   another type
    // - calls Deref::deref as many times needed to get a reference to match the
    //   parameter's type
    // - example: &String -> &str because String implements Deref trait so that
    //   it returns &str

    // Example: MyBox<T> implements Deref, so using hello on &MyBox<String>,
    // Rust turns it into &String by calling deref, and then deref again for &String -> &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Without deref coercion -> hello(&(*m)[..])
    // where (*m) dereferences MyBox<String> into String
    // then & and [..] to take a string slice of String to match the whole
    // string

    /* ---------------------------------------------------------------------- */
    /* Drop trait                                                             */
    /* ---------------------------------------------------------------------- */
    // - variables are dropped in the reverse order of their creation
    // - to force early clean up of a value, call std::mem::drop function
    // - std::mem::drop fn is a destructor (cleans up an instance), analogous to
    //   constructor (which creates an instance)
    // - not possible to explicitly call Drop::drop method, because when value
    //   goes out of scope, it is always automatically called, so calling Drop::drop
    //   would be trying to clean up the same value twice
    create_smart_pointers();

    /* ---------------------------------------------------------------------- */
    /* Rc<T>                                                                  */
    /* ---------------------------------------------------------------------- */
    // - only immutable references
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // at the end when b then a goes out of scope, the count becomes 0 and the
    // Rc<MyRcList> is cleaned up completely

    /* ---------------------------------------------------------------------- */
    /* Rc<T> with RefCell<T> for multiple ownership of mutable data           */
    /* ---------------------------------------------------------------------- */
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcRefCellCons(Rc::clone(&value), Rc::new(RcRefCellNil)));
    let b = RcRefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RcRefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {a:?}");
    println!("b before = {b:?}");
    println!("c before = {c:?}");

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    /* ---------------------------------------------------------------------- */
    /* Weak<T>                                                                */
    /* ---------------------------------------------------------------------- */
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent before: {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent inner: {:?}", leaf.parent.borrow().upgrade());
    }

    // at this point, even though leaf.parent "points" to branch, it does so as
    // a Weak<T> instance, which means branch will still be dropped as it
    // becomes out of scope, even though its weak count was still 1 != 0

    println!("leaf parent after: {:?}", leaf.parent.borrow().upgrade());
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn create_smart_pointers() {
    // c will be dropped first, immediately at drop(c), i.e. before end of fn
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);

    // d will be dropped third
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    // e will be dropped second
    let e = CustomSmartPointer {
        data: String::from("even more stuff"),
    };
    println!("CustomSmartPointers created");
}
