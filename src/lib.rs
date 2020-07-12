use core::{slice, ptr};

pub struct Loaf<T> {
    pub some: [T; 1],
    pub rest: [T],
}

/* Basics */
impl<T> Loaf<T> {
    pub fn len(&self) -> usize {
        self.some.len() + self.rest.len()
    }

    /* Using bracket syntax on arrays has the same performance 
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
            None    => &self.some[0],
        }
    }
    pub fn last_mut(&mut self) -> &mut T {
        match self.rest.last_mut() {
            Some(x) => x,
            None    => &mut self.some[0],
        }
    }
    pub fn split_first(&self) -> (&T, &[T]) {
        (&self.some[0], &self.rest)
    }
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        (&mut self.some[0], &mut self.rest)
    }
}

/* Slice unsafe code */
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
}

/* Boxed slice unsafe code */
impl<T> Loaf<T> {
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

