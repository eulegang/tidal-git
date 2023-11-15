use std::{fs, io, path::Path};

use gpgme::Context;

pub fn token(path: &Path) -> io::Result<Vec<u8>> {
    gpgme::init();

    let mut ctx = Context::from_protocol(gpgme::Protocol::OpenPgp)?;
    let mut plaintext = Vec::new();
    let cipher = fs::read_to_string(path)?;

    ctx.decrypt(cipher, &mut plaintext)?;

    Ok(plaintext)
}
