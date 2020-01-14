use lazy_static::lazy_static;
use url::Url;

lazy_static! {
    pub static ref DEFAULT_SERVER: Url = Url::parse("https://authserver.mojang.com").unwrap();
}
