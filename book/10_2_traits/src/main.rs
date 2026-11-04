use std::fmt::{Debug, Display};

/* -------------------------------------------------------------------------- */
/* Traits                                                                     */
/* -------------------------------------------------------------------------- */
// - defines shared behaviour (similar to interfaces)
// - behaviour = methods that can be called on that type
// - can use trait bounds to specify that a generic type can be any type that
//   has certain behaviour
// - trait definitions = group method signatures together to define a set of
//   behaviours necessary to accomplish some purpose
// - each type implementing the trait must provide its own custom behaviour for
//   the body of each method within the trait that has no default implementation
pub trait Summary {
    // method signature without default implementation
    fn summarise_author(&self) -> String;

    // and one with  default implementation
    // can call any other method within trait, even ones without default impl
    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }
    // more methods signatures here
    // each can end in semicolon or have some default behaviour specified
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

/* -------------------------------------------------------------------------- */
/* Specific implementation of trait                                           */
/* -------------------------------------------------------------------------- */
impl Summary for SocialPost {
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
    // replaces default implementation
    fn summarise(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

/* -------------------------------------------------------------------------- */
/* Using default implementation                                               */
/* -------------------------------------------------------------------------- */
// Having impl of trait without implementation of method -> use default impl for
// that trait's method
// but still need to implement the methods that have no default implementations
// defined in trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("{}", self.author)
    }
    // no implementation for fn summarise -> use default implementation defined
    // in trait
}

/* -------------------------------------------------------------------------- */
/* Using traits as parameters                                                 */
/* -------------------------------------------------------------------------- */
// By using impl Trait syntax -> syntactic sugar for trait bound
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarise())
}

// By using trait bound syntax
pub fn notify_again<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarise())
}

// Multiple trait bounds with + syntax
pub fn notify_and_display(item: &(impl Summary + Display)) -> String {
    format!("Breaking news! {}", item.summarise())
}
pub fn notify_and_display_again<T: Summary + Display>(item: &T) -> String {
    format!("Breaking news! {}", item.summarise())
}

// Using where clause to clarify trait bounds

// Before
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

// After
fn some_function_but_clearer<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

/* -------------------------------------------------------------------------- */
/* Returning types that implement traits                                      */
/* -------------------------------------------------------------------------- */
// - Use impl Trait to return a single type
//   so can't return different types even though they implement the same Trait
// - useful for closures and iterators that create long types, so impl Trait
//   syntax makes signatures more concise
fn returns_summarisable() -> impl Summary {
    SocialPost {
        username: String::from("random"),
        content: String::from("what's up"),
        reply: false,
        repost: false,
    }
}

/* -------------------------------------------------------------------------- */
/* Using trait bounds to conditionally implement methods                      */
/* -------------------------------------------------------------------------- */
struct Pair<T> {
    x: T,
    y: T,
}

// methods implemented for any generic type T
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// methods implemented only for types T that implement specific traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Blanket implementations                                                    */
/* -------------------------------------------------------------------------- */
// trait implemented for any type implementing another trait
trait Shout {
    fn shout(&self) -> String;
}

// Blanket impl: any type that implements Display also gets Shout
impl<T: Display> Shout for T {
    fn shout(&self) -> String {
        format!("{}!", self.to_string().to_uppercase())
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarise());
}
