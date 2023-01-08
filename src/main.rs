use dotenv::dotenv;
use std::env;
use slash_rs::Client;
use actix_web;

#[actix_web::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("Could not read token");
    let mut client = Client::new(&token);
    let res = match client.connect().await {
        Ok(res) => res,
        Err(_err) => panic!("{}", _err)
    };

    println!("{:?}", &res.url);
}