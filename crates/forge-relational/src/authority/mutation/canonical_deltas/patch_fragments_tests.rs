use forge_foundational::facade::{
    AspectValue as FoundationalAspectValue, InternedString as FoundationalInternedString,
};
use forge_foundational::{
    aspects, AspectContractRevision, AspectIdentity, AspectKey as FoundationalAspectKey,
    ScalarAspectType,
};

use crate::identity::data::{EntityId, KindId, PartitionId};
use crate::publication::patch::data::{
    AspectKey, PatchDetail, PublishedAuthoritativePatchOperation, PublishedAuthoritativePatchValue,
    RecordStructuralChange,
};
use crate::schema::data::AspectPlanRevision;
use crate::transactions::data::RecordRef;

use super::data::{
    CanonicalAspectDeltaEvidence, CanonicalRecordAspectDelta, EvaluatedAspectBinding,
    LifecycleTransitionClass,
};

#[test]
fn scalar_changed_binding_materializes_foundational_whole_aspect_set() {
    let fragment = scalar_delta("name", "alice")
        .into_foundational_patch_fragment(PatchDetail::DenseBitset(Vec::new()))
        .expect("foundational scalar patch fragment");

    assert_eq!(fragment.patch.whole_aspect_sets().count(), 1);
    assert_eq!(fragment.patch.whole_aspect_clears().count(), 0);
    let published_record = fragment.published_record();
    assert_eq!(
        published_record.authoritative_changed_aspects(),
        crate::publication::patch::data::CanonicalAspectSet::new([AspectKey::new("name").unwrap()])
    );
    assert!(matches!(
        published_record.authoritative_patch.operations.as_slice(),
        [PublishedAuthoritativePatchOperation::WholeAspectSet {
            aspect_key,
            value: PublishedAuthoritativePatchValue::Scalar(value),
        }] if aspect_key == &AspectKey::new("name").unwrap()
            && value == &FoundationalAspectValue::String(
                FoundationalInternedString::from("alice")
            )
    ));
}

fn scalar_delta(key: &str, value: &str) -> CanonicalRecordAspectDelta {
    CanonicalRecordAspectDelta {
        target: RecordRef::Entity(EntityId::new(PartitionId(1), 0, 1)),
        kind_id: KindId(7),
        plan_revision: AspectPlanRevision(1),
        structural_change: RecordStructuralChange::Updated,
        changed_aspects: crate::publication::patch::data::CanonicalAspectSet::new([
            AspectKey::new(key).unwrap(),
        ]),
        evaluated_bindings: smallvec::smallvec![
            EvaluatedAspectBinding {
                aspect_key: AspectKey::new(key).unwrap(),
                contract: aspects()
                    .contract()
                    .for_key(
                        FoundationalAspectKey::new(key)
                            .expect("foundational key for scalar binding"),
                    )
                    .identified_by(AspectIdentity(9))
                    .at_revision(AspectContractRevision(1))
                    .scalar(ScalarAspectType::String),
                changed: true,
                aspect_shape: forge_foundational::AspectShape::Scalar(ScalarAspectType::String),
                evidence: CanonicalAspectDeltaEvidence::ScalarAspectValueTransition {
                    locator: authoritative_value_locator(&AspectKey::new(key).unwrap()),
                    old_present: true,
                    new_present: true,
                    old_value: Some(FoundationalAspectValue::String(
                        FoundationalInternedString::from("before"),
                    )),
                    new_value: Some(FoundationalAspectValue::String(
                        FoundationalInternedString::from(value),
                    )),
                },
            },
            EvaluatedAspectBinding {
                aspect_key: AspectKey::new("target").unwrap(),
                contract: aspects()
                    .contract()
                    .for_key(
                        FoundationalAspectKey::new("target")
                            .expect("foundational key for endpoint binding"),
                    )
                    .identified_by(AspectIdentity(10))
                    .at_revision(AspectContractRevision(1))
                    .reference_entity(),
                changed: false,
                aspect_shape: forge_foundational::AspectShape::Reference(
                    forge_foundational::ReferenceAspectType::Entity,
                ),
                evidence: CanonicalAspectDeltaEvidence::EndpointIdentity {
                    locator: authoritative_value_locator(&AspectKey::new("target").unwrap()),
                    old: None,
                    new: None,
                },
            },
            EvaluatedAspectBinding {
                aspect_key: AspectKey::new("lifecycle").unwrap(),
                contract: aspects()
                    .contract()
                    .for_key(
                        FoundationalAspectKey::new("lifecycle")
                            .expect("foundational key for lifecycle binding"),
                    )
                    .identified_by(AspectIdentity(11))
                    .at_revision(AspectContractRevision(1))
                    .scalar(ScalarAspectType::String),
                changed: false,
                aspect_shape: forge_foundational::AspectShape::Scalar(ScalarAspectType::String),
                evidence: CanonicalAspectDeltaEvidence::Lifecycle {
                    locator: authoritative_value_locator(&AspectKey::new("lifecycle").unwrap()),
                    transition: LifecycleTransitionClass::NoTransition,
                },
            }
        ],
        contains_opaque_aspect: false,
    }
}

fn authoritative_value_locator(
    aspect_key: &AspectKey,
) -> forge_foundational::facade::AspectValueLocator {
    forge_foundational::facade::AspectValueLocator::whole_aspect(
        forge_foundational::facade::AspectLocator::new(
            forge_foundational::facade::LocatorAuthority::Authoritative,
            aspect_key.clone(),
        ),
    )
}
