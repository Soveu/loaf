#![cfg(all(feature = "alloc", feature = "nightly"))]

use crate::alloc::{boxed::Box, vec::Vec};
use crate::LoafN;
use core::ops::{Deref, DerefMut};

#[derive(Clone)]
#[repr(transparent)]
pub struct LoafNVec<T, const N: usize> {
    inner: Vec<T>,
}

impl<T, const N: usize> LoafNVec<T, N> {
    pub fn from_vec(vec: Vec<T>) -> Result<Self, Vec<T>> {
        if vec.len() < N {
            return Err(vec);
        }

        let s = Self { inner: vec };
        return Ok(s);
    }
    pub fn into_vec(self) -> Vec<T> {
        return self.inner;
    }

    pub fn as_loaf(&self) -> &LoafN<T, N> {
        unsafe { LoafN::from_slice_unchecked(&self.inner) }
    }
    pub fn as_mut_loaf(&mut self) -> &mut LoafN<T, N> {
        unsafe { LoafN::from_slice_mut_unchecked(&mut self.inner) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner
    }
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.inner.len() == N {
            return None;
        }

        return self.inner.pop();
    }

    pub fn into_boxed_loaf(self) -> Box<LoafN<T, N>> {
        let boxed = self.inner.into_boxed_slice();
        return match LoafN::try_from_boxed_slice(boxed) {
            Ok(b) => b,
            Err(_) => unreachable!(),
        };
    }
}

impl<T, const N: usize> Deref for LoafNVec<T, N> {
    type Target = LoafN<T, N>;
    fn deref(&self) -> &Self::Target {
        self.as_loaf()
    }
}

impl<T, const N: usize> DerefMut for LoafNVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_loaf()
    }
}
