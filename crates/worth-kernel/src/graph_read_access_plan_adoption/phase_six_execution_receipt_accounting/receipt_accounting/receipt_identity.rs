use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessReceiptIdentity {
    source_kind: String,
    source_projection_digest: String,
    read_family_identity_digest: Option<String>,
    requirement_row_digest: Option<String>,
    query_family_digest_seed: String,
    query_posture: String,
    touched_authority_digest: String,
    execution_basis: String,
    policy_narrowing_digest: Option<String>,
    plan_digest: Option<String>,
    receipt_digest: Option<String>,
    execution_counter_digest: Option<String>,
    identity_digest: String,
}

pub(crate) struct WorthGraphReadAccessReceiptIdentityInput {
    pub source_kind: String,
    pub source_projection_digest: String,
    pub read_family_identity_digest: Option<String>,
    pub requirement_row_digest: Option<String>,
    pub query_family_digest_seed: String,
    pub query_posture: String,
    pub touched_authority_digest: String,
    pub execution_basis: String,
    pub policy_narrowing_digest: Option<String>,
    pub plan_digest: Option<String>,
    pub receipt_digest: Option<String>,
    pub execution_counter_digest: Option<String>,
}

impl WorthGraphReadAccessReceiptIdentity {
    pub(crate) fn from_input(input: WorthGraphReadAccessReceiptIdentityInput) -> Self {
        let identity_digest = stable_digest(&[
            "worth_graph_read_access_receipt_identity_v1".to_string(),
            format!("source_kind:{}", input.source_kind),
            format!("source:{}", input.source_projection_digest),
            format!(
                "read_family:{}",
                input
                    .read_family_identity_digest
                    .as_deref()
                    .unwrap_or("none")
            ),
            format!(
                "requirement:{}",
                input.requirement_row_digest.as_deref().unwrap_or("none")
            ),
            format!("query_family:{}", input.query_family_digest_seed),
            format!("posture:{}", input.query_posture),
            format!("touched_authority:{}", input.touched_authority_digest),
            format!("basis:{}", input.execution_basis),
            format!(
                "policy_narrowing:{}",
                input.policy_narrowing_digest.as_deref().unwrap_or("none")
            ),
            format!("plan:{}", input.plan_digest.as_deref().unwrap_or("none")),
            format!(
                "receipt:{}",
                input.receipt_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "counters:{}",
                input.execution_counter_digest.as_deref().unwrap_or("none")
            ),
        ]);
        Self {
            source_kind: input.source_kind,
            source_projection_digest: input.source_projection_digest,
            read_family_identity_digest: input.read_family_identity_digest,
            requirement_row_digest: input.requirement_row_digest,
            query_family_digest_seed: input.query_family_digest_seed,
            query_posture: input.query_posture,
            touched_authority_digest: input.touched_authority_digest,
            execution_basis: input.execution_basis,
            policy_narrowing_digest: input.policy_narrowing_digest,
            plan_digest: input.plan_digest,
            receipt_digest: input.receipt_digest,
            execution_counter_digest: input.execution_counter_digest,
            identity_digest,
        }
    }

    pub fn source_kind(&self) -> &str {
        &self.source_kind
    }

    pub fn source_projection_digest(&self) -> &str {
        &self.source_projection_digest
    }

    pub fn read_family_identity_digest(&self) -> Option<&str> {
        self.read_family_identity_digest.as_deref()
    }

    pub fn requirement_row_digest(&self) -> Option<&str> {
        self.requirement_row_digest.as_deref()
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn query_posture(&self) -> &str {
        &self.query_posture
    }

    pub fn touched_authority_digest(&self) -> &str {
        &self.touched_authority_digest
    }

    pub fn execution_basis(&self) -> &str {
        &self.execution_basis
    }

    pub fn policy_narrowing_digest(&self) -> Option<&str> {
        self.policy_narrowing_digest.as_deref()
    }

    pub fn plan_digest(&self) -> Option<&str> {
        self.plan_digest.as_deref()
    }

    pub fn receipt_digest(&self) -> Option<&str> {
        self.receipt_digest.as_deref()
    }

    pub fn execution_counter_digest(&self) -> Option<&str> {
        self.execution_counter_digest.as_deref()
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}
