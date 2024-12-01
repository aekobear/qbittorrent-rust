//! # Qbittorrent-rust
//! 
//! Qbittorrent-rust is an asynchronous library that makes interfacing with the Qbittorrent WebUI API simple and intuitive, 
//! keeping things as simple as possible while still letting you have maximum freedom of use.
//! 
//! ## Design
//! 
//! The library is designed to have as few obscuring types as possible between your code and the Qbittorrent API on purpose, 
//! and its philosophy is to give the user the most freedom possible while working with it.
//! 
//! ## Usage
//! 
//! The library's main structure is [`QbitApi`], which provides all the methods to the Qbittorrent WebUI API.
//! There are 7 categories of methods, corresponding to the the various categories of requests in the Qbittorrent WebUI API documentation:
//!
//! | Name | Use |
//! | ------ | ------ |
//! | torrents | holds everything related to torrents. |
//! | log |  holds everything related to logging. |
//! | app |  holds everything related to the WebUI API application. |
//! | transfer |  holds everything related to transfer information. |
//! | sync |  holds everything related to synchronization. |
//! | search |  holds everything related to searching ans searching plugins. |
//! | rss |  holds everything related to RSS. |
//!
//! each method in [`QbitApi`] starts with its category, followed by the method's name, all in snake case. example: `torrents_add_torrent`


pub mod core;
pub mod misc;
pub mod error_handling;
pub mod api_fns;
pub mod macros;

use crate as nothing;

pub use error_handling::errors::Error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
