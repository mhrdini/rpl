/* -------------------------------------------------------------------------- */
/* Lifetimes                                                                  */
/* -------------------------------------------------------------------------- */
// - compiler's way of tracking how long a reference is valid
// - a generic that ensure that references are valid as long as we need them to be
// - every reference in Rust has a lifetime, which is the scope for which that
//   reference is valid
// - just like types, lifetimes are implicit and inferred
//   - types only need to be annotated when multiple types are possible
//   - lifetimes only need to be annotated when the compiler can't infer how
//     references relate to each other, e.g. a function that takes multiple
//     references and returns one, or a struct that holds a reference
// - aim: to prevent dangling references that cause illegal referencing

/* invalid ref because reference refers to memory with shorter lifetime       */
// fn invalid() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {r}");
// }

use std::fmt::Display;

/* valid ref because data has longer lifetime than reference                  */
fn valid() {
    let x = 5;
    let r = &x;
    println!("r: {r}");
}

/* -------------------------------------------------------------------------- */
/* Generic lifetimes: Lifetime annotations                                    */
/* -------------------------------------------------------------------------- */
// - diagnostic: return type needs generic lifetime parameter on it
// - aim: to connect the lifetimes of various parameters and return values of functions
//   i.e. to define relationship between references in a function
// - why: so borrow checker can analyse and allow memory-safe operations
//   and disallow operations that would create dangling pointers or violate memory safety

// solution: lifetime annotations
//   - &i32         -> ref
//   - &'a i32      -> ref with explicit lifetime
//   - &'a mut i32  -> mutable ref with explicit lifetime

// syntax: define lifetime as generic (in angle brackets) after function name
// as short name after an ', and add them to references written after & and
// before the data type

// which references to add lifetime to?
// - references that have relationship with each other
// - if specifying a lifetime for a return value:
//   return value lifetime must be related to lifetime to at least one of the parameters

/* Example                                                                    */
// - for some lifetime 'a:
//   - fn takes two parameters, both string slices,
//     that will live for at least as long as lifetime 'a
//   - fn returns a string slice
//     that will live for at least as long as lifetime 'a
// - this means that lifetime of return reference is the same as the smaller of
//   the lifetimes of the values referred to by the function arguments
// - given this function signature, borrow checker will reject any values that
//   don't adhere to these constraints

// when we pass concrete references to longest:
// - the concrete lifetime that is substituted for 'a is
//   the part of the scope of x that OVERLAPS with the scope of y
// - i.e. the generic lifetime 'a will get the concrete lifetime that is equal
//   to the smaller of the lifetimes of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/* -------------------------------------------------------------------------- */
/* In struct definitions: Lifetimes of borrowed type fields                   */
/* -------------------------------------------------------------------------- */
// defined as generic, in angle brackets, like generic types, and used in at
// least one reference field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl Display for ImportantExcerpt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.part)
    }
}

/* -------------------------------------------------------------------------- */
/* Lifetime elision rules                                                     */
/* -------------------------------------------------------------------------- */
// Rules the compiler follows to figure out lifetimes of references if there
// aren't any explicit annotations

// Terminology:
// - Input lifetimes = Lifetimes on function/method parameters
// - Output lifetimes = Lifetimes on return values

// 3 rules:
// (1) Compiler assigns lifetime parameter to each input reference parameter
// (2) If there is exactly ONE input lifetime parameter, that lifetime is
//     assigned to ALL output lifetime parameters
// (3) If there are multiple input lifetime parameters, but ONE of them is &self
//     or &mut self because the fn is a method, the lifetime of self is assigned
//     to ALL output lifetime parameters

// Rubric: If compiler gets to end of 3 rules and there are still reference for
// which it can't figure out lifetimes -> Compiler stops with error

// These rules decide whether or not lifetimes can be "elided" i.e. dropped

/* Example 1: Compiles                                                        */
fn first_word_implicit(s: &str) -> &str {
    "first"
}

// 1. each input reference parameter gets own lifetime -> applies -> check
// 2. if only one input lifetime parameter, apply same lifetime to ALL output ->
//    applies -> check
// 3. multiple input lifetime parameter + is method = apply self lifetime to ALL
//    output lifetime params -> does not apply
// Outcome: Lifetimes for all references figured out -> Compiles
fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    "first"
}

/* Example 2: Error                                                           */

// fn longest_word_implicit_invalid(x: &str, y: &str) -> &str { // needs lifetime parameters
//     "longest"
// }

// 1. each input reference parameter gets own lifetime -> applies -> check
// 2. if only one input lifetime parameter, apply same lifetime to ALL output ->
//    does not apply
// 3. multiple input lifetime parameter + is method = apply self lifetime to ALL
//    output lifetime params -> does not apply
// Outcome: Output lifetime not figured out -> Error = Does not compile
// fn longest_word_explicit_still_invalid<'a, 'b>(x: &'a str, y: &'b str) -> &str {
//     "longest"
// }

/* -------------------------------------------------------------------------- */
/* Generic lifetimes in impl / method definitions                             */
/* -------------------------------------------------------------------------- */
// General template for generic lifetimes in impl
// impl<1> Struct/Enum/Union/Type alias/Local trait <2> {
// <1> = Declaration of generic NAMED parameters
// <2> = Arguments passed to type's generic parameters:
//       - in same order as declaration
//       - lifetime argument can either be:
//         - name declared in <1>, e.g. 'a
//         - anonymous lifetime '_
//         - 'static, which doesn't have to be declared in <1>

/* Example                                                                  */
// In explicit form:
// impl<'a> ImportantExcerpt<'a> {

// In implicit form:
impl ImportantExcerpt<'_> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

/* -------------------------------------------------------------------------- */
/* Static lifetime: 'static                                                   */
/* -------------------------------------------------------------------------- */
// A 'static reference lives for the entire duration of the program

// Here, <3>  passes:
// - 'a to the first slot,
// - 'static to the second, and
// - 'b to the third.
//  'static never needs declaring, so it's valid
struct Struct<'a, 'b, 'c>
where
    'a: 'c, // see next section: Outlives bound
{
    first: &'a str,
    second: &'b str,
    third: &'c str,
}

// explicit form:
// impl<'a, 'b> Struct<'a, 'static, 'b> {
//     fn static_part(&self) -> &'static str {
//         self.second // OK: 'b here is 'static
//     }
// }

// implicit form:
impl Struct<'_, 'static, '_> {
    fn static_part(&self) -> &'static str {
        self.second // OK: 'b here is 'static
    }
}

/* -------------------------------------------------------------------------- */
/* Outlives bound: Using where clause on lifetimes                            */
/* -------------------------------------------------------------------------- */
// A bound (:) with a lifetime on the RHS
// so that LHS lives as long as RHS, i.e. references in LHS must not expire
// before RHS does

// Compare to a trait bound, which has a trait on the RHS
// (Trait bound: LHS must implement RHS)

// e.g. where 'a: 'c
// LHS = 'a, RHS = 'c
// so all instances of 'a must live as long as all instances of 'c

// As a guarantee that every use of LHS covers/lives as long as every use of RHS,
// i.e. references with LHS lifetime are valid for entire span of RHS' lifetime

// LHS = slot that's required to be the longer lifetime
// RHS = slot that's required to be the shorter lifetime

/* Outlives bound using static                                                */
// 'static: 'x -> Valid because 'static outlives every lifetime
// 'x: 'static -> Only valid IFF 'x IS ALSO 'static
struct Wrapper<'a, T> {
    value: &'a T, // infers T: 'a
}

// T: 'static -> T contains no references that could expire
fn needs_static<T: 'static>(x: T) {} // 

fn main() {
    /* Same concrete lifetimes                                                */
    let x = String::from("abcd");
    let y = "xyz";
    let result = longest(x.as_str(), y);
    println!("The longest string is {result}");

    /* ---------------------------------------------------------------------- */

    /* Different concrete lifetimes                                           */
    let x = String::from("long string is long");
    {
        let y = String::from("xyz");
        let result = longest(x.as_str(), y.as_str());
        println!("The longest string is {result}");
    }
    // can't use result here, because lifetime of result = smaller of the
    // lifetimes of x and y = lifetime of y

    /* ---------------------------------------------------------------------- */

    /* Incorrect: Lifetime of borrowed value too short                        */
    // result uses borrow of y, and is used after inner scope,
    // so y's lifetime must be valid at that point.
    // but y's lifetime is inner scope, therefore result cannot be used
    let x = String::from("long string is long");
    let result;
    {
        let y = String::from("xyz");
        result = longest(x.as_str(), y.as_str());
    }
    // println!("The longest string is {result}"); // uncomment to see error

    /* ---------------------------------------------------------------------- */

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {i}");
}
