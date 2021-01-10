use moisrs;

#[test]
fn test_requirement_creation_properties() {
    let dummy_requirement =
        moisrs::Requirement::new(
            "Test (REQUIREMENT) name",
            "Test <EXPLANATION>",
            "Abandoned",
            "High",
        );

    assert_eq!("Test (REQUIREMENT) name", dummy_requirement.name);
    assert_eq!("Test <EXPLANATION>", dummy_requirement.explanation);
    assert_eq!("Abandoned", dummy_requirement.status);
    assert_eq!("High", dummy_requirement.priority);
}

#[test]
fn test_specification_creation_properties() {
    let dummy_specification =
        moisrs::Specification::new(
            "MOTEST",
            "devel <devel6@gmail.com>",
            "1.0.0",
            "Development",
            &mut vec![moisrs::Requirement::new("METAOMATA", "NONE", "Complete", "Low")]
        );
    assert_eq!("MOTEST", dummy_specification.name);
    assert_eq!("devel <devel6@gmail.com>", dummy_specification.maintainer);
    assert_eq!("1.0.0", dummy_specification.target_version);
    assert_eq!("Development", dummy_specification.status);
    assert_eq!("Complete", dummy_specification.requirements[0].status);
    assert_eq!("Low", dummy_specification.requirements[0].priority);
}
