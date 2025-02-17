# Lesson 4 (Using structures for related data)
## Introduction
A **structure**, or **struct**, is a configurable data type that allows you to name and package together several related values that make up a semantic group.

**Example of struct**
```rust
struct User
{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
This is part of **initiation of struct** and **using it**
```rust
// We make user1 changable ...
let mut user1 = User{
    username: String::from("someusername123"),
    email: String::from("4iMlU@example.com"),
    sign_in_count: 1,
    active: true,
};
// Here is changing of field in the struct
user1.email = String::from("anotheremail@example.com");
```
We also can create another user `user2` by using fields of `user1`
```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```
The code above is equivalent to this code:
```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```
## Tuple structures
**Example**
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
println!("black = x: {}, y: {}, z: {}", black.0, black.1, black.2);
println!("origin = x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);
```
**Example of calculating area by using tuple struct**
```rust
fn main(){
    let rect = (30, 50);
    println!(
        "Area of rectangle with width {} and height {} is {}",
        rect.0, rect.1, area(rect)
    );
}
fn area(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}
```
**Example of calculating area by using struct**
```rust
fn main(){
    let rect = Rectangle{ width: 30, height: 50};
    println!(
        "Area of rectangle with width {} and height {} is {}",
        rect.width, rect.height, area(&rect)
    );
}
// Here is structure Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}
// Function for calculation of Rectangle (passing rectangle without taking it for ownership (by link))
fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
```
We can add usefull **functionality** for our structure
```rust
struct Rectangle {
    width: u32,
    height: u32,
}
println!("rect1 равен {}", rect1);
```
Here we'll got an error: **error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`**, so we need to print debug information `#[derive(Debug)]` by using format string  `{:?}`.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
println!("rect1 равен {:?}", rect1);
```
## Syntax of methods
Methods are similar to functions: they are declared using the `fn` keyword and name, they can have parameters and a return value, they contain code that is executed when they are called from another location.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// impl (method implementation)
impl Rectangle {
    fn area(&self) -> u32 {
        // Context of implementation
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "Area of rectangle is equal {} square pixels.",
        rect1.area()
    );
}
```
## Linked functions
Another useful property of impl blocks is that we are allowed to define functions in impl blocks that do not take self as a parameter.
```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
let sq = Rectangle::square(3);
```
The syntax `::` is used for both related functions and namespaces created by modules.
## Multiple impl blocks
Each structure can have several impl blocks.
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
   impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```