pub struct Credentials {
    pub username: String,
    pub password: String,
}
pub struct On {
    pub namespace: &'static str,
    pub database: &'static str,
}
pub struct ConnectionOptions {
    pub connection_url: &'static str,
    pub auth: Option<Credentials>,
    pub on: Option<On>,
}
