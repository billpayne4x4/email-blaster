use std::io::Write;
use crate::contacts::Contact;

pub struct Log {
    pub file: std::fs::File,
}

impl Log {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(path)?;
        writeln!(file, r#""email","first_name","last_name","result""#)?;
        Ok(Self {
            file,
        })
    }

    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.file.flush()?;
        Ok(())
    }

    pub fn write(&mut self, contact: &Contact, isSuccess: bool) -> Result<(), Box<dyn std::error::Error>> {
        let status = if isSuccess { "success" } else { "failed" };
        writeln!(self.file, r#""{}","{}","{}","{}""#, contact.email, contact.first_name, contact.last_name, status)?;
        Ok(())
    }
}