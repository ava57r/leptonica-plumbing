use super::leptonica_destroy::LeptonicaDestroy;
use std::ops::{Deref, DerefMut};

/**
 * A wrapper for ref counted leptonica pointers that can be safely mutated.
 *
 * For example if it is the only reference.
 */
pub struct RefCountedExclusive<T: LeptonicaDestroy> {
    inner: T,
}

impl<T: LeptonicaDestroy> RefCountedExclusive<T> {
    /**
     Creates a new ref counted exclusive wrapper

     # Safety

     It must be safe for this wrapper to destroy (decrement the ref count).
     The ref count must have already been incremented before being passed to `new`.
     The pointer must not be mutated whilst this wrapper exists, except via this wrapper.
    */
    unsafe fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: LeptonicaDestroy> Drop for RefCountedExclusive<T> {
    fn drop(&mut self) {
        self.inner.destroy()
    }
}

impl<T: LeptonicaDestroy> Deref for RefCountedExclusive<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: LeptonicaDestroy> DerefMut for RefCountedExclusive<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
