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
    requirements: Option<Vec<Requirement>>,
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

fn show_requirement_prompt() -> Result<Requirement, promptly::ReadlineError> {
    let name: String = promptly::prompt("Requirement")?;
    let explanation: String = promptly::prompt("Explanation")?;
    let priority: String = promptly::prompt("Priority")?;

    Ok(Requirement::new(&name, &explanation, &priority))
}

fn main() -> Result<(), promptly::ReadlineError> {
    let name: String = promptly::prompt("Project Name")?;
    let maintainer: String = promptly::prompt("Maintainer")?;
    let target_version: String = promptly::prompt("Target Version")?;
    let status: String = promptly::prompt("Status")?;

    let mut spec = Specification {
        name: name,
        last_revised: chrono::Utc::today(),
        maintainer: maintainer,
        target_version: target_version,
        status: status,
        requirements: None,
    };

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
    
    Ok(())
}
