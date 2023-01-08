use reqwest;
use error_chain::error_chain;
use crate::http::discord::GatewayResponse;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub struct Client {
    token: String,
    http: reqwest::Client
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {token: String::from(token), http: reqwest::Client::new()}
    }

    pub async fn gateway(&self, version: u32) -> Result<GatewayResponse> {
        let base_url = String::from(format!("https://discord.com/api/v{}", version));
        let res = self.http
        .get(format!("{}/gateway/bot", base_url))
        .header("Authorization", format!("Bot {}", self.token))
        .send()
        .await?
        .json::<GatewayResponse>()
        .await?;

        println!("{:?}", &res.url);

        Ok(res)
    }
}