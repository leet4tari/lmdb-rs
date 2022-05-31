use std::env;
extern crate gcc;

fn main() {
    let target = std::env::var("TARGET").unwrap();

    let mut config = gcc::Config::new();
    config.file("mdb/libraries/liblmdb/mdb.c")
          .file("mdb/libraries/liblmdb/midl.c");
    config.opt_level(2);

    if target.contains("dragonfly") {
        config.flag("-DMDB_DSYNC=O_SYNC");
        config.flag("-DMDB_FDATASYNC=fsync");
    }

    if target.contains("apple-darwin") {
        if target.contains("x86_64") {
            config.flag("--target=x86_64-apple-macos10.12");
        }

        if target.contains("aarch64") {
            config.flag("--target=arm64-apple-macos11");
        }
    }

    if let Ok(target_arch) = env::var("LMDB_TARGET") {
        //config.flag_if_supported(&format!("-march={}", target_arch));
        config.flag(&format!("--target={}", target_arch));
    }

    config.compile("liblmdb.a");
}
