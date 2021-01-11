/*  Copyright (c) 2020-2021, Momozor <momozor4@gmail.com>
 *  All rights reserved.
 *
 *  This source code is licensed under the BSD-style license found in the
 *  LICENSE file in the root directory of this source tree.
 */

extern crate chrono;
extern crate serde;
extern crate serde_yaml;
#[macro_use] extern crate prettytable;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Requirement {
    pub name: String,
    pub explanation: String,
    pub status: String,
    pub priority: String,
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

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Specification {
    pub name: String,
    #[serde(with = "moisrs_date")] 
    pub last_revised: chrono::DateTime<chrono::Utc>,
    pub maintainer: String,
    pub target_version: String,
    pub status: String,
    pub requirements: Vec<Requirement>,
}

impl Requirement {
    pub fn new(name: &str, explanation: &str, status: &str, priority: &str) -> Self {
        Requirement {
            name: name.to_string(),
            explanation: explanation.to_string(),
            status: status.to_string(),
            priority: priority.to_string(),
        }
    }
}

impl Specification {
    pub fn new(
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


fn create_requirement_rows(mut table: prettytable::Table, specification: Specification) -> prettytable::Table {
    for requirement in specification.requirements.iter() {
        table.add_empty_row();
        table.add_row(prettytable::row!["Requirement Name: ", requirement.name]);
        table.add_row(prettytable::row!["Explanation: ", requirement.explanation]);
        table.add_row(prettytable::row!["Status: ", requirement.status]);
        table.add_row(prettytable::row!["Priority: ", requirement.priority]);
    }
    table
}

pub fn create_table(specification: Specification) -> prettytable::Table {
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["Project Name: ", specification.name]);
    table.add_row(prettytable::row!["Last Revised: ", specification.last_revised]);
    table.add_row(prettytable::row!["Maintainer: ", specification.maintainer]);
    table.add_row(prettytable::row!["Target Version: ", specification.target_version]);
    table.add_row(prettytable::row!["Status: ", specification.status]);
    
    create_requirement_rows(table, specification);
    
    table
}
