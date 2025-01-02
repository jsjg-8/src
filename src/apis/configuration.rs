use reqwest::Client;
use std::time::Duration;
use thiserror::Error;

/// Authentication methods supported by the Kill Bill API
#[derive(Debug, Clone)]
pub enum Auth {
    /// Basic authentication with username and optional password
    Basic {
        username: String,
        password: Option<String>,
    },
    /// OAuth2 authentication with access token
    OAuth {
        access_token: String,
    },
    /// Bearer token authentication
    Bearer {
        token: String,
    },
    /// API key authentication with optional prefix
    ApiKey {
        key: String,
        prefix: Option<String>,
    },
}

/// Possible configuration errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid base path: {0}")] InvalidBasePath(String),
    #[error("Invalid timeout value: {0}")] InvalidTimeout(String),
    #[error("Client creation failed: {0}")] ClientError(#[from] reqwest::Error),
}

/// Configuration builder for Kill Bill API client
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    base_path: String,
    user_agent: Option<String>,
    timeout: Option<Duration>,
    auth: Option<Auth>,
    client: Option<Client>,
}

/// Main configuration struct for Kill Bill API client
#[derive(Debug, Clone)]
pub struct Configuration {
    pub(crate) base_path: String,
    pub(crate) user_agent: Option<String>,
    pub(crate) client: Client,
    pub(crate) auth: Option<Auth>,
}

impl ConfigBuilder {
    /// Create a new configuration builder with default values
    pub fn new() -> Self {
        Self {
            base_path: "http://localhost".to_owned(),
            user_agent: Some("KillBill-Rust-Client/0.24.10".to_owned()),
            timeout: Some(Duration::from_secs(30)),
            auth: None,
            client: None,
        }
    }

    /// Set the base path for the API
    pub fn base_path<S: Into<String>>(mut self, base_path: S) -> Self {
        self.base_path = base_path.into();
        self
    }

    /// Set the user agent string
    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set basic authentication
    pub fn basic_auth<S: Into<String>>(mut self, username: S, password: Option<S>) -> Self {
        self.auth = Some(Auth::Basic {
            username: username.into(),
            password: password.map(Into::into),
        });
        self
    }

    /// Set OAuth authentication
    pub fn oauth<S: Into<String>>(mut self, token: S) -> Self {
        self.auth = Some(Auth::OAuth {
            access_token: token.into(),
        });
        self
    }

    /// Set Bearer token authentication
    pub fn bearer<S: Into<String>>(mut self, token: S) -> Self {
        self.auth = Some(Auth::Bearer {
            token: token.into(),
        });
        self
    }

    /// Set API key authentication
    pub fn api_key<S: Into<String>>(mut self, key: S, prefix: Option<S>) -> Self {
        self.auth = Some(Auth::ApiKey {
            key: key.into(),
            prefix: prefix.map(Into::into),
        });
        self
    }

    /// Set a custom HTTP client
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<Configuration, ConfigError> {
        let client = match self.client {
            Some(client) => client,
            None => {
                let mut builder = Client::builder().user_agent(
                    self.user_agent.as_deref().unwrap_or("")
                );

                if let Some(timeout) = self.timeout {
                    builder = builder.timeout(timeout);
                }

                builder.build()?
            }
        };

        Ok(Configuration {
            base_path: self.base_path,
            user_agent: self.user_agent,
            client,
            auth: self.auth,
        })
    }
}

impl Configuration {
    /// Create a new configuration builder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Helper method to get authentication headers
    pub fn get_auth_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(auth) = &self.auth {
            match auth {
                Auth::Basic { username, password } => {
                    if
                        let Ok(auth_value) = reqwest::header::HeaderValue::from_str(
                            &format!(
                                "Basic {}",
                                base64::encode(
                                    format!("{}:{}", username, password.as_deref().unwrap_or(""))
                                )
                            )
                        )
                    {
                        headers.insert(reqwest::header::AUTHORIZATION, auth_value);
                    }
                }
                Auth::OAuth { access_token } => {
                    if
                        let Ok(auth_value) = reqwest::header::HeaderValue::from_str(
                            &format!("OAuth {}", access_token)
                        )
                    {
                        headers.insert(reqwest::header::AUTHORIZATION, auth_value);
                    }
                }
                Auth::Bearer { token } => {
                    if
                        let Ok(auth_value) = reqwest::header::HeaderValue::from_str(
                            &format!("Bearer {}", token)
                        )
                    {
                        headers.insert(reqwest::header::AUTHORIZATION, auth_value);
                    }
                }
                Auth::ApiKey { key, prefix } => {
                    let key_value = match prefix {
                        Some(prefix) => format!("{} {}", prefix, key),
                        None => key.clone(),
                    };

                    if let Ok(api_key_value) = reqwest::header::HeaderValue::from_str(&key_value) {
                        headers.insert("X-Killbill-ApiKey", api_key_value.clone());
                        headers.insert("X-Killbill-ApiSecret", api_key_value);
                    }
                }
            }
        }

        headers
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_headers() {
        let config = Configuration::builder().basic_auth("user", Some("pass")).build().unwrap();

        let headers = config.get_auth_headers();
        assert!(headers.contains_key(reqwest::header::AUTHORIZATION));
    }

    #[test]
    fn test_api_key_headers() {
        let config = Configuration::builder().api_key("test-key", Some("prefix")).build().unwrap();

        let headers = config.get_auth_headers();
        assert!(headers.contains_key("X-Killbill-ApiKey"));
        assert!(headers.contains_key("X-Killbill-ApiSecret"));
    }

    #[test]
    fn test_custom_base_path() {
        let config = Configuration::builder().base_path("https://api.example.com").build().unwrap();

        assert_eq!(config.base_path, "https://api.example.com");
    }
}
