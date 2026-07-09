use std::collections::BTreeSet;

use crate::error::{BridgeStreamError, BridgeStreamErrorKind};
use crate::input::envelope::BridgeCommittedPatchEnvelope;

use super::member::CanonicalStreamMember;
use super::position::CanonicalStreamPosition;
use super::protocol::{AdmittedConsumerContract, ValidatedStreamProtocol};
use super::window::PlannedChangeStreamWindow;
use super::ChangeStreamDeclaration;

pub(crate) fn validate_change_stream_declaration(
    declaration: ChangeStreamDeclaration,
) -> Result<ValidatedStreamProtocol, BridgeStreamError> {
    ValidatedStreamProtocol::from_declaration(declaration)
}

pub(crate) fn resolve_consumer_contract(
    protocol: &ValidatedStreamProtocol,
) -> Result<AdmittedConsumerContract, BridgeStreamError> {
    AdmittedConsumerContract::resolve(protocol)
}

pub(crate) fn plan_change_stream_window(
    contract: &AdmittedConsumerContract,
    envelopes: Vec<BridgeCommittedPatchEnvelope>,
    ordinal_offset: usize,
) -> Result<PlannedChangeStreamWindow, BridgeStreamError> {
    if envelopes.is_empty() {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::InvalidStreamMaterial,
            "A planned change-stream window requires at least one canonical committed change envelope.",
        ));
    }

    let members: Vec<_> = envelopes
        .into_iter()
        .map(CanonicalStreamMember::from_envelope)
        .collect();
    let mut seen_member_keys = BTreeSet::new();
    for member in &members {
        if !seen_member_keys.insert(member.stream_member_identity().to_owned()) {
            return Err(BridgeStreamError::new(
                BridgeStreamErrorKind::NonIdempotentDuplicateObservation,
                "Canonical stream material contained duplicate canonical stream members within one planned window.",
            ));
        }
    }

    let first_branch = members[0].source_branch().clone();
    let first_snapshot = members[0].source_snapshot().clone();
    if contract.admitted_coalescing_family() != super::declaration::StreamCoalescingFamily::None
        && members.iter().any(|member| {
            member.source_branch() != &first_branch || member.source_snapshot() != &first_snapshot
        })
    {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::IllegalCoalescingBoundary,
            "Coalesced stream windows may not cross canonical branch or snapshot boundaries.",
        ));
    }

    let positions = members
        .iter()
        .enumerate()
        .map(|(ordinal_position, member)| {
            CanonicalStreamPosition::new(
                contract.stream_protocol_identity().clone(),
                member,
                ordinal_offset + ordinal_position,
            )
        })
        .collect();

    Ok(PlannedChangeStreamWindow::new(contract, members, positions))
}
