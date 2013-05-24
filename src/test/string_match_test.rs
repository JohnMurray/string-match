use string_match::*;

#[test]
fn test_qs_should_return_09_for_empty_comprator_string() {
    assert!(qs_score("hello", "", 0) == 0.9f32);
}

#[test]
fn test_qs_should_return_0_for_longer_comparator_string() {
    assert!(qs_score("short", "very very very long", 0) == 0f32);
}