# impl-new-derive

The `impl-new-derive` procedural macro generates a `new` constructor for Rust structs. This macro automatically creates a constructor that initializes public fields from provided arguments and initializes private fields with default values using the `Default::default()` method.

## Features

- **Automatically generates a constructor** (`new`) for structs.
- **Handles public fields**: The `new` function takes all public fields of the struct as arguments.
- **Handles private fields**: All non-public fields are automatically initialized with their `Default::default()` values.
- **Supports generic types**: The macro works with both generic and non-generic structs.

## Usage

1. Add the macro to your project by including the crate that defines the procedural macro in your `Cargo.toml`:

```toml
[dependencies]
impl_new_derive = "0.1.0"
```

2. Annotate your struct with `#[derive(ImplNew)]` to automatically generate a `new` constructor.

### Example for a Non-Generic Struct

```rust
use impl_new_derive::ImplNew;

#[derive(ImplNew, Default)]
struct MyStruct {
    pub name: String,
    pub age: u32,
    secret: String, // This field is private
}

fn main() {
    let my_struct = MyStruct::new("John".to_string(), 30);
    println!("Name: {}, Age: {}", my_struct.name, my_struct.age);
}
```

In this example:
- The `new` function takes `name` and `age` as arguments, because they are public fields.
- The `secret` field is initialized to `Default::default()` since it's private and not included in the function arguments.

### Example for a Generic Struct

```rust
use impl_new_derive::ImplNew;

#[derive(ImplNew, Default)]
struct MyStruct<T> {
    pub value: T,
    count: usize, // This field is private
}

fn main() {
    let my_struct = MyStruct::new(42);
    println!("Value: {}", my_struct.value);
}
```

In this generic struct example:
- The `new` function takes `value` as an argument.
- The `count` field, being private, is automatically initialized with `Default::default()`.

## How It Works

When you annotate a struct with `#[derive(ImplNew)]`, the macro performs the following actions:
- It iterates over the fields of the struct.
- For public fields, it adds them as arguments to the generated `new` function.
- For non-public fields, it automatically initializes them with `Default::default()`.
- If the struct contains generics, the macro correctly handles them in the `impl` block.

### Limitations

- The macro only works with structs that have named fields (i.e., `struct` with named members).
- If the struct contains fields that do not implement `Default`, the macro will fail to compile.

## Contributing

Feel free to open issues or pull requests if you have any suggestions or improvements.

## License

This project is licensed under the MIT License.

