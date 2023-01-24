use dotenv::dotenv;
use fast_vk::{Client, Instance};
use spybot::{get_average_friends_age, get_hidden_friends, get_median_friends_age};
use std::{env, collections::HashMap};
pub use thisvk::{UserId, API};
use std::cmp::Ordering;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenv().unwrap();

    let instances = Instance::from_tokens_by_prototype(
        env::var("tokens")?.split(','),
        Instance::builder().time_between_requests(std::time::Duration::from_millis(400)),
    )?;

    let client = Client::from_instances(instances);

    let hunt_id = parse_hunt_id()?;

    let hunt_id = match hunt_id.parse::<UserId>() {
        Ok(number) => number,
        Err(_) => client.users_get().user_id(hunt_id).send().await?[0].id,
    };
    
    let average_age = get_average_friends_age(hunt_id, &client).await?;
    let median_age = get_median_friends_age(hunt_id, &client).await?;
    let hidden_friends = get_hidden_friends(hunt_id, &client).await?;

    pretty_print_median_age(&median_age);
    println!("Average: {}", average_age);
    println!("Hidden: {:?}", hidden_friends);

    Ok(())
}

fn pretty_print_median_age(median_age: &HashMap<UserId, u16>) {
    let mut median_age: Vec<(&UserId, &u16)> = median_age.iter().collect();

    median_age.sort_by(|(a_year, a_times), (b_year, b_times)| {
        match b_times.cmp(a_times) {
            Ordering::Equal => {
                b_year.cmp(a_year)
            },
            other => other
        }
    });
    
    for (year, times) in median_age {
        println!("[{}] {year}", "#".repeat(*times as usize));
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Hunt id is missing")]
    HuntIdIsMissing,
}

fn parse_hunt_id() -> Result<String, Error> {
    env::args().nth(1).ok_or(Error::HuntIdIsMissing)
}
