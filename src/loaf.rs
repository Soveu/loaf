use core::ops::{Deref, DerefMut};
use core::{ptr, slice};
use core::num::NonZeroUsize;

/// Slice that guarantees to have at least one element
/// # Usage
/// The implementation is very minimal, it only contains things that have reason
/// to be here. If you want to use slice iterators or other slice methods,
/// consider using them indirectly via [as_slice](Loaf::as_slice) or
/// [as_mut_slice](Loaf::as_mut_slice)
#[repr(C)]
pub struct Loaf<T, const N: usize = 1usize> {
    pub loaf: [T; N],
    pub rest: [T],
}

impl<T, const N: usize> Loaf<T, N> {
    /// Returns length of the underlying slice
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf: &Loaf<_> = Loaf::from_slice(slice).unwrap();
    /// assert_eq!(loaf.len(), slice.len());
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        /* self.loaf.len() == 1 */
        N + self.rest.len()
    }

    #[inline(always)]
    /// Returns length of the underlying slice
    pub fn len_nonzero(&self) -> NonZeroUsize {
        /* SAFETY: N >= 1 and addition can't overflow, because
         * we would otherwise have more memory than LLVM can handle */
        unsafe { NonZeroUsize::new_unchecked(N.wrapping_add(self.rest.len())) }
    }

    /// Returns a reference to the first element
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf: &Loaf<_> = Loaf::from_slice(slice).unwrap();
    /// assert_eq!(*loaf.first(), 0);
    /// ```
    #[inline(always)]
    pub fn first(&self) -> &T {
        /* Using bracket syntax on arrays has the same performance
         * as using get_unchecked(), plus code does not compile when
         * array has length of zero (can be useful with const generics) */
        &self.loaf[0]
    }
    /// Returns a mutable reference to the first element
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &mut [0u8, 1, 2, 3, 4];
    /// let loaf: &mut Loaf<_> = Loaf::from_slice_mut(slice).unwrap();
    /// *loaf.first_mut() = 42;
    /// assert_eq!(*loaf.first(), 42);
    /// ```
    #[inline(always)]
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.loaf[0]
    }
    /// Returns a reference to the last element
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf: &Loaf<_> = Loaf::from_slice(slice).unwrap();
    /// assert_eq!(*loaf.last(), 4);
    /// ```
    #[inline(always)]
    pub fn last(&self) -> &T {
        match self.rest.last() {
            Some(x) => x,
            None => &self.loaf[0],
        }
    }
    /// Returns a mutable reference to the last element
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &mut [0u8, 1, 2, 3, 4];
    /// let loaf: &mut Loaf<_> = Loaf::from_slice_mut(slice).unwrap();
    /// *loaf.last_mut() = 42;
    /// assert_eq!(*loaf.last(), 42);
    /// ```
    #[inline(always)]
    pub fn last_mut(&mut self) -> &mut T {
        match self.rest.last_mut() {
            Some(x) => x,
            None => &mut self.loaf[0],
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

    /// Casts a `&[T]` with at least one element into `&Loaf<T>`.
    /// If slice does not contain any element, None is returned
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf: &Loaf<_> = Loaf::from_slice(slice).unwrap();
    /// assert_eq!(loaf.loaf, [0]);
    /// assert_eq!(loaf.rest, [1, 2, 3, 4]);
    ///
    /// let slice: &[u8] = &[];
    /// let optionloaf: Option<&Loaf<_>> = Loaf::from_slice(slice);
    /// assert!(optionloaf.is_none());
    /// ```
    pub fn from_slice(slice: &[T]) -> Option<&Self> {
        let len = match slice.len().checked_sub(N) {
            Some(x) => x,
            None => return None,
        };
        let ptr = slice.as_ptr();
        let loaf = unsafe { &*Self::from_raw_parts(ptr, len) };

        return Some(loaf);
    }

    /// Casts a `&mut [T]` with at least one element into `&mut Loaf<T>`.
    /// If slice does not contain any element, None is returned
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &mut [0u8, 1, 2, 3, 4];
    /// let loaf: &mut Loaf<_> = Loaf::from_slice_mut(slice).unwrap();
    /// loaf.loaf[0] = 42;
    /// loaf.rest[3] = 14;
    /// assert_eq!(slice, &[42u8, 1, 2, 3, 14]);
    /// ```
    pub fn from_slice_mut(slice: &mut [T]) -> Option<&mut Self> {
        let len = match slice.len().checked_sub(N) {
            Some(x) => x,
            None => return None,
        };
        let ptr = slice.as_mut_ptr();
        let loaf = unsafe { &mut *Self::from_raw_parts_mut(ptr, len) };

        return Some(loaf);
    }

    /// Casts `&Loaf<T>` into `&[T]`
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf: &Loaf<_> = Loaf::from_slice(slice).unwrap();
    /// assert_eq!(loaf.as_slice(), &[0u8, 1, 2, 3, 4]);
    /// ```
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        let len = self.len();
        let ptr = self as *const Self as *const T;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// Casts `&mut Loaf<T>` into `&mut [T]`
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &mut [0u8, 1, 2, 3, 4];
    /// let loaf: &mut Loaf<_> = Loaf::from_slice_mut(slice).unwrap();
    /// loaf.loaf[0] = 42;
    /// loaf.rest[3] = 14;
    /// assert_eq!(loaf.as_slice(), &[42u8, 1, 2, 3, 14]);
    /// ```
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let len = self.len();
        let ptr = self as *mut Self as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    /// Casts a `&[T]` into `&Loaf<T>`.
    /// # Safety
    /// Slice needs to hold at least one element
    #[inline(always)]
    pub unsafe fn from_slice_unchecked(slice: &[T]) -> &Self {
        let len = slice.len() - N;
        let ptr = slice.as_ptr();
        #[allow(unused_unsafe)]
        unsafe {
            &*Self::from_raw_parts(ptr, len)
        }
    }
    /// Casts a `&mut [T]` into `&mut Loaf<T>`.
    /// # Safety
    /// Slice needs to hold at least one element
    #[inline(always)]
    pub unsafe fn from_slice_mut_unchecked(slice: &mut [T]) -> &mut Self {
        let len = slice.len() - N;
        let ptr = slice.as_mut_ptr();
        #[allow(unused_unsafe)]
        unsafe {
            &mut *Self::from_raw_parts_mut(ptr, len)
        }
    }

    pub fn as_smallest_loaf(&self) -> &Loaf<T, 1> {
        unsafe { Loaf::<T, 1>::from_slice_unchecked(self.as_slice()) }
    }
    pub fn as_smallest_loaf_mut(&mut self) -> &mut Loaf<T, 1> {
        unsafe { Loaf::<T, 1>::from_slice_mut_unchecked(self.as_mut_slice()) }
    }

    /// Returns a reference to the first element and the rest of slice
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &[0u8, 1, 2, 3, 4];
    /// let loaf: &Loaf<_> = Loaf::from_slice(slice).unwrap();
    /// let (first, rest) = loaf.split_first();
    /// assert_eq!(*first, 0);
    /// assert_eq!(rest, &[1, 2, 3, 4]);
    /// ```
    #[inline(always)]
    pub fn split_first(&self) -> (&T, &[T]) {
        let smol = self.as_smallest_loaf();
        (&smol.loaf[0], &smol.rest)
    }
    /// Returns a mutable reference to the first element and the rest of slice
    /// ```
    /// # use loaf::Loaf;
    /// let slice = &mut [0u8, 1, 2, 3, 4];
    /// let loaf: &mut Loaf<_> = Loaf::from_slice_mut(slice).unwrap();
    /// let (first, rest) = loaf.split_first_mut();
    /// *first = 40;
    /// rest[0] = 41;
    /// // slice[0] = 0; // this line does not compile, because slice is borrowed mutably
    /// assert_eq!(*first, 40);
    /// assert_eq!(rest, &[41, 2, 3, 4]);
    /// ```
    #[inline(always)]
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        let smol = self.as_smallest_loaf_mut();
        (&mut smol.loaf[0], &mut smol.rest)
    }
}

#[cfg(feature = "alloc")]
use crate::alloc::boxed::Box;

#[cfg(feature = "alloc")]
/// Avaliable with `alloc` feature
impl<T, const N: usize> Loaf<T, N> {
    /// Consumes a boxed slice returning a boxed Loaf.\
    /// If length of the slice is zero, the Box is returned back as error
    /// ```
    /// # use loaf::Loaf;
    /// let x: Box<[u8]> = Box::new([1, 2, 3]);
    /// let loaf: Box<Loaf<_>> = Loaf::try_from_boxed_slice(x).unwrap();
    /// assert_eq!(loaf.loaf, [1u8]);
    /// assert_eq!(loaf.rest, [2u8, 3]);
    ///
    /// let x: Box<[u8]> = Box::new([]);
    /// let b: Box<[u8]> = match Loaf::<u8>::try_from_boxed_slice(x) {
    ///     Ok(_) => unreachable!(),
    ///     Err(b) => b,
    /// };
    /// ```
    pub fn try_from_boxed_slice(boxed: Box<[T]>) -> Result<Box<Self>, Box<[T]>> {
        let len = match boxed.len().checked_sub(N) {
            Some(x) => x,
            None => return Err(boxed),
        };

        let ptr = Box::into_raw(boxed) as *mut T;
        let loaf = Self::from_raw_parts_mut(ptr, len);

        let result = unsafe { Box::from_raw(loaf) };
        return Ok(result);
    }

    /// Consumes a boxed Loaf returning a boxed slice
    /// ```
    /// # use loaf::Loaf;
    /// let x: Box<[u8]> = Box::new([1, 2, 3]);
    /// let loaf: Box<Loaf<_>> = Loaf::try_from_boxed_slice(x).unwrap();
    /// assert_eq!(loaf.loaf, [1u8]);
    /// assert_eq!(loaf.rest, [2u8, 3]);
    /// assert_eq!(loaf.into_boxed_slice().as_ref(), &[1u8, 2, 3]);
    /// ```
    pub fn into_boxed_slice(self: Box<Self>) -> Box<[T]> {
        let len = self.len();
        let ptr = Box::into_raw(self) as *mut T;
        let fatptr = ptr::slice_from_raw_parts_mut(ptr, len);

        unsafe { Box::from_raw(fatptr) }
    }
}

impl<T, const N: usize> Deref for Loaf<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<T, const N: usize> DerefMut for Loaf<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
