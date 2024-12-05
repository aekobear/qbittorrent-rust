use std::borrow::Borrow;

use proc_macros::Builder;
use serde_json::Value;

use crate::{core::api::QbitApi, error_handling::error_type::ErrorType, url, Error};

/// ## Info
/// Descriptor for which kinds of logs to get with [`QbitApi::log_get_log()`].
pub struct GetLogConfig {
    normal: bool,
    info: bool,
    warning: bool,
    critical: bool,
    last_known_id: i64,
}
impl GetLogConfig {
    /// ## Usage
    /// creates a new [`GetLogConfig`] with default values:
    ///
    /// normal: true,
    /// 
    /// info: true,
    /// 
    /// warning: true,
    /// 
    /// critical: true,
    /// 
    /// last_known_id: -1
    pub fn new() -> GetLogConfig {
        GetLogConfig {
            normal: true,
            info: true,
            warning: true,
            critical: true,
            last_known_id: -1,
        }
    }

    /// ## Usage
    /// Returns the builder for [`GetLogConfig`]: [`GetLogConfigBuilder`].
    pub fn builder() -> GetLogConfigBuilder {
        GetLogConfigBuilder::new()
    }
}

#[derive(Builder)]
/// ## Info
/// Builder struct for [`GetLogConfig`].
/// 
/// ## Fields
/// | Parameter         | Type    | Description                                                            |
/// |-------------------|---------|------------------------------------------------------------------------|
/// | `normal`          | `Bool`    | Include normal messages (default: `true`)                              |
/// | `info`            | `Bool`    | Include info messages (default: `true`)                                |
/// | `warning`         | `Bool`    | Include warning messages (default: `true`)                             |
/// | `critical`        | `Bool`    | Include critical messages (default: `true`)                            |
/// | `last_known_id`   | `Integer` | Exclude messages with "message id" lower or equal `last_known_id` (default: `-1`)  |
pub struct GetLogConfigBuilder {
	normal: Option<bool>,
	info: Option<bool>,
	warning: Option<bool>,
	critical: Option<bool>,
	last_known_id: Option<i64>,
}
impl GetLogConfigBuilder {
    /// ## Usage
    /// creates a new clean instance of [`GetLogConfigBuilder`].
    pub fn new() -> Self {
        Self {
            normal: None,
            info: None,
            warning: None,
            critical: None,
            last_known_id: None,
        }
    }

    /// ## Usage
    /// Returns the finalized [`GetLogConfig`].
    /// If a field is unset, it'll set it to the default:
    /// 
    /// normal: true,
    /// 
    /// info: true,
    /// 
    /// warning: true,
    /// 
    /// critical: true,
    /// 
    /// last_known_id: -1 
    pub fn build(self) -> GetLogConfig {
        let a = match self.info {
            Some(value) => value,
            None => true,
        };

        let b = match self.normal {
            Some(value) => value,
            None => true,
        };

        let c = match self.warning {
            Some(value) => value,
            None => true,
        };

        let d = match self.critical {
            Some(val) => val,
            None => true,
        };

        let e = match self.last_known_id {
            Some(value) => value,
            None => -1,
        };

        return GetLogConfig {
            normal: a,
            info: b,
            warning: c,
            critical: d,
            last_known_id: e,
        };
    }
}

impl QbitApi {
    /// ## Usage
    /// Gets the log as a json [`Value`] based on the [`GetLogConfig`].
    pub async fn log_get_log(
        &mut self,
        config: impl Borrow<GetLogConfig>,
    ) -> Result<Value, crate::Error> {
        Ok(
            serde_json::from_str(Self::log_get_log_raw(self, config).await?.as_str())
                .map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?,
        )
    }

    /// ## Usage
    /// Gets the log as a [`String`] based on the [`GetLogConfig`].
    pub async fn log_get_log_raw(
        &mut self,
        config: impl Borrow<GetLogConfig>,
    ) -> Result<String, crate::Error> {
        let config: &GetLogConfig = config.borrow();

        let url = url!(
            "/log/main",
            ("info", Some(config.info)),
            ("normal", Some(config.normal)),
            ("warning", Some(config.warning)),
            ("critical", Some(config.critical)),
            ("last_known_id", Some(config.last_known_id))
        );
        Self::make_request(self, url, "get_log".to_string()).await
    }

    /// ## Usage
    /// Gets the peer log as a json [`Value`] based on the [`GetLogConfig`].
    pub async fn log_get_peer_log(&mut self, last_known_id: Option<i64>) -> Result<Value, Error> {
        Ok(serde_json::from_str(
            Self::log_get_peer_log_raw(self, last_known_id)
                .await?
                .as_str(),
        )
        .map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?)
    }

    /// ## Usage
    /// Gets the peer log as a [`String`] based on the [`GetLogConfig`].
    pub async fn log_get_peer_log_raw(
        &mut self,
        last_known_id: Option<i64>,
    ) -> Result<String, Error> {
        let x = match last_known_id {
            Some(val) => val,
            None => -1,
        };

        Self::make_request(
            self,
            format!("/log/main?last_known_id={}", x),
            "get_peer_log".to_string(),
        )
        .await
    }
}