/* -------------------------------------------------------------------------- */
/* Iterators                                                                  */
/* -------------------------------------------------------------------------- */
// - pattern to perform task on sequence of items in turn
// - lazy in Rust -> no effect until we call methods that consume the iterator to
//   use it up

/* -------------------------------------------------------------------------- */
/* Iterator trait & next method                                               */
/* -------------------------------------------------------------------------- */
// - only requires implementation for next method
// - why iterator variable must be mutable:
//   - each call to next changes the internal state that iterator uses to keep
//     track of where it is in the sequence
//   - each call to next eats up an item from the iterator
//   - and the iterator is consumed

/* -------------------------------------------------------------------------- */
/* Common methods to create iterators from a collection                       */
/* -------------------------------------------------------------------------- */
// (1) iter method -> produces iterator over immutable references
// - calls to next return immutable refs
// (2) iter_mut -> produces iterator over mutable references
// - calls to next return mutable refs
// (3) into_iter method -> produces iterator that takes ownership
// - calls to next return owned values
// - consumes collection
#[test]
fn iterator_demonstration() {
    let v1 = [1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

/* -------------------------------------------------------------------------- */
/* Methods that consume iterators                                             */
/* -------------------------------------------------------------------------- */
// = consuming adapters -> calling them consumes the itertor

#[test]
fn iterator_sum() {
    let v1 = [1, 2, 3];

    let v1_iter = v1.iter();

    // annotate type because sum doesn't know what type to return
    // generic return type of sum S requires caller to specify type
    // any type that implements Sum<&i32> can be the output, but compiler can't
    // choose for you, even if the only type that implements Sum<&i32> is i32,
    // because impl's aren't fixed
    // so we must annotate i32 as return type, so the compiler can find
    // impl Sum<&i32> for i32 and run its the sum method
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

/* -------------------------------------------------------------------------- */
/* Methods that produce other iterators                                       */
/* -------------------------------------------------------------------------- */
// = iterator adapters -> don't consume iterator, but produce different iters by
//                        changing some aspect of original iterator
// - iterator adapters are lazy, so nothing happens until you call next
#[test]
fn iter_map() {
    let v1 = [1, 2, 3];
    v1.iter().map(|x| x + 1); // lazy; doesn't do anything

    /* ---------------------------------------------------------------------- */
    /* collect method: to produce a new collection                            */
    /* ---------------------------------------------------------------------- */
    // consumes iterator and collects resultant values into collection data type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, [2, 3, 4]);

    /* ---------------------------------------------------------------------- */
    /* for_each method: for side-effects, returns ()                          */
    /* ---------------------------------------------------------------------- */
    // can only read immutable refs, can write mutable refs so use with mutable iterator
    let mut v1 = [1, 2, 3];
    v1.iter_mut().for_each(|x| *x += 1);
    assert_eq!(v1, [2, 3, 4]);

    // for loop
    let mut v1 = [1, 2, 3];
    for x in v1.iter_mut() {
        *x += 1;
    }
    assert_eq!(v1, [2, 3, 4]);

    let mut v1 = [1, 2, 3];
    for x in &mut v1 {
        *x += 1;
    }
    assert_eq!(v1, [2, 3, 4]);
}

/* -------------------------------------------------------------------------- */
/* Iterator + Iterator adapter + Closures that capture environment            */
/* -------------------------------------------------------------------------- */
// - closure gets item from iterator
// - performs body
// - depending on result of body, value may be included in the iteration
//   produced by the iterator adapter

fn main() {
    println!("Hello, world!");
}
