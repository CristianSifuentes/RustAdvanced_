//! Unsafe/FFI-style boundary with explicit invariants.

use std::marker::PhantomData;

/// Step 12: tiny unsafe island with documented contract.
#[derive(Debug, Clone, Copy)]
pub struct NonNullSlice<'a> {
    ptr: *const u8,
    len: usize,
    _lifetime: PhantomData<&'a [u8]>,
}

impl<'a> NonNullSlice<'a> {
    /// # Safety
    /// Caller guarantees that:
    /// 1. `ptr` is non-null and valid for reads of `len` bytes.
    /// 2. The memory outlives `'a`.
    pub unsafe fn new(ptr: *const u8, len: usize) -> Self {
        Self {
            ptr,
            len,
            _lifetime: PhantomData,
        }
    }

    pub fn as_slice(&self) -> &'a [u8] {
        // SAFETY: Invariants are guaranteed by constructor contract.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

pub fn checked_view(input: &[u8]) -> NonNullSlice<'_> {
    // SAFETY: pointer comes from a valid Rust slice, and lifetime is tied to `input`.
    unsafe { NonNullSlice::new(input.as_ptr(), input.len()) }
}

#[cfg(test)]
mod tests {
    use super::checked_view;

    #[test]
    fn checked_view_matches_input() {
        let input = b"abc";
        let view = checked_view(input);
        assert_eq!(view.as_slice(), input);
    }
}
