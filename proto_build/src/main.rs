//! Protobuf source files are in the althea-chain repo, this binary copyies the result to the althea_proto crate
//! for import and use. While this builder generates about a dozen files only one contains all the module
//! proto info and the rest are discarded in favor of upstream cosmos-sdk-proto

// Building new Althea rust proto definitions
// run 'cargo run'
// go to althea_proto/prost
// re-write calls to super::super::cosmos as cosmos-sdk-proto::cosmos

use std::{
    borrow::Cow,
    ffi::OsStr,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process,
};

use cosmos_sdk::{cosmos_main, RootDirs};
use env_logger::Env;
use regex::Regex;
use titan::titan_main;
use walkdir::WalkDir;
use wasmd::wasmd_main;

pub mod cosmos_sdk;
pub mod titan;
pub mod wasmd;

#[macro_use]
extern crate log;

/// Attribute preceeding a Tonic client definition
pub const TONIC_CLIENT_ATTRIBUTE: &str = "#[doc = r\" Generated client implementations.\"]";
/// Attributes to add to gRPC clients
pub const GRPC_CLIENT_ATTRIBUTES: &[&str] = &[
    "#[cfg(feature = \"grpc\")]",
    "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
    TONIC_CLIENT_ATTRIBUTE,
];
/// Root directories of the managed projects

pub const COSMOS_SDK_ROOT: &str = "../cosmos-sdk/";
pub const TENDERMINT_ROOT: &str = "../tendermint/";
pub const IBC_ROOT: &str = "../ibc-go/";
pub const WASMD_ROOT: &str = "../wasmd/";
pub const GRAVITY_ROOT: &str = "../gravity/";
pub const TITAN_ROOT: &str = "../titan/";

/// A temporary directory for proto building
pub const TMP_PATH: &str = "/tmp/proto/";
/// the output directory
pub const COSMOS_OUT_PATH: &str = "../cosmos_sdk_proto/src/prost/";
pub const GRAVITY_OUT_PATH: &str = "../gravity_proto/src/prost/";
pub const WASMD_OUT_PATH: &str = "../wasmd_proto/src/prost/";
pub const TITAN_OUT_PATH: &str = "../titan_proto/src/prost/";

#[derive(Clone)]
pub struct RegexReplace {
    pub regex: Cow<'static, str>, // A regular expression to match faulty generated code with
    pub replace: Cow<'static, str>, // The string to replace the faulty code with
}
impl RegexReplace {
    pub const fn new(regex: &'static str, replace: &'static str) -> Self {
        RegexReplace {
            regex: Cow::Borrowed(regex),
            replace: Cow::Borrowed(replace),
        }
    }
}
// Specifies a replacement of "super::super::...::cosmos" with "cosmos_sdk_proto::cosmos"
pub const COSMOS_SDK_PROTO_IMPORT_REGEX: &str = "(super::)+cosmos";
pub const COSMOS_SDK_PROTO_CRATE_REPLACE: &str = "cosmos_sdk_proto::cosmos";
pub const COSMOS_SDK_PROTO_CRATE_REGEX_REPLACE: RegexReplace = RegexReplace::new(
    COSMOS_SDK_PROTO_IMPORT_REGEX,
    COSMOS_SDK_PROTO_CRATE_REPLACE,
);
// Specifies a replacement of "super::super::...::ibc" with "cosmos_sdk_proto::ibc"
pub const IBC_PROTO_IMPORT_REGEX: &str = "(super::)+ibc";
pub const IBC_PROTO_CRATE_REPLACE: &str = "cosmos_sdk_proto::ibc";
pub const IBC_PROTO_REGEX_REPLACE: RegexReplace =
    RegexReplace::new(IBC_PROTO_IMPORT_REGEX, IBC_PROTO_CRATE_REPLACE);

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // used by protobuf-src in order to compile protoc on the fly rather than depend on system protoc
    std::env::set_var("PROTOC", protobuf_src::protoc());

    // Initiate Cosmos SDK
    cosmos_main(
        RootDirs {
            cosmos: COSMOS_SDK_ROOT.to_string(),
            tendermint: TENDERMINT_ROOT.to_string(),
            ibc: IBC_ROOT.to_string(),
        },
        TMP_PATH,
        COSMOS_OUT_PATH,
    );

    wasmd_main(WASMD_ROOT.to_string(), TMP_PATH, WASMD_OUT_PATH);

    titan_main(TITAN_ROOT.to_string(), TMP_PATH, TITAN_OUT_PATH);
}

struct CompileArgs<'a> {
    proto_path: &'a PathBuf,
    proto_include_paths: &'a [PathBuf],
    replacements: &'a [RegexReplace],
    exclusions: &'a [&'a str],
    tmp_path: &'a Path,
    out_path: &'a Path,
    clean_tmp: bool,
    clean_out: bool,
}
fn compile_protos(
    CompileArgs {
        proto_path,
        proto_include_paths: _,
        replacements,
        exclusions,
        tmp_path,
        out_path,
        clean_tmp,
        clean_out,
    }: CompileArgs,
) {
    info!("Compiling proto files... {:#?}", proto_path);

    // create directories for temporary build dirs
    fs::create_dir_all(tmp_path)
        .unwrap_or_else(|_| panic!("Failed to create {:?}", tmp_path.to_str()));

    run_buf("buf.sdk.gen.yaml", proto_path, tmp_path);

    copy_generated_files(
        tmp_path,
        out_path,
        replacements,
        exclusions,
        clean_tmp,
        clean_out,
    );

    info!("[info ] => Done!");
}

fn copy_generated_files(
    from_dir: &Path,
    to_dir: &Path,
    replacements: &[RegexReplace],
    exclusions: &[&str],
    clean_from: bool,
    clean_to: bool,
) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    if clean_to {
        remove_dir_all(to_dir).unwrap_or_default();
        create_dir_all(to_dir).unwrap();
    }

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(
                replacements,
                exclusions,
                e.path(),
                Path::new(&format!("{}/{}", to_dir.display(), &filename)),
            )
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }

    if clean_from {
        remove_dir_all(from_dir).unwrap_or_default();
        create_dir_all(from_dir).unwrap();
    }
}

fn copy_and_patch(
    replacements: &[RegexReplace],
    exclusions: &[&str],
    src: &Path,
    dest: &Path,
) -> io::Result<()> {
    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in exclusions {
        if let Some(filename) = src.file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = fs::read_to_string(src)?;

    // In plenty of situations we have files which rely on upstream dependencies or simply
    // misreferenced files in the same crate, use the replacements to modify the contents
    for replace in replacements {
        contents = Regex::new(&replace.regex)
            .unwrap()
            .replace_all(&contents, &replace.replace)
            .to_string();
    }
    // Patch each service definition with a feature attribute
    let patched_contents =
        contents.replace(TONIC_CLIENT_ATTRIBUTE, &GRPC_CLIENT_ATTRIBUTES.join("\n"));

    fs::write(dest, patched_contents)
}

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = process::Stdio::inherit();

    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => panic!(
                "error running '{:?}': command not found. Is it installed?",
                cmd.as_ref()
            ),
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("{:?} exited with error code: {:?}", cmd.as_ref(), code),
            None => panic!("{:?} exited without error code", cmd.as_ref()),
        }
    }
}

fn run_buf(config: &str, proto_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    run_cmd(
        "buf",
        [
            "generate",
            "--template",
            config,
            "--include-imports",
            "-o",
            &out_dir.as_ref().display().to_string(),
            &proto_path.as_ref().display().to_string(),
        ],
    );
}
