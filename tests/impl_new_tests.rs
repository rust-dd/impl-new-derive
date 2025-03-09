use impl_new_derive::ImplNew;

#[test]
fn test_impl_new() {
    #[derive(ImplNew)]
    struct Test {
        pub a: i32,
        b: String,
    }

    let test = Test::new(42);
    assert_eq!(test.a, 42);
    assert_eq!(test.b.is_empty(), true);
}

#[test]
fn test_impl_new_with_default() {
    #[derive(ImplNew)]
    struct Test {
        pub a: i32,
        #[default("default_value".to_string())]
        b: String,
    }

    let test = Test::new(42);
    assert_eq!(test.a, 42);
    assert_eq!(test.b, "default_value");
}
