#![cfg(feature = "alloc")]

use super::alloc::{boxed::Box, vec::Vec};
use crate::Loaf;
use core::ops::{Deref, DerefMut};

#[derive(Clone)]
#[repr(transparent)]
pub struct LoafVec<T> {
    inner: Vec<T>,
}

impl<T> LoafVec<T> {
    pub fn from_vec(vec: Vec<T>) -> Result<Self, Vec<T>> {
        if vec.len() < 1 {
            return Err(vec);
        }

        let s = Self { inner: vec };
        return Ok(s);
    }
    pub fn into_vec(self) -> Vec<T> {
        return self.inner;
    }
    
    pub fn as_loaf(&self) -> &Loaf<T> {
        unsafe { Loaf::from_slice_unchecked(&self.inner) }
    }
    pub fn as_mut_loaf(&mut self) -> &mut Loaf<T> {
        unsafe { Loaf::from_slice_mut_unchecked(&mut self.inner) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner
    }
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.inner.len() == 1 {
            return None;
        }

        return self.inner.pop();
    }

    pub fn into_boxed_loaf(self) -> Box<Loaf<T>> {
        let boxed = self.inner.into_boxed_slice();
        return match Loaf::try_from_boxed_slice(boxed) {
            Ok(b) => b,
            Err(_) => unreachable!(),
        };
    }
}

impl<T> Deref for LoafVec<T> {
    type Target = Loaf<T>;
    fn deref(&self) -> &Self::Target {
        self.as_loaf()
    }
}

impl<T> DerefMut for LoafVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_loaf()
    }
}

