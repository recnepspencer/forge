use crate::failure::{StoreError, StoreErrorKind};
use crate::SupportMaintenanceDescriptorRecord;

use super::super::super::super::records::StoreState;
use super::super::super::helpers::{deserialize_json, sqlite_error};
use rusqlite::Connection;

pub(super) fn load_maintenance_descriptor_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT record_key, family_id, support_role, maintenance_key, declaration_id, \
             descriptor_digest, payload_json \
             FROM subscription_support_maintenance_descriptor_records ORDER BY record_key",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                MaintenanceDescriptorIndexedProjection {
                    family_id: row.get(1)?,
                    support_role: row.get(2)?,
                    maintenance_key: row.get(3)?,
                    declaration_id: row.get(4)?,
                    descriptor_digest: row.get(5)?,
                },
                deserialize_json::<SupportMaintenanceDescriptorRecord>(row.get(6)?)?,
            ))
        })
        .map_err(sqlite_error)?;
    for row in rows {
        let (record_key, indexed, record) = row.map_err(sqlite_error)?;
        if record.record_key() != record_key {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support maintenance descriptor record key does not match payload",
            ));
        }
        indexed.verify_matches(&record)?;
        state
            .subscription_support_maintenance_descriptor_records
            .insert(record_key, record);
    }
    Ok(())
}

struct MaintenanceDescriptorIndexedProjection {
    family_id: String,
    support_role: String,
    maintenance_key: String,
    declaration_id: String,
    descriptor_digest: String,
}

impl MaintenanceDescriptorIndexedProjection {
    fn verify_matches(
        &self,
        record: &SupportMaintenanceDescriptorRecord,
    ) -> Result<(), StoreError> {
        let support_role = format!("{:?}", record.support_role());
        let expected = [
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
            (
                "maintenance key",
                self.maintenance_key.as_str(),
                record.maintenance_key(),
            ),
            (
                "declaration id",
                self.declaration_id.as_str(),
                record.declaration_id(),
            ),
            (
                "descriptor digest",
                self.descriptor_digest.as_str(),
                record.descriptor_digest(),
            ),
        ];
        for (label, indexed, payload) in expected {
            if indexed != payload {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    format!(
                        "sqlite subscription-support maintenance descriptor {label} index projection does not match payload"
                    ),
                ));
            }
        }
        Ok(())
    }
}
