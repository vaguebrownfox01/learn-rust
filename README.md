# Learn Rust

-   systems language
-   WASM compatible
-   Better memory management (No garbage collection)
-   Package manager: Cargo

-   cargo
-   cargo new myproject
-   cargo inits
-   cargo run
-   cargo build
-   cargo build --release

rs sandbox

-   pub: public function

Formatting

-   basic formatting
-   positional arg
-   named arg
-   placeholder traits
-   placeholder traits for debug
-   basic math

Variables

-   let
-   let mut
-   const
-   let (,) = (,)

Types

-   int
-   float
-   bool
-   char
-   tuple, array
-   std::i32::MAX

String

-   &str
-   String::from("")
-   .push()
-   .push_str()
-   assert_eq!
-   .len()
-   .capacity()
-   .contains()
-   .split_whitespace()
-   String::with_capacity()

Tuple

-   tup:(type, type) = (val, val);
-   tup.0, tup.1

Array

-   No resize
-   Be mutable
-   arr: [type, size]
-   &arrslice[1..4];
-   use std::mem
-   mem::size_of_val(&arr)
-
