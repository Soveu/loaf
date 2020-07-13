use core::{slice, ptr};

/// Slice that guarantees to have at least one element
#[repr(C)] /* Just to be sure */
pub struct Loaf<T> {
    pub loaf: [T; 1],
    pub rest: [T],
}

impl<T> Loaf<T> {
    pub fn len(&self) -> usize {
        /* self.loaf.len() == 1 */
        1 + self.rest.len()
    }

    /* Using bracket syntax on arrays has the same performance 
     * as using get_unchecked(), plus code does not compile when
     * array has length of zero (can be useful with const generics)
     */
    /// Returns a reference to the first element
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf = Loaf::from_slice(slice).unwrap();
    /// assert_eq!(*loaf.first(), 0);
    /// ```
    pub fn first(&self) -> &T {
        &self.loaf[0]
    }
    /// Returns a mutable reference to the first element
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &mut [0u8, 1, 2, 3, 4];
    /// let loaf = Loaf::from_slice_mut(slice).unwrap();
    /// *loaf.first_mut() = 42;
    /// assert_eq!(*loaf.first(), 42);
    /// ```
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.loaf[0]
    }
    /// Returns a reference to the last element
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf = Loaf::from_slice(slice).unwrap();
    /// assert_eq!(*loaf.last(), 4);
    /// ```
    pub fn last(&self) -> &T {
        match self.rest.last() {
            Some(x) => x,
            None    => &self.loaf[0],
        }
    }
    /// Returns a mutable reference to the last element
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &mut [0u8, 1, 2, 3, 4];
    /// let loaf = Loaf::from_slice_mut(slice).unwrap();
    /// *loaf.last_mut() = 42;
    /// assert_eq!(*loaf.last(), 42);
    /// ```
    pub fn last_mut(&mut self) -> &mut T {
        match self.rest.last_mut() {
            Some(x) => x,
            None    => &mut self.loaf[0],
        }
    }
    /// Returns a reference to the first element and the rest of slice
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf = Loaf::from_slice(slice).unwrap();
    /// let (first, rest) = loaf.split_first();
    /// assert_eq!(*first, 0);
    /// assert_eq!(rest, &[1, 2, 3, 4]);
    /// ```
    pub fn split_first(&self) -> (&T, &[T]) {
        (&self.loaf[0], &self.rest)
    }
    /// Returns a mutable reference to the first element and the rest of slice
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &mut [0u8, 1, 2, 3, 4];
    /// let loaf = Loaf::from_slice_mut(slice).unwrap();
    /// let (first, rest) = loaf.split_first_mut();
    /// *first = 40;
    /// rest[0] = 41;
    /// // slice[0] = 0; // this line does not compile, because slice is borrowed mutably
    /// assert_eq!(*first, 40);
    /// assert_eq!(rest, &[41, 2, 3, 4]);
    /// ```
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        (&mut self.loaf[0], &mut self.rest)
    }

    #[inline(always)]
    fn from_raw_parts(ptr: *const T, len: usize) -> *const Self {
        ptr::slice_from_raw_parts(ptr, len) as *const Self
    }
    #[inline(always)]
    fn from_raw_parts_mut(ptr: *mut T, len: usize) -> *mut Self {
        ptr::slice_from_raw_parts(ptr, len) as *mut Self
    }

    /// Casts a `&[T]` with at least one element into `&Loaf<T>`.
    /// If slice does not contain any element, None is returned
    pub fn from_slice(slice: &[T]) -> Option<&Self> {
        let len = match slice.len().checked_sub(1) {
            Some(x) => x,
            None    => return None,
        };
        let ptr = slice.as_ptr();
        let loaf = unsafe { &*Self::from_raw_parts(ptr, len) };

        return Some(loaf);
    }

    /// Casts a `&mut [T]` with at least one element into `&mut Loaf<T>`.
    /// If slice does not contain any element, None is returned
    pub fn from_slice_mut(slice: &mut [T]) -> Option<&mut Self> {
        let len = match slice.len().checked_sub(1) {
            Some(x) => x,
            None    => return None,
        };
        let ptr = slice.as_mut_ptr();
        let loaf = unsafe { &mut *Self::from_raw_parts_mut(ptr, len) };

        return Some(loaf);
    }

    /// Casts `&Loaf<T>` into `&[T]`
    pub fn as_slice(&self) -> &[T] {
        let len = self.len();
        let ptr = self as *const Self as *const T;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// Casts `&mut Loaf<T>` into `&mut [T]`
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let len = self.len();
        let ptr = self as *mut Self as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    /// Casts a `&[T]` into `&Loaf<T>`.
    /// # Safety
    /// Slice needs to hold at least one element
    pub unsafe fn from_slice_unchecked(slice: &[T]) -> &Self {
        let len = slice.len() - 1;
        let ptr = slice.as_ptr();
        #[allow(unused_unsafe)]
        unsafe { &*Self::from_raw_parts(ptr, len) }
    }
    /// Casts a `&mut [T]` into `&mut Loaf<T>`.
    /// # Safety
    /// Slice needs to hold at least one element
    pub unsafe fn from_slice_mut_unchecked(slice: &mut [T]) -> &mut Self {
        let len = slice.len() - 1;
        let ptr = slice.as_mut_ptr();
        #[allow(unused_unsafe)]
        unsafe { &mut *Self::from_raw_parts_mut(ptr, len) }
    }
}

#[cfg(any(feature = "alloc", doc))]
#[doc(hidden)]
extern crate alloc;

#[cfg(any(feature = "alloc", doc))]
use alloc::boxed::Box;

#[cfg(any(feature = "alloc", doc))]
/// Avaliable with `alloc` feature
impl<T> Loaf<T> {
    /// Consumes a boxed slice returning a boxed Loaf.\
    /// If length of the slice is zero, the Box is returned back as error
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

    /// Consumes a boxed Loaf returning a boxed slice
    pub fn into_boxed_slice(self: Box<Self>) -> Box<[T]> {
        let len = self.len();
        let ptr = Box::into_raw(self) as *mut T;
        let fatptr = ptr::slice_from_raw_parts_mut(ptr, len);

        unsafe { Box::from_raw(fatptr) }
    }
}

