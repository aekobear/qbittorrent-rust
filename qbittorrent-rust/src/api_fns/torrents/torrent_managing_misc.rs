use std::{borrow::Borrow, collections::HashMap};

use serde_json::Value;

use crate::{core::api::Api, error_handling::error_type::ErrorType, fn_hash_value_pair, fn_value_from_string, misc::sep_vec::SepVec, post_request_hash, request_error_focus, torrents_fn_mult_hashes, torrents_fn_mult_hashes_prios, url, Error};

use super::info::TorrentHash;

#[derive(Debug, Clone)]
pub enum TorrentHashesDesc {
    All,
    Hashes(Vec<TorrentHash>)
} impl TorrentHashesDesc {
    pub(crate) fn get_string<S:Into<String>>(&self, separator: S) -> String {
        match self {
            Self::All => String::from("all"),
            Self::Hashes(x) => {
                SepVec::new(x.clone().into_iter().map(|k| k.hash).collect::<Vec<String>>(), separator.into()).to_string()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TorrentContent {
    pub id: u64,
    pub name: String,
}

impl Api {
    pub async fn get_torrent_generic_properties_raw(&mut self, hash: impl Borrow<TorrentHash>) -> Result<String, Error> {
        Ok(self.get_torrent_generic_properties(hash).await?.to_string())
    }

    pub async fn get_torrent_generic_properties(&mut self, hash: impl Borrow<TorrentHash>) -> Result<Value, Error> {
        let mid: &TorrentHash = hash.borrow();
        
        let mut hashmap = HashMap::new();
        
        hashmap.insert("hash", mid.hash.as_str());
        
        serde_json::from_str(self.make_request_with_form_hash("/torrents/properties", "get_torrent_generic_properties", hashmap).await?.as_str()).map_err(|k: serde_json::Error| Error::build(ErrorType::JsonSerdeError(Box::new(k)), None))
    }

    fn_hash_value_pair!(
        /// gets the raw string that describes the torrent trackers.
        get_torrent_trackers_raw,
        /// returns a [`Value`], aka the json that describes the torrent trackers. 
        get_torrent_trackers, 
        "/torrents/trackers"
    );

    fn_hash_value_pair!(get_torrent_web_seeds_raw, get_torrent_web_seeds, "/torrents/webseeds");

    
    pub async fn get_torrent_contents_raw(&mut self, hash: impl Borrow<TorrentHash>, indexes: impl Borrow<Option<Vec<String>>>) -> Result<String, Error> {


        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hash.get_hash());


        if let Some(vec) = indexes.borrow() {
            let sep_vec = SepVec::new(vec, '|').to_string();
            hashmap.insert("indexes", sep_vec);
            self.make_request_with_form_hash("/torrents/files", "get_torrents_contents", hashmap).await
        } else {
            self.make_request_with_form_hash("/torrents/files", "get_torrents_contents", hashmap).await
        }
    }

    pub async fn get_torrent_contents(&mut self, hash: impl Borrow<TorrentHash>, indexes: impl Borrow<Option<Vec<String>>>) -> Result<Value, Error> {
        Ok(serde_json::from_str(self.get_torrent_contents_raw(hash, indexes).await?.as_str()).map_err(|l| Error::build(ErrorType::JsonSerdeError(Box::new(l)), None))?)
    }

    pub async fn torrents_get_files_ids(&mut self, hash: impl Borrow<TorrentHash>) -> Vec<TorrentContent> {
        let x = self.get_torrent_contents(hash, &None).await.unwrap();
        
        let mut res = vec![];

        let len = x.as_array().iter().len();

        for i in x.as_array().iter().zip(0..len) {
            let name = i.0.into_iter().filter_map(|item| item.get("name").and_then(|name| name.as_str().map(|s| s.to_string()))).collect();
            let indx = i.1;
            
            res.push(TorrentContent {
                id: indx as u64,
                name,
            });
        }

        res
    }

    fn_hash_value_pair!(get_torrent_pieces_states_raw, get_torrent_pieces_states, "/torrents/pieceStates");

    fn_hash_value_pair!(get_torrent_pieces_hashes_raw, get_torrent_pieces_hashes, "/torrents/pieceHashes");

    torrents_fn_mult_hashes!(torrents_pause_torrents, "/torrents/pause");

    torrents_fn_mult_hashes!(torrents_resume_torrents, "/torrents/resume");

    pub async fn torrents_delete_torrents(&mut self, hashes: &TorrentHashesDesc, delete_files: &bool) -> Result<(), Error> {
        let hashes_str = hashes.get_string("|");
            
            let url = url!("/torrents/delete", ("hashes", Some(hashes_str)), ("deleteFiles", Some(delete_files)));
    
            self.make_request(url, stringify!($func_name)).await.map_err(|e| {
                Error::build(ErrorType::ReqwestError(Box::new(e)), None)
            })?;
    
            Ok(())
    }

    torrents_fn_mult_hashes!(torrents_recheck_torrents, "/torrents/recheck");

    torrents_fn_mult_hashes!(torrents_reannounce_torrents, "/torrents/reannounce");

    torrents_fn_mult_hashes_prios!(torrents_increase_priority_torrents, "/torrents/increasePrio");

    torrents_fn_mult_hashes_prios!(torrents_decrease_priority_torrents, "/torrents/decreasePrio");

    torrents_fn_mult_hashes_prios!(torrents_set_top_priority_torrents, "/torrents/topPrio");

    torrents_fn_mult_hashes_prios!(torrents_set_bottom_priority_torrents, "/torrents/bottomPrio");


    pub async fn torrents_add_trackers_to_torrent<S:Into<String> + Clone>(&mut self, hash: impl Borrow<TorrentHash>, trackers: impl Borrow<Vec<S>>) -> Result<(), Error> {
        let vec = trackers.borrow().into_iter().map(|s| Into::<String>::into(s.clone()).replace("&", "%26")).collect::<Vec<String>>();

        let sep_vec = SepVec::new(vec, "%0A").to_string();

        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hash.get_hash());
        hashmap.insert("urls", sep_vec);

        self.make_request_with_form_hash("/torrents/addTrackers", "torrents_add_trackers_to_torrent", hashmap).await?;

        Ok(())
    }

    pub async fn torrents_edit_trackers(&mut self, hash: impl Borrow<TorrentHash>, orig_url: impl AsRef<String>, new_url: impl AsRef<String>) -> Result<(), Error>{
        let orig_url: String = orig_url.as_ref().clone();
        let new_url: String = new_url.as_ref().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hash.get_hash());
        hashmap.insert("origUrl", orig_url);
        hashmap.insert("newUrl", new_url);
        
        request_error_focus!(self, torrents_edit_trackers, "/torrents/editTracker", hashmap, (400, ErrorType::MiscError("new_url is not a valid URL".to_string())), (404, ErrorType::TorrentHashNotFound), (409, ErrorType::MiscError("new_url already exists for the torrent or orig_url couldn't be found.".to_string())))?;
        Ok(())
    }

    pub async fn torrents_remove_trackers<S:Into<String> + Clone>(&mut self, hash: impl Borrow<TorrentHash>, urls: impl Borrow<Vec<S>>) -> Result<(), Error> {
        let urls = urls.borrow().clone().into_iter().map(|x|-> String {Into::<String>::into(x.clone())}).collect::<Vec<String>>();
        let urls = SepVec::new(urls, "|");
        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hash.get_hash());
        hashmap.insert("urls", urls.to_string());

        request_error_focus!(self, torrents_remove_trackers, "/torrents/removeTrackers", hashmap, (404, ErrorType::TorrentHashNotFound), (409, ErrorType::MiscError("all urls were not found.".to_string())))?;
        Ok(())
    }

    pub async fn torrents_add_peers<S: Into<String> + Clone>(&mut self, hashes: impl Borrow<Vec<TorrentHash>>, peers: impl Borrow<Vec<S>>) -> Result<(), Error> {
        let peers = peers.borrow().clone().into_iter().map(|x|-> String {Into::<String>::into(x.clone())}).collect::<Vec<String>>();
        let peers = SepVec::new(peers, "|");
        
        let hashes = SepVec::new(hashes.borrow().iter().map(|h| h.hash.clone()), "|");
        
        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hashes.to_string());
        hashmap.insert("urls", peers.to_string());

        request_error_focus!(self, torrents_remove_trackers, "/torrents/addPeers", hashmap, (400, ErrorType::MiscError("none of the supplied peers are valid".to_string())))?;
        Ok(())
    }

    pub async fn torrents_set_file_priority(&mut self, hash: impl Borrow<TorrentHash>, ids: impl Borrow<Vec<TorrentContent>>) -> Result<(), Error> {
        let hash = hash.get_hash();
        let ids = SepVec::new(ids.borrow().clone().iter().map(|x| x.id.to_string()), "|");

        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hash);
        hashmap.insert("id", ids.to_string());

        request_error_focus!(self, torrents_set_file_priority, "/torrents/filePrio", hashmap, (400, ErrorType::MiscError("the priority is invalid or at least one file id is not a valid integer".to_string())), (404, ErrorType::TorrentHashNotFound), (409, ErrorType::MiscError("the torrent metadata hasn't downloaded yet or at least one file id was not found".to_string())))?;
        Ok(())
    }

    pub async fn torrents_get_torrent_download_limit_raw(&mut self, hashes: impl Borrow<TorrentHashesDesc>) -> Result<String, Error>  {
        let hashes: TorrentHashesDesc = hashes.borrow().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hashes", hashes.get_string("|"));

        self.make_request_with_form("/torrents/downloadLimit", "torrents_get_torrent_download_limit", hashmap).await
    }

    pub async fn torrents_get_torrent_download_limit(&mut self, hash: impl Borrow<TorrentHashesDesc>) -> Result <Value, crate::Error> {
        Ok(serde_json::from_str(self.torrents_get_torrent_download_limit_raw(hash).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?)
    }

    pub async fn torrents_set_torrent_download_limit(&mut self, hashes: impl Borrow<TorrentHashesDesc>, limit: u64) -> Result<(), Error>  {
        let hashes: TorrentHashesDesc = hashes.borrow().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hashes", hashes.get_string("|"));
        hashmap.insert("limit", limit.to_string());

        self.make_request_with_form("/torrents/setDownloadLimit", "torrents_get_torrent_download_limit", hashmap).await?;
        Ok(())
    }

    pub async fn torrents_set_torrent_share_limit(&mut self, hashes: impl Borrow<TorrentHashesDesc>, ratio_limit: i32, seeding_time_limit: i32, inactive_seeding_time_limit: i32) -> Result<(), Error> {
        let hashes: TorrentHashesDesc = hashes.borrow().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hashes", hashes.get_string("|"));
        hashmap.insert("ratioLimit", ratio_limit.to_string());
        hashmap.insert("seedingTimeLimit", seeding_time_limit.to_string());
        hashmap.insert("inactiveSeedingTimeLimit", inactive_seeding_time_limit.to_string());

        self.make_request_with_form("/torrents/setShareLimits", "torrents_get_torrent_download_limit", hashmap).await?;
        
        Ok(())
    }

    pub async fn torrents_get_torrent_upload_limit_raw(&mut self, hashes: impl Borrow<TorrentHashesDesc>) -> Result<String, Error>  {
        let hashes: TorrentHashesDesc = hashes.borrow().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hashes", hashes.get_string("|"));

        self.make_request_with_form("/torrents/uploadLimit", "torrents_get_torrent_download_limit", hashmap).await
    }

    pub async fn torrents_get_torrent_upload_limit(&mut self, hash: impl Borrow<TorrentHashesDesc>) -> Result <Value, crate::Error> {
        Ok(serde_json::from_str(self.torrents_get_torrent_download_limit_raw(hash).await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?)
    }

    pub async fn torrents_set_torrent_upload_limit(&mut self, hashes: impl Borrow<TorrentHashesDesc>, limit: u64) -> Result<(), Error>  {
        let hashes: TorrentHashesDesc = hashes.borrow().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hashes", hashes.get_string("|"));
        hashmap.insert("limit", limit.to_string());

        self.make_request_with_form("/torrents/setUploadLimit", "torrents_get_torrent_download_limit", hashmap).await?;
        Ok(())
    }

    pub async fn torrents_set_torrent_download_location(&mut self, hashes: impl Borrow<TorrentHashesDesc>, location: impl AsRef<String>) -> Result<(), Error>  {
        let hashes: TorrentHashesDesc = hashes.borrow().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hashes", hashes.get_string("|"));
        hashmap.insert("location", location.as_ref().to_string());

        request_error_focus!(self, torrents_set_torrent_download_location, "/torrents/setLocation", hashmap, (400, ErrorType::MiscError("save path is empty".to_string())), (404, ErrorType::TorrentHashNotFound), (403, ErrorType::MiscError("user does not have write access to directory".to_string())),(409, ErrorType::MiscError("unable to create save path directory".to_string())))?;
        Ok(())
    }

    pub async fn torrents_set_torrent_name(&mut self, hash: impl Borrow<TorrentHash>, name: impl Into<String>) -> Result<(), Error>  {
        let hash: TorrentHash = hash.borrow().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hash.hash.to_string());
        hashmap.insert("name", name.into().replace(" ", "%20").replace("&", "%26").to_string());

        request_error_focus!(self, torrents_set_torrent_name, "/torrents/rename", hashmap, (404, ErrorType::TorrentHashNotFound), (409, ErrorType::MiscError("torrent name is empty".to_string())))?;
        Ok(())
    }

    pub async fn torrents_set_torrent_category(&mut self, hash: impl Borrow<TorrentHashesDesc>, category_name: impl Into<String>) -> Result<(), Error>  {
        let hash: TorrentHashesDesc = hash.borrow().clone();

        let mut hashmap = HashMap::new();

        hashmap.insert("hashes", hash.get_string("|"));
        hashmap.insert("category", category_name.into().replace(" ", "").replace("&", "%26").to_string());

        request_error_focus!(self, torrents_set_torrent_category, "/torrents/setCategory", hashmap, (409, ErrorType::MiscError("category name does not exist".to_string())))?;
        Ok(())
    }

    pub async fn torrents_get_all_categories_raw(&mut self) -> Result<String, Error>  {
        self.make_request("/torrents/categories", "torrents_get_all_categories_raw").await
    }

    pub async fn torrents_get_all_categories(&mut self) -> Result <Value, crate::Error> {
        Ok(serde_json::from_str(self.torrents_get_all_categories_raw().await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?)
    }

    pub async fn torrents_add_new_category(&mut self, category_name: impl Into<String>, save_path: Option<impl Into<String>>) -> Result<(), Error>  {

        let mut hashmap: HashMap<&str, String> = HashMap::new();
        let cat = category_name.into().replace(" ", "").replace("&", "%26").to_string();
        match save_path {
            Some(path) => {
                hashmap.insert("category", cat);
                hashmap.insert("savePath", path.into());
                request_error_focus!(self, torrents_add_new_category, "/torrents/createCategory", hashmap, (400, ErrorType::MiscError("category name is empty".to_string())), (409, ErrorType::MiscError("category name is invalid".to_string())))?;
                return Ok(())
            },
            
            None => {
                hashmap.insert("category", cat);
                request_error_focus!(self, torrents_add_new_category, "/torrents/createCategory", hashmap, (400, ErrorType::MiscError("category name is empty".to_string())), (409, ErrorType::MiscError("category name is invalid".to_string())))?;
                return Ok(())
            }
        }        
    }

    pub async fn torrents_edit_category(&mut self, category_name: impl Into<String>, save_path: Option<impl Into<String>>) -> Result<(), Error>  {
        let mut hashmap: HashMap<&str, String> = HashMap::new();
        let cat = category_name.into().replace(" ", "").replace("&", "%26").to_string();
        match save_path {
            Some(path) => {
                hashmap.insert("category", cat);
                hashmap.insert("savePath", path.into());
                request_error_focus!(self, torrents_add_new_category, "/torrents/editCategory", hashmap, (400, ErrorType::MiscError("category name is empty".to_string())), (409, ErrorType::MiscError("category editing failed".to_string())))?;
                return Ok(())
            },
            
            None => {
                hashmap.insert("category", cat);
                request_error_focus!(self, torrents_add_new_category, "/torrents/editCategory", hashmap, (400, ErrorType::MiscError("category name is empty".to_string())), (409, ErrorType::MiscError("category editing failed".to_string())))?;
                return Ok(())
            }
        }        
    }

    pub async fn torrents_remove_categories<S:Into<String>+ Clone>(&mut self, categories_name: impl Borrow<Vec<S>>) -> Result<(), Error>{
        let mut hashmap = HashMap::new();
        let sep_vec: SepVec<String, &str> = SepVec::new(categories_name.borrow().into_iter().map(|s| Into::<String>::into(s.clone())), "%0A");
        hashmap.insert("categories", sep_vec.to_string());

        self.make_request_with_form("/torrents/removeCategories", "torrents_remove_categories", hashmap).await?;
        Ok(())
    }

    pub async fn torrents_get_all_tags_raw(&mut self) -> Result<String, Error>  {
        self.make_request("/torrents/tags", "torrents_get_all_tags_raw").await
    }

    pub async fn torrents_get_all_tags(&mut self) -> Result <Value, crate::Error> {
        Ok(serde_json::from_str(self.torrents_get_all_tags_raw().await?.as_str()).map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))?)
    }

    pub async fn torrents_add_new_tags<S:Into<String> + Clone>(&mut self, tags_name: impl Borrow<Vec<S>>) -> Result<(), Error>  {

        let mut hashmap: HashMap<&str, String> = HashMap::new();
        let sep_vec: SepVec<String, &str> = SepVec::new(tags_name.borrow().into_iter().map(|k| Into::<String>::into(k.clone())), ",");
        hashmap.insert("tags", sep_vec.to_string());

        self.make_request_with_form("/torrents/createTags", "torrents_add_new_tags", hashmap).await?;

        Ok(())
    }

    pub async fn torrents_remove_tags<S:Into<String> + Clone>(&mut self, tags_name: impl Borrow<Vec<S>>) -> Result<(), Error>  {

        let mut hashmap: HashMap<&str, String> = HashMap::new();
        let sep_vec: SepVec<String, &str> = SepVec::new(tags_name.borrow().into_iter().map(|k| Into::<String>::into(k.clone())), ",");
        hashmap.insert("tags", sep_vec.to_string());

        self.make_request_with_form("/torrents/deleteTags", "torrents_remove_tags", hashmap).await?;

        Ok(())
    }

    pub async fn torrents_set_automatic_torrents_management(&mut self, hashes: impl Borrow<TorrentHashesDesc>, enabled: bool) -> Result<(), Error> {
        let mut hashmap = HashMap::new();
        hashmap.insert("hashes", hashes.borrow().get_string("|"));
        hashmap.insert("enable", enabled.to_string());

        self.make_request_with_form("/torrents/setAutoManagement", "torrents_set_automatic_torrents_manaement", hashmap).await?;
        Ok(())
    }

    pub async fn torrents_toggle_sequential_download(&mut self, hashes: impl Borrow<Vec<TorrentHash>>) -> Result<(), Error> {
        let url = url!("/torrents/toggleSequentialDownload", ("hashes", Some(hashes.borrow().into_iter().map(|l| l.hash.clone()).collect::<Vec<String>>().join("|"))));

        self.make_request(url, "torrents_toggle_sequential_download").await?;
        Ok(())
    }

    pub async fn set_first_last_piece_priority(&mut self, hashes: impl Borrow<Vec<TorrentHash>>) -> Result<(), Error> {
        let url = url!("/torrents/toggleFirstLastPiecePrio", ("hashes", Some(hashes.borrow().into_iter().map(|l| l.hash.clone()).collect::<Vec<String>>().join("|"))));

        self.make_request(url, "set_first_last_piece_priority").await?;
        Ok(())
    }

    pub async fn torrents_set_force_start(&mut self, hashes: impl Borrow<TorrentHashesDesc>, enabled: bool) -> Result<(), Error> {
        let mut hashmap = HashMap::new();
        hashmap.insert("hashes", hashes.borrow().get_string("|"));
        hashmap.insert("value", enabled.to_string());

        self.make_request_with_form("/torrents/setForceStart", "torrents_set_force_start", hashmap).await?;
        Ok(())
    }

    pub async fn torrents_set_super_seeding(&mut self, hashes: impl Borrow<TorrentHashesDesc>, enabled: bool) -> Result<(), Error> {
        let mut hashmap = HashMap::new();
        hashmap.insert("hashes", hashes.borrow().get_string("|"));
        hashmap.insert("value", enabled.to_string());

        self.make_request_with_form("/torrents/setSuperSeeding", "torrents_set_super_seeding", hashmap).await?;
        Ok(())
    }

    pub async fn torrents_rename_file(&mut self, hash: impl Borrow<TorrentHash>, old_path: impl Into<String>, new_path: impl Into<String>) -> Result<(), Error> {
        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hash.get_hash());
        hashmap.insert("oldPath", old_path.into());
        hashmap.insert("newPath", new_path.into());
        
        request_error_focus!(self, torrents_rename_file, "/torrents/renameFile", hashmap, (400, ErrorType::MiscError("missing new_path parameter".to_string())), (409, ErrorType::MiscError("invalid new_path or old_path, or newPath already in use".to_string())))?;
        Ok(())
    }

    pub async fn torrents_rename_folder(&mut self, hash: impl Borrow<TorrentHash>, old_path: impl Into<String>, new_path: impl Into<String>) -> Result<(), Error> {
        let mut hashmap = HashMap::new();

        hashmap.insert("hash", hash.get_hash());
        hashmap.insert("oldPath", old_path.into());
        hashmap.insert("newPath", new_path.into());
        
        request_error_focus!(self, torrents_rename_folder, "/torrents/renameFolder", hashmap, (400, ErrorType::MiscError("missing new_path parameter".to_string())), (409, ErrorType::MiscError("invalid new_path or old_path, or newPath already in use".to_string())))?;
        Ok(())
    }
}

trait GetHash {
    fn get_hash(&self) -> String;
}

impl<X: Borrow<TorrentHash>> GetHash for X {
    fn get_hash(&self) -> String {
        let x: &TorrentHash = self.borrow();
        x.hash.clone()
    }
}


#[tokio::test]
//#[should_panic]
async fn test() {
    let mut api = Api::new("http://localhost:6011/", crate::core::creds::Credentials::new("admin", "123456")).await.unwrap();

    // let hash = api.get_hashes().await.unwrap()[0].clone();
    // println!("{:?}", hash);
    // api.torrents_set_torrent_name(hash, "archlinux").await.unwrap();

    let x = api.torrents_remove_categories(vec!["something"]).await;
    println!("{:?}", x)
}