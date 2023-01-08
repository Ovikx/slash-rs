use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GatewayResponse {
    pub url: String,
    pub shards: i32,
    pub session_start_limit: SessionStartLimit
}

#[derive(Debug, Deserialize)]
pub struct SessionStartLimit {
    pub total: i64,
    pub remaining: i64,
    pub reset_after: i64,
    pub max_concurrency: i32
}