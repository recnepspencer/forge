use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnerFixtureDependency {
    pub provider: String,
    pub rationale: String,
}

pub(super) fn admitted_cross_owner_dependencies(owner: &str) -> BTreeSet<OwnerFixtureDependency> {
    match owner {
        "worth-store-blob-chunks" => admissions(&[(
            "worth-store-offline-verifier",
            "blob backup artifacts are checked through the offline verifier public contract",
        )]),
        "worth-store-buffer-pool" => admissions(&[(
            "worth-store-readiness",
            "resident-frame owner tests enter through physical-substrate readiness",
        )]),
        "worth-store-extensions" => admissions(&[
            (
                "worth-store-aspect-native",
                "extension admission is bound to native aspect identity",
            ),
            (
                "worth-store-authority",
                "extension admission requires current store authority",
            ),
            (
                "worth-store-security",
                "extension target tests exercise the public security-scope contract",
            ),
        ]),
        "worth-store-layout-indexes" => admissions(&[(
            "worth-store-aspect-native",
            "layout owner fixtures bind physical stores to native aspect identity",
        )]),
        "worth-store-offline-verifier" => admissions(&[
            (
                "worth-store-aspect-native",
                "offline observations verify aspect-bound physical artifacts",
            ),
            (
                "worth-store-authority",
                "offline hostile tests distinguish current authority from observed evidence",
            ),
        ]),
        "worth-store-operations" => admissions(&[(
            "worth-store-readiness",
            "operational owner tests exercise readiness-gated recovery admission",
        )]),
        "worth-store-physical-backend" => admissions(&[
            (
                "worth-store-aspect-native",
                "backend placement tests bind target identity to native aspects",
            ),
            (
                "worth-store-readiness",
                "backend dirty-mmap tests enter through substrate readiness",
            ),
        ]),
        "worth-store-recovery-physics" => admissions(&[(
            "worth-store-readiness",
            "page-LSN publication tests require a closed substrate readiness chain",
        )]),
        "worth-store-replication" => admissions(&[(
            "worth-store-wal",
            "replication owner tests consume the WAL record contract they replicate",
        )]),
        "worth-store-wal" => admissions(&[(
            "worth-store-aspect-native",
            "WAL security metadata tests bind records to native aspect identity",
        )]),
        _ => BTreeSet::new(),
    }
}

fn admissions(rows: &[(&str, &str)]) -> BTreeSet<OwnerFixtureDependency> {
    rows.iter()
        .map(|(provider, rationale)| OwnerFixtureDependency {
            provider: (*provider).to_owned(),
            rationale: (*rationale).to_owned(),
        })
        .collect()
}
