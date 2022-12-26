use std::env;
use std::path::PathBuf;

use asn1_compiler::{
    generator::{Codec, Derive, Visibility},
    Asn1Compiler,
};

fn main() -> std::io::Result<()> {
    let module = "ngap.rs";
    let spec_file_name = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("specs")
        .join("ngap.asn");
    let spec_files = vec![spec_file_name];
    let rs_module = PathBuf::from(env::var("OUT_DIR").unwrap()).join(module);
    let rs_module = rs_module.to_str().unwrap();

    let mut compiler = Asn1Compiler::new(
        rs_module,
        false,
        &Visibility::Public,
        vec![Codec::Aper],
        vec![Derive::Debug, Derive::Serialize, Derive::Deserialize],
    );

    compiler.compile_files(&spec_files)?;

    Ok(())
}
