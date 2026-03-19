/// Creates an array by evaluating a function for each index in `0..N` in const contexts.
///
/// # Examples
///
/// ```
/// use const_tools::from_indices;
///
/// const fn powers_of_two<const N: usize>() -> [u32; N] {
///     from_indices!(N, |index| 1 << index)
/// }
///
/// assert!(matches!(powers_of_two::<4>(), [1, 2, 4, 8]));
/// ```
#[macro_export]
macro_rules! from_indices {
    ($($from_indices_args:tt)*) => {
        $crate::__from_indices__parse!([$($from_indices_args)*] $crate::__from_indices__expand!(<>))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __from_indices__parse {
    // [args] [cb]
    ([$N:expr, |$iip:pat_param| $($body:tt)*] $($cb:tt)*) => {
        $crate::__call!($($cb)*([$N] [$iip] [$($body)*]))
    };
    ([$N:expr, $fn:expr] $($cb:tt)*) => {
        $crate::__call!($($cb)*([$N] [index] [$fn(index)]))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __from_indices__expand {
    // [N] [iip] [body]
    ([$N:expr] [$iip:pat_param] [$body:expr]) => {{
        let mut oa = $crate::__maybe_uninit_array_uninit::<_, { $N }>();
        let mut index = 0;
        #[deny(unreachable_code)]
        while index < oa.len() {
            let $iip = index;
            oa[index].write($body);
            index += 1;
        }
        assert!(
            index == oa.len(),
            "break is not allowed because a value must be written into every array element"
        );
        unsafe { $crate::__maybe_uninit_array_assume_init(oa) }
    }};
}
