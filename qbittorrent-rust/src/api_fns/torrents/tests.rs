use crate::{
    api_fns::torrents::torrents::TorrentType,
    core::{api::QbitApi, creds::Credentials},
};

use super::{add_torrent::TorrentAddDescriptor, torrents::Torrent};

// #[test]
// fn prob_the_only_time_ive_ever_prayed() {
//     let x = TorrentAddDescriptor::builder()
//         .torrents(vec![
//             Torrent::new("aaaaaa", TorrentType::Url),
//             Torrent::new("/home/arch/hiiiii", TorrentType::TorrentFile),
//         ])
//         .savepath("/downloads".to_string())
//         .cookie("my_cookie_value")
//         .category("Movies")
//         .tags(vec!["action", "comedy"])
//         .skip_checking(true)
//         .paused(false)
//         .root_folder("root_folder")
//         .rename("my_torrent")
//         .up_limit(1024)
//         .dl_limit(2048)
//         .ratio_limit(2.5)
//         .seeding_time_limit(120)
//         .auto_tmm(true)
//         .sequential_download(true)
//         .first_last_piece_prio(false)
//         .build()
//         .unwrap();

//     println!("{:?}a", x)
// }

#[tokio::test]
async fn nvm_this_is_the_second_time_ive_ever_prayed() {
    let mut api = QbitApi::new(
        "http://localhost:6011/",
        Credentials::new("admin", "123456"),
    )
    .await
    .unwrap();
    let descriptor = TorrentAddDescriptor::builder(vec![
        Torrent::new(TorrentType::TorrentFile("/home/arch/Downloads/archlinux-2024.11.01-x86_64.iso.torrent")),
    ])
        .tags(vec!["aaaa1".to_string(), "aaaaaa2".to_string()])
        .rename("hiiiii")
        .build()
        .unwrap();
    api.torrents_add_torrent(descriptor).await.unwrap();
}

#[tokio::test]
async fn nvm_this_is_the_second_time_ive_ever_prayed_pt2() {
    let mut api = QbitApi::new(
        "http://localhost:6011/",
        Credentials::new("admin", "123456"),
    )
    .await
    .unwrap();
    let x = api.app_version().await.unwrap();
    println!("{}", x);
}

#[tokio::test]
pub async fn test() {
    let mut api = QbitApi::new(
        "http://localhost:6011/",
        Credentials::new("admin", "123456"),
    )
    .await
    .unwrap();

    let x = api.app_get_default_save_path().await;

    println!("{:?}", x);
}