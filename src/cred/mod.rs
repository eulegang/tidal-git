use std::io;

mod gpg;

pub fn find_token(host: &str) -> io::Result<String> {
    let mut cfg = dirs::config_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, ""))?;
    cfg.push("creds");
    cfg.push(format!("{host}.gpg"));

    if let Ok(token) = gpg::token(&cfg) {
        return Ok(String::from_utf8(token).unwrap());
    }

    todo!()
}
