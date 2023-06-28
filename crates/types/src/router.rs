use regex::Regex;

// IsAlpha defines a regular expression for matching against alpha values.
pub fn is_alpha(s: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9]+$")
        .expect("never fails")
        .is_match(s)
}

#[test]
fn test_is_alpha() {
    assert!(is_alpha("abc123"));
}
