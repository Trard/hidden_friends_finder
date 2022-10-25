pub use thisvk::{UserId, API};
use std::env;
use fast_vk::{Client, Instance};
use dotenv::dotenv;
use spybot::get_hidden_friends;
use spybot::get_average_friends_age;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let instances = Instance::from_tokens_by_prototype(
        env::var("tokens").unwrap().split(','),
        Instance::builder().time_between_requests(std::time::Duration::from_millis(400))
    ).unwrap();

    let client = Client::from_instances(instances);
    
    let hunt_id = parse_hunt_id().unwrap();
    
    let hunt_id = match hunt_id.parse::<UserId>() {
        Ok(number) => number,
        Err(_) => {
            client.users_get().user_id(hunt_id).send().await.unwrap()[0].id
        }
    };

    let average_age = get_average_friends_age(hunt_id, &client).await;
    //let hidden_friends = get_hidden_friends(hunt_id, &client).await;

    println!("{:?}", average_age)
}


#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Hunt id Is missing")]
    HuntIdIsMissing,
}

fn parse_hunt_id() -> Result<String, Error> {
    env::args()
        .nth(1).ok_or(Error::HuntIdIsMissing)
}
