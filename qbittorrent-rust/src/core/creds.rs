/// ## Description
/// represents credentials to an account.
#[derive(Debug, Clone)]
pub struct Credentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Credentials {
    /// # Usage
    /// creates a new instance of [`Credentials`]
    pub fn new<T: Into<String>>(username: T, password: T) -> Self {
        let usrname: String = Into::into(username);
        let passwd: String = Into::into(password);
        Self {
            username: usrname,
            password: passwd
        }
    }
}