use fast_vk::{Client, Result};
use thisvk::{UserId, API, UsersFields};
use chrono::naive::NaiveDateTime;

pub async fn get_average_friends_age(hunt_id: UserId, client: &Client) -> Result<NaiveDateTime> {
    let friends = client.friends_get().user_id(hunt_id).send().await?.items;

    let friends_count = friends.len();

    let age_sum = client
        .users_get()
        .user_ids(friends)
        .fields(vec![UsersFields::bdate])
        .send()
        .await
        .unwrap()
        .iter()
        .fold(0,|mut age_sum, user| {
            //println!("{:?}", user);

            if let Some(bdate) = user.bdate {
                age_sum += bdate.and_hms(0, 0, 0).timestamp();
            }

            age_sum
        });
    
    println!("{}", age_sum);

    let average = age_sum / (friends_count as i64);

    let average_date = NaiveDateTime::from_timestamp(average, 0);

    Ok(average_date)
}
