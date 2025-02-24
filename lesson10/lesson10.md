# Automized tests
`Tests` are functions that verify that non-test code functions in the expected way. The test function bodies usually perform the following actions:
1. Configure the necessary data or status.
2. Execute the code that you want to test.
3. Confirm that the results meet expectations.
## The anatomy of the testing function
When we create a new library project using `Cargo`, a `test module` with a `test function` inside is automatically generated. This module helps you write tests, so you don't have to search for the exact structure and syntax of test functions every time you start a new project.
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
The `cargo test` command performs all the tests in the project.
## Checking the results using the `assert!` macro
The `assert!` macro provided by the standard library is useful when you want to make sure that a condition in a test is set to `true`. We are giving the `assert!` macro command an argument that is calculated as a `Boolean`. If the value is `true`, then the `assert!` macro does nothing, and the test is `successful`. If the value is `false`, then the `assert!` macro calls the macro command `panic!`, which causes the test to fail. Using the `assert!` macro it helps us verify that the code is functioning exactly as we expect it to.
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        // --...--
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(!smaller.can_hold(&larger));
    }
}
```
## Checking equality using the `assert_eq!` macros and `assert_ne!`
```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```
## Adding error messages for the user
```rust
pub fn greeting(name: &str) -> String {
    format!("Здравствуй {}!", name)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Keron");
        assert!(
            result.contains("Keron"),
            "Greeting doesn't contain name, input value  `{}`", result
        );
    }
}
```
## Checking for panic using the `should_panic` attribute
```rust
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Значение догадки должно быть между 1 и 100, получено {}.", value);
        }
        Guess {
            value
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
262