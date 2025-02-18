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
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
When people talk about `strings` they usually mean type `String` and `string` slice type `&str`, not just one of these types. The standard library also includes a number of other `string` types, such as `OsString`, `OsStr`,`CString`, and `CStr`. Library packaging can provide even more options for storing string data.
Creating a new instance of the `String` type:
```rust
let mut s = String::new(); 
```
Using the `to_string` method to instantiate a type `String` from a `string` literal
```rust
let data = "some text";
let s = data.to_string();
// this method also works directly on a literal
let s = "some text".to_string();
```
Using the `String::from function` to create an instance type `String` from a `string` literal
```rust
let s = String::from("some text");
```
An instance of type `String` can grow in size, and its contents can change, as does the contents of an instance of type `Vec<Т>` if you add to it more data. You can also conveniently use the `+` operator or the macro comand `format!` for concatenation of String type values.
Adding a string slice to the end of a String value using the push_str method
```rust
let mut s = String::from("foo");
s.push_str("bar");
// as a result we'll get string "foobar"
```
The `push_str` method takes a string slice because we don't necessarily want to take ownership of this parameter.
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 = {}", s2);
// as a result "s2 = bar", so method .push_str() doesn't take to ownership variable s2, and we can use s2.
```
**Concatenation using the `+` operator or the `format!`**
Using the `+` operator to combine two values ​​of type `String` to new `String` value
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = s1 + "-" + &s2 + "-" + &s3; // note: s1 has been moved here and can no longer be used
```
For more complex `string` combinations, we can use the `format!` macro:
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
```
## Indexing in rows
Trying to use indexing syntax with value type `String`
```rust
let s1 = String::from("hello");
let h = s1[0];
```
We will get an **error**: error[E0277]: the trait bound `std::string::String: std::ops::Index<{integer}>`
is not satisfied
```rust
let hello = "Hello";
let answer = &hello[0];
```
This code also forbiden and we will get an **error**: error[E0277]: the type `str` cannot be indexed by `{integer}`
**Slicing strings**

```rust
let hello = "Здравствуйте";
let s = &hello[0..2];
// this line show "З" because each letter has a length 2 bytes (UTF-8), 
// and we take first two bytes, as a result we will get "З"
println!("s: {}", s); 
```
**String Iteration Methods**
Call
The `chars` method on "Здравствуйте" allocates and returns two char values, and you can iterate over the result to access each element
```rust
for c in "Здравствуйте".chars(){
    println!("{}", c);
}
```
## Storing Keys with Associated Values in hash mappings
The last common collection would be the `hash map`. Type `HashMap<K, V>` stores a mapping of keys of type `K` to values ​​of type `V`.
Creating a new `hash map` and inserting multiple keys and values:
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```
Another way to construct a `hash map` is to use `collect` method on a `vector` of `tuples`, where each `tuple` consists of a `key` and its `values`
```rust
use std::collections::HashMap;
let teams = vec![String::from("Синяя"), String::from("Желтая")];
let initial_scores = vec![10, 50];
let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
```
**Hash Mappings and Ownership**
Example shows that the `keys` and `values` ​​are owned by the `hashmap` after they are inserted
```rust
use std::collections::HashMap;
let field_name = String::from("Favorite collor");
let field_value = String::from("Синий");
let mut map = HashMap::new();
map.insert(field_name, field_value);
// This line raise ERROR, as they already in ownership in map, so we can't use them
// println!("field_name: {}, field_value: {}", field_name, field_value)
```
**Access to hash values**
Access to the Blue Team score stored in the `hash mapping`
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
// get value from map
let team_name = String::from("Blue");
let score = scores.get(&team_name);
```
We can iterate through each key/value pair of the hash mapping in the same way as we do with vectors using the for loop:
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yelloq"), 50);
for (key, value) in &scores{
    println!("{}: {}", key, value);
}
```
**Updating the hash mapping**
Replacing a saved value with a specific key
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
// Replacing value for Blue key
scores.insert(String::from("Blue"), 25);
```
Using the `entry` method to `insert` only if the key doesn't matter yet
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);
// this value will not be inserted as key already exist
scores.entry(String::from("Blue")).or_insert(50);
println!("{:?}", scores);
```
The `or_insert` method for enumerating `Entry` is defined to return a mutable reference to the value of the corresponding `Entry key`, if that `key` exists.
Counting the number of occurrences of words using a hash mapping that stores words and numbers
```rust
use std::collections::HashMap;
let text = "Hello world beautiful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    // method .or_insert() returns a mutable reference
    let count = map.entry(word).or_insert(0);
    // at here we increment a value through the mutable reference
    *count += 1;
}
println!("{:?}", map);
```