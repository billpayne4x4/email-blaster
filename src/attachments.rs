use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use std::fs;
use mime::Mime;

struct Attachment {
    data: String,
    mime: Mime,
}

impl Attachment {
    pub fn new(data: String, mime: Mime) -> Self {
        Self {
            data,
            mime,
        }
    }
}

struct Attachments {
    attachments: Vec<Attachment>,
}

impl Attachments {
    pub fn new(folder: String) -> Self {
        let mut attachments = Vec::new();
        let paths = fs::read_dir(folder).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let data = general_purpose::STANDARD.encode(fs::read_to_string(path).unwrap());
            let mime = mime_guess::from_path(path.clone()).first_or_octet_stream();
            attachments.push(Attachment::new(data, mime));
        }

        Self {
            attachments: Vec::new(),
        }
    }

    pub fn add(&mut self, data: String, mime: Mime) {
        self.attachments.push(Attachment::new(data, mime));
    }

    pub fn iter(&self) -> std::slice::Iter<Attachment> {
        self.attachments.iter()
    }
}