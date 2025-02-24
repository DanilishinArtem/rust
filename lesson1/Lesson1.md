# Lesson 1 (Beginnings)
## Hello world example
main.rs has the follow strusture:
```rust
fn main() {
    println!("Hello, world!");
}
```
println! - ```"!"``` means than calling macrocommand, without ```"!"``` calling function.

## Cargo package manager
**Cargo**  - is system of building and package manager of the Rust. Most of Rust users use this tool for managing Rust projects, because Cargo does a lot of additional work instead of user (for example building project, downloading libraries and dependensies, building libraries). For creating the project by using Cargo we should do command ```cargo new project_name```. Cargo creates this kind of structure:
```plaitext
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        ├── examples
        ├── project_name
        ├── project_name.d
        └── incremental
```

Cargo reates TOML file (*Tom’s Obvious, Minimal Language*). For buiding project ```cargo build``` command creates executable file in the folder *./target/debug/project_name*. For compiling the code we should run ```cargo run```. For checking code we should run ```cargo check``` (this command check all dependences and source conde for possibilities of compiling  (without of creating binary file)). 
Command ***cargo build --release*** for compilation and optimization.

## Game of guessing number
***Part of reading number from user***
```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please, enter the guessing number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error in reading line");
    // Converting the String type of guess to unsigned type int
    let guess: u32 = guess.trim().parse().expect("Please, type a number");
    println!("You guess number: {}", guess);
}
```
***Part of generating secret number***
```rust
use std::cmp::Ordering;

fn main() {
    // Random secret number generation
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // Previous code
    // ...
    // Part of comparison two numbers
    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
}
```
#### Types of variables (by default all variable are not changable):

***Not changable variable***
```rust
let foo = 5;
```
***Changable variable***
```rust
let mut foo = 5;
```
 ***io::Result***
method read_line return io::Result which is enum. For type Result variants are ```Ok``` and ```Err```. Result has field ```expect```. If there is an Error in read_line method, then Result ignore Ok field and return Err field. 
***Converting to another type***
```rust
let guess: u32 = guess.trim().parse().expect("Please, type a number");
```
Method ```.trim()``` deletes all spaces in the string at the begin and at the end. Method ```.parse()```, which is defined for String type convert variable to another type, as we defined new type as u32, ```.parse()``` method convert to u32 type, and also this method return Result.

#### Cycle game by using infinite loop
```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop 
    {
        println!("Please, type a number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error in reading line");
        let guess: u32 = guess.trim().parse().expect("Please, type a number");
        println!("You guess number: {}", guess);
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => 
            {
                println!("You win");
                break;
            }
        }
    }
}
```
The ```match``` expression consists of branches. The arm consists of a template and code that must be executed if the value set at the beginning of the match expression matches the branch template.
#### Processing invalid data entry
```rust
// --skipping---
io::stdin().read_line(&mut guess).expect("Error in reading line");
let guess: u32 = match guess.trim().parse()
{
    Ok(num) => num,
    Err(_) => continue;
};
println!("You guess number: {}", guess);
// ---skipping---
```