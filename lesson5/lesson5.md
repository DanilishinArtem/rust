# Enumerations and pattern matching
Let's say we need to work with IP addresses. Currently, two main standards are used for IP addresses: version four and version six. These possibilities for an IP address are the only ones that the program will encounter.
we can solve this problem by defining the IpAddrKind enumeration and listing the possible types of IP addresses, V4 and V6. They are called enumeration options:
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
We can create instances of each of the two enumeration options `IpAddrKind` as follows
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
We can implement the idea in several ways:
- By using additional struct `IpAddr`
```rust
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```
- By using tuple struct
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```
- By using user defined structures
```rust
struct Ipv4Addr {
    // --defining--
}
struct Ipv6Addr {
    // --defining--
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
## Option enumeration and its advantages over null values
The `Option` type is used in many places because it encodes an extremely common scenario in which a value may be something or may not be anything. The Rust language does not have a language design for `null`, which is found in many other languages. `Null` is a value that implies that there is no value.
The `Rust` language does not have `null`, but it does have an enumeration that encodes the idea of a present or missing value. This enumeration is `Option<T>`, and the standard library defines it as follows:
```rust
enum Option<T> {
    Some(T),
    None,
}
```
We can use `Some` and `None` directly without the `Option::` prefix. The `Option<T>` enumeration is still a regular enumeration, and `Some(T)` and `None` are still variants of the `Option<T>` type.
Here are some examples of using `Option` enumeration values for numeric and string types:
```rust
let some_number = Some(5);
let some_string = Some("строковый литерал");
let absent_number: Option<i32> = None;
```
If we use None instead of `Some`, then we need to specify which type of `Option<T>` we have, because the compiler cannot logically deduce the type that Some option will contain only from the value `None`.
## The `match` expression as a flow control statement
139