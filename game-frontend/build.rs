#![allow(clippy::unwrap_used)]

use leptos_i18n_build::{
    Config, FileFormat, ParseOptions, TranslationsInfos, options::CodegenOptions,
};
use std::path::PathBuf;

fn prost_build() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto");
    tonic_prost_build::configure()
        .build_transport(false)
        .compile_protos(
            &[
                "../proto/common/v1/common.proto",
                "../proto/game/v1/game.proto",
            ],
            &["../proto"],
        )?;
    Ok(())
}

fn i18n_build() -> Result<(), Box<dyn std::error::Error>> {
    let i18n_mod_directory = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("i18n");
    let attributes = "#![allow(clippy::pedantic, clippy::nursery, clippy::expect_used)]".parse()?;
    let codegen_options = CodegenOptions::default().top_level_attributes(Some(attributes));
    let parse_options = ParseOptions::default().file_format(FileFormat::Toml);
    let cfg = Config::new("en")?
        .add_locale("fr")?
        .parse_options(parse_options);
    let translations_infos = TranslationsInfos::parse(cfg)?;
    translations_infos.emit_diagnostics();
    translations_infos.rerun_if_locales_changed();
    translations_infos.generate_i18n_module_with_options(i18n_mod_directory, codegen_options)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build()?;
    i18n_build()?;
    Ok(())
}
