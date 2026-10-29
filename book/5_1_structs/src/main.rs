// structs
#[derive(Debug)]
struct User {
    // fields
    active: bool,
    username: String, // owned by instance
    email: String,    // owned by instance
    sign_in_count: u64,
}

// tuple struct: behaves like types, but accessible and destructurable like tuples
struct Color(i32, i32, i32);

#[derive(Debug)]
// unit-like structs: has no fields, behave similarly to ()
struct AlwaysEqual;

fn main() {
    // create a User instance
    // making a struct mutable makes all its fields mutable
    let mut user1 = User {
        active: true,
        username: String::from("user"),
        email: String::from("user@email.com"),
        sign_in_count: 1,
    };

    /* ---------------------------------------------------------------------- */

    // accessing a field value
    user1.username = String::from("sameuser");

    /* ---------------------------------------------------------------------- */

    let user2 = User {
        email: String::from("anotheruser@email.com"),
        /* Struct update syntax                                               */
        /* - Comes last for any remaining fields                              */
        /* - This moves data, so user1 is invalid after creating user2        */
        /*   because the String in the username field was moved from user1 to */
        /*   user2.                                                           */
        /* - If only active and sign_in_count values from user1 were used to  */
        /*   create user2, then user1 remains valid after creating user2.     */
        /*   This is because active and sign_in_count are types that have     */
        /*   the Copy trait, thus copying the data and not moving the data    */
        /*   from user1.                                                      */
        ..user1
    };

    // error: borrow of partially moved value: user1
    // partial move occurs because user1.username has type String,
    // which does not implement the Copy trait
    // println!("{:?}", user1);

    // but after changing username, thus removing the borrow of user1.username,
    // user1 is valid again
    user1.username = String::from("user");
    println!("{:?}", user1);
    println!("{:?}", user2);

    /* ---------------------------------------------------------------------- */

    let user3 = build_user(
        String::from("yetanotheruser@email.com"),
        String::from("yetanotheruser"),
    );

    println!("{:?}", user3);

    /* ---------------------------------------------------------------------- */

    // tuple struct
    let mut blackish = Color(0, 0, 0);

    blackish.0 = 1;

    // destructuring tuple structs
    let Color(r, g, b) = blackish;

    println!("blackish -> rgb({r}, {g}, {b})");

    /* ---------------------------------------------------------------------- */

    // unit-like struct: used as markers, to implement a trait without
    // assigning any data
    let subject = AlwaysEqual;

    println!("{:?}", subject);

    /* ---------------------------------------------------------------------- */
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // field init shorthand: used when var has same name as field
        username,
        email,
        sign_in_count: 1,
    }
}
