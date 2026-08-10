use crate::failure::{StoreError, StoreErrorKind};
use crate::SupportMaintenanceDebtRecord;

use super::super::super::super::records::StoreState;
use super::super::super::helpers::{deserialize_json, sqlite_error};
use rusqlite::Connection;

pub(super) fn load_maintenance_debt_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT record_key, action_id, family_id, support_role, verdict, payload_json \
             FROM subscription_support_maintenance_debt_records ORDER BY record_key",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                MaintenanceDebtIndexedProjection {
                    action_id: row.get(1)?,
                    family_id: row.get(2)?,
                    support_role: row.get(3)?,
                    verdict: row.get(4)?,
                },
                deserialize_json::<SupportMaintenanceDebtRecord>(row.get(5)?)?,
            ))
        })
        .map_err(sqlite_error)?;
    for row in rows {
        let (record_key, indexed, record) = row.map_err(sqlite_error)?;
        if record.record_key() != record_key {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support maintenance debt record key does not match payload",
            ));
        }
        indexed.verify_matches(&record)?;
        state
            .subscription_support_maintenance_debt_records
            .insert(record_key, record);
    }
    Ok(())
}

struct MaintenanceDebtIndexedProjection {
    action_id: String,
    family_id: String,
    support_role: String,
    verdict: String,
}

impl MaintenanceDebtIndexedProjection {
    fn verify_matches(&self, record: &SupportMaintenanceDebtRecord) -> Result<(), StoreError> {
        let support_role = format!("{:?}", record.support_role());
        let verdict = format!("{:?}", record.verdict());
        let expected = [
            (
                "action id",
                self.action_id.as_str(),
                record.action_id().as_str(),
            ),
            (
                "family id",
                self.family_id.as_str(),
                record.family_id().as_str(),
            ),
            (
                "support role",
                self.support_role.as_str(),
                support_role.as_str(),
            ),
            ("verdict", self.verdict.as_str(), verdict.as_str()),
        ];
        for (label, indexed, payload) in expected {
            if indexed != payload {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    format!(
                        "sqlite subscription-support maintenance debt {label} index projection does not match payload"
                    ),
                ));
            }
        }
        Ok(())
    }
}
