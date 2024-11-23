use std::collections::HashMap;

use serde_json::Value;

use crate::{core::api::Api, error_handling::error_type::ErrorType, misc::sep_vec::SepVec, Error};

use crate::{post_request, post_request_no_return};

impl Api {
    post_request_no_return!{set_upload_limit, "/transfer/setUploadLimit", (limit, u64)}

    post_request!{get_alt_speed_limits, "/transfer/speedLimitsMode"}

    post_request_no_return!{toggle_alt_speed_limits, "/transfer/toggleSpeedLimitsMode"}

    post_request!{get_global_download_limit, "/transfer/downloadLimit"}

    post_request!{get_global_upload_limit, "/transfer/uploadLimit"}

    pub async fn ban_peers<S: Into<String>>(&mut self, peers: Vec<S>) -> Result<(), Error> {
        let x = SepVec::new(peers.into_iter().map(|s| s.into()).collect::<Vec<String>>(), "|");

        let mut hashmap = HashMap::new();

        let y = x.to_string();

        hashmap.insert("peers", y.as_str());

        self.make_request_with_form("/transfer/banPeers", "ban_peers", hashmap).await?;
        Ok(())
    }

    post_request!{get_global_transfer_info_raw, "/transfer/info"}

    pub async fn get_global_transfer_info(&mut self) -> Result<Value, Error> {
        serde_json::from_str(Self::get_global_transfer_info_raw(self).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }
}

#[tokio::test]
async fn test() {
    let mut api = Api::new("http://localhost:6011///", crate::core::creds::Credentials::new("admin", "123456")).await.unwrap();
    let _thing = api.set_upload_limit(0).await.unwrap();

    //println!("{}", thing)
}
