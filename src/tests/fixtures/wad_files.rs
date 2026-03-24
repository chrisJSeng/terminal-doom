use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub fn write_temp_test_file(contents: &[u8]) -> PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("doom-terminal-wad-test-{unique_suffix}.wad"));
    fs::write(&path, contents).expect("temporary wad test file should be writable");
    path
}

#[allow(dead_code)]
pub fn cleanup_temp_test_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}
