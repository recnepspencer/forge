#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiScrollVirtualizationPosture {
    NonVirtualized,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiScrollOffsetAllocationPosture {
    ProjectedInteractionOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAdmittedScrollOwnedContract {
    pub(super) neighborhood_identity: crate::evidence::UiAllocationNeighborhoodIdentity,
    pub(super) graph_generation: crate::graph::UiGraphGeneration,
    pub(super) measurement_basis_identity_digest: u64,
    pub(super) measurement_basis_generation: crate::evidence::UiMeasurementBasisGeneration,
    pub(super) coordinate_ownership: crate::evidence::UiLayoutOperatorContractIdentity,
    pub(super) planning_input_identity_digest: u64,
    pub(super) source_evidence_identity_digest: u64,
    pub(super) source_generation_digest: u64,
    pub(super) source: UiAdmittedScrollExtentSource,
    pub(super) virtualization: UiScrollVirtualizationPosture,
    pub(super) offset_allocation: UiScrollOffsetAllocationPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiAdmittedScrollExtentSource {
    HostViewport {
        witness: crate::evidence::UiHostMeasurementAuthorityWitness,
    },
    QueryContent(UiAdmittedScrollQuerySource),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAdmittedScrollQuerySource {
    pub(super) query_authority: worth_ui_query_binding::WorthUiQueryAuthorityHandle,
    pub(super) authority_index_key: worth_ui_query_binding::WorthUiQueryAuthorityIndexKey,
    pub(super) target: crate::graph::UiGraphNodeIdentity,
}

impl UiAdmittedScrollOwnedContract {
    pub(crate) fn identity_digest(&self) -> u64 {
        self.neighborhood_identity.identity_digest()
            ^ self.measurement_basis_identity_digest.rotate_left(11)
            ^ self.measurement_basis_generation.raw().rotate_left(17)
            ^ self.coordinate_ownership.identity_digest().rotate_left(19)
            ^ self.planning_input_identity_digest.rotate_left(23)
            ^ self.source_evidence_identity_digest.rotate_left(37)
            ^ self.source_generation_digest.rotate_left(47)
            ^ self.source.identity_digest()
    }

    pub(crate) fn source(&self) -> &UiAdmittedScrollExtentSource {
        &self.source
    }

    pub(crate) fn neighborhood_identity(
        &self,
    ) -> &crate::evidence::UiAllocationNeighborhoodIdentity {
        &self.neighborhood_identity
    }

    pub(crate) fn graph_generation(&self) -> crate::graph::UiGraphGeneration {
        self.graph_generation
    }

    pub fn virtualization(&self) -> UiScrollVirtualizationPosture {
        self.virtualization
    }

    pub fn offset_allocation(&self) -> UiScrollOffsetAllocationPosture {
        self.offset_allocation
    }
}

impl std::hash::Hash for UiAdmittedScrollExtentSource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.identity_digest());
    }
}

impl UiAdmittedScrollExtentSource {
    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::HostViewport { witness } => {
                crate::declaration::stable_text_digest("worth-ui.scroll-source.host")
                    ^ witness.identity_digest().rotate_left(17)
            }
            Self::QueryContent(source) => {
                crate::declaration::stable_text_digest("worth-ui.scroll-source.query")
                    ^ source.identity_digest().rotate_left(17)
            }
        }
    }
}

impl UiAdmittedScrollQuerySource {
    pub(crate) fn query_authority(&self) -> &worth_ui_query_binding::WorthUiQueryAuthorityHandle {
        &self.query_authority
    }

    pub(crate) fn authority_index_key(
        &self,
    ) -> &worth_ui_query_binding::WorthUiQueryAuthorityIndexKey {
        &self.authority_index_key
    }

    pub(crate) fn target(&self) -> crate::graph::UiGraphNodeIdentity {
        self.target
    }

    pub(super) fn identity_digest(&self) -> u64 {
        crate::declaration::stable_text_digest("worth-ui.scroll-query-source")
            ^ crate::declaration::stable_text_digest(
                self.authority_index_key.projection_source_identity(),
            )
            .rotate_left(7)
            ^ crate::declaration::stable_text_digest(self.authority_index_key.query_basis_digest())
                .rotate_left(17)
            ^ crate::declaration::stable_text_digest(
                self.authority_index_key.projection_contract_digest(),
            )
            .rotate_left(29)
            ^ crate::declaration::stable_text_digest(
                self.authority_index_key
                    .projection_consumption_receipt_digest(),
            )
            .rotate_left(41)
            ^ self.target.digest().rotate_left(53)
    }
}
