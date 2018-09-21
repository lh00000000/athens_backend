use sendgrid::sg_client::SGClient;
use sendgrid::mail::Mail;
use unidecode::unidecode;
use rocket::response::NamedFile;
use std::io::prelude::*;

use super::settings;

pub fn send_email(to_email: &str, from_name: &str, personality: &str) {
    let sg = SGClient::new(settings::get_config("sendgrid_api_key"));

    let from_name = unidecode(&from_name.replace(' ', "."));
    let from_email = format!(
        "{}@{}",
        &from_name,
        settings::get_config("domain")
    );
    let to_email = unidecode(to_email);
    let mut email_body = String::new();
    NamedFile::open("static/email.html").unwrap().file().read_to_string(&mut email_body);
    email_body = email_body
        .replace("{domain}", &settings::get_config("domain"))
        .replace("{personality}", personality);

    let mut mail_info = Mail::new();
    mail_info.add_to(to_email.clone());
    mail_info.add_from(from_name.clone());
    mail_info.add_subject(settings::get_config("email_subject"));
    mail_info.add_html(email_body);
    mail_info.add_from_name(from_name.clone());

    match sg.send(mail_info) {
        Err(err) => {
            error!("Sendgrid Error: {}", err);
            error!("to: {} from: {} personality: {}", to_email, from_email, personality)
        },
        Ok(body) => (),
    };
}
