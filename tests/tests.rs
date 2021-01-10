use moisrs;

#[test]
fn test_requirement_creation() {
    assert_eq!(4, moisrs::Requirement::new("Fire", "tango", "soup"))
}
