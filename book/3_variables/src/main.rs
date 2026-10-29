fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The value of x is: {x}");

    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut count = 0;
    let x = 'counting_up: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count;
            }
            remaining -= 1;
        }
        count += 1;
    };

    println!("The value of x is: {x}");

    count = 3;
    while count != 0 {
        count -= 1;
    }

    println!("Lift off!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}")
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("Lift off! (Again)");

    let f: f32 = 32.0;
    println!("{f} fahrenheit in celsius: {}", f_to_c(f));
    let c: f32 = 0.0;
    println!("{c} celsius in fahrenheit: {}", c_to_f(c));
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f32) -> f32 {
    (c * 1.8) + 32.0
}
