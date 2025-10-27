#[macro_export]
macro_rules! unzip {
    (let ($($oap:pat_param),* $(,)?) = $($unzip_arg:tt)*) => {
        $crate::__zip_left!(
            [$(($oap, ))*]
            [(oa0, oi0) (oa1, oi1) (oa2, oi2) (oa3, oi3) (oa4, oi4) (oa5, oi5) (oa6, oi6) (oa7, oi7) (oa8, oi8) (oa9, oi9) (oa10, oi10) (oa11, oi11)]
            "unsupported number of outputs"
            $crate::unzip!(@parse_unzip_arg [$($unzip_arg)*] <>)
        )
    };
    // [iae] [(oap, oa, oi)]
    (@parse_unzip_arg [map!($($map_args:tt)*)] $($rest:tt)*) => {
        $crate::map!(@parse [$($map_args)*] $crate::unzip!(@parse_map_body <> $($rest)*))
    };
    (@parse_unzip_arg [$iae:expr] [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::unzip!(@expand [
            $crate::destructure!(let ($($oi),*) = ii);
        ] [($iae, ii, ia)] [$(($oap, $oa, $oi))*])
    };
    // [map_body] [(iae, iip, ia)] [(oap, oa, oi)]
    (@parse_map_body [{ $($body:tt)* }] $($rest:tt)*) => {
        $crate::unzip!(@parse_map_body_block [$($body)*] [] $($rest)*)
    };
    (@parse_map_body [($($oie:expr),* $(,)?)] $i:tt [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::unzip!(@expand [] $i [$(($oap, $oa, $oie))*])
    };
    (@parse_map_body [$body:expr] $i:tt [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::unzip!(@expand [
            let item = $body;
            $crate::destructure!(let ($($oi),*) = item);
        ] $i [$(($oap, $oa, $oi))*])
    };
    // [map_body_block_stmts] [body_acc] [(iae, iip, ia)] [(oap, oa, oi)]
    (@parse_map_body_block [] [$($body:tt)*] $i:tt [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::unzip!(@expand [
            let item = {
                $($body)*
            };
            $crate::destructure!(let ($($oi),*) = item);
        ] $i [$(($oap, $oa, $oi))*])
    };
    (@parse_map_body_block [($($oie:expr),* $(,)?)] [$($body:tt)*] $i:tt [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::unzip!(@expand [$($body)*] $i [$(($oap, $oa, $oie))*])
    };
    (@parse_map_body_block [$head:tt $($tail:tt)*] [$($body:tt)*] $($rest:tt)*) => {
        $crate::unzip!(@parse_map_body_block [$($tail)*] [$($body)* $head] $($rest)*)
    };
    // [body]  [(iae, iip, ia)] [(oap, oa, oie)]
    (@expand [$($body:tt)*] [$(($iae:expr, $iip:pat_param, $ia:ident))*] [$(($oap:pat_param, $oa:ident, $oie:expr))*]) => {
        $(
            let mut $oa = $crate::__maybe_uninit_array_uninit();
        )*
        $(
            let $ia = ::core::mem::ManuallyDrop::new($iae);
            let $ia = $crate::__manually_drop_inner_ref(&$ia);
        )*
        let len = $crate::__same_len!($(&$oa,)* $(&$ia,)*);
        let mut index = 0;
        while index < len {
            $(
                let $iip = unsafe { ::core::ptr::read(&$ia[index]) };
            )*
            $($body)*
            $(
                $oa[index].write($oie);
            )*
            index += 1;
        }
        assert!(
            index == len,
            "break is not allowed because a value must be written into every array element"
        );
        $(
            let $oap = unsafe { $crate::__maybe_uninit_array_assume_init($oa) };
        )*
    };
}
