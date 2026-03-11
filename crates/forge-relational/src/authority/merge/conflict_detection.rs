use std::collections::BTreeSet;

use crate::diagnostics::data::DiagnosticCode;
use crate::transactions::data::{CommitConflict, ExistingRecordTarget, TransactionIntent};

pub(crate) fn detect_conflicting_updates(
    intents: &[TransactionIntent],
) -> Result<(), CommitConflict> {
    let mut seen_updates = BTreeSet::new();
    for intent in intents {
        if let Some(target) = intent.existing_record_target() {
            if !seen_updates.insert(target) {
                let detail = match target {
                    ExistingRecordTarget::Entity(entity_id) => {
                        format!("conflicting entity intent for slot {}", entity_id.local_slot.0)
                    }
                    ExistingRecordTarget::Relation(relation_id) => {
                        format!("conflicting relation intent for slot {}", relation_id.local_slot.0)
                    }
                };
                return Err(CommitConflict {
                    code: DiagnosticCode::ConflictingIntent,
                    detail,
                });
            }
        }
    }

    let mut seen_relation_creates = BTreeSet::new();
    for intent in intents {
        let mut identities = Vec::new();
        intent.collect_relation_identities(&mut identities);
        for identity in identities {
            if !seen_relation_creates.insert(identity) {
                return Err(CommitConflict {
                    code: DiagnosticCode::DuplicateRelationIdentity,
                    detail: "duplicate relation identity in merged plan".to_string(),
                });
            }
        }
    }
    Ok(())
}
