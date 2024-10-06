//! # impl_new_derive
//!
//! `ImplNew` is a Rust procedural macro that automatically generates a constructor (`new` method)
//! for structs with named fields. It initializes public fields from provided arguments, and private
//! fields are automatically initialized using `Default::default()`.
//!
//! ## Features
//! - **Automatic constructor generation**: Generates a `new` method for structs.
//! - **Public fields**: Public fields are passed as parameters to the `new` method.
//! - **Private fields**: Private fields are initialized with `Default::default()`.
//! - **Generics support**: The macro works for both generic and non-generic structs.
//!
//! ## Usage
//!
//! Add the following dependency to your `Cargo.toml` file to use the macro:
//!
//! ```toml
//! [dependencies]
//! impl_new_derive = "0.1.0"
//! ```
//!
//! Then, annotate your struct with `#[derive(ImplNew)]` to generate the `new` method.
//!
//! ### Example for a Non-Generic Struct
//!
//! ```rust
//! use impl_new_derive::ImplNew;
//!
//! #[derive(ImplNew, Default)]
//! struct MyStruct {
//!     pub name: String,
//!     pub age: u32,
//!     secret: String, // This field is private
//! }
//!
//! fn main() {
//!     let my_struct = MyStruct::new("John".to_string(), 30);
//!     println!("Name: {}, Age: {}", my_struct.name, my_struct.age);
//! }
//! ```
//!
//! In this example:
//! - `name` and `age` are public fields and are passed as arguments to the `new` function.
//! - `secret` is a private field and is automatically initialized to its default value.
//!
//! ### Example for a Generic Struct
//!
//! ```rust
//! use impl_new_derive::ImplNew;
//!
//! #[derive(ImplNew, Default)]
//! struct MyStruct<T> {
//!     pub value: T,
//!     count: usize, // This field is private
//! }
//!
//! fn main() {
//!     let my_struct = MyStruct::new(42);
//!     println!("Value: {}", my_struct.value);
//! }
//! ```
//!
//! ## How It Works
//!
//! When the `ImplNew` macro is applied to a struct, the macro performs the following actions:
//! - Iterates over the struct's fields.
//! - Public fields are added as parameters to the generated `new` function.
//! - Non-public fields are initialized with `Default::default()`.
//! - If the struct contains generics, the macro correctly handles them in the `impl` block.
//!
//! ## Limitations
//! - The `ImplNew` macro only works for structs with named fields.
//! - Private fields must implement `Default`, or the macro will fail to compile.
//!
//! ## License
//!
//! Licensed under the MIT License.
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed, Visibility};

#[proc_macro_derive(ImplNew)]
pub fn derive_impl_new(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = if let syn::Data::Struct(data) = input.data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = data.fields {
            named
        } else {
            panic!("`ImplNew` macro can only be used on structs with named fields");
        }
    } else {
        panic!("`ImplNew` macro can only be used on structs");
    };

    let pub_fields = fields
        .iter()
        .filter(|f| matches!(f.vis, Visibility::Public(_)))
        .collect::<Vec<_>>();

    let non_pub_fields: Vec<_> = fields
        .iter()
        .filter(|f| !matches!(f.vis, Visibility::Public(_)))
        .collect::<Vec<_>>();

    let pub_field_names = pub_fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let pub_field_types = pub_fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let non_pub_field_names = non_pub_fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[must_use]
            pub fn new(#(#pub_field_names: #pub_field_types),*) -> Self {
                Self {
                    #(#pub_field_names),*,
                    #(#non_pub_field_names: Default::default()),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
