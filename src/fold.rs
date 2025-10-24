#[macro_export]
macro_rules! fold {
    // [args] cb
    (@parse [zip!($($iae:expr),* $(,)?), $init:expr, |$acc:pat_param, ($($iip:pat_param),* $(,)?)| $($body:tt)*] $($cb:tt)*) => {
        $crate::__zip_left!(
            [$(($iae, $iip, ))*]
            [(ia0) (ia1) (ia2) (ia3) (ia4) (ia5) (ia6) (ia7) (ia8) (ia9) (ia10) (ia11)]
            "unsupported number of inputs"
            $($cb)*([$init] [$($body)*] [$acc] <>)
        )
    };
    (@parse [$iae:expr, $init:expr, |$acc:pat_param, $iip:pat_param| $($body:tt)*] $($cb:tt)*) => {
        $crate::__call!($($cb)*([$init] [$($body)*] [$acc] [($iae, $iip, ia0)]))
    };
    (@parse [$iae:expr, $init:expr, $fn:expr] $($cb:tt)*) => {
        $crate::__call!($($cb)*([$init] [$fn(acc, ii)] [acc] [($iae, ii, ia0)]))
    };
    // [init] [body] [acc] [(iae, iip, ia)]
    (@expand [$init:expr] [$body:expr] [$acc:pat_param] [$(($iae:expr, $iip:pat_param, $ia:ident))+]) => {{
        $(
            let $ia = ::core::mem::ManuallyDrop::new($iae);
            let $ia = $crate::__manually_drop_inner_ref(&$ia);
        )*
        let len = $crate::__same_len!($($ia),*);
        let mut acc = $init;
        let mut index = 0;
        while index < len {
            $(
                let $iip = unsafe { ::core::ptr::read(&$ia[index]) };
            )*
            let $acc = acc;
            acc = $body;
            index += 1;
        }
        assert!(
            index == len,
            "break is not allowed because a value must be written into every array element"
        );
        acc
    }};
    ($($fold_args:tt)*) => {
        $crate::fold!(@parse [$($fold_args)*] $crate::fold!(@expand <>))
    };
}
