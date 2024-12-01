use std::{borrow::Borrow, collections::HashMap};

use serde::Serialize;
use serde_json::Value;

use crate::{
    core::api::QbitApi, error_handling::error_type::ErrorType, hashmap, misc::sep_vec::SepVec,
    request_error_focus, Error,
};

#[derive(Debug, Clone)]
pub struct SearchPlugin {
    pub name: String,
    //                 v id v | v name v
    pub categories: Vec<(String, String)>,
}
impl SearchPlugin {
    /// creates a new [`Torrent`].
    /// the `Result` returned by this function can be `unwrap`ped without worry as long as the file path is readable.
    ///
    /// # WARNING
    /// - if the [`TorrentType`] is `Url`, you will need to use an url. if it is a `RawTorrent` you will need to use a file path.
    /// - attention!!! the contents of the file in case of `TorrentType::RawTorrent` will NOT be read by this function, but by the [`Api::add_torrent`] function! make sure the path is accessible.
    pub fn new<S: Into<String> + Clone, X: Into<String> + Clone>(
        search_plugin: S,
        categories: impl Borrow<Vec<(X, X)>>,
    ) -> Self {
        Self {
            name: search_plugin.into(),
            categories: Borrow::<Vec<(X, X)>>::borrow(&categories)
                .into_iter()
                .map(|(g, h)| {
                    (
                        Into::<String>::into(g.clone()),
                        Into::<String>::into(h.clone()),
                    )
                })
                .collect::<Vec<(String, String)>>(),
        }
    }

    pub fn from_vec<S: Into<String> + Clone, X: Into<String> + Clone>(
        search_plugins_vec: impl Borrow<Vec<S>>,
        categories: impl Borrow<Vec<Vec<(X, X)>>>,
    ) -> Vec<Self> {
        let x: &Vec<S> = search_plugins_vec.borrow();

        x.into_iter()
            .zip(Borrow::<Vec<Vec<(X, X)>>>::borrow(&categories).iter())
            .map(|k| Self::new::<S, X>(k.0.clone(), k.1.clone()))
            .collect()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_categories(&self) -> Vec<(String, String)> {
        self.categories.clone()
    }

    pub fn get_ids(&self) -> Vec<String> {
        self.get_categories().iter().map(|x| x.0.clone()).collect()
    }

    pub fn get_names(&self) -> Vec<String> {
        self.get_categories().iter().map(|x| x.1.clone()).collect()
    }
}

#[derive(Debug, Clone)]
pub enum SearchPluginsDescriptor {
    All,
    Enabled,
    Custom(Vec<SearchPlugin>),
}
impl SearchPluginsDescriptor {
    pub fn get_inner(&self) -> Vec<SearchPlugin> {
        match self {
            SearchPluginsDescriptor::All => {
                panic!("tried to get the inner value of SearchPulginsDescriptor::All")
            }
            SearchPluginsDescriptor::Enabled => {
                panic!("tried to get the inner value of SearchPulginsDescriptor::Enabled")
            }
            SearchPluginsDescriptor::Custom(vec) => vec.clone(),
        }
    }
}

pub enum SearchPluginsSpec {
    All,
    Enabled,
    Custom(Vec<String>),
}

pub enum Categories {
    All,
    Custom(Vec<String>),
}

impl QbitApi {
    pub async fn search_get_search_plugins_raw(&mut self) -> Result<String, crate::Error> {
        self.make_request("/search/plugins", "search_get_search_plugins_raw")
            .await
    }

    pub async fn search_get_search_plugins(&mut self) -> Result<Value, crate::Error> {
        serde_json::from_str(self.search_get_search_plugins_raw().await?.as_str())
            .map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    pub async fn search_get_search_plugins_descriptor(
        &mut self,
    ) -> Result<Vec<SearchPlugin>, Error> {
        let value = self.search_get_search_plugins().await?;
        let names = value
            .as_array()
            .iter()
            .map(|x| {
                x.into_iter()
                    .filter_map(|item| {
                        item.get("name")
                            .and_then(|s| s.as_str().map(|l| l.to_string()))
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
            .iter()
            .flatten()
            .map(|l| l.to_owned())
            .collect::<Vec<String>>();

        let categories = value
            .as_array()
            .unwrap() // Get the outer array
            .iter() // Iterate over each object in the array
            .map(|site| {
                site.get("supportedCategories")
                    .and_then(|categories| categories.as_array()) // Get the `supportedCategories` array
                    .unwrap_or(&vec![]) // Handle potential nulls gracefully
                    .iter() // Iterate over each category
                    .filter_map(|category| {
                        let name = category.get("name")?.as_str()?.to_string();
                        let id = category.get("id")?.as_str()?.to_string();
                        Some((name, id))
                    })
                    .collect::<Vec<(String, String)>>()
            })
            .collect::<Vec<Vec<(String, String)>>>();

        Ok(SearchPlugin::from_vec(names, categories))
    }

    pub async fn search_start(
        &mut self,
        pattern: impl Into<String>,
        plugins: impl Borrow<SearchPluginsSpec>,
        categories: impl Borrow<Categories>,
    ) -> Result<u64, Error> {
        let plugins = plugins.borrow().to_owned();
        let categories = categories.borrow().to_owned();
        let mut hashmap: HashMap<&str, String> = HashMap::new();
        hashmap.insert("pattern", pattern.into());
        match plugins {
            SearchPluginsSpec::All => hashmap.insert("plugins", "all".to_string()),
            SearchPluginsSpec::Enabled => hashmap.insert("plugins", "enabled".to_string()),
            SearchPluginsSpec::Custom(vec) => {
                hashmap.insert("plugins", SepVec::new(vec, "|").to_string())
            }
        };

        match categories {
            Categories::All => hashmap.insert("category", "all".to_string()),
            Categories::Custom(vec) => hashmap.insert("plugins", SepVec::new(vec, "|").to_string()),
        };

        let string = request_error_focus!(
            self,
            search_start,
            "/search/start",
            hashmap,
            (
                409,
                ErrorType::MiscError(
                    "user has reached the limit of max 'Running' searches (currently set to 5)"
                        .to_string()
                )
            )
        )?;
        let val: Value = serde_json::from_str(string.as_str())
            .map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?;
        Ok(val.get("id").unwrap().as_u64().unwrap())
    }

    pub async fn search_stop(&mut self, id: u64) -> Result<(), Error> {
        let hashmap = hashmap!(("id", id));
        request_error_focus!(
            self,
            search_stop,
            "/search/stop",
            hashmap,
            (
                404,
                ErrorType::MiscError("search job was not found".to_string())
            )
        )?;
        Ok(())
    }

    pub async fn search_status_raw(&mut self, id: Option<u64>) -> Result<String, Error> {
        match id {
            Some(n) => {
                let hashmap = hashmap!(("id", n));
                request_error_focus!(
                    self,
                    search_status_raw,
                    "/search/status",
                    hashmap,
                    (
                        404,
                        ErrorType::MiscError("search job was not found".to_string())
                    )
                )
            }
            None => self
                .make_request("/search/status", "search_status_raw")
                .await
                .map_err(|e| {
                    if let Some(num) = e.code {
                        match num {
                            404 => Error::build(
                                ErrorType::MiscError("search job was not found".to_string()),
                                Some(num),
                            ),

                            _ => e,
                        }
                    } else {
                        e
                    }
                }),
        }
    }

    pub async fn search_status(&mut self, id: Option<u64>) -> Result<Value, Error> {
        serde_json::from_str(self.search_status_raw(id).await?.as_str())
            .map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    pub async fn search_results_raw(
        &mut self,
        id: u64,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<String, Error> {
        let mut hashmap = HashMap::new();
        hashmap.insert("id", id as i64);
        if let Some(lim) = limit {
            hashmap.insert("limit", lim);
        }

        if let Some(ofst) = offset {
            hashmap.insert("offset", ofst);
        }

        let res = request_error_focus!(self, search_results_raw, "/search/results", hashmap, (404, ErrorType::MiscError("search job was not found".to_string())), (409, ErrorType::MiscError("Offset is too large, or too small (e.g. absolute value of negative number is greater than # results)".to_string())))?;

        Ok(res)
    }

    pub async fn search_results(
        &mut self,
        id: u64,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Value, Error> {
        serde_json::from_str(self.search_results_raw(id, limit, offset).await?.as_str())
            .map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    pub async fn search_delete(&mut self, id: u64) -> Result<(), Error> {
        let hashmap = hashmap!(("id", id));
        request_error_focus!(
            self,
            search_stop,
            "/search/delete",
            hashmap,
            (
                404,
                ErrorType::MiscError("search job was not found".to_string())
            )
        )?;
        Ok(())
    }

    pub async fn search_install_plugins<S: Into<String> + Clone>(
        &mut self,
        sources: impl Borrow<Vec<S>>,
    ) -> Result<(), Error> {
        let sources: Vec<S> = sources.borrow().clone();

        let srcs = sources
            .into_iter()
            .map(|k| Into::<String>::into(k.clone()))
            .collect::<Vec<String>>();

        let string = SepVec::new(srcs, "|").to_string();

        let hashmap = hashmap!(("sources", string));

        self.make_request_with_form("/search/installPlugin", "search_install_plugins", hashmap)
            .await?;
        Ok(())
    }

    pub async fn search_uninstall_plugins<S: Into<String> + Clone>(
        &mut self,
        names: impl Borrow<Vec<S>>,
    ) -> Result<(), Error> {
        let sources: Vec<S> = names.borrow().clone();

        let srcs = sources
            .into_iter()
            .map(|k| Into::<String>::into(k.clone()))
            .collect::<Vec<String>>();

        let string = SepVec::new(srcs, "|").to_string();

        let hashmap = hashmap!(("names", string));

        self.make_request_with_form(
            "/search/uninstallPlugin",
            "search_uninstall_plugins",
            hashmap,
        )
        .await?;
        Ok(())
    }

    pub async fn search_enable_plugins<S: Into<String> + Clone>(
        &mut self,
        names: impl Borrow<Vec<S>>,
        enable: bool,
    ) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Temp {
            names: String,
            enable: bool,
        }

        let sources: Vec<S> = names.borrow().clone();

        let srcs = sources
            .into_iter()
            .map(|k| Into::<String>::into(k.clone()))
            .collect::<Vec<String>>();

        let string = SepVec::new(srcs, "|").to_string();

        let temp = Temp {
            names: string,
            enable,
        };

        let response = self
            .reqwest_client
            .post(format!(
                "{}/api/v2{}",
                self.authority, "/search/enablePlugin"
            ))
            .header(
                reqwest::header::COOKIE,
                format!("SID={}", self.get_cookie().await?),
            )
            .form(&temp)
            .send()
            .await
            .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Error::build(
                ErrorType::MiscError(format!(
                    "function name: {}",
                    "search_enable_plugins"
                )),
                Some(response.status().as_u16()),
            ));
        }
    }

    pub async fn search_update_plugins(&mut self) -> Result<(), Error> {
        self.make_request("search/updatePlugins", "search_update_plugins").await?;
        Ok(())
    }
}

#[tokio::test]
async fn get_test() {
    let mut api = QbitApi::new(
        "http://localhost:6011/",
        crate::core::creds::Credentials {
            username: "admin".to_string(),
            password: "123456".to_string(),
        },
    )
    .await
    .unwrap();
    let desc = api.search_get_search_plugins_descriptor().await.unwrap();

    println!("{:?}", desc)
}
