use protobuf_build::*;
use std::fs::{read_dir, remove_file, File};
use std::io::Write;

fn main() {
    let file_names: Vec<_> = read_dir("proto")
        .expect("Couldn't read proto directory")
        .map(|e| {
            format!(
                "proto/{}",
                e.expect("Couldn't list file").file_name().to_string_lossy()
            )
        })
        .collect();

    for f in &file_names {
        println!("cargo:rerun-if-changed={}", f);
    }

    // Generate Prost files.
    generate_prost_files(&file_names, "src/proto/prost");
    //remove_file("src/prost/gogoproto.rs").unwrap();
    //remove_file("src/prost/google.protobuf.rs").unwrap();
    let mod_names = module_names_for_dir("src/proto/prost");
    generate_wrappers(
        &mod_names
            .iter()
            .map(|m| format!("src/proto/prost/{}.rs", m))
            .collect::<Vec<_>>(),
        "src/proto/prost",
    );

    // Generate rust-protobuf files.
    let file_names: Vec<_> = file_names.iter().map(|s| &**s).collect();
    generate_protobuf_files(&file_names, "src/proto/protobuf");

    let mod_names = module_names_for_dir("src/proto/protobuf");

    let out_file_names: Vec<_> = mod_names
        .iter()
        .map(|m| format!("src/proto/protobuf/{}.rs", m))
        .collect();
    let out_file_names: Vec<_> = out_file_names.iter().map(|f| &**f).collect();
}
