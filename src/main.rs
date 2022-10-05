pub use thisvk::UserId;
use std::{env, collections::HashSet};
use fast_vk::{Client, Instance};
use dotenv::dotenv;
use futures::future::join_all;
use thisvk::API;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().unwrap();

    let hunt_id = parse_hunt_id().unwrap_or(551175426);

    let instances = Instance::from_tokens(env::var("tokens").unwrap().split(",")).unwrap();
    let client = Client::from_instances(instances);

    let mut hunt_friends: HashSet<UserId> = client.friends_get()
        .user_id(hunt_id)
        .send().await.unwrap().items
        .into_iter().collect();

    let far_friends = get_friends_of_iter(hunt_friends.clone(), &client).await;
    
    let is_friend = far_friends.into_iter().map(|id| {
        let client = &client;

        async move {
            client.friends_get()
                .user_id(id).send().await.ok()
                ?.items
                .binary_search(&hunt_id).ok().and(Some(id))
        }
    });

    join_all(is_friend).await.into_iter().for_each(|result| {
        if let Some(id) = result {
            hunt_friends.insert(id);
        }
    });

    println!("{:?}", hunt_friends);

    Ok(())
}

async fn get_friends_of_iter(users: impl IntoIterator<Item = UserId>, client: &Client) -> HashSet<UserId> {
    let futures: Vec<_> = users.into_iter().map(|id| {
        let client = client;

        async move {
            match client.friends_get()
                .user_id(id)
                .send().await {
                    Ok(resp) => resp.items,
                    Err(_) => Vec::with_capacity(0)
                }
        }
    }).collect();

    join_all(futures).await.into_iter().flatten().collect()
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