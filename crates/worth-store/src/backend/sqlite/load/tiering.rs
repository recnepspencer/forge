use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::{
    StoreState, TierRecallCompletionState, TierRecallRecord, TierResidencyRecord,
    TierTransferRecord,
};
use super::super::helpers::sqlite_error;

pub(super) fn load_tiering(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    load_tier_residency_records(connection, state)?;
    load_tier_transfer_records(connection, state)?;
    load_tier_recall_records(connection, state)?;
    Ok(())
}

fn load_tier_residency_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "
            SELECT artifact_key, artifact_family, canonical_residence, canonical_replica_locator, verification_label
            FROM tier_residency_records
            ORDER BY artifact_key
            ",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            let artifact_family_label = row.get::<_, String>(1)?;
            let artifact_family = crate::PlacementArtifactFamily::from_label(
                &artifact_family_label,
            )
            .ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(format!(
                        "unknown placement artifact family `{artifact_family_label}`"
                    ))),
                )
            })?;
            let residence_label = row.get::<_, String>(2)?;
            let canonical_residence = crate::TierResidenceClass::from_label(&residence_label)
                .ok_or_else(|| {
                    rusqlite::Error::FromSqlConversionFailure(
                        2,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::other(format!(
                            "unknown tier residence `{residence_label}`"
                        ))),
                    )
                })?;

            Ok(TierResidencyRecord {
                artifact_key: row.get(0)?,
                artifact_family,
                canonical_residence,
                canonical_replica_locator: row.get(3)?,
                verification_label: row.get(4)?,
            })
        })
        .map_err(sqlite_error)?;

    for row in rows {
        let record = row.map_err(sqlite_error)?;
        state
            .tier_residency_records
            .insert(record.artifact_key.clone(), record);
    }
    Ok(())
}

fn load_tier_recall_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "
            SELECT coalescing_key, artifact_family, scope_class, scope_key, execution_origin,
                   artifact_key, recall_cost_class, amplification_budget, completion_state
            FROM tier_recall_records
            ORDER BY coalescing_key
            ",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            let artifact_family_label = row.get::<_, String>(1)?;
            let artifact_family = crate::PlacementArtifactFamily::from_label(
                &artifact_family_label,
            )
            .ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(format!(
                        "unknown placement artifact family `{artifact_family_label}`"
                    ))),
                )
            })?;
            let scope_class_label = row.get::<_, String>(2)?;
            let scope_class = crate::PlacementObservationScopeClass::from_label(&scope_class_label)
                .ok_or_else(|| {
                    rusqlite::Error::FromSqlConversionFailure(
                        2,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::other(format!(
                            "unknown placement observation scope `{scope_class_label}`"
                        ))),
                    )
                })?;
            let execution_origin_label = row.get::<_, String>(4)?;
            let execution_origin = crate::PlacementExecutionOrigin::from_label(
                &execution_origin_label,
            )
            .ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    4,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(format!(
                        "unknown placement execution origin `{execution_origin_label}`"
                    ))),
                )
            })?;
            let recall_cost_label = row.get::<_, String>(6)?;
            let recall_cost_class = crate::RecallCostClass::from_label(&recall_cost_label)
                .ok_or_else(|| {
                    rusqlite::Error::FromSqlConversionFailure(
                        6,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::other(format!(
                            "unknown recall cost class `{recall_cost_label}`"
                        ))),
                    )
                })?;
            let amplification_budget_label = row.get::<_, String>(7)?;
            let amplification_budget =
                crate::RecallAmplificationBudget::from_label(&amplification_budget_label)
                    .ok_or_else(|| {
                        rusqlite::Error::FromSqlConversionFailure(
                            7,
                            rusqlite::types::Type::Text,
                            Box::new(std::io::Error::other(format!(
                        "unknown recall amplification budget `{amplification_budget_label}`"
                    ))),
                        )
                    })?;
            let completion_state_label = row.get::<_, String>(8)?;
            let completion_state = TierRecallCompletionState::from_label(&completion_state_label)
                .ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    8,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(format!(
                        "unknown tier recall completion state `{completion_state_label}`"
                    ))),
                )
            })?;

            Ok(TierRecallRecord {
                coalescing_key: row.get(0)?,
                artifact_family,
                scope_class,
                scope_key: row.get(3)?,
                execution_origin,
                artifact_key: row.get(5)?,
                recall_cost_class,
                amplification_budget,
                completion_state,
            })
        })
        .map_err(sqlite_error)?;

    for row in rows {
        let record = row.map_err(sqlite_error)?;
        state
            .tier_recall_records
            .insert(record.coalescing_key.clone(), record);
    }
    Ok(())
}

fn load_tier_transfer_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "
            SELECT artifact_key, artifact_family, source_residence, target_residence, execution_origin,
                   source_replica_locator, transferred_replica_locator, verification_label, cutover_completed
            FROM tier_transfer_records
            ORDER BY artifact_key
            ",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            let artifact_family_label = row.get::<_, String>(1)?;
            let artifact_family = crate::PlacementArtifactFamily::from_label(
                &artifact_family_label,
            )
            .ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(format!(
                        "unknown placement artifact family `{artifact_family_label}`"
                    ))),
                )
            })?;
            let source_residence_label = row.get::<_, String>(2)?;
            let source_residence = crate::TierResidenceClass::from_label(&source_residence_label)
                .ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(format!(
                        "unknown source tier residence `{source_residence_label}`"
                    ))),
                )
            })?;
            let target_residence_label = row.get::<_, String>(3)?;
            let target_residence = crate::TierResidenceClass::from_label(&target_residence_label)
                .ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    3,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(format!(
                        "unknown target tier residence `{target_residence_label}`"
                    ))),
                )
            })?;
            let execution_origin_label = row.get::<_, String>(4)?;
            let execution_origin = crate::PlacementExecutionOrigin::from_label(
                &execution_origin_label,
            )
            .ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    4,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(format!(
                        "unknown placement execution origin `{execution_origin_label}`"
                    ))),
                )
            })?;

            Ok(TierTransferRecord {
                artifact_key: row.get(0)?,
                artifact_family,
                source_residence,
                target_residence,
                execution_origin,
                source_replica_locator: row.get(5)?,
                transferred_replica_locator: row.get(6)?,
                verification_label: row.get(7)?,
                cutover_completed: row.get::<_, i64>(8)? != 0,
            })
        })
        .map_err(sqlite_error)?;

    for row in rows {
        let record = row.map_err(sqlite_error)?;
        state
            .tier_transfer_records
            .insert(record.artifact_key.clone(), record);
    }
    Ok(())
}
