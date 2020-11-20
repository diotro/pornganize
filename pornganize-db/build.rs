use protobuf_codegen_pure::{
    Codegen as ProtobufCodegen,
    Customize as ProtobufCustomize
};
use walkdir::{WalkDir, DirEntry};

const DATABASE_VERSION: &str = "0.0.1";

fn process_proto_file(entry: &DirEntry) -> Option<String> {
    let path = String::from(entry.path().to_str().unwrap());
    if entry.file_type().is_dir() {
        None
    } else {
        println!("cargo:rerun-if-changed={}", path);
        Some(path)
    }
}

fn build_protos(protos: &[String]) {
    ProtobufCodegen::new()
        .out_dir("src/model/messages")
        .include("protos")
        .inputs(protos)
        .customize(ProtobufCustomize {
            expose_fields: Some(true),
            generate_accessors: Some(false),
            lite_runtime: Some(true),
            serde_derive: Some(true),
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .run()
        .unwrap();
}

fn main() {
    println!("cargo:rustc-env=PORNGANIZE_DB_VERSION={}", DATABASE_VERSION);
    let protos: Vec<String> = WalkDir::new("protos")
        .into_iter()
        .filter_map(|x| {process_proto_file(&x.unwrap())})
        .collect();
    build_protos(&protos);
}
