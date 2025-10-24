#[inline]
#[doc(hidden)]
pub const fn __maybe_uninit_array_uninit<T, const N: usize>() -> [::core::mem::MaybeUninit<T>; N] {
    [const { ::core::mem::MaybeUninit::uninit() }; N] // <- is the const { ... } here necessary?
}
