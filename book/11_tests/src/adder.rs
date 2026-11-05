pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // attribute to indicate test fn
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); // macro to assert equality
    }

    #[test]
    #[ignore = "just because"] // attribute to ignore test
    fn should_fail() {
        panic!("Make this test fail");
    }
}
