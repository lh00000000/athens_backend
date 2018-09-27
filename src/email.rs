use std::io::prelude::*;
use std::net::TcpListener;
use std::io::BufReader;

use sendgrid::sg_client::SGClient;
use sendgrid::mail::Mail;
use unidecode::unidecode;
use rocket::response::NamedFile;
use select::document::Document;
use select::predicate::{Name};
use oauth2::{Config, Token};
use url::Url;

use super::settings;
use super::api::Consent;

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
    NamedFile::open("static/templates/email.html").unwrap().file().read_to_string(&mut email_body);
    email_body = email_body
        .replace("{domain}", &settings::get_config("domain"))
        .replace("{personality}", personality);

    let mut mail_info = Mail::new();
    mail_info.add_to(to_email.clone());
    mail_info.add_from(from_name.clone());
    mail_info.add_subject(settings::get_config("email_subject"));
    mail_info.add_html(email_body);
    mail_info.add_from_name(from_name.clone());

//    println!("{:?}", mail_info);

//    match sg.send(mail_info) {
//        Err(err) => println!("Error: {}", err),
//        Ok(body) => println!("Response: {}", body),
//    };
}

pub fn send_emails(consent: &Consent) {
    for email in get_email_contacts(&consent.access_token) {
        send_email(
            &email,
            &consent.from_name,
            &consent.personality,
        )
    }
}

pub fn get_email_contacts(access_token: &str) -> Vec<String> {
    let client = reqwest::Client::new();
    let emails = client.get("https://www.google.com/m8/feeds/contacts/default/full?max-results=1000")
        .bearer_auth(access_token)
        .send().unwrap().text().unwrap();

    let document = Document::from(emails.as_str());
    document.find(Name("gd:email")).map(
        |node| node.attr("address").unwrap().to_string()
    ).collect()
}

fn get_access_token() -> Token {
    let google_client_id = settings::get_config("google_client_id");
    let google_client_secret = settings::get_config("google_client_secret");
    let auth_url = "https://accounts.google.com/o/oauth2/v2/auth";
    let token_url = "https://www.googleapis.com/oauth2/v3/token";

    // Set up the config for the Google OAuth2 process.
    let mut config = Config::new(google_client_id, google_client_secret, auth_url, token_url);

    // This example is requesting access to the "calendar" features and the user's profile.
    config = config.add_scope("https://www.googleapis.com/auth/contacts.readonly");
    config = config.add_scope("https://www.googleapis.com/auth/plus.me");

    // This example will be running its own server at localhost:8080.
    // See below for the server implementation.
    config = config.set_redirect_url("http://localhost:8080");

    // Set the state parameter (optional)
    config = config.set_state("1234");

    // Generate the authorization URL to which we'll redirect the user.
    let authorize_url = config.authorize_url();

    println!("Open this URL in your browser:\n{}\n", authorize_url.to_string());

    // These variables will store the code & state retrieved during the authorization process.
    let mut code = String::new();
    let mut state = String::new();
    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                {
                    let mut reader = BufReader::new(&stream);

                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    let code_pair = url.query_pairs().find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    }).unwrap();

                    let (_, value) = code_pair;
                    code = value.into_owned();

                    let state_pair = url.query_pairs().find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    }).unwrap();

                    let (_, value) = state_pair;
                    state = value.into_owned();
                }

                let message = "Go back to your terminal :)";
                let response = format!("HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}", message.len(), message);
                stream.write_all(response.as_bytes()).unwrap();

                // The server will terminate itself after collecting the first code.
                break;
            }
            Err(_) => {}
        }
    };

    println!("Google returned the following code:\n{}\n", code);
    println!("Google returned the following state:\n{}\n", state);

    // Exchange the code with a token.
    let token = config.exchange_code(code);

    println!("Google returned the following token:\n{:?}\n", token);
    token.unwrap()
}

