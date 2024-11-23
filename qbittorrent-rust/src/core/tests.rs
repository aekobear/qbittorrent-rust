use crate::core::{api::Api, creds::Credentials};

#[tokio::test]
async fn test() {
    let mut api = Api::new("http://localhost:6011", Credentials::new("admin", "123456")).await.unwrap();

    let thing = api.get_preferences_raw().await.unwrap();

    println!("{}", thing)
}

