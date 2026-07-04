use super::family_kind::TouchedGraphParityFamilyKind;
use super::residue_classification::TouchedGraphParityResidueClassification;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TouchedGraphParityQuerySurfaceKind {
    NotQuery,
    SupportPosture,
    ConsumerResidue,
    BoundaryEnvelope,
}

impl TouchedGraphParityQuerySurfaceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotQuery => "not_query",
            Self::SupportPosture => "support_posture",
            Self::ConsumerResidue => "consumer_residue",
            Self::BoundaryEnvelope => "boundary_envelope",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchedGraphParityCoverageContributor {
    current_surface: &'static str,
    source_path: &'static str,
    upstream_authority_source: &'static str,
    selected_route_or_equivalence_source: &'static str,
    public_or_internal_consumer_kind: &'static str,
    replacement_lane: &'static str,
    selected_identity_fields_consumed: &'static [&'static str],
    query_surface_kind: TouchedGraphParityQuerySurfaceKind,
    ordinary_path_live_caller_surface: &'static str,
    ordinary_path_live_caller_path: &'static str,
}

impl TouchedGraphParityCoverageContributor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        current_surface: &'static str,
        source_path: &'static str,
        upstream_authority_source: &'static str,
        selected_route_or_equivalence_source: &'static str,
        public_or_internal_consumer_kind: &'static str,
        replacement_lane: &'static str,
        selected_identity_fields_consumed: &'static [&'static str],
        query_surface_kind: TouchedGraphParityQuerySurfaceKind,
        ordinary_path_live_caller_surface: &'static str,
        ordinary_path_live_caller_path: &'static str,
    ) -> Self {
        Self {
            current_surface,
            source_path,
            upstream_authority_source,
            selected_route_or_equivalence_source,
            public_or_internal_consumer_kind,
            replacement_lane,
            selected_identity_fields_consumed,
            query_surface_kind,
            ordinary_path_live_caller_surface,
            ordinary_path_live_caller_path,
        }
    }

    pub const fn current_surface(&self) -> &'static str {
        self.current_surface
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn upstream_authority_source(&self) -> &'static str {
        self.upstream_authority_source
    }

    pub const fn selected_route_or_equivalence_source(&self) -> &'static str {
        self.selected_route_or_equivalence_source
    }

    pub const fn public_or_internal_consumer_kind(&self) -> &'static str {
        self.public_or_internal_consumer_kind
    }

    pub const fn replacement_lane(&self) -> &'static str {
        self.replacement_lane
    }

    pub const fn selected_identity_fields_consumed(&self) -> &'static [&'static str] {
        self.selected_identity_fields_consumed
    }

    pub const fn query_surface_kind(&self) -> TouchedGraphParityQuerySurfaceKind {
        self.query_surface_kind
    }

    pub const fn ordinary_path_live_caller_surface(&self) -> &'static str {
        self.ordinary_path_live_caller_surface
    }

    pub const fn ordinary_path_live_caller_path(&self) -> &'static str {
        self.ordinary_path_live_caller_path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchedGraphParityCoverageRow {
    family_kind: TouchedGraphParityFamilyKind,
    current_owner_crate: &'static str,
    contributor: TouchedGraphParityCoverageContributor,
    ordinary_path_reachable: bool,
    residue_classification: TouchedGraphParityResidueClassification,
}

impl TouchedGraphParityCoverageRow {
    pub const fn from_contributor(
        family_kind: TouchedGraphParityFamilyKind,
        current_owner_crate: &'static str,
        contributor: TouchedGraphParityCoverageContributor,
        ordinary_path_reachable: bool,
        residue_classification: TouchedGraphParityResidueClassification,
    ) -> Self {
        Self {
            family_kind,
            current_owner_crate,
            contributor,
            ordinary_path_reachable,
            residue_classification,
        }
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_surface(&self) -> &'static str {
        self.contributor.current_surface()
    }

    pub const fn source_path(&self) -> &'static str {
        self.contributor.source_path()
    }

    pub const fn current_owner_crate(&self) -> &'static str {
        self.current_owner_crate
    }

    pub const fn upstream_authority_source(&self) -> &'static str {
        self.contributor.upstream_authority_source()
    }

    pub const fn selected_route_or_equivalence_source(&self) -> &'static str {
        self.contributor.selected_route_or_equivalence_source()
    }

    pub const fn public_or_internal_consumer_kind(&self) -> &'static str {
        self.contributor.public_or_internal_consumer_kind()
    }

    pub const fn replacement_lane(&self) -> &'static str {
        self.contributor.replacement_lane()
    }

    pub const fn selected_identity_fields_consumed(&self) -> &'static [&'static str] {
        self.contributor.selected_identity_fields_consumed()
    }

    pub const fn query_surface_kind(&self) -> TouchedGraphParityQuerySurfaceKind {
        self.contributor.query_surface_kind()
    }

    pub const fn ordinary_path_live_caller_surface(&self) -> &'static str {
        self.contributor.ordinary_path_live_caller_surface()
    }

    pub const fn ordinary_path_live_caller_path(&self) -> &'static str {
        self.contributor.ordinary_path_live_caller_path()
    }

    pub const fn ordinary_path_reachable(&self) -> bool {
        self.ordinary_path_reachable
    }

    pub const fn residue_classification(&self) -> TouchedGraphParityResidueClassification {
        self.residue_classification
    }

    pub const fn residue_posture(&self) -> &'static str {
        self.residue_classification.as_str()
    }
}
