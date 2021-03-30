#![cfg(feature = "alloc")]

use crate::alloc::{boxed::Box, vec::Vec};
use crate::Loaf;
use core::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct LoafVec<T, const N: usize> {
    inner: Vec<T>,
    _marker: [(); N],
}

impl<T, const N: usize> LoafVec<T, N> {
    pub fn from_vec(vec: Vec<T>) -> Result<Self, Vec<T>> {
        if vec.len() < N {
            return Err(vec);
        }

        let s = Self {
            inner: vec,
            _marker: [(); N],
        };
        return Ok(s);
    }
    pub fn into_vec(self) -> Vec<T> {
        return self.inner;
    }

    pub fn as_loaf(&self) -> &Loaf<T, N> {
        unsafe { Loaf::from_slice_unchecked(&self.inner) }
    }
    pub fn as_mut_loaf(&mut self) -> &mut Loaf<T, N> {
        unsafe { Loaf::from_slice_mut_unchecked(&mut self.inner) }
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

    pub fn into_boxed_loaf(self) -> Box<Loaf<T, N>> {
        let boxed = self.inner.into_boxed_slice();
        return match Loaf::try_from_boxed_slice(boxed) {
            Ok(b) => b,
            Err(_) => unreachable!(),
        };
    }
}

impl<T, const N: usize> Deref for LoafVec<T, N> {
    type Target = Loaf<T, N>;
    fn deref(&self) -> &Self::Target {
        self.as_loaf()
    }
}

impl<T, const N: usize> DerefMut for LoafVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_loaf()
    }
}
