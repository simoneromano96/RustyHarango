use chameleon_db_client::DBClient;

fn main() {
    let client = DBClient::new("http://localhost:8529");
    // println!("{:?}", client.is_db_available());
    // println!("{:?}", client.authenticate("root", "password123"));
    // println!("{:?}", client.authenticate("root", "password"));
    // let databases = client.get_all_databases().unwrap();
    // println!("{:?}, {:?}", databases, databases[0]);
    // client.select_database("test");
    // let collections = client.get_all_collections().unwrap();
    // println!("{:?}", collections);
    // let collection = client.get_collection("test1").unwrap();
    // println!("{:?}", collection);
    // let new_collection = client.post_collection("test3").unwrap();
    // println!("{:?}", new_collection);
}
