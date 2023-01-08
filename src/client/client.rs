use awc;
use crate::http::discord::GatewayResponse;

pub struct Client {
    pub token: String,
    pub http: awc::Client,
    pub wss_url: Option<String>
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {token: String::from(token), http: awc::Client::new(), wss_url: None}
    }

    pub async fn gateway(&mut self, version: u32) -> Result<GatewayResponse, Box<dyn std::error::Error>> {
        let base_url = String::from(format!("https://discord.com/api/v{}", version));
        print!("HI");
        let res = self.http
        .get(format!("{}/gateway/bot", base_url))
        .insert_header(("Authorization", format!("Bot {}", self.token)))
        .send()
        .await?
        .json::<GatewayResponse>()
        .await?;

        self.wss_url = Some(String::from(&res.url));
        print!("{:?}", &res);
        Ok(res)
    }
}