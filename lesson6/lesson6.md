# Managing growing projects with packages, packages, and modules
The Rust language has a number of tools to manage the organization of code, including which details are demonstrated, which details are confidential, and which names are found in each program area.
- **Package (Пакет):** Cargo Package Manager, which allows you to create, test, and share packages.
- **Crate (Упаковка):** A module tree that produces a library or executable file.
- **Modules and use:** They allow you to manage the organization, visibility, and privacy of paths.
- **Ways:** The method of naming an element, such as a structure, function, or module.
## Packages and packages
A `Crate` is a binary or library file. A `Package` is one or more packages that provide functionality. The package contains the Cargo.toml file, which describes how to create these packages.
The `Package` must include **zero or one library package**, but no more. It can contain as many binary packages as you need, but it must contain at least one `Package` (library or binary).
## Defining modules for scope and privacy management
In this section, we will talk about modules and other parts of the modular system, namely the paths that allow you to name elements, the `use` keyword that introduces the path into the scope, and the `pub` keyword that makes elements public. We will also discuss the `as` keyword, external packages, and the `glob` operator. For now.
## Paths to link to an element in the module tree
**The path takes two forms:**
- The absolute path starts from the root of the package, using the package name or the literal `crate`.
- The relative path starts from the current module and uses the `self`, `super`, or identifier in the current module.

Adding the pub keyword to mod hosting and fn add_to_waitlist allows you to call the function from eat_at_restaurant

**src/lib.rs**
```rust
mod front_of_house {
    // make this module public for possibility to use it
    mod hosting {
        // make this function public for possibility to use it
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```
## Starting relative paths using super
We can also construct relative paths that start in the parent module by using the `super` keyword at the beginning of the path.
**Example:** Calling a function using a relative path starting with `super`

**src/lib.rs**
```rust
fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super as like beginning of the folder
        super::serve_order();
    }
    fn cook_order() {}
}
```
The `fix_incorrect_order` function is located in the `back_of_house` module, so we can use `super` to navigate to the parent `back_of_house` module, which in this case is `crate`, the root.
## Designating structures and enumerations as public
Designating an enumeration as `public` makes all its variants `public`.

**src/lib.rs**
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
}
}
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```
Introducing the module into the scope using `use`

**src/lib.rs**
```rust
mod front_of_house {
    pub mod hosting {
    pub fn add_to_waitlist() {}
    }
}
// Here is the part of introduring the module into the scope using `use`
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
Introducing two types with the same name into the same scope requires the use of their `parent modules`.

**src/lib.rs**
```rust
use std::fmt;
use std::io;
fn function1() -> fmt::Result {
    // --something--
}
fn function2() -> io::Result<()> {
    // --something--
}
```
Or we can provide new names using the `as` keyword

**src/lib.rs**
```rust
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {
    // --something--
}
fn function2() -> IoResult<()> {
    // --something--
}
```

When we enter a name in the scope using the `use` keyword, the name available in the new scope is confidential. To enable the code that calls our code to refer to this name as if it were defined in the scope of this code, we can combine `pub` and `use`. This technique is called `re-export`.

**src/lib.rs**
```rust
mod front_of_house {
        pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
Specifying a `nested path` for introducing multiple elements with the same prefix into the scope.
```rust
use std::{io, cmp::Ordering};
```
## The glob operator
If we want to bring into view all the `public` elements defined in the path, then we can specify this path followed by the operator `glob*`
```rust
use std::collections::*; 
```
## Splitting modules into different files
Declaration of the `front_of_house` module, the body of which will be located in **src/front_of_house.rs**

**src/lib.rs**
```rust
mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
And **src/front_of_house.rs** gets definitions from the body of the `front_of_house` module

**src/front_of_house.rs**
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
Tree of the `restaurant` project
```plaitext
.
├── Cargo.toml
└── src
    ├── front_of_house.rs
    └── lib.rs
```