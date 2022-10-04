use spy_bot::FriendsGraph;
use std::env;
use fast_vk::{Client, Instance};
use dotenv::dotenv;
use futures::future::join_all;
use thisvk::API;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().unwrap();

    let hunt_id = parse_hunt_id().unwrap_or(551175426);
    let mut friends = FriendsGraph::new();

    let instances = Instance::from_tokens(env::var("tokens").unwrap().split(",")).unwrap();
    let client = Client::from_instances(instances);

    let hunt_friends = client.friends_get().user_id(hunt_id).send().await.unwrap().items;

    friends.add_friends_to(hunt_id, hunt_friends);

    println!("{:?}", friends.get_friends(&hunt_id));

    Ok(())
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