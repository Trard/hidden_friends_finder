pub use thisvk::UserId;
use std::env;
use fast_vk::{Client, Instance};
use dotenv::dotenv;
use hidden_friends_finder::get_hidden_friends;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let hunt_id = parse_hunt_id().unwrap();

    let instances = Instance::from_tokens(env::var("tokens").unwrap().split(",")).unwrap();
    let client = Client::from_instances(instances);

    let hidden_friends = get_hidden_friends(hunt_id, &client).await;

    println!("{:?}", hidden_friends)
}


#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Hunt id Is missing")]
    HuntIdIsMissing,
    #[error("Cannot parse hunt id")]
    CannotParseHuntId
}

fn parse_hunt_id() -> Result<u32, Error> {
    env::args()
        .skip(1).next().ok_or(Error::HuntIdIsMissing)?
        .parse().map_err(|_|Error::CannotParseHuntId)
}
