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
197