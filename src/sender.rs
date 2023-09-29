use lettre::message::{header, MultiPart, MultiPartBuilder, SinglePart};
use lettre::{Message, SmtpTransport, Transport};
use walkdir::WalkDir;
use std::path::{Path};
use crate::config::Config;
use crate::contacts::Contact;
use crate::email_body::EmailBody;

pub struct Sender {
    subject: String,
    config: Config,
    email_body: EmailBody,
    target_folder: String,
}

impl Sender {
    pub fn new(subject: &String, config: Config, target_folder: &String) -> Self {
        Self {
            subject : subject.clone(),
            config : config.clone(),
            email_body: EmailBody::new(Path::new(target_folder).join("email.html").to_str().unwrap().to_string()),
            target_folder : target_folder.clone(),
        }
    }

    pub fn send(&self, contact: &Contact) -> bool{
        let email_builder = Message::builder()
            .to(contact.email.parse().unwrap())
            .from(self.config.from.parse().unwrap())
            .subject(self.subject.clone())
            .body(self.email_body.render(contact))
            .unwrap();

        let mut email_multipart = MultiPartBuilder::new();
        let mut related_part_builder = MultiPartBuilder::new();
        let image_dir = Path::new(self.target_folder.clone().as_str()).join("images");

        for entry in WalkDir::new(&image_dir) {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_contents = std::fs::read(path).unwrap();
                let mime_type = mime::TEXT_HTML; // Just as an example, replace with the actual mime type.
                let content_id_header = header::ContentId::from(format!("<{}@yourdomain.com>", path.file_name().unwrap().to_str().unwrap()));
                let content_type_header = header::ContentType::parse(mime_type.to_string().as_str()).unwrap(); // Parse from MIME type

                let image_part = SinglePart::builder()
                    .header(content_type_header)
                    .header(content_id_header)
                    .body(file_contents);


                //related_part_builder = related_part_builder....build().singlepart(image_part);
            }
        }

        true
    }
}

/*fn add_attachment() -> Vec<SinglePart> {
    let mut res: Vec<lettre::message::MultiPart>;
    image_body.iter().enumerate().map( |(i, product)| {
        //   product.crop.leftDistanceToCutLine.iter().enumerate().map( |(ind, left)| {
        let content_id = String::from("") + &0.to_string() + &"-" + &0.to_string() + &"-" + &image_body[0].picture.name;
        let filebody = fs::read("src/images/".to_string() + &image_body[0].picture.name+ &"." + &"jpeg").unwrap();
        let content_type = "image/jpeg".parse().unwrap();
        Attachment::new_inline(content_id).body(filebody, content_type) // return attachment
        //   })
    })
        .collect::<Vec<lettre::message::SinglePart>>()
}*/