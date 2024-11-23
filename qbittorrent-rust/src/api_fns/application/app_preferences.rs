use serde_json::Value;

use crate::error_handling::error_type::ErrorType;
use crate::{core::api::Api, Error};

use std::borrow::Borrow;
use std::collections::HashMap;


/// Represents the qBittorrent application configuration.
/// docs: <https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1)#get-application-preferences>
///
/// # WARNING
/// - this is used at the user's disclosure. be careful when using this. read the docs to not make this panic.
///
/// # PANICS
/// - if any of the fields aren't what qbittorrent would expect. example: an int that can only be 1 or 2 has value 3.
#[derive(Debug, Default, serde::Serialize)]
pub struct QBittorrentConfig {
    // General settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_subfolder_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_paused_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete_mode: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preallocate_all: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_files_ext: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tmm_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub torrent_changed_tmm_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_path_changed_tmm_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_changed_tmm_enabled: Option<bool>,

    // Paths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_path_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_path: Option<String>,

    // Scan directories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_dirs: Option<HashMap<String, u8>>,

    // Export directories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_dir_fin: Option<String>,

    // Email notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_sender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_smtp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_ssl_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_auth_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notification_password: Option<String>,

    // Auto-run settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorun_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorun_program: Option<String>,

    // Queueing settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queueing_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_downloads: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_torrents: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_uploads: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_count_slow_torrents: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_torrent_dl_rate_threshold: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_torrent_ul_rate_threshold: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_torrent_inactive_timer: Option<u32>,

    // Share ratio settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ratio_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ratio: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ratio_act: Option<u32>,

    // Connection settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upnp: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_port: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dl_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connec: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connec_per_torrent: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads_per_torrent: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_tracker_timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_piece_extent_affinity: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bittorrent_protocol: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_utp_rate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_tcp_overhead: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_lan_peers: Option<bool>,

    // Alternative speed limits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_dl_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_up_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_from_hour: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_from_min: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_hour: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_min: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_days: Option<u8>,

    // Peer settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dht: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsd: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<u32>,

    // Proxy settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_peer_connections: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_auth_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_torrents_only: Option<bool>,

    // IP Filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter_trackers: Option<bool>,

    // Web UI settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_domain_list: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_port: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_upnp: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_csrf_protection_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_clickjacking_protection_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_secure_cookie_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_max_auth_fail_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_ban_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_session_timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_host_header_validation_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_local_auth: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_auth_subnet_whitelist_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_auth_subnet_whitelist: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_webui_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_webui_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_https: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_https_key_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_https_cert_path: Option<String>,

    // Dynamic DNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_service: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyndns_domain: Option<String>,

    // RSS settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_refresh_interval: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_max_articles_per_feed: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_processing_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_auto_downloading_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_download_repack_proper_episodes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss_smart_episode_filters: Option<String>,

    // Tracker addition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_trackers_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_trackers: Option<String>,

    // Custom HTTP headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_use_custom_http_headers_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ui_custom_http_headers: Option<String>,

    // Seeding time settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_seeding_time_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_seeding_time: Option<u32>,

    // Announce settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_to_all_tiers: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_to_all_trackers: Option<bool>,

    // Cache settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_io_threads: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_ips: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_memory_use: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_interface_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_network_interface: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_cache: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_cache_ttl: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_tracker_port: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_coalesce_read_write: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_embedded_tracker: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_multi_connections_from_same_ip: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_os_cache: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_upload_suggestions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_pool_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outgoing_ports_max: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outgoing_ports_min: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recheck_completed_torrents: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_peer_countries: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_resume_data_interval: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_buffer_low_watermark: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_buffer_watermark: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_buffer_watermark_factor: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket_backlog_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_choking_algorithm: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_slots_behavior: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upnp_lease_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub utp_tcp_mixed_mode: Option<u32>,
}
impl QBittorrentConfig {
    /// Creates a new builder instance for the configuration.
    pub fn builder() -> QBittorrentConfigBuilder {
        QBittorrentConfigBuilder::default()
    }

    // pub(crate) fn get_value(&self) -> Value {

    // }
}

#[derive(Default)]
pub struct QBittorrentConfigBuilder {
    config: QBittorrentConfig,
}

impl QBittorrentConfigBuilder {
    // General settings
    pub fn locale<S: Into<String>>(mut self, value: S) -> Self {
        self.config.locale = Some(value.into());
        self
    }

    pub fn create_subfolder_enabled(mut self, value: bool) -> Self {
        self.config.create_subfolder_enabled = Some(value);
        self
    }

    pub fn start_paused_enabled(mut self, value: bool) -> Self {
        self.config.start_paused_enabled = Some(value);
        self
    }

    pub fn auto_delete_mode(mut self, value: u32) -> Self {
        self.config.auto_delete_mode = Some(value);
        self
    }

    pub fn preallocate_all(mut self, value: bool) -> Self {
        self.config.preallocate_all = Some(value);
        self
    }

    pub fn incomplete_files_ext(mut self, value: bool) -> Self {
        self.config.incomplete_files_ext = Some(value);
        self
    }

    pub fn auto_tmm_enabled(mut self, value: bool) -> Self {
        self.config.auto_tmm_enabled = Some(value);
        self
    }

    pub fn torrent_changed_tmm_enabled(mut self, value: bool) -> Self {
        self.config.torrent_changed_tmm_enabled = Some(value);
        self
    }

    pub fn save_path_changed_tmm_enabled(mut self, value: bool) -> Self {
        self.config.save_path_changed_tmm_enabled = Some(value);
        self
    }

    pub fn category_changed_tmm_enabled(mut self, value: bool) -> Self {
        self.config.category_changed_tmm_enabled = Some(value);
        self
    }

    // Paths
    pub fn save_path<S: Into<String>>(mut self, value: S) -> Self {
        self.config.save_path = Some(value.into());
        self
    }

    pub fn temp_path<S: Into<String>>(mut self, value: S) -> Self {
        self.config.temp_path = Some(value.into());
        self
    }

    pub fn temp_path_enabled(mut self, value: bool) -> Self {
        self.config.temp_path_enabled = Some(value);
        self
    }

    // Scan directories
    pub fn scan_dirs<S: Into<String>>(mut self, value: HashMap<S, u8>) -> Self {
        let mut hash = HashMap::new();

        for (x, y) in value.into_iter() {
            if !(y == 0 || y == 1) {
                panic!("the values aren't valid. (scan_dirs)");
            }
            hash.insert(x.into(), y);
        }

        self.config.scan_dirs = Some(hash);
        self
    }

    // Export directories
    pub fn export_dir<S: Into<String>>(mut self, value: S) -> Self {
        self.config.export_dir = Some(value.into());
        self
    }

    pub fn export_dir_fin<S: Into<String>>(mut self, value: S) -> Self {
        self.config.export_dir_fin = Some(value.into());
        self
    }

    // Email notifications
    pub fn mail_notification_enabled(mut self, value: bool) -> Self {
        self.config.mail_notification_enabled = Some(value);
        self
    }

    pub fn mail_notification_sender<S: Into<String>>(mut self, value: S) -> Self {
        self.config.mail_notification_sender = Some(value.into());
        self
    }

    pub fn mail_notification_email<S: Into<String>>(mut self, value: S) -> Self {
        self.config.mail_notification_email = Some(value.into());
        self
    }

    pub fn mail_notification_smtp<S: Into<String>>(mut self, value: S) -> Self {
        self.config.mail_notification_smtp = Some(value.into());
        self
    }

    pub fn mail_notification_ssl_enabled(mut self, value: bool) -> Self {
        self.config.mail_notification_ssl_enabled = Some(value);
        self
    }

    pub fn mail_notification_auth_enabled(mut self, value: bool) -> Self {
        self.config.mail_notification_auth_enabled = Some(value);
        self
    }

    pub fn mail_notification_username<S: Into<String>>(mut self, value: S) -> Self {
        self.config.mail_notification_username = Some(value.into());
        self
    }

    pub fn mail_notification_password<S: Into<String>>(mut self, value: S) -> Self {
        self.config.mail_notification_password = Some(value.into());
        self
    }

    // autorun settings
    pub fn autorun_enabled(mut self, value: bool) -> Self {
        self.config.autorun_enabled = Some(value);
        self
    }

    pub fn autorun_program<S: Into<String>>(mut self, value: S) -> Self {
        self.config.autorun_program = Some(value.into());
        self
    }

    // Queuing stuff
    pub fn queueing_enabled(mut self, value: bool) -> Self {
        self.config.queueing_enabled = Some(value);
        self
    }

    pub fn max_active_downloads(mut self, value: u32) -> Self {
        self.config.max_active_downloads = Some(value);
        self
    }

    pub fn max_active_torrents(mut self, value: u32) -> Self {
        self.config.max_active_torrents = Some(value);
        self
    }

    pub fn max_active_uploads(mut self, value: u32) -> Self {
        self.config.max_active_uploads = Some(value);
        self
    }

    pub fn dont_count_slow_torrents(mut self, value: bool) -> Self {
        self.config.dont_count_slow_torrents = Some(value);
        self
    }

    pub fn slow_torrent_dl_rate_threshold(mut self, value: u32) -> Self {
        self.config.slow_torrent_dl_rate_threshold = Some(value);
        self
    }

    pub fn slow_torrent_ul_rate_threshold(mut self, value: u32) -> Self {
        self.config.slow_torrent_ul_rate_threshold = Some(value);
        self
    }

    pub fn slow_torrent_inactive_timer(mut self, value: u32) -> Self {
        self.config.slow_torrent_inactive_timer = Some(value);
        self
    }

    // Share ratio settings

    pub fn max_ratio_enabled(mut self, value: bool) -> Self {
        self.config.max_ratio_enabled = Some(value);
        self
    }

    pub fn max_ratio(mut self, value: f32) -> Self {
        self.config.max_ratio = Some(value);
        self
    }

    pub fn max_ratio_act(mut self, value: u32) -> Self {
        if !(value == 1 || value == 0) {
            panic!("value not in the expected range. (max_ratio_act)")
        }

        self.config.max_ratio_act = Some(value);
        self
    }

    // Connection settings
    pub fn listen_port(mut self, value: u32) -> Self {
        self.config.listen_port = Some(value);
        self
    }

    pub fn upnp(mut self, value: bool) -> Self {
        self.config.upnp = Some(value);
        self
    }

    pub fn random_port(mut self, value: bool) -> Self {
        self.config.random_port = Some(value);
        self
    }

    pub fn dl_limit(mut self, value: u32) -> Self {
        self.config.dl_limit = Some(value);
        self
    }

    pub fn up_limit(mut self, value: u32) -> Self {
        self.config.up_limit = Some(value);
        self
    }

    pub fn max_connec(mut self, value: u32) -> Self {
        self.config.max_connec = Some(value);
        self
    }

    pub fn max_connec_per_torrent(mut self, value: u32) -> Self {
        self.config.max_connec_per_torrent = Some(value);
        self
    }

    pub fn max_uploads(mut self, value: u32) -> Self {
        self.config.max_uploads = Some(value);
        self
    }

    pub fn max_uploads_per_torrent(mut self, value: u32) -> Self {
        self.config.max_uploads_per_torrent = Some(value);
        self
    }

    pub fn stop_tracker_timeout(mut self, value: u32) -> Self {
        self.config.stop_tracker_timeout = Some(value);
        self
    }

    pub fn enable_piece_extent_affinity(mut self, value: bool) -> Self {
        self.config.enable_piece_extent_affinity = Some(value);
        self
    }

    pub fn bittorrent_protocol(mut self, value: u32) -> Self {
        if !((0..3).contains(&value)) {
            panic!("value not in the expected range. (bittorrent protocol)")
        }

        self.config.bittorrent_protocol = Some(value);
        self
    }

    pub fn limit_utp_rate(mut self, value: bool) -> Self {
        self.config.limit_utp_rate = Some(value);
        self
    }

    pub fn limit_tcp_overhead(mut self, value: bool) -> Self {
        self.config.limit_tcp_overhead = Some(value);
        self
    }

    pub fn limit_lan_peers(mut self, value: bool) -> Self {
        self.config.limit_lan_peers = Some(value);
        self
    }

    // Alternative speed limits
    pub fn alt_dl_limit(mut self, value: u32) -> Self {
        self.config.alt_dl_limit = Some(value);
        self
    }

    pub fn alt_up_limit(mut self, value: u32) -> Self {
        self.config.alt_up_limit = Some(value);
        self
    }

    pub fn scheduler_enabled(mut self, value: bool) -> Self {
        self.config.scheduler_enabled = Some(value);
        self
    }

    pub fn schedule_from_hour(mut self, value: u32) -> Self {
        self.config.schedule_from_hour = Some(value);
        self
    }

    pub fn schedule_from_min(mut self, value: u32) -> Self {
        self.config.schedule_from_min = Some(value);
        self
    }

    pub fn schedule_to_hour(mut self, value: u32) -> Self {
        self.config.schedule_to_hour = Some(value);
        self
    }

    pub fn schedule_to_min(mut self, value: u32) -> Self {
        self.config.schedule_to_min = Some(value);
        self
    }

    pub fn scheduler_days(mut self, value: u8) -> Self {
        if !((0_u8..10).contains(&value)) {
            panic!("value not in the range expected. (scheduer_days)")
        }

        self.config.scheduler_days = Some(value);
        self
    }

    // Peer settings
    pub fn dht(mut self, value: bool) -> Self {
        self.config.dht = Some(value);
        self
    }

    pub fn pex(mut self, value: bool) -> Self {
        self.config.pex = Some(value);
        self
    }

    pub fn lsd(mut self, value: bool) -> Self {
        self.config.lsd = Some(value);
        self
    }

    pub fn encryption(mut self, value: u32) -> Self {
        if !((0..3).contains(&value)) {
            panic!("value not in the expected range. (encryption)")
        }

        self.config.encryption = Some(value);
        self
    }

    // Proxy settings
    pub fn proxy_type(mut self, value: i32) -> Self {
        match value {
            -1 | 1 | 2 | 3 | 4 | 5 => {}
            _ => panic!("value not in the expected range. (prozy type)"),
        }

        self.config.proxy_type = Some(value);
        self
    }

    pub fn proxy_ip<S: Into<String>>(mut self, value: S) -> Self {
        self.config.proxy_ip = Some(value.into());
        self
    }

    pub fn proxy_port(mut self, value: u32) -> Self {
        self.config.proxy_port = Some(value);
        self
    }

    pub fn proxy_peer_connections(mut self, value: bool) -> Self {
        self.config.proxy_peer_connections = Some(value);
        self
    }

    pub fn proxy_auth_enabled(mut self, value: bool) -> Self {
        self.config.proxy_auth_enabled = Some(value);
        self
    }

    pub fn proxy_username<S: Into<String>>(mut self, value: S) -> Self {
        self.config.proxy_username = Some(value.into());
        self
    }

    pub fn proxy_password<S: Into<String>>(mut self, value: S) -> Self {
        self.config.proxy_password = Some(value.into());
        self
    }

    pub fn proxy_torrents_only(mut self, value: bool) -> Self {
        self.config.proxy_torrents_only = Some(value);
        self
    }

    // IP Filter
    pub fn ip_filter_enabled(mut self, value: bool) -> Self {
        self.config.ip_filter_enabled = Some(value);
        self
    }

    pub fn ip_filter_path<S: Into<String>>(mut self, value: S) -> Self {
        self.config.ip_filter_path = Some(value.into());
        self
    }

    pub fn ip_filter_trackers(mut self, value: bool) -> Self {
        self.config.ip_filter_trackers = Some(value);
        self
    }

    // Web UI settings
    pub fn web_ui_domain_list<S: Into<String>>(mut self, value: S) -> Self {
        self.config.web_ui_domain_list = Some(value.into());
        self
    }

    pub fn web_ui_address<S: Into<String>>(mut self, value: S) -> Self {
        self.config.web_ui_address = Some(value.into());
        self
    }

    pub fn web_ui_port(mut self, value: u32) -> Self {
        self.config.web_ui_port = Some(value);
        self
    }

    pub fn web_ui_upnp(mut self, value: bool) -> Self {
        self.config.web_ui_upnp = Some(value);
        self
    }

    pub fn web_ui_username<S: Into<String>>(mut self, value: S) -> Self {
        self.config.web_ui_username = Some(value.into());
        self
    }

    pub fn web_ui_password<S: Into<String>>(mut self, value: S) -> Self {
        self.config.web_ui_password = Some(value.into());
        self
    }

    pub fn web_ui_csrf_protection_enabled(mut self, value: bool) -> Self {
        self.config.web_ui_csrf_protection_enabled = Some(value);
        self
    }

    pub fn web_ui_clickjacking_protection_enabled(mut self, value: bool) -> Self {
        self.config.web_ui_clickjacking_protection_enabled = Some(value);
        self
    }

    pub fn web_ui_secure_cookie_enabled(mut self, value: bool) -> Self {
        self.config.web_ui_secure_cookie_enabled = Some(value);
        self
    }

    pub fn web_ui_max_auth_fail_count(mut self, value: u32) -> Self {
        self.config.web_ui_max_auth_fail_count = Some(value);
        self
    }

    pub fn web_ui_ban_duration(mut self, value: u32) -> Self {
        self.config.web_ui_ban_duration = Some(value);
        self
    }

    pub fn web_ui_session_timeout(mut self, value: u32) -> Self {
        self.config.web_ui_session_timeout = Some(value);
        self
    }

    pub fn web_ui_host_header_validation_enabled(mut self, value: bool) -> Self {
        self.config.web_ui_host_header_validation_enabled = Some(value);
        self
    }

    pub fn bypass_local_auth(mut self, value: bool) -> Self {
        self.config.bypass_local_auth = Some(value);
        self
    }

    pub fn bypass_auth_subnet_whitelist_enabled(mut self, value: bool) -> Self {
        self.config.bypass_auth_subnet_whitelist_enabled = Some(value);
        self
    }

    pub fn bypass_auth_subnet_whitelist<S: Into<String>>(mut self, value: S) -> Self {
        self.config.bypass_auth_subnet_whitelist = Some(value.into());
        self
    }

    pub fn alternative_webui_enabled(mut self, value: bool) -> Self {
        self.config.alternative_webui_enabled = Some(value);
        self
    }

    pub fn alternative_webui_path<S: Into<String>>(mut self, value: S) -> Self {
        self.config.alternative_webui_path = Some(value.into());
        self
    }

    pub fn use_https(mut self, value: bool) -> Self {
        self.config.use_https = Some(value);
        self
    }

    pub fn ssl_key<S: Into<String>>(mut self, value: S) -> Self {
        self.config.ssl_key = Some(value.into());
        self
    }

    pub fn ssl_cert<S: Into<String>>(mut self, value: S) -> Self {
        self.config.ssl_cert = Some(value.into());
        self
    }

    pub fn web_ui_https_key_path<S: Into<String>>(mut self, value: S) -> Self {
        self.config.web_ui_https_key_path = Some(value.into());
        self
    }

    pub fn web_ui_https_cert_path<S: Into<String>>(mut self, value: S) -> Self {
        self.config.web_ui_https_cert_path = Some(value.into());
        self
    }

    // Dynamic DNS
    pub fn dyndns_enabled(mut self, value: bool) -> Self {
        self.config.dyndns_enabled = Some(value);
        self
    }

    pub fn dyndns_service(mut self, value: u32) -> Self {
        if !(value == 0 || value == 1) {
            panic!("value not in the expected range. (dyndns_service)")
        }

        self.config.dyndns_service = Some(value);
        self
    }

    pub fn dyndns_username<S: Into<String>>(mut self, value: S) -> Self {
        self.config.dyndns_username = Some(value.into());
        self
    }

    pub fn dyndns_password<S: Into<String>>(mut self, value: S) -> Self {
        self.config.dyndns_password = Some(value.into());
        self
    }

    pub fn dyndns_domain<S: Into<String>>(mut self, value: S) -> Self {
        self.config.dyndns_domain = Some(value.into());
        self
    }

    // RSS settings
    pub fn rss_refresh_interval(mut self, value: u32) -> Self {
        self.config.rss_refresh_interval = Some(value);
        self
    }

    pub fn rss_max_articles_per_feed(mut self, value: u32) -> Self {
        self.config.rss_max_articles_per_feed = Some(value);
        self
    }

    pub fn rss_processing_enabled(mut self, value: bool) -> Self {
        self.config.rss_processing_enabled = Some(value);
        self
    }

    pub fn rss_auto_downloading_enabled(mut self, value: bool) -> Self {
        self.config.rss_auto_downloading_enabled = Some(value);
        self
    }

    pub fn rss_download_repack_proper_episodes(mut self, value: bool) -> Self {
        self.config.rss_download_repack_proper_episodes = Some(value);
        self
    }

    pub fn rss_smart_episode_filters<S: Into<String>>(mut self, value: S) -> Self {
        self.config.rss_smart_episode_filters = Some(value.into());
        self
    }

    // Tracker addition
    pub fn add_trackers_enabled(mut self, value: bool) -> Self {
        self.config.add_trackers_enabled = Some(value);
        self
    }

    pub fn add_trackers<S: Into<String>>(mut self, value: S) -> Self {
        self.config.add_trackers = Some(value.into());
        self
    }

    // Custom HTTP headers
    pub fn web_ui_use_custom_http_headers_enabled(mut self, value: bool) -> Self {
        self.config.web_ui_use_custom_http_headers_enabled = Some(value);
        self
    }

    pub fn web_ui_custom_http_headers<S: Into<String>>(mut self, value: S) -> Self {
        self.config.web_ui_custom_http_headers = Some(value.into());
        self
    }

    // Seeding time settings
    pub fn max_seeding_time_enabled(mut self, value: bool) -> Self {
        self.config.max_seeding_time_enabled = Some(value);
        self
    }

    pub fn max_seeding_time(mut self, value: u32) -> Self {
        self.config.max_seeding_time = Some(value);
        self
    }

    // Announce settings
    pub fn announce_ip<S: Into<String>>(mut self, value: S) -> Self {
        self.config.announce_ip = Some(value.into());
        self
    }

    pub fn announce_to_all_tiers(mut self, value: bool) -> Self {
        self.config.announce_to_all_tiers = Some(value);
        self
    }

    pub fn announce_to_all_trackers(mut self, value: bool) -> Self {
        self.config.announce_to_all_trackers = Some(value);
        self
    }

    // Cache settings
    pub fn async_io_threads(mut self, value: u32) -> Self {
        self.config.async_io_threads = Some(value);
        self
    }

    pub fn banned_ips<S: Into<String>>(mut self, value: S) -> Self {
        self.config.banned_ips = Some(value.into());
        self
    }

    pub fn checking_memory_use(mut self, value: u32) -> Self {
        self.config.checking_memory_use = Some(value);
        self
    }

    pub fn current_interface_address<S: Into<String>>(mut self, value: S) -> Self {
        self.config.current_interface_address = Some(value.into());
        self
    }

    pub fn current_network_interface<S: Into<String>>(mut self, value: S) -> Self {
        self.config.current_network_interface = Some(value.into());
        self
    }

    pub fn disk_cache(mut self, value: u32) -> Self {
        self.config.disk_cache = Some(value);
        self
    }

    pub fn disk_cache_ttl(mut self, value: u32) -> Self {
        self.config.disk_cache_ttl = Some(value);
        self
    }

    pub fn embedded_tracker_port(mut self, value: u32) -> Self {
        self.config.embedded_tracker_port = Some(value);
        self
    }

    pub fn enable_coalesce_read_write(mut self, value: bool) -> Self {
        self.config.enable_coalesce_read_write = Some(value);
        self
    }

    pub fn enable_embedded_tracker(mut self, value: bool) -> Self {
        self.config.enable_embedded_tracker = Some(value);
        self
    }

    pub fn enable_multi_connections_from_same_ip(mut self, value: bool) -> Self {
        self.config.enable_multi_connections_from_same_ip = Some(value);
        self
    }

    pub fn enable_os_cache(mut self, value: bool) -> Self {
        self.config.enable_os_cache = Some(value);
        self
    }

    pub fn enable_upload_suggestions(mut self, value: bool) -> Self {
        self.config.enable_upload_suggestions = Some(value);
        self
    }

    pub fn file_pool_size(mut self, value: u32) -> Self {
        self.config.file_pool_size = Some(value);
        self
    }

    pub fn outgoing_ports_max(mut self, value: u32) -> Self {
        self.config.outgoing_ports_max = Some(value);
        self
    }

    pub fn outgoing_ports_min(mut self, value: u32) -> Self {
        self.config.outgoing_ports_min = Some(value);
        self
    }

    pub fn recheck_completed_torrents(mut self, value: bool) -> Self {
        self.config.recheck_completed_torrents = Some(value);
        self
    }

    pub fn resolve_peer_countries(mut self, value: bool) -> Self {
        self.config.resolve_peer_countries = Some(value);
        self
    }

    pub fn save_resume_data_interval(mut self, value: u32) -> Self {
        self.config.save_resume_data_interval = Some(value);
        self
    }

    pub fn send_buffer_low_watermark(mut self, value: u32) -> Self {
        self.config.send_buffer_low_watermark = Some(value);
        self
    }

    pub fn send_buffer_watermark(mut self, value: u32) -> Self {
        self.config.send_buffer_watermark = Some(value);
        self
    }

    pub fn send_buffer_watermark_factor(mut self, value: u32) -> Self {
        self.config.send_buffer_watermark_factor = Some(value);
        self
    }

    pub fn socket_backlog_size(mut self, value: u32) -> Self {
        self.config.socket_backlog_size = Some(value);
        self
    }

    pub fn upload_choking_algorithm(mut self, value: u32) -> Self {
        if !((0..3).contains(&value)) {
            panic!("value not in the expected range. (upload_choking_algorithm)")
        }

        self.config.upload_choking_algorithm = Some(value);
        self
    }

    pub fn upload_slots_behavior(mut self, value: u32) -> Self {
        if !(value == 0 || value == 1) {
            panic!("value not in the expected range. (upload_slots_behavior)")
        }

        self.config.upload_slots_behavior = Some(value);
        self
    }

    pub fn upnp_lease_duration(mut self, value: u32) -> Self {
        self.config.upnp_lease_duration = Some(value);
        self
    }

    pub fn utp_tcp_mixed_mode(mut self, value: u32) -> Self {
        if !(value == 0 || value == 1) {
            panic!("value not in the expected range. (utp_tcp_mixed_mode)")
        }

        self.config.utp_tcp_mixed_mode = Some(value);
        self
    }

    pub fn build(self) -> QBittorrentConfig {
        self.config
    }
}

impl Api {
    pub async fn get_preferences(&mut self) -> Result<Value, Error> {
        serde_json::from_str(Self::get_preferences_raw(self).await?.as_str())
            .map_err(|e| Error::build(ErrorType::JsonSerdeError(Box::new(e)), None))
    }

    crate::post_request!(get_preferences_raw, "/app/preferences");

    pub async fn set_preferences(&mut self, config: impl Borrow<QBittorrentConfig>) -> Result<(), Error> {
        let mut hashmap = HashMap::new();

        hashmap.insert("json", config.borrow());

        self.make_request_with_form("/app/setPreferences", "set_preferences", hashmap).await?;
        Ok(())
    }
}
