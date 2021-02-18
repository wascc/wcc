use std::env;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;

/// Helper function to create the `wash` binary process
#[allow(unused)]
pub(crate) fn wash() -> std::process::Command {
    test_bin::get_test_bin("wash")
}

#[allow(unused)]
pub(crate) fn output_to_string(output: std::process::Output) -> String {
    String::from_utf8(output.stdout).expect("failed to convert output bytes to String")
}

#[allow(unused)]
pub(crate) fn test_dir_with_subfolder(subfolder: &str) -> PathBuf {
    let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
    let with_subfolder = PathBuf::from(format!("{}/tests/fixtures/{}", root_dir, subfolder));
    remove_dir_all(with_subfolder.clone());
    create_dir_all(with_subfolder.clone());
    with_subfolder
}

#[allow(unused)]
pub(crate) fn test_dir_file(subfolder: &str, file: &str) -> PathBuf {
    let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
    PathBuf::from(format!(
        "{}/tests/fixtures/{}/{}",
        root_dir, subfolder, file
    ))
}
