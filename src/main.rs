/*  Copyright (c) 2020-2021, Momozor <momozor4@gmail.com>
 *  All rights reserved.
 *
 *  This source code is licensed under the BSD-style license found in the
 *  LICENSE file in the root directory of this source tree.
 */

extern crate chrono;
extern crate clap;
extern crate serde;
extern crate serde_yaml;
extern crate promptly;

use std::fs;
use std::process;
use std::path;
use moisrs::{Requirement, Specification, create_table};

const MAJOR_VERSION: i32 = 0;
const MINOR_VERSION: i32 = 6;
const PATCH_VERSION: i32 = 0;
const SPECIFICATION_FILE: &str = "SPECIFICATION";

fn requirement_prompt() -> Result<Requirement, promptly::ReadlineError> {
    let name: String = promptly::prompt("Requirement Name")?;
    let explanation: String = promptly::prompt("Explanation")?;
    let status: String = promptly::prompt("Status")?;
    let priority: String = promptly::prompt("Priority")?;

    Ok(Requirement::new(&name, &explanation, &status,  &priority))
}

fn print_specification_as_table(filename: String) {
    let specification_source: String = fs::read_to_string(filename)
        .expect(&*format!("Cannot open {} file for reading!", SPECIFICATION_FILE));
    let specification: Specification =
        match serde_yaml::from_str(&specification_source) {
            Ok(specification) => specification,
            Err(_) => panic!(format!("{} file cannot be parsed!", SPECIFICATION_FILE)),
    };
    create_table(specification).printstd();
}

fn main() -> Result<(), promptly::ReadlineError> {
    let matches = clap::App::new("moisrs")
        .version(&*format!("{}.{}.{}", MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION))
        .about("Generate and view informal software requirement specification (ISRS) easily")
        .author("Momozor <momozor4@gmail.com>")
        .arg("-s, --show 'View existing SPECIFICATION file in ASCII table format'")
        .get_matches();

    match matches.occurrences_of("show") {
        1 => {
            print_specification_as_table(SPECIFICATION_FILE.to_string());
            process::exit(0);
        },
        _ => ()
    }

    if path::Path::new(SPECIFICATION_FILE).exists() {
        println!("");
        let prompt_message: String = format!("Existing {} file already exist. Overwrite?", SPECIFICATION_FILE);
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
            &specification.requirements.push(match requirement_prompt() {
                Ok(requirement) => requirement,
                Err(_) => panic!("Requirement is empty!"),
            });
        }
    }

    match serde_yaml::to_string(&specification) {
        Ok(specification) => {
            fs::write(SPECIFICATION_FILE, specification)
                .expect("Cannot write to specification file!");
        },
        Err(_) => 
            panic!("Cannot convert specification structure to YAML string!"),
    }

    Ok(())
}

