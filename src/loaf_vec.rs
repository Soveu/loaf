#![cfg(feature = "alloc")]

use super::alloc::vec::Vec;
use crate::Loaf;

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
}

