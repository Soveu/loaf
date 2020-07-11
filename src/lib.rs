/* TODO */
//#![no_std]
//#![feature(const_generics)]

use core::{mem, slice, ptr};

/* split_first depends on it, please don't touch! */
const N: usize = 1;

#[derive(Debug)]
#[repr(C)]
pub struct Loaf<T> {
    pub some: [T; N],
    pub rest: [T],
}

/* Basics */
impl<T> Loaf<T> {
    pub fn len(&self) -> usize {
        self.some.len() + self.rest.len()
    }

    /* Thanks that array[0] does not compile when array is [T; 0],
     * this part can be 100% safe
     * Using bracket syntax on arrays has the same performance 
     * as using get_unchecked()
     */
    pub fn first(&self) -> &T {
        &self.some[0]
    }
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.some[0]
    }
    pub fn last(&self) -> &T {
        match self.rest.last() {
            Some(x) => x,
            None    => &self.some[N-1],
        }
    }
    pub fn last_mut(&mut self) -> &mut T {
        match self.rest.last_mut() {
            Some(x) => x,
            None    => &mut self.some[N-1],
        }
    }

    /* NOTE: this works only for N = 1 */
    pub fn split_first(&self) -> (&T, &[T]) {
        assert!(N == 1);
        (&self.some[0], &self.rest)
    }
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        assert!(N == 1);
        (&mut self.some[0], &mut self.rest)
    }
}

/* Slice unsafe code */
impl<T> Loaf<T> {
    pub fn from_slice<'a>(slice: &'a [T]) -> Option<&'a Loaf<T>> {
        let len = match slice.len().checked_sub(N) {
            Some(x) => x,
            None    => return None,
        };
        let ptr = slice.as_ptr();

        unsafe { mem::transmute(slice::from_raw_parts(ptr, len)) }
    }

    pub fn from_slice_mut<'a>(slice: &'a mut [T]) -> Option<&'a mut Loaf<T>> {
        let len = match slice.len().checked_sub(N) {
            Some(x) => x,
            None    => return None,
        };
        let ptr = slice.as_mut_ptr();

        unsafe { mem::transmute(slice::from_raw_parts_mut(ptr, len)) }
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
}

/* Boxed slice unsafe code */
impl<T> Loaf<T> {
    pub fn try_from_boxed_slice(boxed: Box<[T]>) -> Result<Box<Loaf<T>>, Box<[T]>> {
        let len = match boxed.len().checked_sub(N) {
            Some(x) => x,
            None    => return Err(boxed),
        };

        let ptr = Box::into_raw(boxed) as *mut T;
        let fatptr = ptr::slice_from_raw_parts_mut(ptr, len);

        let result: Box<Loaf<T>> = unsafe { mem::transmute(Box::from_raw(fatptr)) };
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
        let array: &[u8] = &[1, 2, 3, 4];
        let loaf: &Loaf<u8> = Loaf::from_slice(array).unwrap();
        assert_eq!(*loaf.first(), 1);
    }
}

