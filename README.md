# `const-tools`

This crate provides macros that help write `const` functions.
Please view the documentation at [docs.rs/const-tools](https://docs.rs/const-tools/latest/const_tools/).

## Motivation

Stable rust (as of Oct 2025) does not allow destructuring of tuples, structs and arrays in const contexts:

```rust
const fn into_inner<T>(wrap: (T,)) -> T {
    let (value,) = wrap;
    value
}
```

gives:

```txt
error[E0493]: destructor of `Wrap<T>` cannot be evaluated at compile-time
```

Additionally, moving values out of arrays and building new ones is tricky:

```rust
const fn wrap_all<T, const N: usize>(value: [T; N]) -> [(T,); N] {
    // Create uninitialized output array
    let mut oa: [std::mem::MaybeUninit<(T,)>; N] =
        [const { std::mem::MaybeUninit::uninit() }; N];

    // Wrap input to prevent drop
    let ia = std::mem::ManuallyDrop::new(value);
    // Get reference to inner value (Deref not available in const)
    let ia: &[T; N] = unsafe { std::mem::transmute(&ia) };

    let mut index = 0;
    while index < N {
        // Read a single item from the input array
        let item = unsafe { std::ptr::read(&ia[index]) };
        // Initialize the element in the output array
        oa[index].write((item,));
        index += 1;
    }

    // All elements have been initialized
    unsafe { std::mem::transmute_copy(&oa) }
}
```

This library provides macros to make all of this safe and easy.

```rust
use const_tools::{destructure, map};

const fn into_inner<T>(value: (T,)) -> T {
    destructure!(let (inner,) = value);
    inner
}

const fn wrap_all<T, const N: usize>(value: [T; N]) -> [(T,); N] {
    map!(value, |item| (item,))
}
```
