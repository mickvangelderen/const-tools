// TODO: Replace with stable version when stabilized.
#[inline]
#[doc(hidden)]
pub const unsafe fn __maybe_uninit_array_assume_init<T, const N: usize>(
    array: [::core::mem::MaybeUninit<T>; N],
) -> [T; N] {
    // SAFETY: MaybeUninit<T> and T are guaranteed to have the same layout
    unsafe { ::core::mem::transmute_copy(&array) }
}
