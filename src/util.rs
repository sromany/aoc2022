
pub fn get_auth_cookie_session_from_envfile() -> String {
    extern crate envfile;
    use envfile::EnvFile;
    use std::path::Path;
    let session_cookie = EnvFile::new(&Path::new("./src/.env")).unwrap().get("session").expect("Error: no key session in .env file").to_owned();
    return session_cookie
}

pub fn read_from_input_test_file<S: Into<String>>(dayname: S) -> String {
    use std::fs;
    fs::read_to_string(format!("./src/tests/{}", dayname.into())).expect("Error when reading day05 test esample")
}