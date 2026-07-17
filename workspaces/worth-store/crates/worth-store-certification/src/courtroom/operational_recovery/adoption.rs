#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedArtifactCategory {
    Decision,
    Failure,
    Comparison,
    Support,
    Provenance,
    Receipt,
    Lineage,
    Performance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofFoundationalAdoptionRow {
    store_source: &'static str,
    category: SharedArtifactCategory,
    shared_role: &'static str,
    basis_loss: &'static str,
    freshness_loss: &'static str,
    comparison_contract: &'static str,
    reverse_flow_compile_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S10ProofFoundationalAdoptionMatrix {
    rows: Vec<ProofFoundationalAdoptionRow>,
}

impl S10ProofFoundationalAdoptionMatrix {
    pub(super) fn canonical() -> Self {
        let rows = [
            row(
                "OperationalAuditRecord",
                SharedArtifactCategory::Decision,
                "canonical operational transition fact",
                "owner authority and private control payload are omitted",
                "point-in-time durable control generation",
                "operation id + sequence + transition identity",
                "shared_audit_record_cannot_construct_control_record",
            ),
            row(
                "OperationalEvidenceExport",
                SharedArtifactCategory::Support,
                "terminal support projection",
                "readmission capability and owner receipts are omitted",
                "complete audit terminal identity",
                "canonical export identity",
                "terminal_export_cannot_construct_authorization",
            ),
            row(
                "OperationalAuditSupportPayload",
                SharedArtifactCategory::Support,
                "profiled support widening payload",
                "only explicitly materialized descriptive surfaces survive",
                "requested/admitted/materialized profile identity",
                "Foundational support materialization plan",
                "support_bundle_cannot_construct_operational_authority",
            ),
            row(
                "ForensicCustodyRecord",
                SharedArtifactCategory::Provenance,
                "observation-only custody provenance",
                "custody assertions remain evidence, never restore authority",
                "acquisition clock provenance",
                "forensic bundle and custody identities",
                "forensic_bundle_cannot_construct_restore_source",
            ),
            row(
                "ReplicaPromotionReceipt",
                SharedArtifactCategory::Lineage,
                "promoted lineage observation",
                "external fence capability is retained only by Store",
                "promoted epoch and durable target identity",
                "promotion receipt + fence + publication chain",
                "lineage_projection_cannot_mint_primary_serve_lease",
            ),
            row(
                "OperationalCounterReceipt",
                SharedArtifactCategory::Performance,
                "operation-bound execution measurement",
                "counters carry no execution or readmission authority",
                "exact session identity and execution phase",
                "counter receipt + scenario scale identity",
                "counter_receipt_cannot_construct_execution_ready_plan",
            ),
        ];
        Self {
            rows: rows.to_vec(),
        }
    }

    pub fn rows(&self) -> &[ProofFoundationalAdoptionRow] {
        &self.rows
    }

    pub(super) fn evidence_identity(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-proof-foundational-adoption-v1");
        for row in &self.rows {
            update_text(&mut digest, row.store_source);
            digest.update([category_tag(row.category)]);
            update_text(&mut digest, row.shared_role);
            update_text(&mut digest, row.basis_loss);
            update_text(&mut digest, row.freshness_loss);
            update_text(&mut digest, row.comparison_contract);
            update_text(&mut digest, row.reverse_flow_compile_gate);
        }
        digest.finalize().into()
    }
}

fn update_text(digest: &mut sha2::Sha256, value: &str) {
    use sha2::Digest;
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value.as_bytes());
}

const fn category_tag(category: SharedArtifactCategory) -> u8 {
    match category {
        SharedArtifactCategory::Decision => 1,
        SharedArtifactCategory::Failure => 2,
        SharedArtifactCategory::Comparison => 3,
        SharedArtifactCategory::Support => 4,
        SharedArtifactCategory::Provenance => 5,
        SharedArtifactCategory::Receipt => 6,
        SharedArtifactCategory::Lineage => 7,
        SharedArtifactCategory::Performance => 8,
    }
}

const fn row(
    store_source: &'static str,
    category: SharedArtifactCategory,
    shared_role: &'static str,
    basis_loss: &'static str,
    freshness_loss: &'static str,
    comparison_contract: &'static str,
    reverse_flow_compile_gate: &'static str,
) -> ProofFoundationalAdoptionRow {
    ProofFoundationalAdoptionRow {
        store_source,
        category,
        shared_role,
        basis_loss,
        freshness_loss,
        comparison_contract,
        reverse_flow_compile_gate,
    }
}

impl ProofFoundationalAdoptionRow {
    pub const fn store_source(self) -> &'static str {
        self.store_source
    }
    pub const fn category(self) -> SharedArtifactCategory {
        self.category
    }
    pub const fn shared_role(self) -> &'static str {
        self.shared_role
    }
    pub const fn basis_loss(self) -> &'static str {
        self.basis_loss
    }
    pub const fn freshness_loss(self) -> &'static str {
        self.freshness_loss
    }
    pub const fn comparison_contract(self) -> &'static str {
        self.comparison_contract
    }
    pub const fn reverse_flow_compile_gate(self) -> &'static str {
        self.reverse_flow_compile_gate
    }
}
