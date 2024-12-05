/// ## Description
/// describes the type of error which occurred.
#[derive(Debug)]
pub enum ErrorType {
    TorrentsNotSet,
    TorrentFilePathError,
    TorrentHashNotFound,
    TorrenQueueingNotEnabled,
    WrongCreds,
    TooManyFailedAttempts,
    ParameterNotExpected,
    MiscNetError(u16),
    MiscError(String),
    ReqwestError(Box<dyn std::error::Error>),
    JsonSerdeError(Box<dyn std::error::Error>),
}

impl ErrorType {
    pub(crate) fn get_message(&self) -> String {
        match self {
            ErrorType::TorrentsNotSet => "no torrents were specified.".to_string(),
            ErrorType::TorrentFilePathError => "the path specified doesn't exist, is malformed, or the file it points to couldn't be read.".to_string(),
            ErrorType::TorrentHashNotFound => "the specified torrent hash couldn't be found.".to_string(),
            ErrorType::TorrenQueueingNotEnabled => "torrent queuing id not enabled.".to_string(),
            ErrorType::WrongCreds => "the credetials are wrong.".to_string(),
            ErrorType::TooManyFailedAttempts => "the user has been banned for an amount of time because of too many failed login attempts.".to_string(),
            ErrorType::ParameterNotExpected => "one or more of the parameters speciied were wrong".to_string(),
            ErrorType::MiscError(e) => format!("Something went wrong. {}", e),
            ErrorType::ReqwestError(e) => format!("there was an error while handling networking. error: {}", e),
            ErrorType::JsonSerdeError(e) => format!("there was an error while handling JSON data. error: {}", e),
            ErrorType::MiscNetError(e) => format!("there was an error during a request. error code: {}", e),
        }
    }
}