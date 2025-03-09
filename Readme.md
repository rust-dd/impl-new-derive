# impl-new-derive

The `impl-new-derive` procedural macro generates a `new` constructor for Rust structs. This macro automatically creates a constructor that initializes:

- **Public fields** from provided arguments (added as parameters to `new`).
- **Private fields** with default values using either:
  - An expression specified by `#[default(...)]`, **or**
  - `Default::default()` if no custom default expression is provided.

## Features

- **Automatically generates a `new` constructor** for structs.
- **Handles public fields**: The `new` function takes all public fields of the struct as arguments.
- **Handles private fields**:
  - If the private field has a `#[default(...)]` attribute, that expression is used for initialization.
  - Otherwise, the private field is initialized with `Default::default()`.
- **Supports generic types**: The macro works with both generic and non-generic structs.

## Usage

1. Add the macro to your project by including it in your `Cargo.toml`:

   ```toml
   [dependencies]
   impl_new_derive = "0.1"
   ```

2. Annotate your struct with `#[derive(ImplNew)]` to automatically generate a `new` constructor.
   Optionally, you can also derive `Default` if needed elsewhere in your code.

### Example for a Non-Generic Struct

```rust
use impl_new_derive::ImplNew;

#[derive(ImplNew, Default)]
struct MyStruct {
    pub name: String,
    pub age: u32,

    // Private field; no custom default attribute,
    // so it will be initialized using Default::default()
    secret: String,
}

fn main() {
    // The generated constructor requires arguments
    // for each PUBLIC field in the struct.
    let my_struct = MyStruct::new("John".to_string(), 30);
    println!("Name: {}, Age: {}", my_struct.name, my_struct.age);

    // 'secret' is private and gets a default value of "" (the Default for String).
}
```

### Using `#[default(...)]` for Private Fields

```rust
use impl_new_derive::ImplNew;

#[derive(ImplNew)]
struct Credentials {
    pub username: String,

    // Private field with a custom default value.
    // This field doesn't appear as a parameter in the `new` method.
    // Instead, it will be automatically set to "empty_token".to_string().
    #[default(\"empty_token\".to_string())]
    token: String,
}

fn main() {
    // The `new` function is generated only for public fields,
    // so we pass just `username`.
    let creds = Credentials::new(\"alice\".to_string());
    println!(\"Username: {}, Token: {}\", creds.username, creds.token);
    // Prints: Username: alice, Token: empty_token
}
```

### Example for a Generic Struct

```rust
use impl_new_derive::ImplNew;

#[derive(ImplNew, Default)]
struct MyStruct<T> {
    pub value: T,

    // Private field without custom default, so it uses Default::default()
    count: usize,
}

fn main() {
    // In a generic struct, only the public fields appear in `new`.
    let my_struct = MyStruct::new(42);

    // 'count' is private, and we didn't give it a `#[default(...)]`,
    // so it's initialized with `Default::default()`, i.e. 0.
    println!(\"Value: {}, Count: {}\", my_struct.value, my_struct.count);
    // Prints: Value: 42, Count: 0
}
```

## How It Works

When you annotate a struct with `#[derive(ImplNew)]`, the macro performs the following actions:

1. It iterates over the fields of the struct.
2. For each **public** field, it adds a corresponding parameter to the generated `new` method.
3. For each **private** field, it checks if a `#[default(expr)]` attribute is present:
   - If yes, it uses `expr` to initialize that field.
   - Otherwise, it uses `Default::default()` to initialize that field.
4. If the struct contains generics, the macro automatically handles them in the generated `impl`.

### Limitations

- Only works for structs with named fields.
- If a private field doesn't implement `Default` and does not have a `#[default(...)]` attribute, the macro fails to compile.
- If you do use `#[default(...)]`, the expression inside must be a valid Rust expression for that field's type.

## Contributing

Feel free to open issues or pull requests if you have any suggestions or improvements.

## License

This project is licensed under the MIT License.
