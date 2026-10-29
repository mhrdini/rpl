/* -------------------------------------------------------------------------- */
/* Structs                                                                    */
/* -------------------------------------------------------------------------- */
// Create custom types with associated pieces of data.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Can have multiple impl blocks, useful with generic types and traits
// Associated functions: functions defined in impl block
// Methods: associated functions with &self as first parameter, i.e. needs instance
impl Rectangle {
    // can have same name as a field
    // usually it's as a getter (only returning the value of the field itself)
    fn width(&self) -> bool {
        self.width > 0 // not a getter
        // self.width // a getter, change return type to u32
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }

    /* ---------------------------------------------------------------------- */
    /* Associated functions that are not methods                              */
    /* ---------------------------------------------------------------------- */
    // - Often used as constructors (often named new) that returns new instance
    // - Self = alias for type that appears after impl keyword
    // - To call, use :: syntax with struct name, e.g. Rectangle::square(3);
    // - Function is namespaced by the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: !dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    /* ---------------------------------------------------------------------- */
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of {:?} is {}", rect1, rect1.area());

    if rect1.width() {
        println!("It has a non-zero width, {}", rect1.width);
    }

    /* ---------------------------------------------------------------------- */
    /* Automatic referencing and dereferencing                                */
    /* ---------------------------------------------------------------------- */
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    let square = Rectangle::square(20);

    // When calling a method with object.something(),
    // Rust automatically adds in &, &mut, or * so that object matches the
    // signature of the something method

    // The following is the same:
    rect1.can_hold(&rect2);
    (&rect1).can_hold(&rect2);

    println!("Can square hold rect2? {}", square.can_hold(&rect2));
}
