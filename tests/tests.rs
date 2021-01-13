#[macro_use] extern crate prettytable;
use moisrs;

#[test]
fn requirement_creation_properties() {
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
fn specification_creation_properties() {
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

#[test]
fn table_creation() {
    let specification =
        moisrs::Specification::new(
            "MOTEST",
            "devel <devel6@gmail.com>",
            "1.0.0",
            "Development",
            &mut vec![moisrs::Requirement::new("METAOMATA", "NONE", "Complete", "Low")]);
    
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["Project Name: ", specification.name]);
    table.add_row(prettytable::row!["Last Revised: ", specification.last_revised]);
    table.add_row(prettytable::row!["Maintainer: ", specification.maintainer]);
    table.add_row(prettytable::row!["Target Version: ", specification.target_version]);
    table.add_row(prettytable::row!["Status: ", specification.status]);

    table.add_empty_row();
    table.add_row(prettytable::row!["Requirement Name: ", specification.requirements[0].name]);
    table.add_row(prettytable::row!["Explanation: ", specification.requirements[0].explanation]);
    table.add_row(prettytable::row!["Status: ", specification.requirements[0].status]);
    table.add_row(prettytable::row!["Priority: ", specification.requirements[0].priority]);
    
    assert_eq!(table, moisrs::create_table(specification));
}
