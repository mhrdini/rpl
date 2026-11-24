/* -------------------------------------------------------------------------- */
/* Asynchronous programming                                                   */
/* -------------------------------------------------------------------------- */
// - abstraction to allow and coordinate pause points and eventual results in
//   our code, that didn't block the run of the rest of tasks of the program,
//   allowing the runtime to choose best order and manner in which to run tasks

// - keywords: futures, streams, async and await syntax, multithreading

// - running async code usually happens concurrently
//   - caveat: concurrency may use parallelism under the hood depending on hardware, OS,
//     and async runtime

/* -------------------------------------------------------------------------- */
/* Concurrency vs Parallelism                                                 */
/* -------------------------------------------------------------------------- */
// - concurrency -> a single actor works on several different tasks before any of
//   them are complete, multitasking and making progress on one at a time by
//   switching between them
//   - I/O bound

// - parallelism -> multiple actors take on a single task and work on it alone
//   perhaps simultaneously
//   - CPU bound
//   - if work is very parallelisable, i.e. processing data where each part can
//     be processed separately -> use threads

// - can be partially parallel, where some some workflows are serial, i.e. can
//   happen only in a series, one task after the other
// - can be partially concurrent, where one of your own tasks depends on another
//   one of your tasks

// e.g. single CPU core doing concurrent work, multiple CPU's doing concurrent
// work in parallel to each other

// when to use which?
// - if work is very CONCURRENT i.e. handling input from bunch of different
//   sources that may come in at different intervals or rates
//   => use ASYNC
// - if work is very PARALLELISABLE, i.e. processing data where each part can
//   be processed separately
//   => use THREADS

/* -------------------------------------------------------------------------- */
/* Futures                                                                    */
/* -------------------------------------------------------------------------- */
// - future -> a value that may not be ready not, but will become ready at some
//   point in future (same concept as JS promise, but different syntax handling)
//   - types that implement Future trait
//   - holds its own metadata about progress made so far and what "ready" means

// futures are lazy -> won't do anything until call to .await
// just like iterators are lazy -> won't do anything until .next()

// - Future trait -> so different async operations can be implemented with
//   different data structures using a common interface

/* -------------------------------------------------------------------------- */
/* Async vs threads                                                           */
/* -------------------------------------------------------------------------- */
// - for thread model:
//   - concurrency between synchronous threads
//     - threads act as boundary for sets of SYNCHRONOUS operations
//   - managed by OS, dependent on support by OS and hardware
//   - threads are relatively heavyweight

// - for async model:
//   - concurrency between asynchronous futures managed within tasks/join
//     handles
//     - tasks act as boundary for sets of ASYNCHRONOUS operations
//     - futures are most granular unit of concurrency, can represent tree of
//       other futures
//   - managed by runtime (library-level code)
//   - tasks are relatively lightweight

/* -------------------------------------------------------------------------- */
/* async and await                                                            */
/* -------------------------------------------------------------------------- */

// - async -> applied to blocks and fns to indicate they can be interrupted and
//   resumed
//   - when applied to:
//     (1) a block -> compiled as anonymous data type that implements Future
//                    trait
//     (2) a function -> compiled as non-async function whose body is an async
//                       block, whose return type is the anonymous data type
//                       (that implements Future trait) that async block
//                       compiles to

// - await -> within async block/fn, applied to a future, to wait for it to
//   become ready
//   - called as .await postfix that you can chain

// - polling -> process of checking with a future to see if its value is
//   available yet

use std::pin::{Pin, pin};
use std::task::{Context, Poll};
use std::{env, thread, time::Duration};
use trpl::{Either, Html};

/// Get `<title>` element from an HTML page
async fn page_title(url: &str) -> (&str, Option<String>) {
    // await for server to send back first part of response
    // await to get text of response

    // let resp = trpl::get(url).await;
    // let text = resp.text().await;
    let text = trpl::get(url).await.text().await;

    // parse raw text into Html type
    // Html::select_first returns Option<ElementRef> based a selector
    // return Option<String> for if title element exists or not

    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}

/* -------------------------------------------------------------------------- */
/* Async traits: Future trait                                                 */
/* -------------------------------------------------------------------------- */
/* has Output as associated type                                              */
// - what future eventually produces
// - analogous to Iterator trait's associated type Item

/* has poll method                                                            */
// - takes:
//   - a Pin<&mut self>
//   - mut reference to a Context type -> used by runtime to re-check future at
//     the right time
// - returns: Poll<Self::Output>
// - usually not called directly
// - usually panics when called if future is Ready

/* Poll<T> type                                                               */
// - variants:
//   - Pending -> indicates that future still has work to do, telling caller to
//     check again later
//   - Ready(T) -> indicates that future has finished work and T value is
//     available -> should not poll again once future is Ready

/* -------------------------------------------------------------------------- */
/* Pin type & pin! macro                                                      */
/* -------------------------------------------------------------------------- */
/* What pinning means                                                         */
// - to pin value that pointer points to -> to guarantee that the value won't be
//   moved in memory, even if the pointer does
// - directly awaiting a future PINS the future implicitly

/* How to pin                                                                 */
// - use Pin type and pin! macro from std::pin

/* Pin type                                                                   */
// - wrapper for pointer-like types
//   (i.e. types that implement Deref or DerefMut) -> &, &mut, Box, Rc, etc.

/* pin! macro                                                                 */
// - pins a value in place, on the stack
// - creates a local variable holding the data
// - returns a Pin<&mut T> reference pointing at the local variable
// - since it creates a sized reference -> usable as indirection
//   - allocation-free compared to Box, which allocates in heap

/* -------------------------------------------------------------------------- */
/* Unpin trait                                                                */
/* -------------------------------------------------------------------------- */
// - informs compiler that a given type does NOT need to uphold any guarantees
//   about whether value in question can be safely moved

// - for types that have no internal references that make it unsafe to move
//   around, e.g. String

// - marker trait (like Sync and Send), so has no functionality on its own
// - marker traits exist only to tell compiler it's safe to use the type
//   implementing a given trait in a particular context

// to NOT implement Unpin for a type -> to make the type NEED to be pinned to be
// safe when the pointer it belongs to is moved:
//
// impl !Unpin for T
//
// where T is the name of a type DOES need to uphold those guarantees to be
// safe whenever a pointer to that type is used in a Pin

/* -------------------------------------------------------------------------- */
/* trpl::join_all: The motivating concrete example for Pin type + Unpin trait */
/* -------------------------------------------------------------------------- */
// - trpl::join_all can take an arbitrary number of futures that need not be
//   known at compile time
// - because a vector can only have the same type,
//   we attempt to fix this by:
//   - turning each future into a trait object -> dyn Future<Output = ()>
//     - this is because under the hood, the compiler automatically generates
//       a unique enum type for each individual future, making them not of the
//       same concrete type
//   - encase each in Box for 2 reasons
//     (1) because dyn Future<Output =()> is unsized and Box<T> becomes
//         sized indirection
//       - recall: Box<T> is a smart pointer used for indirection, so it has a
//         fixed size and lives in the stack, regardless of how small/big the
//         heap data that it points to might be
//     (2) because Box<T> also implements Future if T: Future is Unpin
// - join_all returns a new future (struct) JoinAll<F> where F:Future, no direct
//   awaiting happening

// - problem:
//   - Box<T> only implements Future if the T implements Unpin trait,
//   - BUT: dyn Future<Output = ()> isn't Unpin (because the concrete futures hiding
//     behind it might be self-referential)
//   - so this Box<T> is Future impl never applies to Box<dyn Future<Output = ()>>

// - solution: use pin! macro and Pin type
//   - to sidestep the problem brought by simultaneously:
//     - needing T to be Pin since T is a Future and may be self-referential, which
//       brings problems when moving it (as we are inserting it into a Vec)
//     - but also needing T to be Unpin if Box<T> were to implement Future

/* -------------------------------------------------------------------------- */
/* pin! macro vs Box::pin                                                     */
/* -------------------------------------------------------------------------- */
// - Box::pin which creates the unconditionally pinned Pin<Box<F>>, i.e. no
//   Unpin bound needed for F
// - difference:
//   - Box::pin -> allocates on heap as before, pins on heap
//   - pin! -> pins on stack, avoids heap allocation

/* -------------------------------------------------------------------------- */
/* Await points: How it works under the hood + its relation to Pin            */
/* -------------------------------------------------------------------------- */
// - each series of await points in a future/async block get compiled into
//   a state machine:
//   - an enum with one variant per "segment" of code between await points/end
//     of async block
//   - each variant holds whatever data is needed for that segment (by either
//     taking ownership of or holding references to them)
//   - this is for Rust to make sure that data movement follows all safety
//     rules, including borrowing

// - when we move a future (inserting into data structure or returning it from a
//   function) -> we need to move the state machine

// problem: a given variant can end up holding a reference to data that lives in
// another field of the very same struct -> become self-referential
// - i.e. has fields that references other fields of itself

// even BIGGER problem: a self-referential data-type is dangerous to move in memory
// - a reference always points to a concrete address in memory
// - moving the whole structure (e.g. pushing into Vec or returning from fn)
//   won't automatically update any internal pointers, and they will still point
//   to old address
// - so it would mean the old memory:
//   (1) becomes stale -> not kept in sync with real data anymore -> end up
//   reading garbage
//   (2) could be used for something unrelated -> end up reading data belonging
//   to something else entirely

// solution idea: don't let the data in memory move in the first place when
// the future it belongs to moves
// concrete solution: wrap the pointer values in Pin, e.g. Pin<Box<T>> and use
// pin! macro when declaring the future

// recall: Pin<T>
// - not a pointer, but just a compile-time wrapper around pointer-like types,
//   i.e. it can wrap any type implementing Deref or DerefMut such that,
//   the thing the pointer points to is guaranteed NOT to move
//   - while usual Rust borrow check already disallows you to move a value while
//     something else is already actively referencing it -> Pin applies this to
//     self-referential values across moves of its wrapping pointer
// - e.g. with Pin<Box<T>>, this pins the T data type value being pointed to,
//   NOT the Box pointer, so that Box pointer can move around freely, but
//   the data it points to is pinned to remain in the same place

/* -------------------------------------------------------------------------- */
/* Streams: Futures in sequence                                               */
/* -------------------------------------------------------------------------- */
// - pattern of working with items sequentially over time
// - streams are futures -> can be used with other futures

// examples:
// - batch up events to avoid too many networking calls
// - set timeouts on sequences of long-running operations
// - throttle/debounce UI events

// - async channel receivers -> streams
//   - general-purpose stream API: provides the next item asynchronously
//   - trpl::Receiver: waits for to receive messages

// - not in std library, but is a mix of Iterator and Future:
//   Iterator::next -> Option<Self::Item> = idea of sequence
//   Future::poll -> Poll<Self::Output> = idea of readiness of a single output
//   over time

// - so combined together:
//   Stream::poll_next -> Poll<Option<Self::Item>> = the idea of a sequence of
//   items that become ready over time

trait Stream {
    type Item;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>>;
}

/* -------------------------------------------------------------------------- */
/* Using trpl::StreamExt trait                                                */
/* -------------------------------------------------------------------------- */
// - StreamExt is automatically implemented for every type that implements
//   Stream, but defined separately
// - so it needs to be explicitly used -> use trpl::StreamExt

use trpl::StreamExt;

/* -------------------------------------------------------------------------- */
/* Async and threads together                                                 */
/* -------------------------------------------------------------------------- */

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    /* ---------------------------------------------------------------------- */
    /* Runtime: Running async code with trpl::block_on                        */
    /* ---------------------------------------------------------------------- */
    // - main cannot be made async because async code needs a runtime,
    //   i.e. a Rust crate that manages the details of executing async code,
    //   and the main function isn't a runtime

    // why a runtime is needed to run async code:
    // - at each await point:
    //   - Rust pauses async block, and
    //   - hands control back to runtime
    //   (everything between await points is synchronous)
    // - Rust keeps track of state involved in async block so that runtime can
    //   do some other work and then come back when it's ready to advance that
    //   async block

    // - trpl::block_on
    //   - takes a future and blocks current thread until
    //     future runs to completion, returning whatever future produced
    //     (under the hood, block_on sets up runtime using tokio crate)
    //   - everything within it runs linearly -> no concurrency
    trpl::block_on(async {
        if args.len() >= 1 {
            let url = &args[0];
            let (_, title) = page_title(url).await;
            match title {
                Some(title) => println!("The title for {url} was {title}"),
                None => println!("{url} had no title"),
            }
        }
    });

    /* ---------------------------------------------------------------------- */
    /* trpl::select                                                           */
    /* ---------------------------------------------------------------------- */
    // - to race 2 futures, finishes as soon as one of them is done
    // - not fair -> always polls arguments in the other in which they are
    //   passed (some other implementations randomly choose though)
    // - needs to be await'ed

    trpl::block_on(async {
        if args.len() >= 2 {
            let title_fut_1 = page_title(&args[0]);
            let title_fut_2 = page_title(&args[1]);

            let (url, maybe_title) =
                match trpl::select(title_fut_1, title_fut_2).await {
                    Either::Left(left) => left,
                    Either::Right(right) => right,
                };

            println!("{url} returned first");

            match maybe_title {
                Some(title) => println!("Its page title was: '{title}'"),
                None => println!("It had no title."),
            }
        }
    });

    /* ---------------------------------------------------------------------- */
    /* Concurrency with async using tasks                                     */
    /* ---------------------------------------------------------------------- */
    // - trpl::spawn_task -> create a new join handle that takes a future
    //   - join handles, i.e. tasks are similar to threads but, instead of being
    //     managed by OS, they are managed by runtime
    //   - tasks act as boundary for sets of asynchronous operations
    //     -> concurrency is possible both between and within tasks
    //     -> tasks can switch between futures in its body
    // - trpl::sleep -> to block execution of current task for a duration
    // - to run, we must call await on:
    //   - futures & join handles
    //   - sleep calls

    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // let handle = trpl::spawn_task(fut1);
        // handle.await.unwrap();

        /* ------------------------------------------------------------------ */
        /* trpl::join + join! macro                                           */
        /* ------------------------------------------------------------------ */
        // - join is fair -> i.e. checks each future equally often, alternative
        //   between them, never lets one race ahead if the other is ready
        // - so running it creates the exact same order every time

        // - trpl::join for 2 futures (need to be await'ed), or
        // - trpl::join! macro for arbitrary # of futures (number
        //   of futures MUST BE KNOWN at COMPILE time)

        // - returns a future that is the result of the 2+ futures

        trpl::join(fut1, fut2).await;

        /* ------------------------------------------------------------------ */
        /* async Rust (futures) vs threads                                    */
        /* ------------------------------------------------------------------ */
        // - threads -> OS decides which thread to check and how long to let it
        //              run
        // - futures -> runtime decides which task to check (and possibility to
        //              use OS threads under hood as part of managing concurrency)
    });

    /* ---------------------------------------------------------------------- */
    /* Message passing with async: trpl::channel                              */
    /* ---------------------------------------------------------------------- */
    // - use trpl::channel
    //   - also multiple-producer, so can call clone on tx if we want to send
    //     messages from multiple futures
    // - instead of mpsc::channel (multiple-producer, single-consumer channel
    //   API used with threads)

    // - runs within async block -> everything within it can avoid blocking

    // - uses mutable receiver rx
    // - rx.recv() method produces future (instead of value directly)
    //   - so needs to be await'ed
    // - does not block, but hands control back to runtime, until either message
    //   is received or send side of channel closes

    trpl::block_on(async {
        // channel will only close if:
        // - we call rx.close, or
        // - tx is dropped
        let (tx, mut rx) = trpl::channel();
        let tx_1 = tx.clone();

        // problem 1: block_on runs linearly -> cannot simulate async receiving + sending
        // - therefore to create an asynchronous workflow, the sending using tx
        //   and the receiving using rx needs to be within async block, i.e. future
        // solution for p1: put async code to be run into async block, thus producing a
        // future

        // problem 2: program doesn't exit because:
        // - future returned from trpl::join completes only when both futures
        //   have completed
        // - tx_fut future completes once it finishes sleeping after sending
        //   last val in vals
        // - rx_fut future completes once while let loop ends
        // - while let loop won't end until awaiting rx.recv gives None
        // - awaiting rx.recv will return None only when channel is closed
        // - channel is closed only when rx.close or tx is dropped
        // - rx.close is not called and tx won't be dropped until block_on block
        //  future ends
        // - block_on block can't end because it is blocked on trpl::join
        //   completing -> which brings us back to the first point

        // solution for p2: need to force move ownership of tx by using async move
        // when declaring the future that uses it tx_fut, so that tx will be
        // dropped once tx_fut future is run to completion

        // problem 3: the potentially self-referential futures are being moved
        // into a Vec, which will introduce memory staleness or incorrectness
        // since internal pointers are not automatically updated upon move

        // solution for p3: pin the futures using pin! macro and declare Pin
        // type in vector declaration
        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        // runtime will pause the future until it is ready
        // for each arriving message ->  resolves to Some(message)
        // when channel closes -> future will resolve to None to indicate that
        //                        there are no more values and indicate
        //                        to stop polling/awaiting

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_1_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx_1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        /* ---------------------------------------------------------------------- */
        /* trpl::join_all                                                         */
        /* ---------------------------------------------------------------------- */
        // - takes a vector containing some number of futures that won't be known
        //   until runtime

        // run the futures equally via either:
        // - join (2, await, compile time),
        // trpl::join(tx_fut, rx_fut).await;

        // - join! (2+, compile time)
        // trpl::join!(tx_fut, tx_1_fut, rx_fut);

        // - join_all(unknown, runtime)
        // put each future within a Box -> makes them trait objects -> so each
        // anonymous future produced by these types (each async block has its
        // own enum under the hood according to the compiler) are treated as the
        // same type, because all of them implement Future trait
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx_fut, tx_1_fut, rx_fut];
        trpl::join_all(futures).await;
    });

    /* ---------------------------------------------------------------------- */
    /* trpl::yield_now                                                        */
    /* ---------------------------------------------------------------------- */
    // - to force progress by returning control to runtime
    // - to not block and starve other futures if code does not have await
    // - better for overall performance, instead of sleep
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };
        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });

    /* ---------------------------------------------------------------------- */
    /* Async streams vs Sync iterators                                        */
    /* ---------------------------------------------------------------------- */
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        // need to use trpl::StreamExt for .next() to work, and make stream
        // behave like an iterator
        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

/* -------------------------------------------------------------------------- */
/* Creating our own async abstractions: a timeout function                    */
/* -------------------------------------------------------------------------- */
async fn timeout<F: Future>(
    future: F,
    dur: Duration,
) -> Result<F::Output, Duration> {
    // because select is not fair in that it will poll by order in which future
    // args are passed, it will try to run the future first, so it gets a chance
    // to complete even if duration is very short
    // if future finishes first -> return Left
    // if duration finishes first -> return Right
    match trpl::select(future, trpl::sleep(dur)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(dur),
    }
}
