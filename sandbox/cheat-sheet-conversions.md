# Rust Collection Types Cheat Sheet

## Vector to Tuple

```rust
// Convert a vector to a tuple using iterators
let vec_data = vec![1, 2, 3];
let tuple_data: (i32, i32, i32) = vec_data.into_iter().collect_tuple().unwrap();
```

## Tuple to Vector

```rust
// Convert a tuple to a vector using iterators
let tuple_data = (4, 5, 6);
let vec_data: Vec<i32> = tuple_data.into_iter().collect();
```

## Vector to HashSet

```rust
use std::collections::HashSet;

// Convert a vector to a HashSet using iterators
let vec_data = vec![1, 2, 3];
let hashset_data: HashSet<i32> = vec_data.into_iter().collect();
```

## HashSet to Vector

```rust
// Convert a HashSet to a vector using iterators
use std::collections::HashSet;

let hashset_data: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
let vec_data: Vec<i32> = hashset_data.into_iter().collect();
```

## Vector of Tuples to HashMap

```rust
use std::collections::HashMap;

// Convert a vector of tuples to a HashMap using iterators
let vec_data = vec![(1, "one"), (2, "two"), (3, "three")];
let hashmap_data: HashMap<i32, &str> = vec_data.into_iter().collect();
```

## HashMap to Vector of Tuples

```rust
// Convert a HashMap to a vector of tuples using iterators
use std::collections::HashMap;

let hashmap_data: HashMap<i32, &str> = vec![(1, "one"), (2, "two"), (3, "three")].into_iter().collect();
let vec_data: Vec<(i32, &str)> = hashmap_data.into_iter().collect();
```

## Tuple to HashMap

```rust
// Convert a tuple to a HashMap using iterators
use std::collections::HashMap;

let tuple_data = (1, "one");
let mut hashmap_data = HashMap::new();
hashmap_data.insert(tuple_data.0, tuple_data.1);
```

## HashMap to Tuple

Converting a HashMap to a Tuple is not directly supported since a HashMap can contain multiple key-value pairs. Tuples represent a fixed number of elements, and HashMaps may have variable sizes.

Remember to check the official Rust documentation for the most up-to-date information on collections and conversions: [Rust Collections documentation](https://doc.rust-lang.org/std/collections/index.html).
