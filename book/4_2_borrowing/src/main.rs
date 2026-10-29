fn main() {
    /* ---------------------------------------------------------------------- */
    /* Borrowing                                                              */
    /* - Borrowing is the action of creating a reference.                     */
    /* - Just like variables, references are immutable by default.            */
    /* - A reference's scope starts from where it is introduced,              */
    /*   until the last time it is used.                                      */
    /* - The value pointed to by the reference is not dropped when            */
    /*   the reference stops being used, because the reference                */
    /*   does not have ownership.                                             */
    /* - Functions taking references as parameters have no need               */
    /*   to return values in order to give back ownership,                    */
    /*   because ownership was never had to begin with.                       */
    /* ---------------------------------------------------------------------- */

    /* ---------------------------------------------------------------------- */
    /* Rules of References                                                    */
    /* - At any given time, you can have either one mutable reference, or     */
    /*   any number of immutable references.                                  */
    /* - Rust invalidates references to deallocated data (so no dangling).    */
    /* ---------------------------------------------------------------------- */

    let s1 = String::from("hello");

    // create a reference, &s1 = s in the calculate fn
    // this reference refers to the value of s1, but does not own the value itself
    let len = calculate_length_by_ref(&s1);
    // therefore when s goes out of scope in the calculate fn,
    // because s does not have ownership of the value it refers to, the String is not dropped
    println!(
        "borrowing allowing using reference to s1 as parameter to fn, moving 'ownership' of the reference, but not the value\n\t{s1} -> {len}"
    );

    /* -------------------------------------------------------------------------- */

    let mut s = String::from("hello");
    change(&mut s);
    println!("mutable reference to change value referred to by parameter &mut s within fn\n\t{s}");

    /* -------------------------------------------------------------------------- */

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        change(r1);
        println!(
            "changing the value by mutable reference to s (&mut s) in an inner scope (curly braces) r1\n\t{r1}"
        )
    }
    let r2 = &mut s;
    println!("a different mutable reference to the same changed s (&mut s) r2\n\t{r2}");

    /* ---------------------------------------------------------------------- */

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!(
        "introduce and use (make in-scope) multiple immutable references to same mutable value (&mut s) r1, r2\n\t{r1}, {r2}"
    ); // scope of r1 and r2 end here, which is before r3 is created

    // scopes don't overlap, so code compiles
    let r3 = &mut s;
    println!(
        "before introducing and using the mutable reference to same mutable value (&mut s) r3\n\t{r3}"
    )
}

/* Borrowing helper functions                                                 */

fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!")
}
