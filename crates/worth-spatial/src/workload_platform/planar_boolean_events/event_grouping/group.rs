#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEventGroupKind {
    CoincidentPoint,
    CoincidentInterval,
}

impl PlanarBooleanEventGroupKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::CoincidentPoint => "coincident-point",
            Self::CoincidentInterval => "coincident-interval",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanEventGroup {
    group_identity: String,
    kind: PlanarBooleanEventGroupKind,
    canonical_group_key: String,
    point_event_identities: Vec<String>,
    interval_event_identities: Vec<String>,
    segment_pair_identities: Vec<String>,
    participating_carrier_identities: Vec<String>,
    source_endpoint_identities: Vec<String>,
    source_interval_identities: Vec<String>,
}

impl PlanarBooleanEventGroup {
    pub(crate) fn new(input: PlanarBooleanEventGroupInput) -> Self {
        Self {
            group_identity: input.group_identity,
            kind: input.kind,
            canonical_group_key: input.canonical_group_key,
            point_event_identities: input.point_event_identities,
            interval_event_identities: input.interval_event_identities,
            segment_pair_identities: input.segment_pair_identities,
            participating_carrier_identities: input.participating_carrier_identities,
            source_endpoint_identities: input.source_endpoint_identities,
            source_interval_identities: input.source_interval_identities,
        }
    }

    pub fn group_identity(&self) -> &str {
        &self.group_identity
    }

    pub fn kind(&self) -> PlanarBooleanEventGroupKind {
        self.kind
    }

    pub fn canonical_group_key(&self) -> &str {
        &self.canonical_group_key
    }

    pub fn point_event_identities(&self) -> &[String] {
        &self.point_event_identities
    }

    pub fn interval_event_identities(&self) -> &[String] {
        &self.interval_event_identities
    }

    pub fn segment_pair_identities(&self) -> &[String] {
        &self.segment_pair_identities
    }

    pub fn participating_carrier_identities(&self) -> &[String] {
        &self.participating_carrier_identities
    }

    pub fn source_endpoint_identities(&self) -> &[String] {
        &self.source_endpoint_identities
    }

    pub fn source_interval_identities(&self) -> &[String] {
        &self.source_interval_identities
    }
}

pub(crate) struct PlanarBooleanEventGroupInput {
    pub(crate) group_identity: String,
    pub(crate) kind: PlanarBooleanEventGroupKind,
    pub(crate) canonical_group_key: String,
    pub(crate) point_event_identities: Vec<String>,
    pub(crate) interval_event_identities: Vec<String>,
    pub(crate) segment_pair_identities: Vec<String>,
    pub(crate) participating_carrier_identities: Vec<String>,
    pub(crate) source_endpoint_identities: Vec<String>,
    pub(crate) source_interval_identities: Vec<String>,
}
