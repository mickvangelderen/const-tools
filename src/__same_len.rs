/// Asserts that two arrays are of the same length at compile-time.
#[inline]
#[doc(hidden)]
pub const fn __same_len<A, B, const N: usize>(_: &[A; N], _: &[B; N]) {}

/// Asserts that any number of arrays are of the same length at compile-time and returns the length as a value.
#[doc(hidden)]
#[macro_export]
macro_rules! __same_len {
    ($head:expr $(,$tail:expr)* $(,)?) => {{
        $crate::__same_len!(@check $head $(,$tail)*);
        $head.len()
    }};
    (@check $a:expr) => {};
    (@check $a:expr, $b:expr) => {
        $crate::__same_len($a, $b);
    };
    (@check $a:expr, $b:expr, $($rest:tt)*) => {
        $crate::__same_len($a, $b);
        $crate::__same_len!(@check $b, $($rest)*);
    };
}
