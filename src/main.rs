use std::collections::HashMap;
use std::collections::HashSet;

type UserId = i32;

pub struct FriendsGraph {
    internal: HashMap<UserId, HashSet<UserId>>
}

impl FriendsGraph {
    pub fn new() -> FriendsGraph {
        FriendsGraph {
            internal: HashMap::new()
        }
    }

    pub fn add_friends_to(&mut self, id: UserId, friends: Vec<UserId>) {
        for friend in friends {
            self.add_friendship(id, friend);
        }
    }

    pub fn add_friendship(&mut self, user1: UserId, user2: UserId) {
        self.internal.entry(user1).or_insert(HashSet::new()).insert(user2);
        self.internal.entry(user2).or_insert(HashSet::new()).insert(user1);
    }

    pub fn get_friends(&self, user: &UserId) -> Option<&HashSet<UserId>> {
        self.internal.get(user)
    }
}

fn main() {
    let mut foo = FriendsGraph::new();

    for i in 0..100 {
        foo.add_friends_to(i, vec![1,2,3,4,5,6,7,8,9]);
    }

    for i in 0..100 {
        println!("{}: {:?}", i, foo.get_friends(&i).unwrap());
    }
}