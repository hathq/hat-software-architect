//! Local document admission. No URL fetching, custody, parsing or authority.
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Read,
    path::Path,
};

pub const MAX_DOCUMENT_BYTES: u64 = 1_048_576;

/// Opens one bounded regular file without following the final link.
/// Parent directories must be canonical and owned by the caller.
///
/// # Errors
/// Rejects links, special files, growth, changed handle metadata or excess bytes.
pub fn read(path: &Path) -> Result<Vec<u8>, String> {
    let mut file = open(path)?;
    let before = file.metadata().map_err(message)?;
    if !before.is_file() || before.len() > MAX_DOCUMENT_BYTES {
        return Err("document-not-regular-or-oversized".into());
    }
    let mut bytes = Vec::new();
    file.by_ref()
        .take(MAX_DOCUMENT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(message)?;
    let after = file.metadata().map_err(message)?;
    let named = fs::symlink_metadata(path).map_err(message)?;
    if bytes.len() as u64 > MAX_DOCUMENT_BYTES
        || before.len() != bytes.len() as u64
        || after.len() != before.len()
        || after.modified().ok() != before.modified().ok()
        || !after.is_file()
        || !same_file(&before, &named)
        || !same_file(&before, &after)
    {
        return Err("document-changed-or-oversized".into());
    }
    Ok(bytes)
}

/// Reads one lowercase digest-named JSON file from an admitted private store.
///
/// # Errors
/// Rejects non-digest paths before access and verifies the exact raw bytes.
pub fn read_digest(root: &Path, digest: &str) -> Result<Vec<u8>, String> {
    if !valid_digest(digest) {
        return Err("document-reference-invalid".into());
    }
    let bytes = read(&root.join(format!("{digest}.json")))?;
    if hex::encode(Sha256::digest(&bytes)) != digest {
        return Err("document-digest-mismatch".into());
    }
    Ok(bytes)
}

#[cfg(unix)]
fn open(path: &Path) -> Result<File, String> {
    use std::os::unix::fs::OpenOptionsExt;
    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC)
        .open(path)
        .map_err(message)
}
#[cfg(not(unix))]
fn open(_: &Path) -> Result<File, String> {
    Err("secure-document-admission-unsupported".into())
}
fn message(error: impl std::fmt::Display) -> String {
    error.to_string()
}

#[cfg(unix)]
fn same_file(a: &fs::Metadata, b: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    a.dev() == b.dev() && a.ino() == b.ino() && b.is_file()
}
#[cfg(not(unix))]
fn same_file(_: &fs::Metadata, _: &fs::Metadata) -> bool {
    false
}

pub(crate) fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
