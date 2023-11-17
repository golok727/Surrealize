/// Your Surreal database credentials.
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Holds the namespace and database name to connect to
pub struct On {
    pub namespace: &'static str,
    pub database: &'static str,
}
/// The connection options for the DataStore to start
/// Holds the connection_url, auth credentials and the namespace and database to connect to
pub struct ConnectionOptions {
    pub connection_url: &'static str,
    pub auth: Option<Credentials>,
    pub on: Option<On>,
}

impl Credentials {
    #[allow(unused)]
    fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
    /// Returns a default credentials with username = "root" and password = "root"
    fn root() -> Self {
        Self {
            username: "root".into(),
            password: "root".into(),
        }
    }
}

impl On {
    #[allow(dead_code)]
    fn new(namespace: &'static str, database: &'static str) -> Self {
        Self {
            namespace,
            database,
        }
    }
    /// Gives a default implementation for the On struct whih namespace = "development" and
    /// database = "test"
    fn development() -> Self {
        Self {
            namespace: "development",
            database: "test",
        }
    }
}
/// Returns a default connection options with host: localhost:8000, auth: {username: "root",
/// password: "root"} , namespace: "development" and database of "test"
impl Default for ConnectionOptions {
    fn default() -> Self {
        Self {
            connection_url: "127.0.0.1:8000",
            auth: Credentials::root().into(),
            on: On::development().into(),
        }
    }
}

impl ConnectionOptions {
    #[allow(dead_code)]
    fn default_no_auth() -> Self {
        Self {
            auth: None,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_connection_options_no_auth() {
        let options = ConnectionOptions::default_no_auth();
        assert!(options.auth.is_none());
    }
}

