use std::path::Path;

use crate::{compile_protos, RegexReplace};

pub const EXCLUDED_PROTO_PACKAGES: &[&str] = &[
    "cosmos",
    "tendermint",
    "cosmos_proto",
    "gogoproto",
    "google",
    "amino",
    "cosmwasm",
];

pub fn titan_main(root: String, tmp_dir: &str, out_dir: &str) {
    let regex_replacements = vec![];

    compile_titan_protos_and_services(
        Path::new(&root),
        Path::new(tmp_dir),
        Path::new(out_dir),
        &regex_replacements,
    );
}

fn compile_titan_protos_and_services(
    root: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "Compiling titan .proto files to Rust into '{}'...",
        out_path.display(),
    );

    let proto_path = root.join("proto");

    let proto_include_paths = [];

    compile_protos(crate::CompileArgs {
        proto_path: &proto_path,
        proto_include_paths: &proto_include_paths,
        replacements: &regex_replacements,
        exclusions: EXCLUDED_PROTO_PACKAGES,
        tmp_path,
        out_path,
        clean_tmp: false,
        clean_out: true,
    });

    info!("=> Done!");
}
