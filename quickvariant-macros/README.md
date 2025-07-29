# QuickVariant
This library is designed with reference to C++'s `std::variant`, allowing you to create variants using only type names.

# Warning
This library uses unsafe code. Please follow the usage guidelines carefully when using it.

# Example
```rust
// Please be sure to initialize using quickvariant::macros::make_variant!.
let mut variant = quickvariant::macros::make_variant!(i32, u64, String);

unsafe {
    variant.set(String::from("Hello, World")).unwrap();
}

println!("{:#?}", variant.get::<String>().unwrap());
```

# License
QuickVariant is licensed under the MIT license (see LICENSE in the main repository).
