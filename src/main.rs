use spy_bot::FriendsGraph;

fn main() {
    let mut foo = FriendsGraph::new();

    for i in 0..100 {
        foo.add_friends_to(i, vec![1,2,3,4,5,6,7,8,9]);
    }

    for i in 0..100 {
        println!("{}: {:?}", i, foo.get_friends(&i).unwrap());
    }
}