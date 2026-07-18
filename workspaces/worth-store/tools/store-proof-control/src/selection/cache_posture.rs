use std::collections::BTreeSet;

use super::{ProofExecutionUnit, StoreBuildProfileIdentity};

pub(super) fn cache_posture(units: &[ProofExecutionUnit]) -> String {
    let profiles: BTreeSet<_> = units.iter().map(|unit| unit.build_profile).collect();
    if profiles == BTreeSet::from([StoreBuildProfileIdentity::LocalTest]) {
        "local-test; incremental=true; clean-or-warm target root admitted".to_owned()
    } else if profiles == BTreeSet::from([StoreBuildProfileIdentity::CiTest]) {
        "ci-test; incremental=false; evidence validity is independent of local incremental state"
            .to_owned()
    } else {
        format!(
            "mixed declared profiles [{}]; cache identity is profile-bound",
            profiles
                .iter()
                .map(|profile| profile.cargo_profile())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
