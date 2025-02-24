# Lesson 2 (Concepts of programming)
## Variability of variables

Example with unchangable variable:
```rust
fn main() {
 let x = 5;
 println!("Значение x равно {}", x);
 x = 6;
 println!("Значение x равно {}", x);
}
```
As a result we'll get ```Error```:  ***error[E0384]: cannot assign twice to immutable variable `x`***
We should use changable variable:
```rust
fn main() {
 let mut x = 5;
 println!("Значение x равно {}", x);
 x = 6;
 println!("Значение x равно {}", x);
}
```
Example of defining `const` variable:
```rust
const MAX_POINTS: u32 = 100_000;
```
## Shading (Затенение)
In Rast, we can declare a new variable with the same name as the previous variable, and the new variable will shade the previous one. Shadowing is different from labeling a variable as a `mut`. Using let, we can perform several transformations of the value, but after completing the transformations, the variable will be immutable.
Example:
```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("Значение x равно {}", x);
}
```
Another difference between mut and shadowing is that since we are practically creating a new variable when we use the let keyword again, we can change the type of the value by using the same name repeatedly.
Example:
```rust
let spaces = " ";
let spaces = spaces.len();
```
However, if we try to use mut for this, as shown here, we will get a compile-time error ***"error[E0308]: mismatched types"***:
```rust
let mut spaces = " ";
spaces = spaces.len();
```
## Data types

#### Integer type

Integer types in the Rust language

| Length | Signed | Unsigned |
|--------|--------|----------|
|  8bit  |   i8   |    u8    |
| 16bit  |  i16   |    u16   |
| 32bit  |  i32   |    u32   |
| 64bit  |  i64   |    u64   |
| 128bit |  i128  |    u128  |
| arch   |  isize |    usize |

#### Floating point type
There are two types of floating point numbers: f32, f64.
Example:
```rust
let x: f32 = 2.0; // f32
let y: f64 = 3.0; // f64
```
#### Boolean type
```rust
let t = true;
let f: bool = false; // с явной аннотацией типа
```
#### Symbolic type
```rust
let c = 'z';
let heart_eyed_cat = '😻';
```
#### Tuple type
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
println!("First number: {}, second number: {}, third number: {}", tup.0, tup.1, tup.2);
```
#### Array type
```rust
let a = [1, 2, 3, 4, 5];
let b: [i32; 5] = [1, 2, 3, 4, 5];
println!("array a = [{}, {}, {}, {}, {}]", a[0], a[1], a[2], a[3], a[4]);
println!("array b = [{}, {}, {}, {}, {}]", b[0], b[1], b[2], b[3], b[4]);
```
#### Functions
For Rust it doesn't matter where function is defined, it's care only that function is defined.
```rust
fn main() {
    println!("Main Function");
    another_function();
}
fn another_function() {
    println!("Another Function");
}
```
***Parameters of the function***
```rust
fn main() {
    another_function(5);
}
fn another_function(x: i32) {
    println!("Value x: {}", x);
}
```
#### Instructions and expressions in function bodies
- ***Instructions*** - commands for the computer to perform a certain action, they do not return a value.
- ***Expression*** - evaluates the resulting value.
```rust
let y = 6; // This is instruction
```
This is example has an `error`, because instruction doesn't return value, so we can't to set instruction `let` to another variable. 
```rust
et x = (let y = 6);
```
Macrocomand calling is a part of expression. The block that we use to create new areas, `{}`, is an expression, for example:
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("Значение y равно {}", y);
}
```
#### Functions with returning values
```rust
fn five() -> i32 {
    return 5;
}
fn main() {
    let x = five();
    println!("Значение x равно {}", x);
}
```
#### If expression
```rust
fn main(){
    let x = 3;
    if x < 5 {
        println!("Expression should be True");
    }else{
        println!("Expression should be False");
    }
}
```
Expression in the `If` should be boolean, here is an example with an `Error`, because type is `int` but not `boolean`
```rust
fn main() {
    let number = 3;
    if number {
        println!("Number should be equal three");
    }
}
```
***Expression `if` with several `else`***
```rust
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("Number devided by 4");
    } else if number % 3 == 0 {
        println!("Number devided by 3");
    } else if number % 2 == 0 {
        println!("Number devided by 2");
    } else {
        println!("number not devided by 4, 3 and 2");
    }
}
```
***Using `if` in instruction `let`***
```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("Value of a number = {}", number);
}
```
***Repeating by using cycles***
- `loop`
- `while`
- `for`

**[1]** The `loop` keyword says that you need to execute a block of code over and over an infinite number of times or until you tell it to stop.
Example:
```rust
fn main() {
    loop {
    println!("One more time!");
    }
}
```
**[2]** `while` is often useful when a program evaluates a condition inside a loop. And as long as the condition is true, the loop is executed.
```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("Number {}", number);
        number = number - 1;
    }
    println!("End of while");
}
```
**[3]** `for` is using for loop and execute the code for each item in the collection.
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Value = {}", element);
    }
    // reverce indexing
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
```
