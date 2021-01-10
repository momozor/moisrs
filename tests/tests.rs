use moisrs;

#[test]
fn test_requirement_creation_properties() {
    let dummy_requirement =
        moisrs::Requirement::new(
            "Test (REQUIREMENT) name",
            "Test <EXPLANATION>",
            "High",
        );

    assert_eq!("Test (REQUIREMENT) name", dummy_requirement.name);
    assert_eq!("Test <EXPLANATION>", dummy_requirement.explanation);
    assert_eq!("High", dummy_requirement.priority);
}
