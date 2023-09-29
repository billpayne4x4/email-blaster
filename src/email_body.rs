use std::fs;
use crate::contacts::Contact;

pub struct EmailBody {
    body: String,
}

impl EmailBody {
    pub fn new(file: String) -> Self {
        let body = fs::read_to_string(file).unwrap();

        Self {
            body,
        }
    }

    pub fn render(&self, contact: &Contact) -> String {
        self.body.clone()
            .replace("[email]", contact.email.as_str())
            .replace("[first_name]", contact.first_name.as_str())
            .replace("[last_name]", contact.last_name.as_str())
            .replace("[full_name]", contact.full_name().as_str())
    }
}