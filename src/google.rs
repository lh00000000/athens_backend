use frank_jwt::{Algorithm, encode, decode};
use std::time::SystemTime;

use super::settings;


#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    access_token: String,
    expires_in: i64,
    token_type: String,
}

pub fn set_service_account_token() -> String {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let mut payload = json!({
      "iss": settings::get_config("google_service_email"),
      "scope": "https://www.googleapis.com/auth/analytics.readonly",
      "aud": "https://www.googleapis.com/oauth2/v4/token",
      "exp": now+3600,
      "iat": now
    });


    let header = json!({"alg":"RS256","typ":"JWT"});
    let jwt = encode(header, &settings::get_config("google_service_private_key").unwrap(), &payload, Algorithm::RS256).unwrap();

    let client = reqwest::Client::new();
    let params = [("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"), ("assertion", &format!("{}", jwt))];

    let access_token: AccessToken = client.post("https://www.googleapis.com/oauth2/v4/token")
        .form(&params)
        .send()
        .unwrap()
        .json()
        .unwrap();

    settings::set_config("analytics_token_initialized_at", &format!("{}", now));
    settings::set_config("google_analytics_access_token", &format!("{}", &access_token.access_token));

    access_token.access_token
}

pub fn get_analytics() {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let access_token = match settings::get_config("google_analytics_token_initialized_at") {
        Some(time) => match now - time.parse::<u64>().unwrap() < 3599 {
            true => settings::get_config("google_analytics_access_token").unwrap(),
            false => set_service_account_token()
        }
        None => {
            set_service_account_token()
        }
    };
    let client = reqwest::Client::new();
    let analytics_request_body = json!({
        "reportRequests": [
            {
                "viewId": "181843413",
                "dateRanges": [
                    {
                        "startDate": "1000daysAgo",
                        "endDate": "today"
                    }
                ],
                "dimensions": [
                    {
                        "name": "ga:country"
                    }
                ]
            }
        ]
    });

    let mut analytics_resp = client.post("https://analyticsreporting.googleapis.com/v4/reports:batchGet")
        .json(&analytics_request_body)
        .bearer_auth(access_token)
        .send()
        .unwrap();

    println!("{}", analytics_resp.text().unwrap());
}