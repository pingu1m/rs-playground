# Rust HashSet (`HashSet<T>`) Cheat Sheet

## Initialization

```rust
use std::collections::HashSet;

// Create an empty HashSet
let mut empty_set: HashSet<i32> = HashSet::new();

// Create a HashSet with initial values
let mut unique_numbers: HashSet<i32> = HashSet::new();
unique_numbers.insert(1);
unique_numbers.insert(2);
```

## Insertion

```rust
// Insert a value into the HashSet
unique_numbers.insert(3);
```

## Checking Existence

```rust
// Check if a value exists in the HashSet
let contains_two = unique_numbers.contains(&2);
```

## Removal

```rust
// Remove a value from the HashSet (returns true if the value was present)
let removed = unique_numbers.remove(&1);
```

## Iteration

```rust
// Iterate over elements immutably
for num in &unique_numbers {
    println!("{}", num);
}

// Iterate over elements mutably
for num in &mut unique_numbers {
    *num *= 2; // Double each element
}
```

## Set Operations

```rust
use std::collections::HashSet;

let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
let set_b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();

// Union of two sets
let union: HashSet<_> = set_a.union(&set_b).cloned().collect();

// Intersection of two sets
let intersection: HashSet<_> = set_a.intersection(&set_b).cloned().collect();

// Difference of two sets
let difference: HashSet<_> = set_a.difference(&set_b).cloned().collect();
```

## Other Methods

```rust
// Get the number of elements in the HashSet
let num_elements = unique_numbers.len();

// Check if the HashSet is empty
let is_empty = unique_numbers.is_empty();
```

Remember to check the official Rust documentation for the most up-to-date information and additional methods: [Rust HashSet documentation](https://doc.rust-lang.org/std/collections/struct.HashSet.html).
