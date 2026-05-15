use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionTopologyDomain {
    Families,
    Eligibility,
    Decisions,
    Handoffs,
    Trace,
    Dx,
    Support,
    Certification,
}

impl ForgeQueryIntentAdmissionTopologyDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Families => "intent_admission/families",
            Self::Eligibility => "intent_admission/eligibility",
            Self::Decisions => "intent_admission/decisions",
            Self::Handoffs => "intent_admission/handoffs",
            Self::Trace => "intent_admission/trace",
            Self::Dx => "intent_admission/dx",
            Self::Support => "intent_admission/support",
            Self::Certification => "intent_admission/certification",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionTopologyAuditRow {
    domain: ForgeQueryIntentAdmissionTopologyDomain,
    ownership_detail: &'static str,
    row_digest: String,
}

impl ForgeQueryIntentAdmissionTopologyAuditRow {
    pub fn domain(&self) -> ForgeQueryIntentAdmissionTopologyDomain {
        self.domain
    }

    pub fn ownership_detail(&self) -> &'static str {
        self.ownership_detail
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionTopologyAudit {
    rows: Vec<ForgeQueryIntentAdmissionTopologyAuditRow>,
    topology_digest: String,
}

impl ForgeQueryIntentAdmissionTopologyAudit {
    pub(crate) fn new() -> Self {
        let rows = vec![
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Families,
                "owns family inventory labels and raw/common/advanced admission surfaces",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Eligibility,
                "owns shared eligibility vocabulary and runtime-family authority adapters",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Decisions,
                "owns admitted advisory and violation decision artifacts and plan shaping",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Handoffs,
                "owns typed execution handoffs bindings and provenance-link sealing",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Trace,
                "owns decision-trace rows evidence posture and offline-readable envelopes",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Dx,
                "owns common-path advanced-path and consumer-facing transcript surfaces",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Support,
                "owns executable support matrices deferred posture and covered-entrypoint inventory",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Certification,
                "owns audits bundles representative rows slope reports and compile-fail manifests",
            ),
        ];
        let topology_digest = hash_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            topology_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryIntentAdmissionTopologyAuditRow] {
        &self.rows
    }

    pub fn topology_digest(&self) -> &str {
        &self.topology_digest
    }
}

fn topology_row(
    domain: ForgeQueryIntentAdmissionTopologyDomain,
    ownership_detail: &'static str,
) -> ForgeQueryIntentAdmissionTopologyAuditRow {
    ForgeQueryIntentAdmissionTopologyAuditRow {
        domain,
        ownership_detail,
        row_digest: hash_parts(&[
            "forge_query_intent_admission_topology_audit_row_v1".to_string(),
            format!("domain:{}", domain.as_str()),
            format!("ownership:{ownership_detail}"),
        ]),
    }
}
