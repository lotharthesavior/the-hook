
# The Hook

[![Build and Test](https://github.com/lotharthesavior/the-hook/actions/workflows/tests.yml/badge.svg)](https://github.com/lotharthesavior/the-hook/actions/workflows/tests.yml)

This Rust library provides a simple filtering mechanism where functions (filters) can be registered and applied to values. Filters are associated with hooks (unique names), allowing multiple transformations to be applied to values sequentially in order of priority.

## Features
- Register filters for a given hook.
- Apply filters to values of a specific type.
- Remove individual filters or clear all filters for a hook.
- Supports multiple data types with type safety.

## Installation

To use this library, add it as a dependency in your Cargo project:

```bash
cargon add hook
```

OR

```toml
[dependencies]
hook = "0.1"
```

## Usage

### Registering Filters

Filters are registered using `add_filter`. Each filter is associated with a hook (string identifier) and is executed based on its priority (lower values run earlier).

```rust
use rust_filters::{add_filter, apply_filters};

let hook = "modify_number";
add_filter(hook, 10, |v: i32| v + 5);
add_filter(hook, 20, |v: i32| v * 2);
```

### Applying Filters

Filters are applied using `apply_filters`. The value is transformed sequentially by all filters registered under the given hook.

```rust
let result = apply_filters("modify_number", 10);
assert_eq!(result, 30); // (10 + 5) * 2
```

### Removing Filters

A filter can be removed using its unique ID returned from `add_filter`.

```rust
let hook = "modify_number";
let filter_id = add_filter(hook, 10, |v: i32| v + 5);
remove_filter(hook, filter_id);
```

### Removing All Filters for a Hook

```rust
remove_all_filters("modify_number");
```

## API Reference

### `add_filter<T>(hook: &str, priority: i32, callback: impl Fn(T) -> T) -> u64`
Registers a filter callback for a specific hook.

- `hook`: Name of the filter hook.
- `priority`: Determines the execution order (lower runs first).
- `callback`: Transformation function.
- Returns: A unique filter ID.

### `apply_filters<T>(hook: &str, value: T) -> T`
Applies all filters registered under the given hook to the provided value.

- `hook`: Name of the filter hook.
- `value`: Initial value before transformations.
- Returns: Transformed value.

### `remove_filter(hook: &str, id: u64) -> bool`
Removes a specific filter by ID.

- `hook`: Name of the filter hook.
- `id`: Filter ID returned by `add_filter`.
- Returns: `true` if the filter was removed, `false` otherwise.

### `remove_all_filters(hook: &str)`
Removes all filters for the specified hook.

- `hook`: Name of the filter hook.

## Testing

Run tests using:

```sh
cargo test
```

## License
This library is released under the MIT License.

