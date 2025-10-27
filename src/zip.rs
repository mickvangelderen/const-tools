#[doc(hidden)]
#[macro_export]
macro_rules! __zip__expand {
    ([$(($iae:expr, $ia:ident))*]) => {{
        let mut oa = $crate::__maybe_uninit_array_uninit();
        $(
            let $ia = ::core::mem::ManuallyDrop::new($iae);
            let $ia = $crate::__manually_drop_inner_ref(&$ia);
        )*
        let len = $crate::__same_len!(&oa $(,$ia)*);
        let mut index = 0;
        while index < len {
            oa[index].write(($(unsafe { ::core::ptr::read(&$ia[index]) }),*));
            index += 1;
        }
        unsafe { $crate::__maybe_uninit_array_assume_init(oa) }
    }};
}

/// Zips multiple arrays into an array of tuples in const contexts.
///
/// # Examples
///
/// ```
/// use const_tools::zip;
///
/// const fn zip_two<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
///     zip!(a, b)
/// }
/// ```
#[macro_export]
macro_rules! zip {
    ($($iae:expr),* $(,)?) => {
        $crate::__zip_left!(
            [$(($iae, ))*]
            [(ia0) (ia1) (ia2) (ia3) (ia4) (ia5) (ia6) (ia7) (ia8) (ia9) (ia10) (ia11)]
            "unsupported number of inputs"
            $crate::__zip__expand!
        )
    };
}
