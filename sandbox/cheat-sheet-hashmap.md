# Rust HashMap (`HashMap<K, V>`) Cheat Sheet

## Initialization

```rust
use std::collections::HashMap;

// Create an empty HashMap
let mut empty_map: HashMap<i32, String> = HashMap::new();

// Create a HashMap with initial values
let mut scores = HashMap::new();
scores.insert("Alice", 42);
scores.insert("Bob", 18);
```

## Insertion and Updating

```rust
// Insert a key-value pair
scores.insert("Charlie", 30);

// Update a value associated with a key
scores.insert("Alice", 50);
```

## Accessing and Retrieving Values

```rust
// Get a reference to a value using the key (returns Option)
let alice_score = scores.get("Alice");

// Access a value using the key (panics if key not present)
let bob_score = scores["Bob"];
```

## Iteration

```rust
// Iterate over key-value pairs immutably
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Iterate over key-value pairs mutably
for (key, value) in &mut scores {
    *value += 5; // Increment each value
}
```

## Removal

```rust
// Remove a key-value pair by key (returns Option)
let removed_score = scores.remove("Charlie");
```

## Checking Existence

```rust
// Check if a key exists in the HashMap
let is_alice_present = scores.contains_key("Alice");
```

## Other Methods

```rust
// Get the number of key-value pairs in the HashMap
let num_entries = scores.len();

// Check if the HashMap is empty
let is_empty = scores.is_empty();
```

Remember to check the official Rust documentation for the most up-to-date information and additional methods: [Rust HashMap documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
