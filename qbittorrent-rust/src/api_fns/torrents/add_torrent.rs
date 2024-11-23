use core::panic;
use std::borrow::Borrow;

use proc_macros::Builder;
use reqwest::header;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    core::api::Api, code, error_handling::{errors::Error, error_type::ErrorType}, misc::sep_vec::SepVec
};

use super::torrents::Torrent;

#[derive(Debug, Clone)]
pub struct TorrentAddDescriptor {
    pub urls: SepVec<String, String>,

    pub paths: Vec<String>,

    /// Download folder path
    pub savepath: Option<String>,

    /// Cookie sent to download the .torrent file
    pub cookie: Option<String>,

    /// Category for the torrent
    pub category: Option<String>,

    /// Tags for the torrent, separated by commas
    pub tags: Option<SepVec<String, char>>,

    /// Skip hash checking (true, false)
    pub skip_checking: Option<bool>,

    /// Add torrents in a paused state (true, false)
    pub paused: Option<bool>,

    /// Create the root folder (true, false, or unset)
    pub root_folder: Option<String>,

    /// Rename the torrent
    pub rename: Option<String>,

    /// Set torrent upload speed limit in bytes per second
    pub up_limit: Option<u64>,

    /// Set torrent download speed limit in bytes per second
    pub dl_limit: Option<u64>,

    /// Set torrent share ratio limit (since qBittorrent v2.8.1)
    pub ratio_limit: Option<f32>,

    /// Set torrent seeding time limit in minutes (since qBittorrent v2.8.1)
    pub seeding_time_limit: Option<u32>,

    /// Use Automatic Torrent Management
    pub auto_tmm: Option<bool>,

    /// Enable sequential download (true, false)
    pub sequential_download: Option<bool>,

    /// Prioritize first and last piece download (true, false)
    pub first_last_piece_prio: Option<bool>,
}
impl TorrentAddDescriptor {
    pub fn new(torrents: Vec<Torrent>) -> Self {
        Self::builder().torrents(torrents).build().unwrap()
    }

    pub fn builder() -> TorrentAddDescriptorBuilder {
        TorrentAddDescriptorBuilder::new()
    }
}

#[derive(Debug, Clone, Builder)]
pub struct TorrentAddDescriptorBuilder {
    pub torrents: Option<Vec<Torrent>>,

    /// Download folder path
    pub savepath: Option<String>,

    /// Cookie sent to download the .torrent file
    pub cookie: Option<String>,

    /// Category for the torrent
    pub category: Option<String>,

    /// Tags for the torrent, separated by commas
    pub tags: Option<Vec<String>>,

    /// Skip hash checking (true, false)
    pub skip_checking: Option<bool>,

    /// Add torrents in a paused state (true, false)
    pub paused: Option<bool>,

    /// Create the root folder (true, false, or unset)
    pub root_folder: Option<String>,

    /// Rename the torrent
    pub rename: Option<String>,

    /// Set torrent upload speed limit in bytes per second
    pub up_limit: Option<u64>,

    /// Set torrent download speed limit in bytes per second
    pub dl_limit: Option<u64>,

    /// Set torrent share ratio limit (since qBittorrent v2.8.1)
    pub ratio_limit: Option<f32>,

    /// Set torrent seeding time limit in minutes (since qBittorrent v2.8.1)
    pub seeding_time_limit: Option<u32>,

    /// Use Automatic Torrent Management
    pub auto_tmm: Option<bool>,

    /// Enable sequential download (true, false)
    pub sequential_download: Option<bool>,

    /// Prioritize first and last piece download (true, false)
    pub first_last_piece_prio: Option<bool>,
}
impl TorrentAddDescriptorBuilder {
    pub fn new() -> Self {
        Self {
            torrents: None,
            savepath: None,
            cookie: None,
            category: None,
            tags: None,
            skip_checking: None,
            paused: None,
            root_folder: None,
            rename: None,
            up_limit: None,
            dl_limit: None,
            ratio_limit: None,
            seeding_time_limit: None,
            auto_tmm: None,
            sequential_download: None,
            first_last_piece_prio: None,
        }
    }

    /// # Info
    /// returns the finalized [`TorrentAddDescriptor`].
    ///
    /// # Errors
    /// - if no torrents were set, it will return [`Error::ApiError(ApiErrors::TorrentError(TorrentErrors::TorrentsNotSet))`]. there MUST be soemthing to send. An empty vector is NOT okay.
    pub fn build(self) -> Result<TorrentAddDescriptor, Error> {
        let (urls, paths) = match self.torrents {
            Some(t) => {
                if t.is_empty() {
                    return Err(Error::build(ErrorType::TorrentsNotSet, None));
                } else {
                    let mut vec_urls = vec![];
                    let mut vec_paths = vec![];

                    for item in t.iter().map(|l| l.get_inner()) {
                        match item {
                            crate::api_fns::torrents::torrents::TorrentInner::Url(url) => {
                                vec_urls.push(url)
                            }
                            crate::api_fns::torrents::torrents::TorrentInner::RawTorrent(path) => {
                                vec_paths.push(path)
                            }
                        }
                    }

                    (SepVec::new(vec_urls, "".to_string()), vec_paths)
                }
            }
            None => {
                return Err(Error::build(ErrorType::TorrentsNotSet, None))
            }
        };

        let tags = self.tags.and_then(|v| Some(SepVec::new(v, ',')));

        Ok(TorrentAddDescriptor {
            urls,
            paths,
            savepath: self.savepath,
            cookie: self.cookie,
            category: self.category,
            tags: tags,
            skip_checking: self.skip_checking,
            paused: self.paused,
            root_folder: self.root_folder,
            rename: self.rename,
            up_limit: self.up_limit,
            dl_limit: self.dl_limit,
            ratio_limit: self.ratio_limit,
            seeding_time_limit: self.seeding_time_limit,
            auto_tmm: self.auto_tmm,
            sequential_download: self.sequential_download,
            first_last_piece_prio: self.first_last_piece_prio,
        })
    }
}

impl Api {
    pub async fn add_torrent(&mut self, descriptor: impl Borrow<TorrentAddDescriptor>) -> Result<(), Error> {
        let descriptor = descriptor.borrow();

        match (
            descriptor.paths.is_empty(),
            descriptor.urls.inner_vec().is_empty(),
        ) {
            (true, true) => panic!(),
            (true, false) => {
                let mut form_urls = reqwest::multipart::Form::new();

                form_urls = form_urls.text("urls", descriptor.urls.to_string());

                form_urls = thing(form_urls, descriptor.clone());

                let response_urls = self
                    .reqwest_client
                    .post(format!("{}/api/v2/torrents/add", self.authority))
                    .multipart(form_urls)
                    .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
                    .send()
                    .await
                    .map_err(|e| {
                        Error::build(ErrorType::ReqwestError(Box::new(e)), None)
                    })?;

                if response_urls.status().is_success() {
                    return Ok(());
                } else {
                    return Err(Error::build(ErrorType::MiscNetError(code!(response_urls).unwrap()), code!(response_urls)));
                }
            }
            (false, true) => {
                let form = torrents_part(&descriptor).await?;

                let response_torrents = self
                    .reqwest_client
                    .post(format!("{}/api/v2/torrents/add", self.authority))
                    .multipart(form)
                    .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
                    .send()
                    .await
                    .map_err(|e| {
                        Error::build(ErrorType::ReqwestError(Box::new(e)), None)
                    })?;

                if response_torrents.status().is_success() {
                    return Ok(());
                } else {
                    return Err(Error::build(ErrorType::MiscNetError(code!(response_torrents).unwrap()), code!(response_torrents)));
                }
            }

            (false, false) => {
                // ---------- TORRENT FILES ----------
                let form_torrents = torrents_part(&descriptor).await?;

                let built_torrents = self
                    .reqwest_client
                    .post(format!("{}/api/v2/torrents/add", self.authority))
                    .multipart(form_torrents)
                    .header(header::COOKIE, format!("SID={}", self.get_cookie().await?));

                // ---------- TORRENT FILES ----------

                // ---------- URLS ----------
                let mut form_urls = reqwest::multipart::Form::new();

                form_urls = form_urls.text("urls", descriptor.urls.to_string());
                form_urls = thing(form_urls, descriptor.clone());

                let built_urls = self
                    .reqwest_client
                    .post(format!("{}/api/v2/torrents/add", self.authority))
                    .multipart(form_urls)
                    .header(header::COOKIE, format!("SID={}", self.get_cookie().await?));

                // ---------- URLS ----------

                let (response_torrents, response_urls) =
                    tokio::join!(built_torrents.send(), built_urls.send());

                let mut thing = (false, false);

                if response_torrents
                    .map_err(|e| {
                        Error::build(ErrorType::ReqwestError(Box::new(e)), None)
                    })?
                    .status()
                    .is_success()
                {
                    thing.0 = true;
                }

                if response_urls
                    .map_err(|e| {
                        Error::build(ErrorType::ReqwestError(Box::new(e)), None)
                    })?
                    .status()
                    .is_success()
                {
                    thing.1 = true
                }

                match thing {
                        (true, true) => return Ok(()),
                        (true, false) => return Err(Error::build(ErrorType::MiscError("something went wrong while adding urls.".to_string()), None)),
                        (false, true) => return Err(Error::build(ErrorType::MiscError("something went wrong while adding torrent files.".to_string()), None)),
                        (false, false) => return Err(Error::build(ErrorType::MiscError("wow, you really messed up. both torrents and urls failed.".to_string()), None)),
                    }
            }
        };
    }
}

fn thing(
    mut form: reqwest::multipart::Form,
    descriptor: TorrentAddDescriptor,
) -> reqwest::multipart::Form {
    if let Some(savepath) = descriptor.savepath {
        form = form.text("savepath", savepath);
    }

    if let Some(cookie) = descriptor.cookie {
        form = form.text("cookie", cookie);
    }

    if let Some(category) = descriptor.category {
        form = form.text("category", category);
    }

    if let Some(tags) = descriptor.tags {
        form = form.text("tags", tags.to_string());
    }

    if let Some(skip_checking) = descriptor.skip_checking {
        form = form.text("skip_checking", skip_checking.to_string());
    }

    if let Some(paused) = descriptor.paused {
        form = form.text("paused", paused.to_string());
    }

    if let Some(root_folder) = descriptor.root_folder {
        form = form.text("root_folder", root_folder);
    }

    if let Some(rename) = descriptor.rename {
        form = form.text("rename", rename);
    }

    if let Some(up_limit) = descriptor.up_limit {
        form = form.text("upLimit", up_limit.to_string());
    }

    if let Some(dl_limit) = descriptor.dl_limit {
        form = form.text("dlLimit", dl_limit.to_string());
    }

    if let Some(ratio_limit) = descriptor.ratio_limit {
        form = form.text("ratioLimit", ratio_limit.to_string());
    }

    if let Some(seeding_time_limit) = descriptor.seeding_time_limit {
        form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
    }

    if let Some(auto_tmm) = descriptor.auto_tmm {
        form = form.text("autoTMM", auto_tmm.to_string());
    }

    if let Some(sequential_download) = descriptor.sequential_download {
        form = form.text("sequentialDownload", sequential_download.to_string());
    }

    if let Some(first_last_piece_prio) = descriptor.first_last_piece_prio {
        form = form.text("firstLastPiecePrio", first_last_piece_prio.to_string());
    }

    form
}

async fn torrents_part(
    descriptor: &TorrentAddDescriptor,
) -> Result<reqwest::multipart::Form, Error> {
    let mut form_torrents = reqwest::multipart::Form::new();
    for path in descriptor.paths.clone() {
        let mut file = File::open(path)
            .await
            .map_err(|_| Error::build(ErrorType::TorrentFilePathError, None))?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .await
            .map_err(|_| Error::build(ErrorType::TorrentFilePathError, None))?;

        // part 4 the multipart form
        let file_part = reqwest::multipart::Part::bytes(buffer)
            .file_name("torrent_file.torrent")
            .mime_str("application/x-bittorrent")
            .unwrap();

        form_torrents = form_torrents.part("torrents", file_part);
    }
    Ok(form_torrents)
}

#[test]
fn aaaa() {
}