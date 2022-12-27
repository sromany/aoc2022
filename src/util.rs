extern crate envfile;
use envfile::EnvFile;
use std::path::Path;

pub fn get_auth_cookie_session_from_envfile() -> String {
    return EnvFile::new(&Path::new(".env")).unwrap().get("session").expect("Error: no key session in .env file").to_owned();
}