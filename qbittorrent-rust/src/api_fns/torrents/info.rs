use std::borrow::Borrow;

use crate::{core::api::Api, error_handling::error_type::ErrorType, misc::sep_vec::SepVec, Error};
use proc_macros::Builder;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TorrentHash {
    pub name: String,
    pub hash: String,
}

impl TorrentHash {
    pub(crate) fn new<S:Into<String>, T:Into<String>>(name: S, hash: T) -> Self {
        TorrentHash { name: name.into(), hash: hash.into() }
    }

    pub(crate) fn new_multiple<S, T, U, V>(names: U, hashes: V) -> Vec<Self> 
    where
        T: Into<String>,
        U: IntoIterator<Item = T>, 
        S: Into<String>,
        V: IntoIterator<Item = S>
    {
        let binding = names.into_iter().map(|x| Into::<String>::into(x)).collect::<Vec<String>>();
        let iter_names = binding.iter();
        let binding = hashes.into_iter().map(|x| Into::<String>::into(x)).collect::<Vec<String>>();
        let iter_hashes = binding.iter();

        let torrent_hashes = iter_names.zip(iter_hashes).map(|(l, d)| Self::new(l, d)).collect::<Vec<Self>>();
        torrent_hashes
    }
}

#[derive(Debug)]
pub enum Category {
    NoCategory,
    AnyCategory,
    Custom(String),
}
impl Category {
    pub fn get_str_category(&self) -> Option<String> {
        match self {
            Category::NoCategory => Some(String::new()),
            Category::AnyCategory => None,
            Category::Custom(s) => Some(s.to_owned()),
        }
    }

    pub fn get_category_from_str<S:Into<String>>(string: S) -> Category {
        let string: String = string.into();
        if string.is_empty() {
            return Category::NoCategory;
        } else {
            return Category::Custom(string);
        };
    }
}

#[derive(Debug)]
pub enum State {
    All,
    Downloading,
    Seeding,
    Completed,
    Paused,
    Active,
    Inactive,
    Resumed,
    Stalled,
    StalledUploading,
    StalledDownloading,
    Errored,
}
impl State {
    pub fn get_str_state(&self) -> String {
        match self {
            State::All => String::from("all"),
            State::Downloading => String::from("downloading"),
            State::Seeding => String::from("seeding"),
            State::Completed => String::from("completed"),
            State::Paused => String::from("paused"),
            State::Active => String::from("active"),
            State::Inactive => String::from("inactive"),
            State::Resumed => String::from("resumed"),
            State::Stalled => String::from("stalled"),
            State::StalledUploading => String::from("stalled_uploading"),
            State::StalledDownloading => String::from("stalled_downloading"),
            State::Errored => String::from("errored"),
        }
    }

    pub fn get_state_from_str<S:Into<String>>(string: S) -> Result<State, Error> {
        let string = string.into();

        if string == "all".to_string() {
            return Ok(State::All);
        } else if string == "downloading".to_string() {
            return Ok(Self::Downloading);
        } else if string == "seeding".to_string() {
            Ok(State::Seeding)
        } else if string == "completed".to_string() {
            Ok(State::Completed)
        } else if string == "paused".to_string() {
            Ok(State::Paused)
        } else if string == "active".to_string() {
            Ok(State::Active)
        } else if string == "inactive".to_string() {
            Ok(State::Inactive)
        } else if string == "resumed".to_string() {
            Ok(State::Resumed)
        } else if string == "stalled".to_string() {
            Ok(State::Stalled)
        } else if string == "stalled_uploading".to_string() {
            Ok(State::StalledUploading)
        } else if string == "stalled_downloading".to_string() {
            Ok(State::StalledDownloading)
        } else if string == "errored".to_string() {
            Ok(State::Errored)
        } else {
            Err(Error::build(ErrorType::ParameterNotExpected, None))            
        }
    }
}

#[derive(Debug, Clone)]
pub struct TorrentListGetConfig {
    filter: Option<String>,
    category: Option<String>,
    tag: Option<String>,
    sort: Option<String>,
    reverse: Option<bool>,
    limit: Option<u64>,
    offset: Option<i64>,
    hashes: Option<SepVec<String, String>>,
}
impl TorrentListGetConfig {
    pub fn new() -> Self {
        TorrentListGetConfigBuilder::new().build()
    }

    pub fn builder() -> TorrentListGetConfigBuilder {
        TorrentListGetConfigBuilder::new()
    }
}

#[derive(Debug, Builder)]
pub struct TorrentListGetConfigBuilder {
    pub filter: Option<State>,
    pub category: Option<Category>,
    pub tag: Option<String>,
    pub sort: Option<String>,
    pub reverse: Option<bool>,
    pub limit: Option<u64>,
    pub offset: Option<i64>,
    pub hashes: Option<Vec<String>>,
}
impl TorrentListGetConfigBuilder {
    pub fn new() -> Self {
        TorrentListGetConfigBuilder {
            filter: None,
            category: None,
            tag: None,
            sort: None,
            reverse: None,
            limit: None,
            offset: None,
            hashes: None,
        }
    }

    pub fn build(self) -> TorrentListGetConfig {
        let filter = self.filter.and_then(|x| Some(x.get_str_state()));
        let category = match self.category {
            Some(x) => x.get_str_category(),
            None => None,
        };

        TorrentListGetConfig {
            filter,
            category,
            tag: self.tag,
            sort: self.sort,
            reverse: self.reverse,
            limit: self.limit,
            offset: self.offset,
            hashes: self.hashes.and_then(|s| Some(SepVec::new(s, String::from("|")))),
        }
    }
}

impl Api {
    pub async fn get_hashes(&mut self) -> Result<Vec<TorrentHash>, Error> {
        let jsons = self.get_torrent_list(TorrentListGetConfig::new()).await?;

        let names: Vec<String> = jsons.as_array().unwrap().iter().map(|k| k.clone()["name"].take().as_str().unwrap().to_string()).collect();
        let hashes: Vec<String> = jsons.as_array().unwrap().iter().map(|k| k.clone()["hash"].take().as_str().unwrap().to_string()).collect();
        Ok(TorrentHash::new_multiple(names, hashes))
    }

    pub async fn get_torrent_list(&mut self, config: impl Borrow<TorrentListGetConfig>) -> Result<Value, Error> {
        serde_json::from_str(Self::get_torrent_list_raw(self, config).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    pub async fn get_torrent_list_raw(&mut self, config: impl Borrow<TorrentListGetConfig>) -> Result<String, Error> {
        let config: TorrentListGetConfig = config.borrow().clone();

        let vec = config.hashes.and_then(|x| Some(x.to_string()));
        
        let url = crate::url!(
            "/torrents/info",
            ("filter", config.filter),
            ("category", config.category),
            ("tag", config.tag),
            ("sort", config.sort),
            ("reverse", config.reverse),
            ("limit", config.limit),
            ("offset", config.offset),
            ("hashes", vec)
        );

        Self::make_request(self, url, "get_torrent_list").await
    }
}

#[tokio::test]
pub async fn test() {
    let mut api = Api::new("http://localhost:6011/", crate::core::creds::Credentials::new("admin", "123456")).await.unwrap();
    let something = api.get_hashes().await.unwrap();
    println!("{:?}", something)
}
