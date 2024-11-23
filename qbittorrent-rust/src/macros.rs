#[macro_export]
macro_rules! post_request {
    ($func_name:ident, $path:expr) => {
        pub async fn $func_name(&mut self) -> Result <String, crate::Error> {
            Self::make_request(self, $path, format!("{}", stringify!($func_name))).await
        }
    };

    ($func_name:ident, $path:expr, $(($name_arg:tt, $type:ty)),+) => {
        pub async fn $func_name(&mut self, $($name_arg: $type),+) -> Result <String, crate::Error> {
            let mut form_data = std::collections::HashMap::new();

            $(
                form_data.insert(stringify!($name_arg), $name_arg.to_string());
            )+


            let response = self.reqwest_client.post(format!("{}/api/v2{}", self.authority, $path))
                .header(reqwest::header::COOKIE, format!("SID={}", self.get_cookie().await?))
                .form(&form_data)
                .send()
                .await
                .map_err(|e| crate::error_handling::flat_error::FlatError::ReqwestError(e.to_string()).unflatten_err())?;

            // Handle the response
            if response.status().is_success() {
                Ok(response.text().await.unwrap_or_default()) // Or return something meaningful
            } else {
                Err(crate::error_handling::flat_error::FlatError::ReqwestError(format!("something went wrong ({}). code: {}", stringify!($func_name), response.status().as_u16())).unflatten_err())
            }
        }
    };
}

#[macro_export]
macro_rules! post_request_no_return {
    ($func_name:ident, $path:expr) => {
        pub async fn $func_name(&mut self) -> Result <(), crate::Error> {
            Self::make_request(self, $path, format!("{}", stringify!($func_name))).await?;
            Ok(())
        }
    };

    ($func_name:ident, $path:expr, $(($name_arg:tt, $type:ty)),+) => {
        pub async fn $func_name(&mut self, $($name_arg: $type),+) -> Result <(), crate::Error> {
            use crate::error_handling::error_type::ErrorType;
            let mut form_data = std::collections::HashMap::new();

            $(
                form_data.insert(stringify!($name_arg), $name_arg.to_string());
            )+


            let response = self.reqwest_client.post(format!("{}/api/v2{}", self.authority, $path))
                .header(reqwest::header::COOKIE, format!("SID={}", self.get_cookie().await?))
                .form(&form_data)
                .send()
                .await
                .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;

            // Handle the response
            if response.status().is_success() {

                Ok(())
            } else {
                Err(Error::build(ErrorType::MiscNetError(response.status().as_u16()), Some(response.status().as_u16())))
            }
        }
    };
}

// #[macro_export]
// macro_rules! builder_fn_adapt {
//     ($fn_name:ident, $typee:ty) => {
//         pub fn $fn_name<T: Into<$typee>>(mut self, $fn_name: T) -> Self {
//             self.$fn_name = Some($fn_name.into());
//             self
//         }
//     };
// }

// #[macro_export]
// macro_rules! builder_fn {
//     ($fn_name:ident, $typee:ty) => {
//         pub fn $fn_name(mut self, $fn_name: $typee) -> Self {
//             self.$fn_name = Some($fn_name);
//             self
//         }
//     };
// }

#[macro_export]
macro_rules! url {
    ($base:expr, $(($name:expr, $value:expr)),+) => {{
        let mut string = String::from($base);

        string.push_str("?");

        $(
            {
                if let Some(inner) = $value {
                    string.push_str(format!("{}={}&", $name, inner.to_string()).as_str());
                }
            }
        )+

        string.pop();

        string
    }
    };
}

#[macro_export]
macro_rules! post_request_hash {
    ($(#[$meta:meta])* $func_name:ident, $path:expr) => {
        $(#[$meta])*
        pub async fn $func_name(&mut self, hash: impl Borrow<TorrentHash>) -> Result <String, crate::Error> {
            let mut hashmap = HashMap::new();
            hashmap.insert("hash", hash.get_hash());
            self.make_request_with_form_hash($path, stringify!($func_name), hashmap).await
        }
    };
}

#[macro_export]
macro_rules! fn_value_from_string {
    ($(#[$meta:meta])* $func_name:ident, $other_func:ident) => {
        $(#[$meta])*
        pub async fn $func_name(&mut self, hash: impl Borrow<TorrentHash>) -> Result <Value, crate::Error> {
            Ok(serde_json::from_str(self.$other_func(hash).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?)
        }
    };
}

#[macro_export]
macro_rules! fn_hash_value_pair {
    ($(#[$meta1:meta])* $func_name:ident,$(#[$meta2:meta])* $other_func:ident, $path:expr) => {
        post_request_hash!($(#[$meta1])* $func_name, $path);
        fn_value_from_string!($(#[$meta2])* $other_func, $func_name);
    };
}

#[macro_export]
macro_rules! torrents_fn_mult_hashes {
    ($func_name:ident, $url:expr) => {
        pub async fn $func_name(&mut self, hashes: impl Borrow<TorrentHashesDesc>) -> Result<(), Error> {
            let hashes_str = hashes.borrow().get_string("|");
            
            let url = url!($url, ("hashes", Some(hashes_str)));
    
            self.make_request(url, stringify!($func_name)).await?;
    
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! torrents_fn_mult_hashes_prios {
    ($func_name:ident, $url:expr) => {
        pub async fn $func_name(&mut self, hashes: impl Borrow<TorrentHashesDesc>) -> Result<(), Error> {
            let hashes_str = hashes.borrow().get_string("|");
            
            let url = url!($url, ("hashes", Some(hashes_str)));
    
            self.make_request(url, stringify!($func_name)).await.map_err(|e| {
                if let Some(num) = e.code {
                    if num == 409 {
                        Error::build(ErrorType::TorrenQueueingNotEnabled, Some(num))
                    } else {
                        e
                    }
                } else {
                    e
                }
            })?;
    
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! code {
    ($response:expr) => {
        Some($response.status().as_u16())
    };
}

#[macro_export]
macro_rules! request_error_focus {
    ($func_name:expr, $url:expr, $hashmap:expr, $(($status_code:expr, $error:expr)),+ ($else_err:expr)) => {
        self.make_request_with_form(url, $func_name, $hashmap).await.map_err(|e| {
            if let Some(num) = e.code {
                match num {
                    $(
                        $status_code => Error::build($error, num),
                    )+
                    _ => $else_err,
                }
            } else {
                e
            }
        })?;
    };

    ($self:expr, $func_name:ident, $url:expr, $hashmap:expr, $(($status_code:expr, $error:expr)),+) => {
        $self.make_request_with_form($url, stringify!($func_name), $hashmap).await.map_err(|e| {
            if let Some(num) = e.code {
                match num {
                    $(
                        $status_code => Error::build($error, Some(num)),
                    )+
                    _ => panic!("code not handled: {}", num),
                }
            } else {
                e
            }
        })
    };
}