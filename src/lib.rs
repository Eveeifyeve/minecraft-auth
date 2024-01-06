#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod oauth;

pub use oauth::Info as OauthInfo;

use rand::Rng;

/// A structure providing login capabilities for Microsoft accounts, using OAuth2.0.
///
/// # Example
///
/// ```rust
/// use minecraft_auth::*;
/// 
/// # tokio::task::spawn(async move {
/// let client_id = "111231209837123098712";
/// 
/// let oauth = Oauth::new(client_id);
/// println!("Login here: {}", oauth.url());
/// 
/// let oauth_info = oauth.launch().await.unwrap();
/// # let _ = oauth_info;
/// # });
/// ```
pub struct Oauth {
    url: String,
    port: u16,
}

impl Oauth {
    /// Initialize OAuth login.
    pub fn new(client_id: &str) -> Self {
        let mut rng = rand::thread_rng();
        let port: u16 = rng.gen_range(25535..=65535);
        let url = format!("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope=openid%20offline_access%20https%3A%2F%2Fgraph.microsoft.com%2Fmail.read&state=12345", client_id, port);

        Self { url, port }
    }

    /// Retrieve the URL where the user would need to log in.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Launch the callback server and wait for the user to authenticate.
    pub async fn launch(&self) -> std::io::Result<OauthInfo> {
        oauth::server(self.port).await
    }
}

/// The credentials to a Minecraft (not Microsoft!) account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthData {
    pub access_token: String,
    pub uuid: Option<String>,
}
