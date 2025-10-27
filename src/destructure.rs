/// Turns the type `&T` into `T` but does not provide an actual implementation. Can be used in type checks.
#[doc(hidden)]
pub const fn __unimplemented_to_owned<T>(_: &T) -> T {
    panic!("no valid implementation exists for this function and it should not be invoked")
}

#[macro_export]
macro_rules! destructure {
    // struct
    (let $S:path { $($field_spec:tt)* } = $value:expr) => {
        $crate::destructure!(@struct ($($field_spec)*) => let $S {} = $value);
    };
    // tuple
    (let ($($var:pat_param),* $(,)?) = $value:expr) => {
        $crate::destructure!(@tuple ($($var),*); (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11) => (); let () = $value);
    };
    (@struct ()
        => let $S:path { $($field:ident: $var:pat_param),* } = $value:expr
    ) => {
        let value = $value;
        let value = ::core::mem::ManuallyDrop::new(value);
        let value = $crate::__manually_drop_inner_ref(&value);
        let __assert_valid_destructure = || {
            #[allow(unused)]
            let $S { $($field: $var),* } = $crate::__unimplemented_to_owned(value);
        };
        // SAFETY: We avoid double free by 1) only reading each field once (since the destructuring is valid) and 2) the original is wrapped in ManuallyDrop.
        $(
            let $var = unsafe { ::core::ptr::addr_of!(value.$field).read() };
        )*
    };
    (@struct ($next_field:ident: $next_var:pat_param $(,)?)
        => let $S:path { $($field:ident: $var:pat_param),* } = $value:expr
    ) => {
        $crate::destructure!(@struct () => let $S { $($field: $var,)* $next_field: $next_var } = $value);
    };
    (@struct ($next_field:ident $(,)?)
        => let $S:path { $($field:ident: $var:pat_param),* } = $value:expr
    ) => {
        $crate::destructure!(@struct () => let $S { $($field: $var,)* $next_field: $next_field } = $value);
    };
    (@struct ($next_field:ident: $next_var:pat_param, $($rest:tt)*)
        => let $S:path { $($field:ident: $var:pat_param),* } = $value:expr
    ) => {
        $crate::destructure!(@struct ($($rest)*) => let $S { $($field: $var,)* $next_field: $next_var } = $value);
    };
    (@struct ($next_field:ident, $($rest:tt)*)
        => let $S:path { $($field:ident: $var:pat_param),* } = $value:expr
    ) => {
        $crate::destructure!(@struct ($($rest)*) => let $S { $($field: $var,)* $next_field: $next_field } = $value);
    };
    (@tuple (); ($($index_rest:tt),*)
        => ($($ty:tt),*); let ($($index:tt: $var:pat_param),*) = $value:expr
    ) => {
        let value: ($($ty,)*) = $value; // asserts correct arity
        let value = ::core::mem::ManuallyDrop::new(value);
        let value = $crate::__manually_drop_inner_ref(&value);
        // SAFETY: We avoid double free by 1) only reading each field once (since they're unique) and 2) the original is wrapped in ManuallyDrop.
        $(
            let $var = unsafe { ::core::ptr::addr_of!(value.$index).read() };
        )*
    };
    (@tuple ($var_head:pat_param $(,$var_tail:pat_param)*); ()
        => ($($ty:tt),*); let ($($index:tt: $var:pat_param),*) = $value:expr
    ) => {
        compile_error!("tuple arity is larger than the maximum supported arity 12")
    };
    (@tuple ($var_head:pat_param $(,$var_tail:pat_param)*); ($index_head:tt $(,$index_tail:tt)*)
        => ($($ty:tt),*); let ($($index:tt: $var:pat_param),*) = $value:expr
    ) => {
        $crate::destructure!(@tuple ($($var_tail),*); ($($index_tail),*) => ($($ty,)* _); let ($($index: $var,)* $index_head: $var_head) = $value);
    };
}
