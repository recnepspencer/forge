use crate::{
    CompactionVisibilityAction, DurabilityRecoveryAction, ImportPublicationAction,
    LeaseReclaimAction, ProtocolFamily, QuarantineReadmissionState, ReplicationAdmissionAction,
    SharedFrontierAction, SourcePrecedenceAction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolFrontierIdentity {
    Durability,
    RecoveryPrecedence,
    Visibility,
    Reachability,
    Quarantine,
    Admission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalProtocolAction {
    DurabilityRecovery(DurabilityRecoveryAction),
    RecoverySourcePrecedence(SourcePrecedenceAction),
    CompactionVisibility(CompactionVisibilityAction),
    LeaseReclaim(LeaseReclaimAction),
    QuarantineReadmission(QuarantineReadmissionState),
    ImportPublication(ImportPublicationAction),
    ReplicationAdmission(ReplicationAdmissionAction),
    SharedFrontier(SharedFrontierAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalProtocolTrace {
    protocol: ProtocolFamily,
    frontier: ProtocolFrontierIdentity,
    actions: Vec<CanonicalProtocolAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalProtocolTraceDenial {
    EmptyTrace,
    ActionFamilyMismatch,
    FrontierMismatch,
}

impl CanonicalProtocolTrace {
    pub fn admit(
        protocol: ProtocolFamily,
        frontier: ProtocolFrontierIdentity,
        actions: impl IntoIterator<Item = CanonicalProtocolAction>,
    ) -> Result<Self, CanonicalProtocolTraceDenial> {
        if !protocol_admits_frontier(protocol, frontier) {
            return Err(CanonicalProtocolTraceDenial::FrontierMismatch);
        }
        let actions = actions.into_iter().collect::<Vec<_>>();
        if actions.is_empty() {
            return Err(CanonicalProtocolTraceDenial::EmptyTrace);
        }
        if actions.iter().any(|action| action.protocol() != protocol) {
            return Err(CanonicalProtocolTraceDenial::ActionFamilyMismatch);
        }
        Ok(Self {
            protocol,
            frontier,
            actions,
        })
    }

    pub const fn protocol(&self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn frontier(&self) -> ProtocolFrontierIdentity {
        self.frontier
    }

    pub fn actions(&self) -> &[CanonicalProtocolAction] {
        &self.actions
    }
}

impl CanonicalProtocolAction {
    pub const fn protocol(self) -> ProtocolFamily {
        match self {
            Self::DurabilityRecovery(_) => ProtocolFamily::DurabilityRecovery,
            Self::RecoverySourcePrecedence(_) => ProtocolFamily::RecoverySourcePrecedence,
            Self::CompactionVisibility(_) => ProtocolFamily::CompactionVisibility,
            Self::LeaseReclaim(_) => ProtocolFamily::LeaseReclaim,
            Self::QuarantineReadmission(_) => ProtocolFamily::QuarantineReadmission,
            Self::ImportPublication(_) => ProtocolFamily::ImportPublication,
            Self::ReplicationAdmission(_) => ProtocolFamily::ReplicationAdmission,
            Self::SharedFrontier(_) => ProtocolFamily::SharedFrontiers,
        }
    }
}

const fn protocol_admits_frontier(
    protocol: ProtocolFamily,
    frontier: ProtocolFrontierIdentity,
) -> bool {
    match protocol {
        ProtocolFamily::DurabilityRecovery => {
            matches!(frontier, ProtocolFrontierIdentity::Durability)
        }
        ProtocolFamily::RecoverySourcePrecedence => {
            matches!(frontier, ProtocolFrontierIdentity::RecoveryPrecedence)
        }
        ProtocolFamily::CompactionVisibility => {
            matches!(frontier, ProtocolFrontierIdentity::Visibility)
        }
        ProtocolFamily::LeaseReclaim => matches!(frontier, ProtocolFrontierIdentity::Reachability),
        ProtocolFamily::QuarantineReadmission => {
            matches!(frontier, ProtocolFrontierIdentity::Quarantine)
        }
        ProtocolFamily::ImportPublication | ProtocolFamily::ReplicationAdmission => {
            matches!(frontier, ProtocolFrontierIdentity::Admission)
        }
        ProtocolFamily::SharedFrontiers => true,
    }
}
