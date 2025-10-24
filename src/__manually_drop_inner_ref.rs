/// Provides access to the inner value of a ManuallyDrop<T> in const contexts (unlike Deref).
#[inline]
#[doc(hidden)]
pub const fn __manually_drop_inner_ref<T>(slot: &::core::mem::ManuallyDrop<T>) -> &T {
    // SAFETY: ManuallyDrop<T> and T are guaranteed to have the same layout
    unsafe { ::core::mem::transmute(slot) }
}
