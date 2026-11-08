pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// NOTE: library crates probably shouldn't print
// should only compute and return data -> easier to write, test, and use
pub fn world() -> String {
    String::from("Hello, world!")
}

#[test]
fn always_passes() {}

#[test]
#[ignore = "just because"]
fn always_fails() {
    panic!("oh no");
}

#[test]
fn add_2_and_2_returns_4() {
    let result = add(2, 2);
    // * (function(params)): want (expected), got (output)
    assert_eq!(result, 4, "add(2, 2): want 4, got {result}");
}

#[test]
fn world_fn_returns_hello_world() {
    let want = "Hello, world!";
    let got = world();
    assert_eq!(want, got, "world: want '{want}', got '{got}'");
}
