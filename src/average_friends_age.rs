use chrono::{naive::NaiveDateTime, Datelike};
use fast_vk::{Client, Result};
use thisvk::{UserId, UsersFields, API};

pub async fn get_average_friends_age(hunt_id: UserId, client: &Client) -> Result<NaiveDateTime> {
    let friends = client.friends_get().user_id(hunt_id).send().await?.items;

    let (age_sum, friends_count) = client
        .users_get()
        .user_ids(friends)
        .fields(vec![UsersFields::bdate])
        .send()
        .await
        .unwrap()
        .iter()
        .fold((0, 0), |(mut age_sum, mut friends_count), user| {
            if let Some(bdate) = user.bdate {
                if bdate.year() == 0 {
                    return (age_sum, friends_count);
                }

                friends_count += 1;
                age_sum += bdate.and_hms(0, 0, 0).timestamp();
            }

            (age_sum, friends_count)
        });

    let average = age_sum / (friends_count as i64);

    let average_date = NaiveDateTime::from_timestamp(average, 0);

    Ok(average_date)
}
