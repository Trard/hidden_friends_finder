pub use thisvk::UserId;
use std::collections::HashSet;
use fast_vk::Client;
use futures::future::join_all;
use thisvk::API;
use tokio::sync::RwLock;
use std::sync::Arc;
use anyhow::Result;

pub async fn get_hidden_friends(hunt_id: UserId, client: &Client) -> Result<HashSet<UserId>> {
    let hunt_friends: HashSet<UserId> = client.friends_get()
        .user_id(hunt_id)
        .send().await.unwrap().items
        .into_iter().collect();

    let public_friends = hunt_friends.clone();

    let checked_users = Arc::new(RwLock::new(HashSet::new()));
    checked_users.write().await.insert(hunt_id);

    let result: Vec<_> = public_friends.iter().map(|friend| recursive_find(*friend, hunt_id, client, Arc::clone(&checked_users))).collect();

    let all_friends: HashSet<UserId> = join_all(result).await.into_iter().flatten().flatten().collect();

    let hidden_friends = &all_friends - &public_friends;

    Ok(hidden_friends)
}

#[async_recursion::async_recursion]
async fn recursive_find(id: UserId, hunt_id: UserId, client: &Client, checked_users: Arc<RwLock<HashSet<u32>>>) -> Option<HashSet<UserId>> {
    if checked_users.read().await.contains(&id) {
        return None
    } else {
        checked_users.write().await.insert(id);
    }

    let friends = match client.friends_get()
        .user_id(id).send().await {
            Ok(resp) => { resp.items },
            Err(error) => {
                match error {
                    fast_vk::Error::VK { .. } => { return None },
                    fast_vk::Error::Arc(error) => match error.as_ref() {
                        fast_vk::Error::VK { .. } => return None,
                        another => {
                            panic!("{}", another);
                        }
                    },
                    another => {
                        panic!("{}", another);
                    }
                }
            }
        };

    if friends.binary_search(&hunt_id).is_ok() {
        let response = join_all(friends.into_iter().map(|id| recursive_find(id, hunt_id, client, Arc::clone(&checked_users)))).await;

        let mut ret: HashSet<UserId> = response.into_iter().flatten().flatten().collect();
        ret.insert(id);

        Some(ret)
    } else {
        None
    }
}
