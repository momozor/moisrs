/*  Copyright (c) 2020, Momozor <momozor4@gmail.com>
 *  All rights reserved.
 *
 *  This source code is licensed under the BSD-style license found in the
 *  LICENSE file in the root directory of this source tree.
 */

extern crate chrono;
extern crate clap;
extern crate serde;
extern crate serde_yaml;
#[macro_use] extern crate prettytable;
extern crate promptly;

use std::fs;
use std::process;
use std::path;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Requirement {
    name: String,
    explanation: String,
    priority: String,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Specification {
    name: String,

    #[serde(with = "moisrs_date")] 
    last_revised: chrono::DateTime<chrono::Utc>,

    maintainer: String,
    target_version: String,
    status: String,
    requirements: Vec<Requirement>,
}

mod moisrs_date {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S UTC";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
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
            last_revised: chrono::Utc::now(),
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

fn show_specification_as_table(filename: String) {
    let specification_source: String = fs::read_to_string(filename)
        .expect("Cannot open SPEC file for reading!");
    let specification: Specification =
        match serde_yaml::from_str(&specification_source) {
            Ok(specification) => specification,
            Err(_) => panic!("SPEC file cannot be parsed!"),
    };

    let mut table = prettytable::Table::new();

    table.add_row(prettytable::row![
        "Project Name: ", specification.name
    ]);
    table.add_row(prettytable::row![
        "Last Revised: ", specification.last_revised
    ]);
    table.add_row(prettytable::row![
        "Maintainer: ", specification.maintainer
    ]);
    table.add_row(prettytable::row![
        "Target Version: ", specification.target_version
    ]);
    table.add_row(prettytable::row![
        "Status: ", specification.status
    ]);

    for requirement in specification.requirements.iter() {
        table.add_empty_row();
        table.add_row(prettytable::row![
            "Requirement Name: ", requirement.name
        ]);
        table.add_row(prettytable::row![
            "Explanation: ", requirement.explanation
        ]);
        table.add_row(prettytable::row![
            "Priority: ", requirement.priority
        ]);
    }
    table.printstd();
}

fn main() -> Result<(), promptly::ReadlineError> {
    let matches = clap::App::new("moisrs")
        .version("0.3.0")
        .about(
            "Generate and view software requirement specification (SRS) easily"
        )
        .author("Momozor <momozor4@gmail.com>")
        .arg("-s, --show 'View existing SPEC file in ASCII table format'")
        .get_matches();

    let specification_file: String = "SPEC".to_string();
    match matches.occurrences_of("show") {
        1 => {
            show_specification_as_table(specification_file);
            process::exit(0);
        },
        _ => ()
    }

    if path::Path::new(&specification_file).exists() {
        println!("");
        let prompt_message: &str = 
            "Existing SPEC file already exist. Override?";
        let do_overwrite: bool = 
            promptly::prompt_default(prompt_message, false)?;

        if !do_overwrite {
            process::exit(0);
        }
    }

    let name: String = promptly::prompt("Project Name")?;
    let maintainer: String = promptly::prompt("Maintainer")?;
    let target_version: String = promptly::prompt("Target Version")?;
    let status: String = promptly::prompt("Status")?;

    let mut specification = Specification::new(
        &name,
        &maintainer,
        &target_version,
        &status,
        &mut vec![],
    );

    let mut want_new_requirement = true;
    while want_new_requirement {
        let do_proceed: bool = 
            promptly::prompt_default("Create a new requirement", false)?;

        if !do_proceed {
            want_new_requirement = false;
        }
        else if do_proceed {
            &specification.requirements.push(match show_requirement_prompt() {
                Ok(requirement) => requirement,
                Err(_) => panic!("Requirement is empty!"),
            });
        }
    }

    match serde_yaml::to_string(&specification) {
        Ok(specification) => {
            fs::write(specification_file, specification)
                .expect("Cannot write to specification file!");
        },
        Err(_) => 
            panic!("Cannot convert specification structure to YAML string!"),
    }

    Ok(())
}
