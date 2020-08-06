#![cfg(feature = "alloc")]

use loaf::{Loaf, loaf_vec::LoafVec};
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

fn slice_deref_check(_: &[u8]) {}
fn mut_slice_deref_check(_: &mut [u8]) {}

#[test]
fn deref() {
    let v = vec![1u8; 4];
    let mut loafv = LoafVec::from_vec(v).unwrap();
    slice_deref_check(&loafv);
    mut_slice_deref_check(&mut loafv);

    let mut loafb = loafv.into_boxed_loaf();
    slice_deref_check(&loafb);
    mut_slice_deref_check(&mut loafb);
}

