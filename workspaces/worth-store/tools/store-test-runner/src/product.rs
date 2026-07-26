use serde::{Deserialize, Serialize};

use crate::classification::CiTestLane;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum CourtroomSelection {
    A,
    B,
    C,
}

impl CourtroomSelection {
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            _ => Err(format!("unknown courtroom `{value}`; expected `a|b|c`")),
        }
    }

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
            Self::C => "c",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub(crate) enum TestProduct {
    Owner {
        package: String,
    },
    Smoke,
    Ui,
    Mutants,
    Courtrooms {
        courtroom: CourtroomSelection,
    },
    Ci {
        lane: CiTestLane,
        shard: Option<(usize, usize)>,
    },
}

impl TestProduct {
    pub(crate) fn name(&self) -> String {
        match self {
            Self::Owner { package } => format!("owner:{package}"),
            Self::Smoke => "smoke".into(),
            Self::Ui => "ui".into(),
            Self::Mutants => "mutants".into(),
            Self::Courtrooms { courtroom } => format!("courtroom:{}", courtroom.label()),
            Self::Ci { lane, shard } => match shard {
                Some((index, count)) => format!("ci:{lane}:{index}/{count}"),
                None => format!("ci:{lane}"),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct SmokeCase {
    pub(crate) package: &'static str,
    pub(crate) target: &'static str,
    pub(crate) filter: &'static str,
}

pub(crate) fn smoke_cases() -> &'static [SmokeCase] {
    &[
        SmokeCase {
            package: "worth-store-certification",
            target: "durability_recovery",
            filter: "wal_durability_ack::crash_after_wal_durability_before_ack_is_unacknowledged_replayable_posture",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "io_scheduling",
            filter: "access_policy::certification_materializes_successful_execution_receipts",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "layout_access",
            filter: "btree_lookup_authority::ordinary_runtime_selects_and_executes_separator_directed_page_lookup",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "operational_security",
            filter: "security_scope_propagation::stable_read_scope_survives_protection_observation_and_decode_entry",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "physical_isolation",
            filter: "stable_read_plan_admission::proof_bearing_read_plan_admits_before_execution_handle",
        },
        SmokeCase {
            package: "worth-store",
            target: "physical_record_journeys",
            filter: "baseline_admission::empty_bootstrap_create_and_reopen_converge",
        },
        SmokeCase {
            package: "worth-store",
            target: "physical_record_journeys",
            filter: "scan_journeys::scan_batch_widths_converge_to_one_physical_sequence",
        },
        SmokeCase {
            package: "worth-store",
            target: "physical_record_journeys",
            filter: "publication_faults::possible_catalog_cutover_is_typed_indeterminate_and_close_adds_no_publication_effect",
        },
    ]
}
