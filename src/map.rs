/// Maps a function over arrays in const contexts.
///
/// # Examples
///
/// ```
/// use const_tools::map;
///
/// struct Wrap<T>(T);
///
/// const fn wrap_all<T, const N: usize>(value: [T; N]) -> [Wrap<T>; N] {
///     map!(value, Wrap)
/// }
/// ```
#[macro_export]
macro_rules! map {
    ($($map_args:tt)*) => {
        $crate::__map__parse!([$($map_args)*] $crate::__map__expand!(<>))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __map__parse {
    // [args] [cb]
    ([zip!($($iae:expr),* $(,)?), |($($iip:pat_param),* $(,)?)| $($body:tt)*] $($cb:tt)*) => {
        $crate::__zip_left!(
            [$(($iae, $iip, ))*]
            [(ia0) (ia1) (ia2) (ia3) (ia4) (ia5) (ia6) (ia7) (ia8) (ia9) (ia10) (ia11)]
            "unsupported number of inputs"
            $($cb)*([$($body)*] <>)
        )
    };
    ([zip!($($iae:expr),* $(,)?), $fn:expr] $($cb:tt)*) => {
        $crate::__zip_left!(
            [$(($iae, ))*]
            [(ii0, ia0) (ii1, ia1) (ii2, ia2) (ii3, ia3) (ii4, ia4) (ii5, ia5) (ii6, ia6) (ii7, ia7) (ii8, ia8) (ii9, ia9) (ii10, ia10) (ii11, ia11)]
            "unsupported number of inputs"
            $crate::__map__parse_fn!([$fn] <> $($cb)*)
        )
    };
    ([$iae:expr, |$iip:pat_param| $($body:tt)*] $($cb:tt)*) => {
        $crate::__call!($($cb)*([$($body)*] [($iae, $iip, ia0)]))
    };
    ([$iae:expr, $fn:expr]  $($cb:tt)*) => {
        $crate::__call!($($cb)*([$fn(ii)] [($iae, ii, ia)]))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __map__parse_fn {
    // [fn] [(iae, ii, ia)]
    ([$fn:expr] [$(($iae:expr, $ii:ident, $ia:ident))*] $($cb:tt)*) => {
        $crate::__call!($($cb)*([$fn(($($ii,)*))] [$(($iae, $ii, $ia))*]))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __map__expand {
    // [body] [(iae, iip, ia)]
    ([$body:expr] [$(($iae:expr, $iip:pat_param, $ia:ident))*]) => {{
        let mut oa = $crate::__maybe_uninit_array_uninit();
        $(
            let $ia = ::core::mem::ManuallyDrop::new($iae);
            let $ia = $crate::__manually_drop_inner_ref(&$ia);
        )*
        let len = $crate::__same_len!(&oa $(,$ia)*);
        let mut index = 0;
        while index < len {
            $(
                let $iip = unsafe { ::core::ptr::read(&$ia[index]) };
            )*
            oa[index].write($body);
            index += 1;
        }
        assert!(
            index == len,
            "break is not allowed because a value must be written into every array element"
        );
        unsafe { $crate::__maybe_uninit_array_assume_init(oa) }
    }};
}
