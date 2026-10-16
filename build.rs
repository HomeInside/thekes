// build.rs
use semver::{Prerelease, Version};
use std::fs;
use std::io::Write;
use std::path::Path;

const BUILD_FILE_NAME: &str = "target/build_version.conf";

fn read_file() -> String {
    match fs::read_to_string(BUILD_FILE_NAME) {
        Ok(c) => c,
        Err(_) => String::from("Error reading file"),
    }
}

fn save_file(data: String) {
    let mut conf_file = fs::File::create(BUILD_FILE_NAME).unwrap();
    conf_file
        .write_all(data.as_bytes())
        .expect("Error while writing to file");
}

fn generate_version(pkg_version: &str) -> String {
    let mut current_version = Version::parse(pkg_version.trim()).unwrap();
    let mut new_build: i32 = 1;

    if !current_version.pre.is_empty() {
        let current_build = current_version.pre.as_str();
        let (_, build_number) = current_build.split_at(6); // extract "build."
        println!("cargo:warning=current build_number: {}", build_number);
        new_build = build_number.parse::<i32>().unwrap_or(0);
        new_build += 1;
    }

    let build_text = format!("build.{}", new_build);
    current_version.pre = Prerelease::new(build_text.as_str()).unwrap();
    println!("cargo:warning=generating build version: {}", build_text);

    current_version.to_string()
}

fn update_build_version() {
    let content_file: String = if Path::new(BUILD_FILE_NAME).exists() {
        read_file()
    } else {
        let pkg_version = env!("CARGO_PKG_VERSION");
        String::from(pkg_version)
    };

    let result = generate_version(content_file.as_str());
    save_file(result);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    update_build_version();
}
