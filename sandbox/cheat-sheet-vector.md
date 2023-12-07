# Rust Vector (Vec<T>) Cheat Sheet

## Initialization

```rust
// Create an empty vector
let mut empty_vec: Vec<i32> = Vec::new();

// Create a vector with elements
let vec_with_elements = vec![1, 2, 3, 4, 5];
```

## Adding Elements

```rust
// Push a single element to the end
vec_with_elements.push(6);

// Push multiple elements at once
vec_with_elements.extend_from_slice(&[7, 8, 9]);
```

## Accessing Elements

```rust

// Access an element by index (returns Option)
let third_element = vec_with_elements.get(2);

// Access an element using indexing (panics if out of bounds)
let first_element = vec_with_elements[0];
```

## Iteration

```rust

// Iterate over elements immutably
for num in &vec_with_elements {
    println!("{}", num);
}

// Iterate over elements mutably
for num in &mut vec_with_elements {
    *num *= 2; // Double each element
}
```

## Removing Elements

```rust

// Pop the last element (returns Option)
let popped_element = vec_with_elements.pop();

// Remove an element by index
vec_with_elements.remove(2);

// Remove all elements satisfying a condition
vec_with_elements.retain(|&x| x % 2 == 0);
```

## Slicing

```rust

// Get a reference to a sub-slice
let sub_slice = &vec_with_elements[1..4];

// Get a mutable reference to a sub-slice
let mut_slice = &mut vec_with_elements[2..];

// Clone a sub-slice
let cloned_slice = vec_with_elements[1..4].to_vec();
```

## Other Methods

```rust

// Get the length of the vector
let length = vec_with_elements.len();

// Check if the vector is empty
let is_empty = vec_with_elements.is_empty();

// Resize the vector
vec_with_elements.resize(8, 0);
```

