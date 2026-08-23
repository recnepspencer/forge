#[path = "build/bidi_tables.rs"]
mod bidi_tables;
#[path = "build/line_tables.rs"]
mod line_tables;
#[path = "build/profile_tables.rs"]
mod profile_tables;

use std::{env, fs, path::PathBuf};

fn main() {
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let profile = crate_dir.join("../../profiles/worth-ui-global-text-v2/unicode/ucd");
    let output_dir = PathBuf::from(env::var_os("OUT_DIR").expect("output directory"));
    let bidi_inputs = [
        profile.join("extracted/DerivedBidiClass.txt"),
        profile.join("BidiBrackets.txt"),
    ];
    let line_inputs = [
        profile.join("LineBreak.txt"),
        profile.join("UnicodeData.txt"),
        profile.join("EastAsianWidth.txt"),
        profile.join("emoji/emoji-data.txt"),
    ];
    let profile_inputs = [
        crate_dir.join("../../profiles/worth-ui-global-text-v2/manifest.toml"),
        crate_dir.join("../../profiles/worth-ui-global-text-v2/unicode/emoji/emoji-test.txt"),
    ];
    for input in bidi_inputs
        .iter()
        .chain(&line_inputs)
        .chain(&profile_inputs)
    {
        println!("cargo:rerun-if-changed={}", input.display());
    }
    fs::write(
        output_dir.join("unicode_17_bidi.rs"),
        bidi_tables::generate(&bidi_inputs[0], &bidi_inputs[1]),
    )
    .expect("write generated bidi tables");
    fs::write(
        output_dir.join("unicode_17_line.rs"),
        line_tables::generate(&line_inputs),
    )
    .expect("write generated line tables");
    fs::write(
        output_dir.join("global_text_profile.rs"),
        profile_tables::generate(&profile_inputs[0], &profile_inputs[1]),
    )
    .expect("write generated text profile tables");
}
