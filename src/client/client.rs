use reqwest;
use crate::http::discord::GatewayResponse;

pub struct Client {
    pub token: String,
    pub http: reqwest::Client,
    pub wss_url: String,
    pub base_url: String
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {
            token: String::from(token),
            http: reqwest::Client::new(),
            base_url: String::from("https://discord.com/api/v10"),
            wss_url: String::from("wss://gateway.discord.gg")
        }
    }

    /// Returns a new Client with default settings
    /// 
    /// ## Arguments
    /// 
    /// * `token` - Token string prefixed by "Bot"
    /// 
    /// ## Examples
    /// ```rs
    /// // Creating the base client
    /// let token = format!("Bot {token_string}");
    /// let mut client = slash_rs::Client::new(&token);
    /// ```
    pub async fn build(token: &str) -> Result<Self, reqwest::Error> {
        let mut client = Client::new(token)
            .set_gateway()
            .await?;
        Ok(client)
    }

    /// Returns a new Client with an updated Gateway URL
    pub async fn set_gateway(self) -> Result<Self, reqwest::Error> {
        let wss_url = self.http
        .get(format!("{}/gateway/bot", &self.base_url))
        .header("Authorization", format!("Bot {}", self.token))
        .send()
        .await?
        .json::<GatewayResponse>()
        .await?
        .url;

        Ok(Client {wss_url, ..self})
    }
}