extern crate chrono;
extern crate serde;
extern crate serde_yaml;
extern crate prettytable;
extern crate promptly;

struct Requirement {
    name: String,
    explanation: String,
    priority: String,
}

struct Specification {
    name: String,
    last_revised: chrono::DateTime<chrono::Utc>,
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
        requirements: Vec<Requirement>
    ) -> Self {
        Specification {
            name: name.to_string(),
            last_revised: chrono::Utc::now(),
            maintainer: maintainer.to_string(),
            target_version: target_version.to_string(),
            status: status.to_string(),
            requirements: requirements
        }
    }
}

fn main() -> Result<(), promptly::ReadlineError> {
    let project_name: String = promptly::prompt("Project Name")?;
    let maintainer: String = promptly::prompt("Maintainer")?;
    let target_version: String = promptly::prompt("Target Version")?;
    let status: String = promptly::prompt("Status")?;

    Ok(())
}
