/// Produces an array of intermediate fold results in const contexts.
///
/// # Examples
///
/// ```
/// use const_tools::scan;
///
/// const fn enumerate<T, const N: usize>(value: [T; N]) -> [(usize, T); N] {
///     scan!(value, 0, |count, item| {
///         let index = *count;
///         *count += 1;
///         (index, item)
///     })
/// }
/// ```
#[macro_export]
macro_rules! scan {
    (zip!($($iae:expr),* $(,)?), $init:expr, |$acc:pat_param, ($($iip:pat_param),* $(,)?)| $body:expr) => {{
        let mut acc = $init;
        $crate::map!(zip!($($iae),*), |($($iip,)*)| {
            let $acc = &mut acc;
            $body
        })
    }};
    (zip!($($iae:expr),* $(,)?), $init:expr, $fn:expr) => {
        $crate::__zip_left!(
            [$(($iae, ))*]
            [(ii0) (ii1) (ii2) (ii3) (ii4) (ii5) (ii6) (ii7) (ii8) (ii9) (ii10) (ii11)]
            "unsupported number of inputs"
            $crate::scan!(@zip_function <> [$init] [$fn])
        )
    };
    ($iae:expr, $init:expr, |$acc:pat_param, $iip:pat_param| $body:expr) => {{
        let mut acc = $init;
        $crate::map!($iae, |$iip| {
            let $acc = &mut acc;
            $body
        })
    }};
    ($iae:expr, $init:expr, $fn:expr) => {{
        let mut acc = $init;
        $crate::map!($iae, |item| {
            $fn(&mut acc, item)
        })
    }};
    (@zip_function [$(($iae:expr, $ii:ident))*] [$init:expr] [$fn:expr]) => {
        $crate::scan!(zip!($($iae),*), $init, |acc, ($($ii,)*)| $fn(acc, ($($ii,)*)))
    };
}
