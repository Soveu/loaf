use core::{slice, ptr};

pub type Loaf<T> = LoafN<T, 1>;

/* (1.46 nightly) Currently it is not possible to define default value for N */
#[repr(C)] /* Just to be sure */
pub struct LoafN<T, const N: usize> {
    pub loaf: [T; N],
    pub rest: [T],
}

impl<T, const N: usize> LoafN<T, N> {
    pub fn len(&self) -> usize {
        N + self.rest.len()
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
            None    => &self.loaf[N-1],
        }
    }
    pub fn last_mut(&mut self) -> &mut T {
        match self.rest.last_mut() {
            Some(x) => x,
            None    => &mut self.loaf[N-1],
        }
    }

    #[inline(always)]
    fn from_raw_parts(ptr: *const T, len: usize) -> *const Self {
        ptr::slice_from_raw_parts(ptr, len) as *const Self
    }
    #[inline(always)]
    fn from_raw_parts_mut(ptr: *mut T, len: usize) -> *mut Self {
        ptr::slice_from_raw_parts(ptr, len) as *mut Self
    }

    pub fn from_slice(slice: &[T]) -> Option<&Self> {
        let len = match slice.len().checked_sub(N) {
            Some(x) => x,
            None    => return None,
        };
        let ptr = slice.as_ptr();
        let loaf = unsafe { &*Self::from_raw_parts(ptr, len) };

        return Some(loaf);
    }

    pub fn from_mut_slice(slice: &mut [T]) -> Option<&mut Self> {
        let len = match slice.len().checked_sub(N) {
            Some(x) => x,
            None    => return None,
        };
        let ptr = slice.as_mut_ptr();
        let loaf = unsafe { &mut *Self::from_raw_parts_mut(ptr, len) };

        return Some(loaf);
    }

    pub fn as_slice(&self) -> &[T] {
        let len = self.len();
        let ptr = self as *const Self as *const T;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let len = self.len();
        let ptr = self as *mut Self as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    pub unsafe fn from_slice_unchecked(slice: &[T]) -> &Self {
        let len = slice.len() - N;
        let ptr = slice.as_ptr();
        #[allow(unused_unsafe)]
        unsafe { &*Self::from_raw_parts(ptr, len) }
    }
    pub unsafe fn from_mut_slice_unchecked(slice: &mut [T]) -> &mut Self {
        let len = slice.len() - N;
        let ptr = slice.as_mut_ptr();
        #[allow(unused_unsafe)]
        unsafe { &mut *Self::from_raw_parts_mut(ptr, len) }
    }

    pub fn as_smallest_loaf(&self) -> &Loaf<T> {
        unsafe { Loaf::from_slice_unchecked(self.as_slice()) }
    }
    pub fn as_smallest_loaf_mut(&mut self) -> &mut Loaf<T> {
        unsafe { Loaf::from_mut_slice_unchecked(self.as_mut_slice()) }
    }

    pub fn split_first(&self) -> (&T, &[T]) {
        let smol = self.as_smallest_loaf();
        (&smol.loaf[0], &smol.rest)
    }
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        let smol = self.as_smallest_loaf_mut();
        (&mut smol.loaf[0], &mut smol.rest)
    }
}

#[cfg(feature = "alloc")]
use super::alloc::boxed::Box;

#[cfg(feature = "alloc")]
impl<T, const N: usize> LoafN<T, N> {
    pub fn try_from_boxed_slice(boxed: Box<[T]>) -> Result<Box<Self>, Box<[T]>> {
        let len = match boxed.len().checked_sub(1) {
            Some(x) => x,
            None    => return Err(boxed),
        };

        let ptr = Box::into_raw(boxed) as *mut T;
        let loaf = Self::from_raw_parts_mut(ptr, len);

        let result = unsafe { Box::from_raw(loaf) };
        return Ok(result);
    }

    pub fn into_boxed_slice(self: Box<Self>) -> Box<[T]> {
        let len = self.len();
        let ptr = Box::into_raw(self) as *mut T;
        let fatptr = ptr::slice_from_raw_parts_mut(ptr, len);

        unsafe { Box::from_raw(fatptr) }
    }
}

