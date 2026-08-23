use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const NOTO_RELEASE: &str = "https://raw.githubusercontent.com/notofonts/notofonts.github.io/noto-monthly-release-2026.08.01/fonts/LICENSE";
const CJK_RELEASE: &str = "https://raw.githubusercontent.com/notofonts/noto-cjk/Sans2.004/LICENSE";
const EMOJI_RELEASE: &str =
    "https://raw.githubusercontent.com/googlefonts/noto-emoji/v2.051/LICENSE";
const LAST_RESORT_RELEASE: &str =
    "https://raw.githubusercontent.com/unicode-org/last-resort-font/17.000/LICENSE";
const UNICODE_RELEASE: &str = "https://www.unicode.org/license.txt";
const UNICODE_LICENSE: &str = "licenses/unicode-data-license.txt";
const NOTO_FONT_PREFIX: &str = "https://raw.githubusercontent.com/notofonts/notofonts.github.io/noto-monthly-release-2026.08.01/fonts/";
const CJK_FONT_SOURCE: &str = "https://raw.githubusercontent.com/notofonts/noto-cjk/Sans2.004/Sans/Variable/OTC/NotoSansCJK-VF.otf.ttc";
const EMOJI_FONT_SOURCE: &str =
    "https://raw.githubusercontent.com/googlefonts/noto-emoji/v2.051/fonts/NotoColorEmoji.ttf";
const LAST_RESORT_FONT_SOURCE: &str = "https://github.com/unicode-org/last-resort-font/releases/download/17.000/LastResort-Regular.ttf";

pub(super) fn validate(root: &Path, manifest: &toml::Value) -> Result<(), String> {
    let inventory = fs::read(root.join(super::string(manifest, "artifact_inventory")?))
        .map_err(|error| error.to_string())?;
    let inventory = super::parse_toml(&inventory)?;
    let licenses = license_records(&inventory)?;
    if licenses.len() != 5 || licenses.get(UNICODE_LICENSE).copied() != Some(UNICODE_RELEASE) {
        return Err("qualified license inventory or Unicode provenance drifted".to_owned());
    }
    for face in super::array(manifest, "face")? {
        let face = face.as_table().ok_or("face is not a table")?;
        let source = super::table_string(face, "source")?;
        let license = super::table_string(face, "license")?;
        let expected = expected_license(source)?;
        if license != expected.0 || licenses.get(license).copied() != Some(expected.1) {
            return Err(format!("font license provenance drifted: {license}"));
        }
    }
    validate_license_text(root, &licenses)
}

fn license_records(inventory: &toml::Value) -> Result<BTreeMap<&str, &str>, String> {
    super::array(inventory, "artifact")?
        .iter()
        .filter_map(toml::Value::as_table)
        .filter(|record| {
            super::table_string(record, "path").is_ok_and(|path| path.starts_with("licenses/"))
        })
        .map(|record| {
            Ok((
                super::table_string(record, "path")?,
                super::table_string(record, "source")?,
            ))
        })
        .collect()
}

fn expected_license(source: &str) -> Result<(&'static str, &'static str), String> {
    if source.starts_with(NOTO_FONT_PREFIX) {
        Ok(("licenses/noto-ofl-1.1.txt", NOTO_RELEASE))
    } else if source == CJK_FONT_SOURCE {
        Ok(("licenses/noto-cjk-ofl-1.1.txt", CJK_RELEASE))
    } else if source == EMOJI_FONT_SOURCE {
        Ok(("licenses/noto-emoji-ofl-1.1.txt", EMOJI_RELEASE))
    } else if source == LAST_RESORT_FONT_SOURCE {
        Ok(("licenses/last-resort-ofl-1.1.txt", LAST_RESORT_RELEASE))
    } else {
        Err(format!("font source has no qualified license: {source}"))
    }
}

fn validate_license_text(root: &Path, licenses: &BTreeMap<&str, &str>) -> Result<(), String> {
    for path in licenses
        .keys()
        .filter(|path| !path.contains("unicode-data"))
    {
        let text = fs::read_to_string(root.join(path)).map_err(|error| error.to_string())?;
        if !text.contains("SIL OPEN FONT LICENSE Version 1.1") {
            return Err(format!("font license text drifted: {path}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_text_profile_rejects_wrong_font_release_provenance() {
        let stale = EMOJI_FONT_SOURCE.replace("v2.051", "v2.050");
        assert!(expected_license(&stale).is_err());
        assert_eq!(
            expected_license(EMOJI_FONT_SOURCE),
            Ok(("licenses/noto-emoji-ofl-1.1.txt", EMOJI_RELEASE))
        );
    }
}
