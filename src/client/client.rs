use awc;
use crate::http::discord::GatewayResponse;

pub struct Client {
    pub token: String,
    pub http: awc::Client,
    pub wss_url: String
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {token: String::from(token), http: awc::Client::new(), wss_url: String::from("wss://gateway.discord.gg")}
    }

    pub async fn connect(&mut self) -> Result<GatewayResponse, Box<dyn std::error::Error>> {
        let base_url = String::from("https://discord.com/api/v10");
        print!("HI");
        let res = self.http
        .get(format!("{}/gateway/bot", base_url))
        .insert_header(("Authorization", format!("Bot {}", self.token)))
        .send()
        .await?
        .json::<GatewayResponse>()
        .await?;

        self.wss_url = String::from(&res.url);
        print!("{:?}", &res);
        Ok(res)
    }
}