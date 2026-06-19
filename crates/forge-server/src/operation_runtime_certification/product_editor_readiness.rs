use super::{
    ForgeServerProductIdempotentReplayCertificationProof,
    ForgeServerProductMutationCertificationProof,
    ForgeServerProductPressureShapeCertificationProof,
    ForgeServerProductRouteParityCertificationProof,
    ForgeServerProductSharedReadCertificationProof,
    ForgeServerProductStaleApplyDenialCertificationProof,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerEditorLikeOperationFixture {
    shared_read_certification: Option<ForgeServerProductSharedReadCertificationProof>,
    mutation_certification: Option<ForgeServerProductMutationCertificationProof>,
    route_parity: Option<ForgeServerProductRouteParityCertificationProof>,
    pressure_shape: Option<ForgeServerProductPressureShapeCertificationProof>,
    stale_apply_denial: Option<ForgeServerProductStaleApplyDenialCertificationProof>,
    idempotent_replay: Option<ForgeServerProductIdempotentReplayCertificationProof>,
}

impl ForgeServerEditorLikeOperationFixture {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_shared_read_certification(
        mut self,
        proof: ForgeServerProductSharedReadCertificationProof,
    ) -> Self {
        self.shared_read_certification = Some(proof);
        self
    }

    pub fn with_mutation_certification(
        mut self,
        proof: ForgeServerProductMutationCertificationProof,
    ) -> Self {
        self.mutation_certification = Some(proof);
        self
    }

    pub fn with_route_parity(
        mut self,
        proof: ForgeServerProductRouteParityCertificationProof,
    ) -> Self {
        self.route_parity = Some(proof);
        self
    }

    pub fn with_pressure_shape(
        mut self,
        proof: ForgeServerProductPressureShapeCertificationProof,
    ) -> Self {
        self.pressure_shape = Some(proof);
        self
    }

    pub fn with_stale_apply_denial(
        mut self,
        proof: ForgeServerProductStaleApplyDenialCertificationProof,
    ) -> Self {
        self.stale_apply_denial = Some(proof);
        self
    }

    pub fn with_idempotent_replay(
        mut self,
        proof: ForgeServerProductIdempotentReplayCertificationProof,
    ) -> Self {
        self.idempotent_replay = Some(proof);
        self
    }

    pub fn canonical_digest(&self) -> String {
        format!(
            "product-editor-like-fixture-v2|shared-read={}|mutation={}|route={}|shape={}|stale={}|replay={}",
            digest_label(self.shared_read_certification.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.mutation_certification.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.route_parity.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.pressure_shape.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.stale_apply_denial.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.idempotent_replay.as_ref().map(|proof| proof.canonical_digest())),
        )
    }

    pub fn missing_proof_labels(&self) -> Vec<&'static str> {
        let mut labels = Vec::new();
        if self.shared_read_certification.is_none() {
            labels.push("shared-read-certification");
        }
        if self.mutation_certification.is_none() {
            labels.push("mutation-certification");
        }
        if self.route_parity.is_none() {
            labels.push("route-parity");
        }
        if self.pressure_shape.is_none() {
            labels.push("pressure-shape");
        }
        if self.stale_apply_denial.is_none() {
            labels.push("stale-apply-denial");
        }
        if self.idempotent_replay.is_none() {
            labels.push("idempotent-replay");
        }
        labels
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductEditorReadinessCertification {
    fixture: ForgeServerEditorLikeOperationFixture,
    missing_proof_labels: Vec<String>,
    ready: bool,
    canonical_digest: String,
}

impl ForgeServerProductEditorReadinessCertification {
    pub(crate) fn new(fixture: ForgeServerEditorLikeOperationFixture) -> Self {
        let missing_proof_labels = fixture
            .missing_proof_labels()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let ready = missing_proof_labels.is_empty();
        let canonical_digest = format!(
            "forge-server-product-editor-readiness-v2|fixture={}|ready={}|missing={}",
            fixture.canonical_digest(),
            ready,
            missing_proof_labels.join(",")
        );
        Self {
            fixture,
            missing_proof_labels,
            ready,
            canonical_digest,
        }
    }

    pub fn fixture(&self) -> &ForgeServerEditorLikeOperationFixture {
        &self.fixture
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn missing_proof_labels(&self) -> &[String] {
        &self.missing_proof_labels
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn digest_label(value: Option<&str>) -> &str {
    value.unwrap_or("missing")
}
