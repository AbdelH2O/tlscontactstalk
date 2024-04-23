use std::env;
use tlscontactstalk::stalker::Stalker;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let creds = match env::var("CREDENTIALS") {
        Ok(val) => val,
        Err(e) => panic!("CREDENTIALS not found in environment: {}", e),
    };
    let username = creds.split(":").next().unwrap();
    let password = creds.split(":").last().unwrap();

    let group = match env::var("GROUP") {
        Ok(val) => val,
        Err(e) => panic!("GROUP not found in environment: {}", e),
    };
    let stalker = Stalker::new(username.to_string(), password.to_string(), group, 60);
    println!("Logging in...");
    println!("{}", stalker.login().unwrap());
}
