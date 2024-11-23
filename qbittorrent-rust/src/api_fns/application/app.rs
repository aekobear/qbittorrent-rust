use serde_json::{self, Value};
use crate::{core::api::Api, error_handling::error_type::ErrorType, post_request, post_request_no_return, Error};

impl Api {
    post_request!{app_version, "/app/version"}

    post_request!{web_api_version, "/app/webapiVersion"}

    post_request!{build_info_raw, "/app/buildInfo"}

    pub async fn build_info(&mut self) -> Result<Value, Error> {
        serde_json::from_str(Self::build_info_raw(self).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    post_request_no_return!{shutdown, "/app/shutdown"}

    post_request!(get_default_save_path, "/app/defaultSavePath");
}

#[tokio::test]
async fn test() {
    let mut api = Api::new("http://localhost:6011", crate::core::creds::Credentials::new("admin", "123456")).await.unwrap();

    println!("{:?}", api);

    let v = api.web_api_version().await.unwrap();

    println!("{}", v)
    
}