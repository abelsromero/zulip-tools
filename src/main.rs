use std::{env, process};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    user_id: i32
}

#[derive(Deserialize, Debug)]
struct Stream {
    stream_id: i32,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Topic {
    name: String,
    max_id: i32,
}

#[derive(Deserialize, Debug)]
struct UsersResponse {
    result: String,
    msg: String,
    members: Vec<User>,
}

#[derive(Deserialize, Debug)]
struct StreamsResponse {
    result: String,
    msg: String,
    streams: Vec<Stream>,
}

#[derive(Deserialize, Debug)]
struct TopicsResponse {
    result: String,
    msg: String,
    topics: Vec<Topic>,
}

struct ZulipDomain {
    name: String,
    account: String,
    api_key: String,
}


const URL_PATTERN: &'static str = "https://{}.zulipchat.com/api/v1/{}";


fn error(message: String) {
    println!("{}", message);
    process::exit(1)
}

// https://docs.rs/reqwest/0.11.2/reqwest
// https://github.com/seanmonstar/reqwestF
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 4 {
        error(format!("Invalid arguments count.\n\tUsage: {} [domain] [account] [password]", args[0]));
    }

    let credentials = ZulipDomain {
        name: args[1].clone(),
        account: args[2].clone(),
        api_key: args[3].clone(),
    };

    let users_response = reqwest::Client::new()
        .get(format!("https://{}.zulipchat.com/api/v1/{}", credentials.name, "users"))
        .basic_auth(&credentials.account, Some(&credentials.api_key))
        .send()
        .await?
        .json::<UsersResponse>()
        .await?;

    println!("Users: {}", users_response.members.len());


    let streams_response = reqwest::Client::new()
        .get(format!("https://{}.zulipchat.com/api/v1/{}", credentials.name, "streams"))
        .basic_auth(&credentials.account, Some(&credentials.api_key))
        .send()
        .await?
        .json::<StreamsResponse>()
        .await?;

    println!("Streams: {}", streams_response.streams.len());

    for stream in streams_response.streams {
        let topics_response = reqwest::Client::new()
            .get(format!("https://{}.zulipchat.com/api/v1/users/me/{}/topics", credentials.name, stream.stream_id))
            .basic_auth(&credentials.account, Some(&credentials.api_key))
            .send()
            .await?
            .json::<TopicsResponse>()
            .await?;

        println!("Stream: {}, topics: {}", stream.name, topics_response.topics.len());
    }


    Ok(())
}

