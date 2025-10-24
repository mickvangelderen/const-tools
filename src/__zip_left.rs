#[doc(hidden)]
#[macro_export]
macro_rules! __zip_left {
    ([$($list1:tt)*] [$($list2:tt)*] $($rest:tt)*) => {
        $crate::__zip_left!(@zip [$($list1)*] [$($list2)*] [] $($rest)*)
    };
    (@zip [] [$($list2:tt)*] [$($zipped:tt)*] $error:literal $($cb:tt)*) => {
        $crate::__call!($($cb)*([$($zipped)*]))
    };
    (@zip [$($list1:tt)+] [] [$($zipped:tt)*] $error:literal $($rest:tt)*) => {
        compile_error!($error);
    };
    (@zip [($($head1:tt)*) $($tail1:tt)*] [($($head2:tt)*) $($tail2:tt)*] [$($zipped:tt)*] $($rest:tt)*) => {
        $crate::__zip_left!(@zip [$($tail1)*] [$($tail2)*] [$($zipped)* ($($head1)* $($head2)*)] $($rest)*)
    };
}
