use crate::identity::hash_parts;

use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeSupportProfile,
};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeStateKind {
    Ready,
    Pending,
    Stale,
    Failed,
    Cancelled,
    Superseded,
    Denied,
    Unsupported,
}

impl ForgeQueryRuntimeStateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Pending => "pending",
            Self::Stale => "stale",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Superseded => "superseded",
            Self::Denied => "denied",
            Self::Unsupported => "unsupported",
        }
    }
}

impl std::fmt::Display for ForgeQueryRuntimeStateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeStateSnapshot {
    kind: ForgeQueryRuntimeStateKind,
    basis_digest: String,
    result_shape_digest: String,
    authority_lane: ForgeQueryAuthorityLane,
    explanation: String,
    state_digest: String,
}

impl ForgeQueryRuntimeStateSnapshot {
    pub fn ready(
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQueryRuntimeStateKind::Ready,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
        )
    }

    pub fn deferred(
        kind: ForgeQueryRuntimeStateKind,
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        assert!(
            kind != ForgeQueryRuntimeStateKind::Ready,
            "ready state should use ForgeQueryRuntimeStateSnapshot::ready"
        );
        Self::new(
            kind,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
        )
    }

    fn new(
        kind: ForgeQueryRuntimeStateKind,
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        let basis_digest = basis_digest.into();
        let result_shape_digest = result_shape_digest.into();
        let explanation = explanation.into();
        let state_digest = hash_parts(&[
            format!("kind:{}", kind.as_str()),
            format!("basis:{basis_digest}"),
            format!("result_shape:{result_shape_digest}"),
            format!("lane:{}", authority_lane.as_str()),
            format!("explanation:{explanation}"),
        ]);
        Self {
            kind,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
            state_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryRuntimeStateKind {
        self.kind
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn result_shape_digest(&self) -> &str {
        &self.result_shape_digest
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    pub fn state_digest(&self) -> &str {
        &self.state_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiFamilyContract {
    family: ForgeQueryRuntimeFacadeFamily,
    status: ForgeQueryRuntimeFamilySupportStatus,
    authority_lanes: Vec<ForgeQueryAuthorityLane>,
    evidence: Vec<String>,
    reason: Option<String>,
    contract_digest: String,
}

impl ForgeQueryRuntimePublicApiFamilyContract {
    fn from_support_row(row: &super::ForgeQueryRuntimeFamilySupport) -> Self {
        let family = row.family();
        let status = row.status();
        let authority_lanes = row.authority_lanes().to_vec();
        let evidence = row.evidence().to_vec();
        let reason = row.denial_reason().map(str::to_string);
        let mut parts = vec![
            format!("family:{}", family.as_str()),
            format!("status:{}", status.as_str()),
        ];
        parts.extend(
            authority_lanes
                .iter()
                .map(|lane| format!("lane:{}", lane.as_str())),
        );
        parts.extend(evidence.iter().map(|item| format!("evidence:{item}")));
        if let Some(reason) = &reason {
            parts.push(format!("reason:{reason}"));
        }
        let contract_digest = hash_parts(&parts);
        Self {
            family,
            status,
            authority_lanes,
            evidence,
            reason,
            contract_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.status
    }

    pub fn authority_lanes(&self) -> &[ForgeQueryAuthorityLane] {
        &self.authority_lanes
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiContract {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    families: Vec<ForgeQueryRuntimePublicApiFamilyContract>,
    stable_family_count: usize,
    deferred_family_count: usize,
    unsupported_family_count: usize,
    contract_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiNamingRow {
    concept: String,
    preferred_name: String,
    alternate_names: Vec<String>,
    boundary_crossing: bool,
    naming_digest: String,
}

impl ForgeQueryRuntimePublicApiNamingRow {
    fn new(
        concept: impl Into<String>,
        preferred_name: impl Into<String>,
        alternate_names: impl IntoIterator<Item = impl Into<String>>,
        boundary_crossing: bool,
    ) -> Self {
        let concept = concept.into();
        let preferred_name = preferred_name.into();
        let alternate_names = alternate_names
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let mut parts = vec![
            format!("concept:{concept}"),
            format!("preferred:{preferred_name}"),
            format!("boundary:{boundary_crossing}"),
        ];
        parts.extend(
            alternate_names
                .iter()
                .map(|name| format!("alternate:{name}")),
        );
        let naming_digest = hash_parts(&parts);
        Self {
            concept,
            preferred_name,
            alternate_names,
            boundary_crossing,
            naming_digest,
        }
    }

    pub fn concept(&self) -> &str {
        &self.concept
    }

    pub fn preferred_name(&self) -> &str {
        &self.preferred_name
    }

    pub fn alternate_names(&self) -> &[String] {
        &self.alternate_names
    }

    pub fn boundary_crossing(&self) -> bool {
        self.boundary_crossing
    }

    pub fn naming_digest(&self) -> &str {
        &self.naming_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiNamingContract {
    rows: Vec<ForgeQueryRuntimePublicApiNamingRow>,
    preferred_entrypoint_count: usize,
    alternate_name_count: usize,
    boundary_crossing_name_count: usize,
    contract_digest: String,
}

impl ForgeQueryRuntimePublicApiNamingContract {
    pub fn standard() -> Self {
        let rows = vec![
            ForgeQueryRuntimePublicApiNamingRow::new(
                "workspace",
                "workspace",
                std::iter::empty::<&str>(),
                false,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "live-view",
                "live_view",
                ["live_view_request", "declare_live_view"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "live-view-builder",
                "live_view closure",
                std::iter::empty::<&str>(),
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "computed",
                "computed",
                [
                    "computed_view",
                    "computed_definition",
                    "declare_maintained_derived_view",
                    "declare_derived_view",
                ],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "computed-builder",
                "computed closure",
                std::iter::empty::<&str>(),
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "effect",
                "effect",
                ["effect_declaration", "declare_effect"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "effect-builder",
                "effect closure",
                std::iter::empty::<&str>(),
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "preview",
                "preview",
                ["preview_with_options"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "branch",
                "branch",
                ["branch_with_options"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "insert",
                "insert",
                ["write + ForgeQueryWriteCommand::InsertAspects"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "update",
                "update",
                ["write + ForgeQueryWriteCommand::UpdateAspect"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "delete",
                "delete",
                ["write + ForgeQueryWriteCommand::Delete"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "batch",
                "batch",
                ["multiple write(...) calls in declared order"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new("intent", "intent", ["execute_intent"], true),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "effect-intent",
                "next_effect_intent",
                ["execute_next_effect_write_intent"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new("read", "read", ["read_live"], true),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "state",
                "state",
                ["snapshot", "ForgeQueryRuntimeStateSnapshot"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new("observe", "observe", ["drain_patches"], true),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "materialize",
                "materialize",
                ["read_derived"],
                true,
            ),
            ForgeQueryRuntimePublicApiNamingRow::new(
                "inspect",
                "inspect",
                std::iter::empty::<&str>(),
                true,
            ),
        ];
        let preferred_entrypoint_count = rows.len();
        let alternate_name_count = rows.iter().map(|row| row.alternate_names().len()).sum();
        let boundary_crossing_name_count =
            rows.iter().filter(|row| row.boundary_crossing()).count();
        let mut parts = vec![
            "forge_query_runtime_public_api_naming_contract_v1".to_string(),
            format!("preferred:{preferred_entrypoint_count}"),
            format!("alternate:{alternate_name_count}"),
            format!("boundary:{boundary_crossing_name_count}"),
        ];
        parts.extend(rows.iter().map(|row| row.naming_digest().to_string()));
        let contract_digest = hash_parts(&parts);
        Self {
            rows,
            preferred_entrypoint_count,
            alternate_name_count,
            boundary_crossing_name_count,
            contract_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryRuntimePublicApiNamingRow] {
        &self.rows
    }

    pub fn preferred_entrypoint_count(&self) -> usize {
        self.preferred_entrypoint_count
    }

    pub fn alternate_name_count(&self) -> usize {
        self.alternate_name_count
    }

    pub fn boundary_crossing_name_count(&self) -> usize {
        self.boundary_crossing_name_count
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn preferred_name_for(&self, concept: &str) -> Option<&str> {
        self.rows
            .iter()
            .find(|row| row.concept() == concept)
            .map(ForgeQueryRuntimePublicApiNamingRow::preferred_name)
    }
}

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
    #[allow(dead_code)]
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

impl ForgeQueryRuntimePublicApiContract {
    pub fn from_support_profile(profile: &ForgeQueryRuntimeSupportProfile) -> Self {
        let families: Vec<_> = profile
            .rows()
            .map(ForgeQueryRuntimePublicApiFamilyContract::from_support_row)
            .collect();
        let stable_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::Supported)
            .count();
        let deferred_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::DeferredDebt)
            .count();
        let unsupported_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::Unsupported)
            .count();
        let mut parts = vec![format!("posture:{}", profile.posture().as_str())];
        parts.extend(
            families
                .iter()
                .map(|family| format!("family:{}", family.contract_digest())),
        );
        parts.push(format!("stable:{stable_family_count}"));
        parts.push(format!("deferred:{deferred_family_count}"));
        parts.push(format!("unsupported:{unsupported_family_count}"));
        let contract_digest = hash_parts(&parts);
        Self {
            backend_posture: profile.posture(),
            families,
            stable_family_count,
            deferred_family_count,
            unsupported_family_count,
            contract_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn families(&self) -> &[ForgeQueryRuntimePublicApiFamilyContract] {
        &self.families
    }

    pub fn stable_family_count(&self) -> usize {
        self.stable_family_count
    }

    pub fn deferred_family_count(&self) -> usize {
        self.deferred_family_count
    }

    pub fn unsupported_family_count(&self) -> usize {
        self.unsupported_family_count
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Option<&ForgeQueryRuntimePublicApiFamilyContract> {
        self.families.iter().find(|row| row.family() == family)
    }
}
