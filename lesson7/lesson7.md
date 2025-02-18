# Shared collections
Unlike the built-in `array` and `tuple` types, the data pointed to by these `collections` is stored on the heap, meaning there is no need to know the amount of data at compile time, and it increases or decreases during program execution.
In this section, we will discuss three collections that are very often used in Rust programs:
- A vector allows you to store a variable number of values side by side.
- A string is a collection of characters.
- Hashable mapping allows you to associate a value with a specific key. It is a specific implementation of a more general data structure called a map.

## Storing lists of values using vectors
Creating a new empty vector for storing values of type `i32`
```rust
let v: Vec<i32> = Vec::new();
```
Creating a new vector containing the values
```rust
let v = vec![1, 2, 3];
```
Using the push method to add values to a vector
```rust
v.push(5);
v.push(6);
v.push(7);
```
Reading vector elements
```rust
fn main(){
    let v = vec![1, 2, 3, 4, 5];
    // first way
    let third: &i32 = &v[2];
    // second way to read by index `v[2]`
    println!("v[2]: {}, third: {}", v[2], third);
    // third way to use .get method
    match v.get(2){
        Some(i) => println!("Third element: {}", i),
        None => println!("There is no third element"),
    }
}
```
Iterating through values in a vector
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```
Using an enumeration to store multiple types

173