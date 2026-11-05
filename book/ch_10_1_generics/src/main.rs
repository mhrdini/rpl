/* -------------------------------------------------------------------------- */
/* Generics                                                                   */
/* -------------------------------------------------------------------------- */
// - abstract stand-ins for concrete types or other properties
//   e.g. generic functions take parameters of some generic type
// - removes duplication of behaviour by parametrising types
// - Rust compiler performs monomorphisation, which replaces the generic
//   type/property with specific definitions of existing instances in the code

/* Generic function                                                           */
// PartialOrd is a trait, see 10_2_traits dir
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/* Generic enums                                                              */
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

/* Generic struct                                                             */
struct Point<T, U> {
    x: T,
    y: U,
}

/* Generic impl and methods                                                   */
// use same name for generic type in impl def as one used in generic struct def
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    // can use different parameterised types only defined in method definition
    fn mixup<X, Y>(self, other: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// can also implement methods only on concrete instances
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    println!("Hello, world!");
}
