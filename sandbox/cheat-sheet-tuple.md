# Rust Tuple Cheat Sheet

## Initialization

```rust
// Create a tuple
let my_tuple: (i32, f64, &str) = (42, 3.14, "hello");
```

## Accessing Elements

```rust
// Access elements using indexing
let first_element = my_tuple.0;  // 42
let second_element = my_tuple.1; // 3.14
let third_element = my_tuple.2;  // "hello"
```

## Destructuring

```rust
// Destructure a tuple
let (a, b, c) = my_tuple;
println!("a: {}, b: {}, c: {}", a, b, c);
```

## Tuple Methods

```rust
// Tuple size (number of elements)
let tuple_size = my_tuple.len();
```

## Returning Multiple Values

```rust
// Function returning a tuple
fn get_coordinates() -> (f64, f64) {
    (3.5, 7.2)
}

let (x, y) = get_coordinates();
```

## Other Tuple Operations

```rust
// Combining tuples
let tuple_a = (1, 2);
let tuple_b = (3, 4);
let combined_tuple = tuple_a.0 + tuple_b.1; // (1, 4)

// Nested tuples
let nested_tuple = ((1, 2), (3, 4));
let nested_value = nested_tuple.0.1; // 2
```

Tuples in Rust are immutable and can hold a fixed number of elements of different types. They are often used for returning multiple values from functions, destructuring, and organizing related data.

Remember to check the official Rust documentation for the most up-to-date information: [Rust Tuple documentation](https://doc.rust-lang.org/std/primitive.tuple.html).
