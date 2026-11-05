#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(coin: &Coin) -> u8 {
        /* ------------------------------------------------------------------ */
        /* match                                                              */
        /* ------------------------------------------------------------------ */
        // match = exhaustive
        // - arms must cover all possibilities
        // arm = pattern => some expression
        // - expression = single line? separate by comma
        // - expression = multiple lines? encapsulate in curly brackets, comma optional
        // pattern match -> execute expression
        // pattern not match -> continue to next arm
        // return value of match = resultant value of expression in matching arm
        /* ------------------------------------------------------------------ */
        /* match: Ownership                                                   */
        /* ------------------------------------------------------------------ */
        // match by value -> moves ownership
        // match by reference -> borrows
        // wildcard _ does not bind inner value -> does not move
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
}

fn main() {
    let coin = Coin::Penny;
    let value = Coin::value_in_cents(&coin);
    println!("The value of {coin:?} is {value} cent(s).");

    /* ---------------------------------------------------------------------- */

    let coin = Coin::Quarter(UsState::Alaska);
    let value = Coin::value_in_cents(&coin);
    println!("The value of {coin:?} is {value} cent(s).");

    /* ---------------------------------------------------------------------- */
    /* Option<T>                                                              */
    /* ---------------------------------------------------------------------- */

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    /* ---------------------------------------------------------------------- */
    /* Catch-all arm using _                                                  */
    /* ---------------------------------------------------------------------- */
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), // variable for any other value
        _ => (), // catch-all with _ + do nothing with ()
    }

    /* ---------------------------------------------------------------------- */
    /* if let                                                                 */
    /* ---------------------------------------------------------------------- */
    // Non-exhaustive single match arm, ignoring all other values
    // - let -> declaring a variable (so we can destructure)
    // - if -> checking if the variable expression holds / is valid
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // Otherwise, use match with more boilerplate, due to needing to be exhaustive
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // We can add an else for if let
    let mut count = 0;
    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = &coin {
        println!("Definitely a quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // equivalent to the following match expression
    let coin = Coin::Dime;
    match &coin {
        Coin::Quarter(state) => println!("Definitely a quarter from {:?}!", state),
        _ => count += 1,
    }

    println!("count: {count}"); //should be 2
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}

/* -------------------------------------------------------------------------- */
/* let...else                                                                 */
/* -------------------------------------------------------------------------- */

fn describe_state_quarter(coin: Coin) -> Option<String> {
    /* Using nested conditionals -> hard to read/see how top branches relate  */

    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!("{state:?} is pretty old, for America!"))
    //     } else {
    //         Some(format!("{state:?} is relatively new."))
    //     }
    // } else {
    //     None
    // }

    /* Using if let -> weird flow                                             */

    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     return None; // return early if not Quarter
    // };

    // if state.existed_in(1900) {
    //     Some(format!("{state:?} is pretty old, for America!"))
    // } else {
    //     Some(format!("{state:?} is relatively new."))
    // }

    /* Using let...else                                                       */
    // Pattern on LHS = Expression on RHS
    // No if branch, just else
    // If pattern match -> bind value from pattern in outer scope
    // If pattern not match -> flow into else arm
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
