# Generalized types, typologies, and life cycle
**Generalizations** are abstract substitutes for specific types or other properties. When we write code, we can express the behavior of generalizations or their relationships to other generalizations without knowing what will take their place during compilation and code execution.
## Removing repetitions by extracting a function
The code for finding the largest number in two lists of numbers
```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Maximum number: {}", largest);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Maximum number: {}", largest);
}
```
This code works. However, the need to repeat the code is exhausting, and errors are possible. If we want to make changes to the code, we also have to update it in several places. In order to eliminate this repetition, we can create an abstraction by defining a function that works with any list of integers passed to it in a parameter. This solution makes the code clearer and allows you to express the idea of finding the largest number in the list abstractly.
Abstract code for finding the largest number in two lists
```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Maximum number: {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("Maximum number: {}", result);
}
```
## Generalized data types
Example shows how to define a `Point<Т>` structure for storing `x` and `y` coordinate values of any type.
```rust
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```
Note that since we used only one generalized type to define `Point<Т>`, this definition suggests that the structure `Point<Т>` is a generalization over a certain type of `T`, and fields `x` and `y` have the same type, whatever it may be.
`Point<Т, U>` is a generalization of two types, as a result of which `x` and `y` can be values of different types.
```rust
struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    let sample = Point { x: 5, y: 10.0 };
}
```
### In the definitions of enumerations
As with structures, we can define enumerations that contain generalized data types in their variants.
```rust
enum Option<T> {
    Some(T),
    None,
}
```
Enumerations can also use more than one generalized type.
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
### In the definitions of methods
We can implement methods in `structures` and `enumerations`, as well as use `generalized` types in their definitions.
Implementation of the `x` method in the `Point<Т>` structure, which will return a reference to a field `x` of type `T`
```rust
struct Point<T> {
    x: T,
    y: T,
}
// This is implementation of method for generalized type
impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}
// This is implementation of method for definded type 
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```
The `Rust` language implements `generalized types` in such a way that, using them, the code does not run slower than when using specific types. Rust achieves this by `monomorphizing` code that uses `generalized methods` at compile time. `Monomorphization` is the process of turning `generalized code` into concrete code by inserting specific types that are used during compilation.
```rust
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}
fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
Since `Rust` compiles `generalized code` into code that specifies the type in each instance, it does not affect the execution time due to the use of generalizations.
## `Traits`: defining collaborative behavior
`Traits` are similar to a tool that is often called `interfaces` in other languages, although with some differences.
`Trait` Summary type, whose behavior is provided by the summarize method
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```
Here we declare a `trait` using the keyword `trait` followed by the `type name`, which in this case is `Summary`. Inside curly brackets, we declare `method` signatures that describe the behavior of types that implement this `type`.
Implementation of the `Summary` type in the `NewsArticle` and `Tweet` types
```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String, 
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
The implementation of a `trait` in a type is similar to the implementation of ordinary methods. The difference is that after the `impl` keyword, we put the name of the type we want to implement, then we use the `for` keyword, and then we specify the name of the type in which we are going to implement the type.
After the realisation of `trait`, methods can be called for instances of the types `NewsArticle` and `Tweet` in the same way as we call the usual methods, for example
```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("Of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};
println!("one new tweet: {}", tweet.summarize());
```
### Default implementations
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Other read...)")
    }
}
```
To use the `default implementation` for summarizing instances of the `NewsArticle` type, instead of defining a custom implementation, we specify an empty `impl` block with the statement 
```rust
impl Summary for NewsArticle {}.
```
### `Traits` as parameters
```rust
pub fn notify(item: impl Summary) {
    println!("Срочные новости! {}", item.summarize());
}
```
Instead of a specific `type` for the item parameter, we specify the keyword `impl` and the name of the `trait`. This parameter accepts any type that implements the specified `trait`.
### Specifying multiple `trait` boundaries using the `+` syntax
We can also specify multiple `trait` boundaries. Let's say we want the notify function to use the `Display` formatting for item as well as the `Summarize` method.
```rust
pub fn notify(item: impl Summary + Display) {...}
```
We can also use the `impl Trait` syntax in the return position to return a value of some type that implements the `trait`, as shown here:
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```
### Fixing the largest function with `trait bounds`
Types like `i32` and `char`, which have a known size, are stored on the stack, so they implement the `Copy` trait. But when we made the largest function generic, it became possible for the parameter list to have types in it that do not implement the `Copy` trait. As a result, we cannot move the value from list[0] to the `largest` variable, which is what causes this error.
A working definition of the largest function, which works on any generic type implementing the `PartialOrd` and `Copy` traits
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```
### Using `Trait Bounds` to Conditionally Implement Methods
By using a `trait` with an `impl` block that uses generic type parameters, we can conditionally implement methods on types that implement the listed traits. For example, the `Pair<Т>` type always implements the `new` function. But `Pair<Т>` implements the `cmp_display` method only if its inner type `T` implements the `PartialOrd` trait, which allows comparison, and the `Display` trait, which allows printing.
```rust
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest term x is equal to {}", self.x);
        } else {
            println!("The largest term y is equal to {}", self.y);
        }
    }
}
```
## Generalized life cycles in functions
The main function, which calls the longest function to find the longest string slice of two
```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("Самая длинная строка равна {}", result);
}
```
An implementation of a `longest` function that returns the longest of two string slices, but which does not yet compile
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
The following error occurs, which is about the life cycle: **error**[E0106]: missing lifetime specifier
So we should use `generalized life cycles`.
Lifecycle annotations have a slightly unusual syntax: parameter names must start with a single quote `(')`. They are usually all lowercase and very short, like generics. Most people use the name `'a`
```rust
&i32 // Link
&'a i32 // A link with an explicit life cycle
&'a mut i32 // Mutable link with an explicit life cycle
```
Definition of the function `longest`, which states that all references in the signature must have the same lifetime `'a`
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
## Thinking in terms of life cycles
242