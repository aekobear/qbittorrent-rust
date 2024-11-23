#[derive(Debug, Clone)]
pub enum TorrentType<S: Clone + Into<String>> {
    Url(S),
    TorrentFile(S),
}

#[derive(Debug, Clone)]
pub(crate) enum TorrentInner {
    Url(String),
    RawTorrent(String),
}


#[derive(Debug, Clone)]
pub struct Torrent {
    inner: TorrentInner,
}
impl Torrent {
    /// creates a new [`Torrent`]. 
    /// the `Result` returned by this function can be `unwrap`ped without worry as long as the file path is readable.
    /// 
    /// # WARNING
    /// - if the [`TorrentType`] is `Url`, you will need to use an url. if it is a `RawTorrent` you will need to use a file path.
    /// - attention!!! the contents of the file in case of `TorrentType::RawTorrent` will NOT be read by this function, but by the [`Api::add_torrent`] function! make sure the path is accessible.
    pub fn new<S: Into<String> + Clone>(torrent_type: TorrentType<S>) -> Self {
        match torrent_type {
            TorrentType::Url(s) => Self{inner: TorrentInner::Url(s.into())},
            TorrentType::TorrentFile(s) =>Self{inner: {TorrentInner::RawTorrent(s.into())}},
        }
    }

    pub(crate) fn get_inner(&self) -> TorrentInner {
        self.inner.clone()
    }

    // pub fn get_type<S: Clone + Into<String>>(&self) -> TorrentType<S> {
    //     match self.get_inner() {
    //         TorrentInner::Url(_) => TorrentType::Url(String::new()),
    //         TorrentInner::RawTorrent(_) => TorrentType::TorrentFile(String::new()),
    //     }
    // }
}