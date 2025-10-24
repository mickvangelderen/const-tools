#[doc(hidden)]
#[macro_export]
// [args] [input] [fn] [pre] [post]
macro_rules! __call__parse_application {
    // no further input, this is the last application
    ([$($args:tt)*] [] [$($fn:tt)*] [$($pre:tt)*] [$($post:tt)*]) => {
        $($fn)*($($pre)* $($args)* $($post)*)
    };
    // found the placeholder, continue with the next application (this musn't be the last)
    ([<> $($pre_post:tt)*] [($($args:tt)*) $($input:tt)*] $fn:tt $pre:tt [$($post:tt)*]) => {
        $crate::__call__parse_application!([$($args)*] [$($input)*] $fn $pre [$($pre_post)* $($post)*])
    };
    // continue searching for the placeholder
    ([$head:tt $($tail:tt)*] $input:tt $fn:tt [$($pre:tt)*] $post:tt) => {
        $crate::__call__parse_application!([$($tail)*] $input $fn [$($pre)* $head] $post)
    };
}

#[doc(hidden)]
#[macro_export]
// [input] [fn]
macro_rules! __call__parse_fn {
    // found the beginning of the first function application, start parsing them
    ([($($args:tt)*) $($input:tt)*] $fn:tt) => {
        $crate::__call__parse_application!([$($args)*] [$($input)*] $fn [] [])
    };
    // continue searching for the first function application
    ([$head:tt $($tail:tt)*] [$($fn:tt)*]) => {
        $crate::__call__parse_fn!([$($tail)*] [$($fn)* $head])
    };
}

/// Implements partial function application for macros. The input is as follows:
///
/// ```txt
/// fn(pre... <> post...)...(args...)
/// ```
///
/// The marker `<>` marks where the next application's arguments will be inserted.
/// The partial function applications can be repeated which makes it easy for macros to add arguments to a callback and forward it.
///
/// # Examples
///
/// ```
/// use const_tools::__call;
///
/// macro_rules! sum {
///     ($($arg:expr),* $(,)?) => {
///         0 $(+ $arg)*
///     };
/// }
///
/// // `__call!` applies partial function applications and then the final arguments (no placeholder).
///
/// assert!(matches!(__call!(sum!(1, <>)(<>, 3)(2)), 6));
///
/// // `__call!` can be used in macros to provide tokens to other macros.
///
/// macro_rules! zip {
///     ([$([$($a:tt)*])*] [$([$($b:tt)*])*] $($cb:tt)*) => {
///         __call!($($cb)*([$([$($a)* $($b)*])*]))
///     };
/// }
///
/// macro_rules! flatten {
///     ([$([$($a:tt)*])*] $($cb:tt)*) => {
///         __call!($($cb)*($($($a)*)*))
///     }
/// }
///
/// assert!(matches!(
///     zip!(
///         [[1] [, 3]]
///         [[, 2] [, 4]]
///         flatten!(<> sum!)
///     ),
///     10
/// ));
/// ```
#[macro_export]
macro_rules! __call {
    ($($input:tt)*) => {
        $crate::__call__parse_fn!([$($input)*] [])
    };
}
