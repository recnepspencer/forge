use serde::{Deserialize, Serialize};

use crate::classification::CiTestLane;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub(crate) enum TestProduct {
    Owner {
        package: String,
    },
    Smoke,
    Ui,
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
            filter: "wal_durability_ack",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "io_scheduling",
            filter: "access_policy",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "layout_access",
            filter: "btree_lookup_authority",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "operational_security",
            filter: "security_scope_propagation",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "physical_isolation",
            filter: "stable_read_plan_admission",
        },
        SmokeCase {
            package: "worth-store-certification",
            target: "s5_tier_movement_future_chunk_compile_fail",
            filter: "future_chunk_placeholder_boundary_misuse_does_not_compile",
        },
    ]
}
