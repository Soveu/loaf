#[rustfmt::skip]
use loaf::Loaf;

#[test]
fn one() {
    let slice: &[u8] = &[1, 2, 3, 4];
    let loaf: &Loaf<u8> = Loaf::from_slice(slice).unwrap();
    assert_eq!(*loaf.first(), 1);
}

#[test]
fn two() {
    let slice: &[u8] = &[];
    assert!(Loaf::<u8>::from_slice(slice).is_none());
}

fn slice_deref_check(_: &[u8]) {}
fn mut_slice_deref_check(_: &mut [u8]) {}

#[test]
fn deref() {
    let mut arr: [u8; 4] = [1, 2, 3, 4];
    let loaf: &mut Loaf<u8> = Loaf::from_slice_mut(&mut arr).unwrap();
    slice_deref_check(loaf);
    mut_slice_deref_check(loaf);

    let loaf: &Loaf<u8> = Loaf::from_slice(&arr).unwrap();
    slice_deref_check(loaf);
}
