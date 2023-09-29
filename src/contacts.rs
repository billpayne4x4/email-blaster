extern crate csv;
use csv::Error as CsvError;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl Contact {
    pub fn new(email: String, first_name: String, last_name: String) -> Self {
        Self {
            email,
            first_name,
            last_name,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub struct Contacts {
    contacts: Vec<Contact>,
}

impl Contacts {
    pub fn new(file: &str) -> Result<Self, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(file)?;
        let contacts: Vec<Contact> = reader
            .deserialize()
            .collect::<Result<_, CsvError>>()?;

        Ok(Self { contacts })
    }

    pub fn iter(&self) -> std::slice::Iter<Contact> {
        self.contacts.iter()
    }
}