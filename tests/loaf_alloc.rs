#![cfg(feature = "alloc")]

use loaf::Loaf;
extern crate alloc;
use alloc::boxed::Box;

#[test]
fn one() {
    let slice: Box<[u8]> = Box::new([1, 2, 3, 4]);
    let loaf: Box<Loaf<u8>> = Loaf::try_from_boxed_slice(slice).unwrap();
    assert_eq!(*loaf.first(), 1);
}

#[test]
fn two() {
    let slice: Box<[u8]> = Box::new([]);
    assert!(Loaf::try_from_boxed_slice(slice).is_err());
}

