# Error Handling
Rust groups `errors` into two main categories: `recoverable` and `fatal`. In case of `recoverable errors`, such as "*File not found*" it is advisable to report the problem to the user and repeat the operation. `Fatal errors` are always symptoms of defects, such as an attempt to access a position behind outside the array.
There are no `exceptions` in Rust. Instead of Therefore, it has a `Result<Т, E>` type for recoverable errors and a macro command `panic!`, which stops execution when the program encounters a fatal error.
## Fatal errors and the `panic!`
Example to raise `panic!` in a simple program:
```rust
fn main() {
    panic!("Hard fail!");
}
```
### Using a traceback when calling panic!
A `backtrace` is a list of all the functions that were called to get to this point. `Backtraces` work the same way in Rust as they do in other languages. The key to reading a `backtrace` is to start at the top and read until you see the files you wrote to.
```bash
rustc program.rs
# This is running of program with the backtrace
RUST_BACKTRACE=1 ./program
```
And instread of output
```
thread 'main' panicked at test.rs:2:5:
Hard fail!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
We will get more detailed information about `Error`
```
thread 'main' panicked at test.rs:2:5:
Hard fail!
stack backtrace:
   0: std::panicking::begin_panic
   1: test::main
   2: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
## `Recoverable` errors using `Result`
`Result` is defined as an `enum` having two options - `Ok` and `Err`, as follows. `T` and `E` are parameters of generic types.
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
`T` represents the value type, which will be returned inside the `Ok` option if successful, and `E` is the `error` type that will be returned inside the `Err` option if it fails.
```rust
let f = File::open("hello.txt");
match f {
    Ok(smth) => {
        println!("File opened successfully");
        smth
    },
    Err(error) => {
        panic!("Froblems opening the file: {:?}", error)
    },
};
```
## Using a match expression with various errors
Example of handling different types of errors in different ways
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(newFile) => newFile,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```
Code above is equivalent to this code (by using closures (method `.unwrap_or_else()`))
```rust
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```
If the value is `Result` is an `Ok` option, then the `unwrap` method will return the value inside `Ok`. If `Result` is equal to the `Err` option, the unwrap method will invoke the macro command `panic!`. Here is an example of the unwrap method in action:
```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
Another method, expect, similar to unwrap, also allows you to select the error message of the macro command panic!.
```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("Problem openning File hello.txt");
}
```
## Error propagation
When you write a function whose implementation calls something that might not work, instead of handling the error inside that function, you can return the error to the calling code, and it will decide what to do. This is called error `propagation`, it gives the calling code more control over where, possibly, more information or algorithms dictate how the error should be handled, rather than what you have in the context of the code.
A function that returns errors to the calling code using the match expression
```rust
use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let s = read_username_from_file();
    println!("{:?}", s);
}
```
This code is equivalent to this code
```rust
use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let s = read_username_from_file();
    println!("{:?}", s);
}
```
If I put the operator `?` after a value of type `Result`, it will work almost the same way as the match expressions that we defined for processing values. Result in first example. If the value of the `Result` type is `Ok`, then the value inside `Ok` will be returned from this expression and the program will continue working. If the value is `Err`, then `Err` will be returned from the entire function, as if we had used the `return` keyword, and the `error` value will be propagated to the calling code.
Building method calls in a chain after the operator
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```
The operator `?` it can only be used in functions that have a return type of `Result`, because it is defined to work in the same way as the match expression.
Example where we will get an `Error` if we apply the operator `?` in the `main` function, which, returns the type `()`
```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt")?;
}
```
Result of the compiling: **error**[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option`.
Sp, correct code should be like this
```rust
use std::error::Error;
use std::fs::File;
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```
Example of the Guess type, which will only work with values between 1 and 100.
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
    pub fn value(&self) -> i32 {
        self.value
    }
}
```
