pub use thisvk::UserId;
use std::{env, collections::HashSet};
use fast_vk::{Client, Instance};
use dotenv::dotenv;
use futures::future::join_all;
use thisvk::API;
use tokio::sync::RwLock;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().unwrap();

    let hunt_id = parse_hunt_id().unwrap();

    let instances = Instance::from_tokens(env::var("tokens").unwrap().split(",")).unwrap();
    let client = Client::from_instances(instances);

    let hunt_friends: HashSet<UserId> = client.friends_get()
        .user_id(hunt_id)
        .send().await.unwrap().items
        .into_iter().collect();

    let public_friends = hunt_friends.clone();

    let checked_users = Arc::new(RwLock::new(HashSet::new()));
    checked_users.write().await.insert(hunt_id);

    let result: Vec<_> = public_friends.iter().map(|friend| recursive_find(*friend, hunt_id, &client, Arc::clone(&checked_users))).collect();

    let all_friends: HashSet<UserId> = join_all(result).await.into_iter().filter_map(|maybe_friends| maybe_friends).flatten().collect();

    let hidden_friends = &all_friends - &public_friends;

    println!("Hidden: {:?}", hidden_friends);
    println!("Checked {} users", checked_users.read().await.len());

    Ok(())
}

#[async_recursion::async_recursion]
async fn recursive_find(id: UserId, hunt_id: UserId, client: &Client, checked_users: Arc<RwLock<HashSet<u32>>>) -> Option<HashSet<UserId>> {
    if checked_users.read().await.contains(&id) {
        return None
    } else {
        checked_users.write().await.insert(id);
    }

    let friends = client.friends_get()
        .user_id(id).send().await.ok()?.items;

    if let Ok(_) = friends.binary_search(&hunt_id) {
        let response = join_all(friends.into_iter().map(|id| recursive_find(id, hunt_id, client, Arc::clone(&checked_users)))).await;

        let mut ret: HashSet<UserId> = response.into_iter().filter_map(|maybe_friends| maybe_friends).flatten().collect();
        ret.insert(id);

        Some(ret)
    } else {
        None
    }
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