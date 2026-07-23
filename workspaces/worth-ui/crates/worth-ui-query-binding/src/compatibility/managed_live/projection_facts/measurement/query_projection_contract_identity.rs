use worth_query::facade::foundation::{ProjectionFactKind, WorthQueryConsumedProjectionAuthority};

/// Binding-owned semantic identity for the Query consumer contract.
///
/// This identity is derived from typed contract requirements and requested
/// facts. It is not derived from Query reporting digests and cannot carry
/// projection authority by itself.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryProjectionContractIdentity(u64);

impl WorthUiQueryProjectionContractIdentity {
    pub(crate) fn from_authority(authority: &WorthQueryConsumedProjectionAuthority) -> Self {
        let contract = authority.consumer_contract();
        let requirements = contract
            .requirements()
            .fold(0x776f_7274_6800_0001, |identity, item| {
                fold_bytes(identity, item.as_str().as_bytes())
            });
        Self(
            contract
                .requested_facts()
                .fold(requirements, |identity, request| {
                    let identity = fold_bytes(identity, fact_kind_name(request.kind()).as_bytes());
                    request.field_path().map_or(identity, |path| {
                        if let Some(canonical) = path.canonical_field_path() {
                            return canonical.fields().iter().fold(
                                identity,
                                |identity, field| {
                                    fold_bytes(identity, field.as_str().as_bytes())
                                },
                            );
                        }
                        let identity = path.native_aspect_key().map_or(identity, |aspect| {
                            fold_bytes(identity, aspect.as_str().as_bytes())
                        });
                        path.native_field_key().map_or(identity, |field| {
                                fold_bytes(identity, field.as_str().as_bytes())
                            })
                    })
                }),
        )
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

fn fact_kind_name(kind: ProjectionFactKind) -> &'static str {
    match kind {
        ProjectionFactKind::EntityIdentity => "entity-identity",
        ProjectionFactKind::ViewLocalIdentity => "view-local-identity",
        ProjectionFactKind::TargetIdentity => "target-identity",
        ProjectionFactKind::SourceReference => "source-reference",
        ProjectionFactKind::EffectContinuity => "effect-continuity",
        ProjectionFactKind::Membership => "membership",
        ProjectionFactKind::RelationEndpoint => "relation-endpoint",
        ProjectionFactKind::DisplayField => "display-field",
        ProjectionFactKind::DerivedField => "derived-field",
    }
}

fn fold_bytes(mut identity: u64, bytes: &[u8]) -> u64 {
    identity = identity.rotate_left(13) ^ bytes.len() as u64;
    for byte in bytes {
        identity ^= u64::from(*byte);
        identity = identity.wrapping_mul(0x100_0000_01b3);
    }
    identity
}
