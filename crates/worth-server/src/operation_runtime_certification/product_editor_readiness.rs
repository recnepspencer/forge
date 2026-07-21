use super::{
    WorthServerProductIdempotentRetryCertificationProof,
    WorthServerProductMutationCertificationProof,
    WorthServerProductPressureShapeCertificationProof,
    WorthServerProductRouteParityCertificationProof,
    WorthServerProductSharedReadCertificationProof,
    WorthServerProductStaleApplyDenialCertificationProof,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthServerEditorLikeOperationFixture {
    shared_read_certification: Option<WorthServerProductSharedReadCertificationProof>,
    mutation_certification: Option<WorthServerProductMutationCertificationProof>,
    route_parity: Option<WorthServerProductRouteParityCertificationProof>,
    pressure_shape: Option<WorthServerProductPressureShapeCertificationProof>,
    stale_apply_denial: Option<WorthServerProductStaleApplyDenialCertificationProof>,
    idempotent_retry: Option<WorthServerProductIdempotentRetryCertificationProof>,
}

impl WorthServerEditorLikeOperationFixture {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_shared_read_certification(
        mut self,
        proof: WorthServerProductSharedReadCertificationProof,
    ) -> Self {
        self.shared_read_certification = Some(proof);
        self
    }

    pub fn with_mutation_certification(
        mut self,
        proof: WorthServerProductMutationCertificationProof,
    ) -> Self {
        self.mutation_certification = Some(proof);
        self
    }

    pub fn with_route_parity(
        mut self,
        proof: WorthServerProductRouteParityCertificationProof,
    ) -> Self {
        self.route_parity = Some(proof);
        self
    }

    pub fn with_pressure_shape(
        mut self,
        proof: WorthServerProductPressureShapeCertificationProof,
    ) -> Self {
        self.pressure_shape = Some(proof);
        self
    }

    pub fn with_stale_apply_denial(
        mut self,
        proof: WorthServerProductStaleApplyDenialCertificationProof,
    ) -> Self {
        self.stale_apply_denial = Some(proof);
        self
    }

    pub fn with_idempotent_retry(
        mut self,
        proof: WorthServerProductIdempotentRetryCertificationProof,
    ) -> Self {
        self.idempotent_retry = Some(proof);
        self
    }

    pub fn canonical_digest(&self) -> String {
        format!(
            "product-editor-like-fixture-v3|shared-read={}|mutation={}|route={}|shape={}|stale={}|retry={}",
            digest_label(self.shared_read_certification.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.mutation_certification.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.route_parity.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.pressure_shape.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.stale_apply_denial.as_ref().map(|proof| proof.canonical_digest())),
            digest_label(self.idempotent_retry.as_ref().map(|proof| proof.canonical_digest())),
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
        if self.idempotent_retry.is_none() {
            labels.push("idempotent-retry");
        }
        labels
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductEditorReadinessCertification {
    fixture: WorthServerEditorLikeOperationFixture,
    missing_proof_labels: Vec<String>,
    ready: bool,
    canonical_digest: String,
}

impl WorthServerProductEditorReadinessCertification {
    pub(crate) fn new(fixture: WorthServerEditorLikeOperationFixture) -> Self {
        let missing_proof_labels = fixture
            .missing_proof_labels()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let ready = missing_proof_labels.is_empty();
        let canonical_digest = format!(
            "worth-server-product-editor-readiness-v2|fixture={}|ready={}|missing={}",
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

    pub fn fixture(&self) -> &WorthServerEditorLikeOperationFixture {
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
