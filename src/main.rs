use std::env;
use std::path::Path;

mod config;
mod log;
mod email_body;
mod attachments;
mod contacts;
mod sender;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(r#"Usage: ./program "<target_folder>" "<subject>""#);
        return;
    }

    let target_folder = &args[1];
    let subject = &args[2];



    let _config = config::Config::new(Path::new(target_folder).join("config.json").to_str().unwrap()).unwrap();
    let mut _log = log::Log::new(Path::new(target_folder).join("log.csv").to_str().unwrap()).unwrap();
    let _sender = sender::Sender::new(subject, _config, target_folder);
    let _contacts = contacts::Contacts::new(Path::new(target_folder).join("contacts.csv").to_str().unwrap()).unwrap();

    for contact in _contacts.iter() {
        println!("Sending email to {} <{}>", contact.full_name(), contact.email);

        let result = _sender.send(contact);
        _ = _log.write(contact, result);
    }
}
