use super::comparison_state::{ComparisonFailure, ComparisonMismatch, ObservedSupplyChainState};
use super::expected_observation::ExpectedSupplyChainObservation;
use super::semantic_key::{EntityKey, EntityKind, SemanticPath};

pub(crate) fn compare_entities(
    expected: &ExpectedSupplyChainObservation,
    observed: &ObservedSupplyChainState,
) -> Result<(), ComparisonFailure> {
    for (key, expected_value) in &expected.entities {
        let Some(observed_value) = observed.entities.get(key) else {
            return Err(ComparisonFailure {
                mismatch: ComparisonMismatch::MissingEntity(*key),
            });
        };
        if expected_value != observed_value {
            return Err(ComparisonFailure {
                mismatch: ComparisonMismatch::EntityValue(SemanticPath::entity(*key)),
            });
        }
    }
    if let Some(key) = observed
        .entities
        .keys()
        .find(|key| !expected.entities.contains_key(key))
    {
        return Err(ComparisonFailure {
            mismatch: ComparisonMismatch::UnexpectedEntity(*key),
        });
    }
    if expected.absent_entities != observed.absent_entities {
        let key = expected
            .absent_entities
            .symmetric_difference(&observed.absent_entities)
            .next()
            .copied()
            .unwrap_or(EntityKey::new(EntityKind::Port, u32::MAX));
        return Err(ComparisonFailure {
            mismatch: ComparisonMismatch::EntityAbsence(key),
        });
    }
    Ok(())
}
