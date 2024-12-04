use serde_json::Value;

use crate::{core::api::QbitApi, error_handling::error_type::ErrorType, Error};

impl QbitApi {
    /// ## Usage
    /// Gets the main sync data as a [`String`].
    pub async fn sync_get_main_data_raw(&mut self, rid: u64) -> Result<String, Error> {
        Self::make_request(self, format!("/sync/maindata?rid={}", rid), "sync_get_main_data".to_string()).await
    }

    /// ## Usage
    /// Gets the main sync data as a json [`Value`].
    pub async fn sync_get_main_data(&mut self, rid: u64) -> Result<Value, Error> {
        serde_json::from_str(Self::sync_get_main_data_raw(self, rid).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    /// ## Usage
    /// Gets the torrents peer's sync data as a [`String`].
    pub async fn sync_get_torrent_peers_data_raw(&mut self, hash: impl AsRef<String>, rid: u64) -> Result<String, Error> {
        Self::make_request(self, format!("/sync/torrentPeers?hash={}&rid={}",hash.as_ref(), rid), "sync_get_torrent_peers_data".to_string()).await
    }

    /// ## Usage
    /// Gets the torrents peer's sync data as a json [`Value`].
    pub async fn sync_get_torrent_peers_data(&mut self, hash: impl AsRef<String>, rid: u64) -> Result<Value, Error> {
        serde_json::from_str(Self::sync_get_torrent_peers_data_raw(self, hash, rid).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }
}