use crate::identity::hash_parts;
use crate::intent_admission::{
    INTENT_ADMISSION_CERTIFICATION_CHILD_MODULES, INTENT_ADMISSION_CERTIFICATION_EXPORTED_SURFACE,
    INTENT_ADMISSION_CERTIFICATION_MODULE_ROOT, INTENT_ADMISSION_DECISIONS_CHILD_MODULES,
    INTENT_ADMISSION_DECISIONS_EXPORTED_SURFACE, INTENT_ADMISSION_DECISIONS_MODULE_ROOT,
    INTENT_ADMISSION_DX_CHILD_MODULES, INTENT_ADMISSION_DX_EXPORTED_SURFACE,
    INTENT_ADMISSION_DX_MODULE_ROOT, INTENT_ADMISSION_ELIGIBILITY_CHILD_MODULES,
    INTENT_ADMISSION_ELIGIBILITY_EXPORTED_SURFACE, INTENT_ADMISSION_ELIGIBILITY_MODULE_ROOT,
    INTENT_ADMISSION_FAMILIES_CHILD_MODULES, INTENT_ADMISSION_FAMILIES_EXPORTED_SURFACE,
    INTENT_ADMISSION_FAMILIES_MODULE_ROOT, INTENT_ADMISSION_HANDOFFS_CHILD_MODULES,
    INTENT_ADMISSION_HANDOFFS_EXPORTED_SURFACE, INTENT_ADMISSION_HANDOFFS_MODULE_ROOT,
    INTENT_ADMISSION_SUPPORT_CHILD_MODULES, INTENT_ADMISSION_SUPPORT_EXPORTED_SURFACE,
    INTENT_ADMISSION_SUPPORT_MODULE_ROOT, INTENT_ADMISSION_TRACE_CHILD_MODULES,
    INTENT_ADMISSION_TRACE_EXPORTED_SURFACE, INTENT_ADMISSION_TRACE_MODULE_ROOT,
};

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
    module_root: &'static str,
    child_modules: &'static [&'static str],
    exported_surface: &'static [&'static str],
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

    pub fn module_root(&self) -> &'static str {
        self.module_root
    }

    pub fn child_modules(&self) -> &[&'static str] {
        self.child_modules
    }

    pub fn exported_surface(&self) -> &[&'static str] {
        self.exported_surface
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
                INTENT_ADMISSION_FAMILIES_MODULE_ROOT,
                INTENT_ADMISSION_FAMILIES_CHILD_MODULES,
                INTENT_ADMISSION_FAMILIES_EXPORTED_SURFACE,
                "owns family inventory labels and raw/common/advanced admission surfaces",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Eligibility,
                INTENT_ADMISSION_ELIGIBILITY_MODULE_ROOT,
                INTENT_ADMISSION_ELIGIBILITY_CHILD_MODULES,
                INTENT_ADMISSION_ELIGIBILITY_EXPORTED_SURFACE,
                "owns shared eligibility vocabulary and runtime-family authority adapters",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Decisions,
                INTENT_ADMISSION_DECISIONS_MODULE_ROOT,
                INTENT_ADMISSION_DECISIONS_CHILD_MODULES,
                INTENT_ADMISSION_DECISIONS_EXPORTED_SURFACE,
                "owns admitted advisory and violation decision artifacts and plan shaping",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Handoffs,
                INTENT_ADMISSION_HANDOFFS_MODULE_ROOT,
                INTENT_ADMISSION_HANDOFFS_CHILD_MODULES,
                INTENT_ADMISSION_HANDOFFS_EXPORTED_SURFACE,
                "owns typed execution handoffs execution bindings and provenance-link sealing",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Trace,
                INTENT_ADMISSION_TRACE_MODULE_ROOT,
                INTENT_ADMISSION_TRACE_CHILD_MODULES,
                INTENT_ADMISSION_TRACE_EXPORTED_SURFACE,
                "owns decision-trace rows evidence posture and offline-readable envelopes",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Dx,
                INTENT_ADMISSION_DX_MODULE_ROOT,
                INTENT_ADMISSION_DX_CHILD_MODULES,
                INTENT_ADMISSION_DX_EXPORTED_SURFACE,
                "owns common-path advanced-path and consumer-facing transcript surfaces",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Support,
                INTENT_ADMISSION_SUPPORT_MODULE_ROOT,
                INTENT_ADMISSION_SUPPORT_CHILD_MODULES,
                INTENT_ADMISSION_SUPPORT_EXPORTED_SURFACE,
                "owns executable support matrices deferred posture and covered-entrypoint inventory",
            ),
            topology_row(
                ForgeQueryIntentAdmissionTopologyDomain::Certification,
                INTENT_ADMISSION_CERTIFICATION_MODULE_ROOT,
                INTENT_ADMISSION_CERTIFICATION_CHILD_MODULES,
                INTENT_ADMISSION_CERTIFICATION_EXPORTED_SURFACE,
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
    module_root: &'static str,
    child_modules: &'static [&'static str],
    exported_surface: &'static [&'static str],
    ownership_detail: &'static str,
) -> ForgeQueryIntentAdmissionTopologyAuditRow {
    ForgeQueryIntentAdmissionTopologyAuditRow {
        domain,
        module_root,
        child_modules,
        exported_surface,
        ownership_detail,
        row_digest: hash_parts(&[
            "forge_query_intent_admission_topology_audit_row_v1".to_string(),
            format!("domain:{}", domain.as_str()),
            format!("module-root:{module_root}"),
            format!("children:{}", child_modules.join("|")),
            format!("exports:{}", exported_surface.join("|")),
            format!("ownership:{ownership_detail}"),
        ]),
    }
}
