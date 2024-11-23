use serde_json::Value;

use crate::{core::api::Api, error_handling::error_type::ErrorType, Error};

impl Api {
    pub async fn sync_get_main_data_raw(&mut self, rid: u64) -> Result<String, Error> {
        Self::make_request(self, format!("/sync/maindata?rid={}", rid), "sync_get_main_data".to_string()).await
    }

    pub async fn sync_get_main_data(&mut self, rid: u64) -> Result<Value, Error> {
        serde_json::from_str(Self::sync_get_main_data_raw(self, rid).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    pub async fn sync_get_torrent_peers_data_raw(&mut self, hash: impl AsRef<String>, rid: u64) -> Result<String, Error> {
        Self::make_request(self, format!("/sync/torrentPeers?hash={}&rid={}",hash.as_ref(), rid), "sync_get_torrent_peers_data".to_string()).await
    }

    pub async fn sync_get_torrent_peers_data(&mut self, hash: impl AsRef<String>, rid: u64) -> Result<Value, Error> {
        serde_json::from_str(Self::sync_get_torrent_peers_data_raw(self, hash, rid).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }
}

#[tokio::test]
pub async fn test() {
    let mut api = Api::new("http://localhost:6011/", crate::core::creds::Credentials::new("admin", "123456")).await.unwrap();

    let x = api.sync_get_main_data_raw(0).await.unwrap();

    println!("{}", x)
}
