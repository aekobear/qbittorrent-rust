use std::collections::HashMap;

use serde_json::Value;

use crate::{
    core::api::QbitApi, error_handling::error_type::ErrorType, misc::sep_vec::SepVec, Error,
};

use crate::{post_request, post_request_no_return};

impl QbitApi {
    post_request_no_return! {
        /// ## Usage
        /// sets the global upload limit.
        ///
        /// ## Arguments
        /// - limit: the limit, in bytes per second; set to 0 for no limit.
        transfer_set_global_upload_limit,
        "/transfer/setUploadLimit",
        (limit, u64)
    }

    post_request_no_return! {
        /// ## Usage
        /// sets the global download limit.
        ///
        /// ## Arguments
        /// - limit: the limit, in bytes per second; set to 0 for no limit.
        transfer_set_global_download_limit,
        "/transfer/setDownloadLimit",
        (limit, u64)
    }

    /// ## Usage
    /// Returns `1` if the alternative speed limits are enabled, `0` if they're not.
    pub async fn transfer_get_alternative_speed_limits(&mut self) -> Result<u8, crate::Error> {
        let resp = Self::make_request(
            self,
            "/transfer/speedLimitsMode",
            format!("{}", "transfer_get_alternative_speed_limits"),
        )
        .await?;
        if resp.chars().next().unwrap() == '0' {
            return Ok(0);
        } else if resp.chars().next().unwrap() == '1' {
            return Ok(1);
        } else {
            return Err(Error::build(ErrorType::ParameterNotExpected, None));
        }
    }

    post_request_no_return! {
        /// ## Usage
        /// Toggles the alternative speed limits.
        transfer_toggle_alternative_speed_limits,
        "/transfer/toggleSpeedLimitsMode"
    }

    /// ## Usage
    /// Gets the global download limit in bytes per second; this value will be 0 if there's no limit.
    pub async fn transfer_get_global_download_limit(&mut self) -> Result<u64, crate::Error> {
        let str = Self::make_request(
            self,
            "/transfer/downloadLimit",
            format!("{}", "transfer_get_global_download_limit"),
        )
        .await?;
        Ok(str
            .parse::<u64>()
            .map_err(|_| Error::build(ErrorType::ParameterNotExpected, None))?)
    }

    /// ## Usage
    /// Gets the global upload limit in bytes per second; this value will be 0 if there's no limit.
    pub async fn transfer_get_global_upload_limit(&mut self) -> Result<u64, crate::Error> {
        let str = Self::make_request(
            self,
            "/transfer/uploadLimit",
            format!("{}", "transfer_get_global_upload_limit"),
        )
        .await?;
        Ok(str
            .parse::<u64>()
            .map_err(|_| Error::build(ErrorType::ParameterNotExpected, None))?)
    }

    /// ## Usage
    /// Bans specified peers.
    /// 
    /// ## Arguments
    /// - peers: a [`Vec`] of strings, where each element is structured as `host:port`
    pub async fn transfer_ban_peers<S: Into<String>>(
        &mut self,
        peers: Vec<S>,
    ) -> Result<(), Error> {
        let x = SepVec::new(
            peers.into_iter().map(|s| s.into()).collect::<Vec<String>>(),
            "|",
        );

        let mut hashmap = HashMap::new();

        let y = x.to_string();

        hashmap.insert("peers", y.as_str());

        self.make_request_with_form("/transfer/banPeers", "ban_peers", hashmap)
            .await?;
        Ok(())
    }

    post_request! {
        /// ## Usage
        /// Gets the global transfer info as a [`String`].
        transfer_get_global_transfer_info_raw,
        "/transfer/info"
    }

    /// ## Usage
    /// Gets the global transfer info as a json [`Value`].
    pub async fn transfer_get_global_transfer_info(&mut self) -> Result<Value, Error> {
        serde_json::from_str(
            Self::transfer_get_global_transfer_info_raw(self)
                .await?
                .as_str(),
        )
        .map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }
}

#[tokio::test]
async fn test() {
    let mut api = QbitApi::new(
        "http://localhost:6011///",
        crate::core::creds::Credentials::new("admin", "123456"),
    )
    .await
    .unwrap();
    let _thing = api.transfer_set_global_upload_limit(0).await.unwrap();

    //println!("{}", thing)
}
