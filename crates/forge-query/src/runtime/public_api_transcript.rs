#[cfg(test)]
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiTranscriptEvidence {
    transcript_family: String,
    support_contract_digest: String,
    state_digest: String,
    live_surface_digest: String,
    computed_surface_digest: String,
    effect_surface_digest: String,
    intent_receipt_digest: String,
    inspection_digest: String,
    unsupported_neighbor_denial_digests: Vec<String>,
    delivery_residue_count: usize,
    authority_lane_digest: String,
    meaningful_assertion_count: usize,
    transcript_digest: String,
}

impl ForgeQueryRuntimePublicApiTranscriptEvidence {
    #[cfg(test)]
    pub(crate) fn new(
        transcript_family: impl Into<String>,
        support_contract_digest: impl Into<String>,
        state_digest: impl Into<String>,
        live_surface_digest: impl Into<String>,
        computed_surface_digest: impl Into<String>,
        effect_surface_digest: impl Into<String>,
        intent_receipt_digest: impl Into<String>,
        inspection_digest: impl Into<String>,
        unsupported_neighbor_denial_digests: impl IntoIterator<Item = impl Into<String>>,
        delivery_residue_count: usize,
        authority_lane_digest: impl Into<String>,
        meaningful_assertion_count: usize,
    ) -> Self {
        let transcript_family = transcript_family.into();
        let support_contract_digest = support_contract_digest.into();
        let state_digest = state_digest.into();
        let live_surface_digest = live_surface_digest.into();
        let computed_surface_digest = computed_surface_digest.into();
        let effect_surface_digest = effect_surface_digest.into();
        let intent_receipt_digest = intent_receipt_digest.into();
        let inspection_digest = inspection_digest.into();
        let unsupported_neighbor_denial_digests = unsupported_neighbor_denial_digests
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let authority_lane_digest = authority_lane_digest.into();
        assert!(
            !unsupported_neighbor_denial_digests.is_empty(),
            "runtime public API transcript evidence must prove at least one unsupported neighbor denial"
        );
        let mut parts = vec![
            "forge_query_runtime_public_api_transcript_evidence_v1".to_string(),
            format!("family:{transcript_family}"),
            format!("support:{support_contract_digest}"),
            format!("state:{state_digest}"),
            format!("live:{live_surface_digest}"),
            format!("computed:{computed_surface_digest}"),
            format!("effect:{effect_surface_digest}"),
            format!("intent:{intent_receipt_digest}"),
            format!("inspection:{inspection_digest}"),
            format!("residue:{delivery_residue_count}"),
            format!("lane:{authority_lane_digest}"),
            format!("assertions:{meaningful_assertion_count}"),
        ];
        parts.extend(
            unsupported_neighbor_denial_digests
                .iter()
                .map(|digest| format!("denial:{digest}")),
        );
        let transcript_digest = hash_parts(&parts);
        Self {
            transcript_family,
            support_contract_digest,
            state_digest,
            live_surface_digest,
            computed_surface_digest,
            effect_surface_digest,
            intent_receipt_digest,
            inspection_digest,
            unsupported_neighbor_denial_digests,
            delivery_residue_count,
            authority_lane_digest,
            meaningful_assertion_count,
            transcript_digest,
        }
    }

    pub fn transcript_family(&self) -> &str {
        &self.transcript_family
    }

    pub fn support_contract_digest(&self) -> &str {
        &self.support_contract_digest
    }

    pub fn state_digest(&self) -> &str {
        &self.state_digest
    }

    pub fn live_surface_digest(&self) -> &str {
        &self.live_surface_digest
    }

    pub fn computed_surface_digest(&self) -> &str {
        &self.computed_surface_digest
    }

    pub fn effect_surface_digest(&self) -> &str {
        &self.effect_surface_digest
    }

    pub fn intent_receipt_digest(&self) -> &str {
        &self.intent_receipt_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn unsupported_neighbor_denial_digests(&self) -> &[String] {
        &self.unsupported_neighbor_denial_digests
    }

    pub fn delivery_residue_count(&self) -> usize {
        self.delivery_residue_count
    }

    pub fn authority_lane_digest(&self) -> &str {
        &self.authority_lane_digest
    }

    pub fn meaningful_assertion_count(&self) -> usize {
        self.meaningful_assertion_count
    }

    pub fn transcript_digest(&self) -> &str {
        &self.transcript_digest
    }
}
