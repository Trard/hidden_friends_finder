use std::collections::HashMap;
use chrono::Datelike;
use vk_executive::{Client, Result};
use thisvk::{UserId, UsersFields, API, User};

pub async fn get_median_friends_age(hunt_id: UserId, client: &Client) -> Result<HashMap<UserId, u16>> {
    let mut median = HashMap::new();

    let friends = client.friends_get().user_id(hunt_id).send().await?.items;

    client
        .users_get()
        .user_ids(friends)
        .fields(vec![UsersFields::bdate])
        .send()
        .await
        .unwrap()
        .iter()
        .for_each(|user| {
            if let User { bdate: Some(bdate), .. } = user {
                let year = bdate.year();

                if year != 0 {
                    *median.entry(year as u32).or_insert(0) += 1u16;
                }
            }
        });

    Ok(median)
}
