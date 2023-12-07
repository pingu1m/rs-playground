# Rust Operations Cheat Sheet

## File Operations

### Read a File

```rust
use std::fs;

let content = fs::read_to_string("file.txt").expect("Error reading the file");
```

### Read a JSON File

```rust
use serde_json;

let content: serde_json::Value = serde_json::from_str(&fs::read_to_string("file.json").expect("Error reading JSON file"))
    .expect("Error parsing JSON");
```

### Write to a File

```rust
use std::fs;

fs::write("output.txt", "Hello, Rust!").expect("Error writing to the file");
```

## I/O Operations

### Read from Stdin

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Error reading from stdin");
```

### Write to Stdout

```rust
use std::io::{self, Write};

io::stdout().write_all(b"Hello, Rust!").expect("Error writing to stdout");
```

### Write to Stderr

```rust
use std::io::{self, Write};

io::stderr().write_all(b"Error: Something went wrong").expect("Error writing to stderr");
```

## Date and Time

### Print Current Date (Year, Month, Day)

```rust
use chrono::prelude::*;

let date = Local::today();
println!("Current Date: {}", date);
```

### Print Current Timestamp (ISO Format)

```rust
use chrono::prelude::*;

let timestamp = Local::now();
println!("Current Timestamp: {}", timestamp.to_rfc3339());
```

### Print Current Epoch Timestamp

```rust
use chrono::Utc;

let epoch_timestamp = Utc::now().timestamp();
println!("Current Epoch Timestamp: {}", epoch_timestamp);
```

### Print Current Timestamp Minus 5 Days

```rust
use chrono::prelude::*;

let timestamp_minus_5_days = Local::now() - Duration::days(5);
println!("Timestamp Minus 5 Days: {}", timestamp_minus_5_days);
```

## String and Str Methods

### Convert String to DateTime

```rust
use chrono::prelude::*;

let date_str = "2023-11-09T12:34:56";
let datetime = DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S").expect("Error parsing datetime");
```

### Most Important Integer Methods

```rust
let number = 42;

// Check if even
let is_even = number % 2 == 0;

// Get absolute value
let abs_value = number.abs();

// Convert to string
let num_str = number.to_string();
```

### Most Important String and Str Methods

```rust
let text = "Hello, Rust!";

// Length of the string
let len = text.len();

// Check if empty
let is_empty = text.is_empty();

// Convert to uppercase
let uppercased = text.to_uppercase();

// Find substring
let contains_rust = text.contains("Rust");
```

## Basic Regular Expressions

```rust
use regex::Regex;

let text = "Hello, Rust!";

// Create a regex pattern
let pattern = Regex::new(r"Rust").expect("Invalid regex pattern");

// Check if the pattern matches
let is_match = pattern.is_match(text);
```

Add the necessary dependencies to your `Cargo.toml` for external crates (`serde_json`, `chrono`, `regex`) used in the examples.
