extern crate chrono;
extern crate serde;
extern crate serde_yaml;
extern crate prettytable;
extern crate promptly;

#[derive(Clone)]
struct Requirement {
    name: String,
    explanation: String,
    priority: String,
}

struct Specification {
    name: String,
    last_revised: chrono::Date<chrono::Utc>,
    maintainer: String,
    target_version: String,
    status: String,
    requirements: Vec<Requirement>,
}

impl Requirement {
    fn new(name: &str, explanation: &str, priority: &str) -> Self {
        Requirement {
            name: name.to_string(),
            explanation: explanation.to_string(),
            priority: priority.to_string(),
        }
    }
}

impl Specification {
    fn new(
        name: &str,
        maintainer: &str,
        target_version: &str,
        status: &str,
        requirements: &mut Vec<Requirement>
    ) -> Self {
        Specification {
            name: name.to_string(),
            last_revised: chrono::Utc::today(),
            maintainer: maintainer.to_string(),
            target_version: target_version.to_string(),
            status: status.to_string(),
            requirements: requirements.to_vec(),
        }
    }
}

fn show_requirement_prompt() -> Result<Requirement, promptly::ReadlineError> {
    let name: String = promptly::prompt("Requirement Name")?;
    let explanation: String = promptly::prompt("Explanation")?;
    let priority: String = promptly::prompt("Priority")?;

    Ok(Requirement::new(&name, &explanation, &priority))
}

fn main() -> Result<(), promptly::ReadlineError> {
    let name: String = promptly::prompt("Project Name")?;
    let maintainer: String = promptly::prompt("Maintainer")?;
    let target_version: String = promptly::prompt("Target Version")?;
    let status: String = promptly::prompt("Status")?;

    let mut spec = Specification::new(
        &name,
        &maintainer,
        &target_version,
        &status,
        &mut vec![],
    );

    let mut want_new_requirement = true;
    while want_new_requirement {
        let do_proceed: bool = promptly::prompt_default("Create a new requirement", false)?;

        if !do_proceed {
            want_new_requirement = false;
        }
        else if do_proceed {
            &spec.requirements.push(match show_requirement_prompt() {
                Ok(requirement) => requirement,
                Err(_) => panic!("Requirement is empty!"),
            });
        }
    }

    println!("{}", spec.last_revised);
    for requirement in spec.requirements.iter() {
        println!("{}", requirement.name);
    }

    Ok(())
}
