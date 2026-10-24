mod support;
use hat_software_architect::document;
use sha2::{Digest, Sha256};
use std::fs;

#[test]
fn admits_only_bounded_digest_addressed_regular_bytes() {
    let temp = support::Scratch::new();
    let bytes = b"{\"evidence\":true}";
    let digest = hex::encode(Sha256::digest(bytes));
    let path = temp.0.join(format!("{digest}.json"));
    fs::write(&path, bytes).unwrap();
    assert_eq!(document::read_digest(&temp.0, &digest).unwrap(), bytes);
    for value in ["../escape", "", &"A".repeat(64)] {
        assert!(document::read_digest(&temp.0, value).is_err());
    }
    fs::write(&path, b"changed").unwrap();
    assert!(document::read_digest(&temp.0, &digest).is_err());
    let file = fs::File::create(&path).unwrap();
    file.set_len(document::MAX_DOCUMENT_BYTES + 1).unwrap();
    assert!(document::read(&path).is_err());
    assert!(document::read(&temp.0).is_err());
}
#[cfg(unix)]
#[test]
fn rejects_links_and_special_files_before_reading() {
    let temp = support::Scratch::new();
    let path = temp.0.join("linked.json");
    std::os::unix::fs::symlink("/dev/zero", &path).unwrap();
    assert!(document::read(&path).is_err());
    assert!(document::read(std::path::Path::new("/dev/null")).is_err());
    let pipe = temp.0.join("pipe");
    assert!(
        std::process::Command::new("mkfifo")
            .arg(&pipe)
            .status()
            .unwrap()
            .success()
    );
    assert!(document::read(&pipe).is_err());
}
