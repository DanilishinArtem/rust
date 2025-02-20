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
## Types: defining collaborative behavior
223