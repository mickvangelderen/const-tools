#[macro_export]
macro_rules! map {
    // [args] [cb]
    (@parse [zip!($($iae:expr),* $(,)?), |($($iip:pat_param),* $(,)?)| $($body:tt)*] $($cb:tt)*) => {
        $crate::__zip_left!(
            [$(($iae, $iip, ))*]
            [(ia0) (ia1) (ia2) (ia3) (ia4) (ia5) (ia6) (ia7) (ia8) (ia9) (ia10) (ia11)]
            "unsupported number of inputs"
            $($cb)*([$($body)*] <>)
        )
    };
    (@parse [zip!($($iae:expr),* $(,)?), $fn:expr] $($cb:tt)*) => {
        $crate::__zip_left!(
            [$(($iae, ))*]
            [(ii0, ia0) (ii1, ia1) (ii2, ia2) (ii3, ia3) (ii4, ia4) (ii5, ia5) (ii6, ia6) (ii7, ia7) (ii8, ia8) (ii9, ia9) (ii10, ia10) (ii11, ia11)]
            "unsupported number of inputs"
            $crate::map!(@parse_fn [$fn] <> $($cb)*)
        )
    };
    (@parse [$iae:expr, |$iip:pat_param| $($body:tt)*] $($cb:tt)*) => {
        $crate::__call!($($cb)*([$($body)*] [($iae, $iip, ia0)]))
    };
    (@parse [$iae:expr, $fn:expr]  $($cb:tt)*) => {
        $crate::__call!($($cb)*([$fn(ii)] [($iae, ii, ia)]))
    };
    // [fn] [(iae, ii, ia)]
    (@parse_fn [$fn:expr] [$(($iae:expr, $ii:ident, $ia:ident))*] $($cb:tt)*) => {
        $crate::__call!($($cb)*([$fn(($($ii,)*))] [$(($iae, $ii, $ia))*]))
    };
    // [body] [(iae, iip, ia)]
    (@expand [$body:expr] [$(($iae:expr, $iip:pat_param, $ia:ident))*]) => {{
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
    ($($map_args:tt)*) => {
        $crate::map!(@parse [$($map_args)*] $crate::map!(@expand <>))
    };
}
