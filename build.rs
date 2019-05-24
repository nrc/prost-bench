use protobuf_build::*;
use std::fs::read_dir;

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

    let mod_names = module_names_for_dir("src/proto/prost");
    generate_wrappers(
        &mod_names
            .iter()
            .map(|m| format!("src/proto/prost/{}.rs", m))
            .collect::<Vec<_>>(),
        "src/proto/prost",
        GenOpt::all(),
    );

    // Generate rust-protobuf files.
    let file_names: Vec<_> = file_names.iter().map(|s| &**s).collect();
    generate_protobuf_files(&file_names, "src/proto/protobuf");
}
