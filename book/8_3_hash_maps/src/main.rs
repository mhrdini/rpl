/* -------------------------------------------------------------------------- */
/* Hash maps: HashMap<K, V>                                                   */
/* -------------------------------------------------------------------------- */
// - mapping of keys of type K to values of type V using hashing fn
// - hashing fn determines how keys and values are placed in memory
// - data stored in heap
// - all keys of same type, all values of same type

use std::collections::HashMap;

fn main() {
    // K and V types are inferred only if empty hash map is inserted into
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /* ---------------------------------------------------------------------- */
    /* Accessing values                                                       */
    /* ---------------------------------------------------------------------- */
    // - using get method on HashMap, borrowing the key value as parameter
    // - get method returns Option<&V>
    let team_name = String::from("Blue");
    // - copied = turns Option<&V> -> Option<V>
    // - unwrap_or = set to a default value if entry doesn't exist
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for team {team_name}: {score}");

    /* ---------------------------------------------------------------------- */
    /* Iterating over key-value pairs                                         */
    /* ---------------------------------------------------------------------- */
    // in arbitrary order
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /* ---------------------------------------------------------------------- */
    /* Ownership                                                              */
    /* ---------------------------------------------------------------------- */
    // - Copy trait annotated -> copied, otherwise -> moved and owned by hash
    //   map thereafter
    // - If we insert references into the hash map, the values are not copied or
    //   moved, but the values must be valid for at least as long as the
    //   references
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Green");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value no longer usable here

    /* ---------------------------------------------------------------------- */
    /* Updating: Overwriting a value                                          */
    /* ---------------------------------------------------------------------- */
    // insert 25 score for team "Blue" in scores hash map
    scores.insert(team_name, 25); // replace previous entry
    println!("{scores:?}");

    /* ---------------------------------------------------------------------- */
    /* Updating: Getting, otherwise adding if entry doesn't exist             */
    /* ---------------------------------------------------------------------- */
    // - entry method returns Entry enum that represents a value that might or
    //   might not exist
    // - or_insert returns a mutable reference (&mut V) to the value for the
    //   specified key
    scores.entry(String::from("Green")).or_insert(30); // new entry -> insert
    scores.entry(String::from("Blue")).or_insert(40); // existing entry -> no insert
    println!("{scores:?}");

    /* ---------------------------------------------------------------------- */
    /* Updating: Based on the old value                                       */
    /* ---------------------------------------------------------------------- */
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // split_whitespace method returns an iterator over subslices, separated by
    // whitespace
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
