// AssertKeyAtLeastLength panics when store key length is less than the given length.
pub fn assert_key_at_least_length(bz: &[u8], length: usize) {
    if bz.len() < length {
        panic!(
            "expected key of length at least {}, got {}",
            length,
            bz.len()
        );
    }
}
// AssertKeyLength panics when store key length is not equal to the given length.
pub fn assert_key_length(bz: &[u8], length: usize) {
    if bz.len() != length {
        panic!(
            "unexpected key length; got: {}, expected: {}",
            bz.len(),
            length
        );
    }
}
