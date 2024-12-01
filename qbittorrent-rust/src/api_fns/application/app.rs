use serde_json::{self, Value};
use crate::{core::api::QbitApi, error_handling::error_type::ErrorType, post_request, post_request_no_return, Error};

impl QbitApi {
    post_request!{app_version, "/app/version"}

    post_request!{app_web_api_version, "/app/webapiVersion"}

    post_request!{app_build_info_raw, "/app/buildInfo"}

    pub async fn app_build_info(&mut self) -> Result<Value, Error> {
        serde_json::from_str(Self::app_build_info_raw(self).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    post_request_no_return!{app_shutdown, "/app/shutdown"}

    post_request!(app_get_default_save_path, "/app/defaultSavePath");
}

#[tokio::test]
async fn test() {
    let mut api = QbitApi::new("http://localhost:6011", crate::core::creds::Credentials::new("admin", "123456")).await.unwrap();

    println!("{:?}", api);

    let v = api.app_web_api_version().await.unwrap();

    println!("{}", v)
    
}