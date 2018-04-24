use std::collections::HashMap;
use reqwest;
use reqwest::{Client, Result, header};
use models::Task;

const API_URL: &str = "https://sm-prod2.any.do";

#[derive(Debug)]
pub struct ApiClient {
    token: String,
    client: Client
}

#[derive(Debug)]
#[derive(Deserialize)]
struct LoginResp {
    auth_token: String
}

header! {
    (XAnydoAuth, "X-Anydo-Auth") => [String]
}


#[allow(dead_code)]
impl ApiClient {
    fn new (email: &str, password: &str) -> Result<ApiClient> {
        let mut body = HashMap::new();
        body.insert("email", email);
        body.insert("password", password);

        let cli = reqwest::Client::new();
        let res: LoginResp = cli.post(&format!("{}{}", API_URL, "/login"))
            .json(&body)
            .send()?
            .json()?;
        
        let mut headers = header::Headers::new();
        headers.set(XAnydoAuth(res.auth_token.clone()));
        headers.set(header::ContentType::json());

        let client = Client::builder()
            .default_headers(headers)
            .build()?;
        
        return Ok(ApiClient{
            token: res.auth_token,
            client: client
        })
    }

    fn sync(include_done: bool, include_deleted: bool, update_since: u64) -> Vec<Task> {
        unimplemented!();
    }
}
