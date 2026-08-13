use super::text_profile_qualification;
use std::sync::OnceLock;

const PROFILE_IDENTITY: &str = "worth-ui-global-text-v2";
const PROFILE_DIGEST: &str = "cec6005c5baef6d69ada9c30c02ced25b0f253f80c012784fe925e307935c3f2";
const PROFILE_PATH: &str = "workspaces/worth-ui/profiles/worth-ui-global-text-v2/manifest.toml";
const OPEN_VERSIONS: &str = "protocol=5;text-profile=worth-ui-global-text-v2;qualification=open";

pub(super) struct ProfileClaim<'a> {
    pub(super) result: &'a str,
    pub(super) identity: &'a str,
    pub(super) digest: &'a str,
    pub(super) platform_versions: &'a str,
}

pub(super) fn validate(claim: ProfileClaim<'_>) -> Result<(), String> {
    if claim.identity != PROFILE_IDENTITY {
        return Err("text profile identity drifted".to_owned());
    }
    match claim.result {
        "OPEN" => validate_open(claim),
        "PROVED" => validate_qualified(claim),
        _ => Err("text profile has an invalid result posture".to_owned()),
    }
}

fn validate_open(claim: ProfileClaim<'_>) -> Result<(), String> {
    let unprepared = claim.digest == "not-qualified" && claim.platform_versions == OPEN_VERSIONS;
    let prepared = claim.digest == PROFILE_DIGEST
        && claim.platform_versions == super::claim_contract::TEXT_PLATFORM_VERSIONS;
    if !unprepared && !prepared {
        return Err("open text profile claim pretends to be qualified".to_owned());
    }
    Ok(())
}

fn validate_qualified(claim: ProfileClaim<'_>) -> Result<(), String> {
    if claim.digest.len() != 64 || !claim.digest.bytes().all(is_lower_hex) {
        return Err("qualified text profile digest is invalid".to_owned());
    }
    if !claim.platform_versions.contains("protocol=5")
        || !claim
            .platform_versions
            .contains("text-profile=worth-ui-global-text-v2")
        || !claim.platform_versions.contains("qualification=closed")
    {
        return Err("qualified text profile versions are incomplete".to_owned());
    }
    let digest = super::source_digest::file_digest(PROFILE_PATH)
        .map_err(|_| "canonical text profile manifest is not qualified".to_owned())?;
    if claim.digest != PROFILE_DIGEST || digest != PROFILE_DIGEST {
        return Err("qualified text profile digest does not match canonical bytes".to_owned());
    }
    static QUALIFICATION: OnceLock<Result<(), String>> = OnceLock::new();
    QUALIFICATION
        .get_or_init(|| text_profile_qualification::validate_profile(PROFILE_DIGEST))
        .clone()
}

fn is_lower_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_profile_claim_is_either_unprepared_or_exactly_prepared() {
        assert!(validate(ProfileClaim {
            result: "OPEN",
            identity: PROFILE_IDENTITY,
            digest: "not-qualified",
            platform_versions: OPEN_VERSIONS,
        })
        .is_ok());
        assert!(validate(ProfileClaim {
            result: "OPEN",
            identity: PROFILE_IDENTITY,
            digest: PROFILE_DIGEST,
            platform_versions: super::super::claim_contract::TEXT_PLATFORM_VERSIONS,
        })
        .is_ok());
        assert!(validate(ProfileClaim {
            result: "OPEN",
            identity: PROFILE_IDENTITY,
            digest: "0000000000000000000000000000000000000000000000000000000000000000",
            platform_versions: super::super::claim_contract::TEXT_PLATFORM_VERSIONS,
        })
        .is_err());
    }
}
