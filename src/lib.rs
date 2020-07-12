//! Why have a slice when you can have a loaf?
//!
//! ## What this is
//! Sometimes you know that a slice _must_ have at least one element in it,
//! but Rust forces you to do "last minute decision" by `unwrap()`ing `Option`
//! from for example `first()` for `split_first()` methods.
//!
//! `Loaf` guarantees to have at least one element by its definition.
//! 
//! ## Safety
//! Currently unsafe code is only used to cast between `[T]` and `Loaf<T>` pointers.
//!
//! Rust's pointers to unsized types consist of two elements: pointer to data and 
//! length of the unsized part \
//! So a fat pointer to `[T]` has a pointer to the beginning of buffer and its length \
//! `Loaf` has an one element array of `T` _and_ `[T]` and because of it when casting
//! from slice pointer, the pointer is kept, but the length is decremented
//! (remember, fat pointer keeps length of the __unsized part__)
//!
//! The one element array is guaranteed to be first, because unsized types must
//! be at the end of struct.

use core::{slice, ptr};

pub struct Loaf<T> {
    pub loaf: [T; 1],
    pub rest: [T],
}

impl<T> Loaf<T> {
    pub fn len(&self) -> usize {
        self.loaf.len() + self.rest.len()
    }

    /* Using bracket syntax on arrays has the same performance 
     * as using get_unchecked(), plus code does not compile when
     * array has length of zero (can be useful with const generics)
     */
    pub fn first(&self) -> &T {
        &self.loaf[0]
    }
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.loaf[0]
    }
    pub fn last(&self) -> &T {
        match self.rest.last() {
            Some(x) => x,
            None    => &self.loaf[0],
        }
    }
    pub fn last_mut(&mut self) -> &mut T {
        match self.rest.last_mut() {
            Some(x) => x,
            None    => &mut self.loaf[0],
        }
    }
    pub fn split_first(&self) -> (&T, &[T]) {
        (&self.loaf[0], &self.rest)
    }
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        (&mut self.loaf[0], &mut self.rest)
    }
}

/* Unsafe code */
impl<T> Loaf<T> {
    pub fn from_slice(slice: &[T]) -> Option<&Loaf<T>> {
        let len = match slice.len().checked_sub(1) {
            Some(x) => x,
            None    => return None,
        };
        let ptr = slice.as_ptr();
        let loaf = ptr::slice_from_raw_parts(ptr, len) as *const Loaf<T>;
        let loaf = unsafe { &*loaf };

        return Some(loaf);
    }

    pub fn from_slice_mut(slice: &mut [T]) -> Option<&mut Loaf<T>> {
        let len = match slice.len().checked_sub(1) {
            Some(x) => x,
            None    => return None,
        };
        let ptr = slice.as_mut_ptr();
        let loaf = ptr::slice_from_raw_parts_mut(ptr, len) as *mut Loaf<T>;
        let loaf = unsafe { &mut *loaf };

        return Some(loaf);
    }

    pub fn as_slice(&self) -> &[T] {
        let len = self.len();
        let ptr = self as *const Loaf<T> as *const T;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let len = self.len();
        let ptr = self as *mut Loaf<T> as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    pub fn try_from_boxed_slice(boxed: Box<[T]>) -> Result<Box<Loaf<T>>, Box<[T]>> {
        let len = match boxed.len().checked_sub(1) {
            Some(x) => x,
            None    => return Err(boxed),
        };

        let ptr = Box::into_raw(boxed) as *mut T;
        let fatptr = ptr::slice_from_raw_parts_mut(ptr, len);

        let result = unsafe { Box::from_raw(fatptr as *mut Loaf<T>) };
        return Ok(result);
    }

    pub fn into_boxed_slice(self: Box<Self>) -> Box<[T]> {
        let len = self.len();
        let ptr = Box::into_raw(self) as *mut Loaf<T> as *mut T;
        let fatptr = ptr::slice_from_raw_parts_mut(ptr, len);

        unsafe { Box::from_raw(fatptr) }
    }
}

#[cfg(test)]
mod tests {
    use super::Loaf;

    #[test]
    fn one() {
        let slice: &[u8] = &[1, 2, 3, 4];
        let loaf: &Loaf<u8> = Loaf::from_slice(slice).unwrap();
        assert_eq!(*loaf.first(), 1);
    }

    #[test]
    fn two() {
        let slice: &[u8] = &[];
        assert!(Loaf::from_slice(slice).is_none());
    }
}

