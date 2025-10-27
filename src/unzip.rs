/// Unzips an array of tuples into multiple arrays in const contexts.
///
/// It is possible to assign the result of the unzipped expression inside of the macro invocation to avoid the need for
/// [`crate::destructure`].
///
/// # Examples
///
/// ```
/// use const_tools::unzip;
///
/// const fn unzip_pairs<A, B, const N: usize>(arr: [(A, B); N]) -> ([A; N], [B; N]) {
///     unzip!(let (a, b) = arr);
///     (a, b)
/// }
/// ```
#[macro_export]
macro_rules! unzip {
    (let ($($oap:pat_param),* $(,)?) = $($unzip_arg:tt)*) => {
        $crate::__zip_left!(
            [$(($oap, ))*]
            [(oa0, oi0) (oa1, oi1) (oa2, oi2) (oa3, oi3) (oa4, oi4) (oa5, oi5) (oa6, oi6) (oa7, oi7) (oa8, oi8) (oa9, oi9) (oa10, oi10) (oa11, oi11)]
            "unsupported number of outputs"
            $crate::__unzip__parse_unzip_arg!([$($unzip_arg)*] <>)
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __unzip__parse_unzip_arg {
    // [iae] [(oap, oa, oi)]
    ([map!($($map_args:tt)*)] $($rest:tt)*) => {
        $crate::__map__parse!([$($map_args)*] $crate::__unzip__parse_map_body!(<> $($rest)*))
    };
    ([$iae:expr] [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::__unzip__expand!([
            $crate::destructure!(let ($($oi),*) = ii);
        ] [($iae, ii, ia)] [$(($oap, $oa, $oi))*])
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __unzip__parse_map_body {
    // [map_body] [(iae, iip, ia)] [(oap, oa, oi)]
    ([{ $($body:tt)* }] $($rest:tt)*) => {
        $crate::__unzip__parse_map_body_block!([$($body)*] [] $($rest)*)
    };
    ([($($oie:expr),* $(,)?)] $i:tt [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::__unzip__expand!([] $i [$(($oap, $oa, $oie))*])
    };
    ([$body:expr] $i:tt [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::__unzip__expand!([
            let item = $body;
            $crate::destructure!(let ($($oi),*) = item);
        ] $i [$(($oap, $oa, $oi))*])
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __unzip__parse_map_body_block {
    // [map_body_block_stmts] [body_acc] [(iae, iip, ia)] [(oap, oa, oi)]
    ([] [$($body:tt)*] $i:tt [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::__unzip__expand!([
            let item = {
                $($body)*
            };
            $crate::destructure!(let ($($oi),*) = item);
        ] $i [$(($oap, $oa, $oi))*])
    };
    ([($($oie:expr),* $(,)?)] [$($body:tt)*] $i:tt [$(($oap:pat_param, $oa:ident, $oi:ident))*]) => {
        $crate::__unzip__expand!([$($body)*] $i [$(($oap, $oa, $oie))*])
    };
    ([$head:tt $($tail:tt)*] [$($body:tt)*] $($rest:tt)*) => {
        $crate::__unzip__parse_map_body_block!([$($tail)*] [$($body)* $head] $($rest)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __unzip__expand {
    // [body]  [(iae, iip, ia)] [(oap, oa, oie)]
    ([$($body:tt)*] [$(($iae:expr, $iip:pat_param, $ia:ident))*] [$(($oap:pat_param, $oa:ident, $oie:expr))*]) => {
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
