
/// ## Info
/// Describes the type of the torrent, either: 
/// a URL, such as a magnet, but any URL is accepted;
/// a torrent file (.torrent), it represents a file path.
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

/// ## Info
/// Represents a torrent.
#[derive(Debug, Clone)]
pub struct Torrent {
    inner: TorrentInner,
}
impl Torrent {
    /// ## Usage
    /// creates a new [`Torrent`]. 
    /// the `Result` returned by this function can be `unwrap`ped without worry as long as the file path is readable.
    /// 
    /// ## WARNING
    /// - the contents of the file in case of `TorrentType::RawTorrent` will NOT be read by this function, but by the `Api::add_torrent` function. Make sure the path is accessible.
    pub fn new<S: Into<String> + Clone>(torrent_type: TorrentType<S>) -> Self {
        match torrent_type {
            TorrentType::Url(s) => Self{inner: TorrentInner::Url(s.into())},
            TorrentType::TorrentFile(s) =>Self{inner: {TorrentInner::RawTorrent(s.into())}},
        }
    }

    pub(crate) fn get_inner(&self) -> TorrentInner {
        self.inner.clone()
    }
}