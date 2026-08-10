use crate::failure::{StoreError, StoreErrorKind};
use crate::SupportActionDurableRecord;

use super::super::super::super::records::StoreState;
use super::super::super::helpers::{deserialize_json, sqlite_error};
use rusqlite::Connection;

pub(super) fn load_action_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT action_id, artifact_id, action_origin, publication_state, payload_json \
             FROM subscription_support_action_records ORDER BY action_id",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                ActionIndexedProjection {
                    artifact_id: row.get(1)?,
                    action_origin: row.get(2)?,
                    publication_state: row.get(3)?,
                },
                deserialize_json::<SupportActionDurableRecord>(row.get(4)?)?,
            ))
        })
        .map_err(sqlite_error)?;
    for row in rows {
        let (action_id, indexed, record) = row.map_err(sqlite_error)?;
        if record.action_id().as_str() != action_id {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support action record key does not match payload action id",
            ));
        }
        indexed.verify_matches(&record)?;
        state
            .subscription_support_action_records
            .insert(action_id, record);
    }
    Ok(())
}

struct ActionIndexedProjection {
    artifact_id: String,
    action_origin: String,
    publication_state: String,
}

impl ActionIndexedProjection {
    fn verify_matches(&self, record: &SupportActionDurableRecord) -> Result<(), StoreError> {
        let action_origin = format!("{:?}", record.action_origin());
        let publication_state = format!("{:?}", record.publication_state());
        if self.artifact_id != record.artifact_id().as_str() {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support action artifact id index projection does not match payload",
            ));
        }
        if self.action_origin != action_origin {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support action origin index projection does not match payload",
            ));
        }
        if self.publication_state != publication_state {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support action state index projection does not match payload",
            ));
        }
        Ok(())
    }
}
