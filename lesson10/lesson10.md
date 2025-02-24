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
        if value < 1  {
            panic!("Value of guess should be more then 1, got {}", value);
        } else if value > 100 {
            panic!("Value of guess should be less then 100, got {}", value);
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
    #[should_panic(expected = "Value of guess should be between 1 and 100.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
## Using the Result<Т, E> type in tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 is not equal 4"))
        }
    }
}
```
When you run multiple tests, by default they run in parallel using threads of execution. This means that the tests will complete faster, making it easier for you to get feedback on whether the code is working. Since the tests are running simultaneously, make sure that they do not depend on each other or on any shared state, including the shared environment, such as the current working directory or environment variables.
If you do not want to run the tests in parallel or want to more precisely control the number of execution threads used, you can send the --testthreads flag and the number of execution threads you want to use to the test binary file.
`
cargo test -- --test-threads=1
`
## Showing the results of the function
```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I've got value {}", a);
    10
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```
If we run `cargo test` then we'll get in result calling this line `let value = prints_and_returns_10(4);` and message `I've got value 8`, but if we run `cargo test --nocapture` tnen we'll get both lines `I've got value 4` and `I've got value 8`.
## Running a subset of tests by name
Sometimes it takes a long time to run a complete set of tests. If you are working with code in a specific area, you may need to perform only those tests that relate to that code. You can select the tests to be performed by giving the cargo test command the name or names of the tests you want to run.
```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```
Result of the command `cargo test` is:
```
running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Result of the command `cargo test one_hundred` is:
```
running 1 test
test tests::one_hundred ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```
## Filtering in order to perform multiple tests
270