use serde_json::{self, Value};
use crate::{core::api::QbitApi, error_handling::error_type::ErrorType, post_request, post_request_no_return, Error};

impl QbitApi {
    post_request!{
        /// ## Usage
        /// Gets the application version.
        /// eg: v4.1.3
        app_version, 
        "/app/version"
    }

    post_request!{
        /// ## Usage
        /// Gets teh WebAPI version.
        /// eg: 2.0
        app_web_api_version, 
        "/app/webapiVersion"
    }

    post_request!{
        /// ## Usage
        /// Gets the build info as a [`String`].
        app_build_info_raw, 
        "/app/buildInfo"
    }

    /// ## Usage
    /// Gets the build info as a json [`Value`].
    pub async fn app_build_info(&mut self) -> Result<Value, Error> {
        serde_json::from_str(Self::app_build_info_raw(self).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    post_request_no_return!{
        /// ## Usage
        /// Shuts down the application.
        app_shutdown,
        "/app/shutdown"
    }

    post_request!(
        /// ## Usage
        /// Gets the default save path.
        app_get_default_save_path, 
        "/app/defaultSavePath"
    );
}