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
    assert!(Loaf::from_slice(slice).is_none());
}

