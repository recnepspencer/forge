use std::collections::BTreeMap;

use worth_signal::facade::Aspect;
use worth_signal::facade::ChangedRegion;

use crate::boundary::errors::WORTHSignalJsError;
use crate::expression::model::SignalValue;
use crate::recipe::model::TransactionOp;

use super::super::aspects::resolve_change_aspects;
use super::super::RuntimeCore;
use super::changes::SetChange;

impl RuntimeCore {
    pub(super) fn collect_changes(
        &mut self,
        ops: &[TransactionOp],
    ) -> Result<Vec<SetChange>, WORTHSignalJsError> {
        let mut deduped = BTreeMap::<String, (SignalValue, Vec<ChangedRegion>, Vec<Aspect>)>::new();
        let mut packed = Vec::new();
        for op in ops {
            match op {
                TransactionOp::Set {
                    id,
                    value,
                    aspect,
                    aspects,
                } => {
                    deduped.insert(
                        id.clone(),
                        (
                            value.clone(),
                            Vec::new(),
                            resolve_change_aspects(*aspect, aspects.as_ref())?,
                        ),
                    );
                }
                TransactionOp::SetWithRegions {
                    id,
                    value,
                    changed_regions,
                    aspect,
                    aspects,
                } => {
                    deduped.insert(
                        id.clone(),
                        (
                            value.clone(),
                            changed_regions.clone(),
                            resolve_change_aspects(*aspect, aspects.as_ref())?,
                        ),
                    );
                }
                TransactionOp::SetMany { values } => {
                    for value in values {
                        deduped.insert(
                            value.id.clone(),
                            (
                                value.value.clone(),
                                Vec::new(),
                                resolve_change_aspects(value.aspect, value.aspects.as_ref())?,
                            ),
                        );
                    }
                }
                TransactionOp::SetManyWithRegions { values } => {
                    for value in values {
                        deduped.insert(
                            value.id.clone(),
                            (
                                value.value.clone(),
                                value.changed_regions.clone(),
                                resolve_change_aspects(value.aspect, value.aspects.as_ref())?,
                            ),
                        );
                    }
                }
                TransactionOp::SetManyKeyed { family_id, values } => {
                    for value in values {
                        let id = self.ensure_source_key(
                            family_id,
                            &value.key,
                            Some(value.value.clone()),
                        )?;
                        deduped.insert(
                            id,
                            (
                                value.value.clone(),
                                Vec::new(),
                                resolve_change_aspects(value.aspect, value.aspects.as_ref())?,
                            ),
                        );
                    }
                }
                TransactionOp::SetPackedGridRgba {
                    family_id,
                    width,
                    height,
                    rgba,
                } => {
                    let expected_len = (*width as usize) * (*height as usize) * 4;
                    if rgba.len() != expected_len {
                        return Err(WORTHSignalJsError::invalid_input(format!(
                            "packed rgba payload for `{family_id}` had {} bytes, expected {expected_len}",
                            rgba.len()
                        )));
                    }
                    let produced_aspects = self
                        .ensure_dense_rgba_grid(family_id, *width, *height)?
                        .produced_aspects
                        .clone();
                    packed.push(SetChange::DenseGridRgba {
                        family_id: family_id.clone(),
                        rgba: rgba.clone(),
                        aspects: produced_aspects,
                    });
                }
            }
        }

        let mut changes = Vec::with_capacity(deduped.len().saturating_add(packed.len()));
        for (id, (value, changed_regions, aspects)) in deduped {
            let node = self.node_for_id(&id)?;
            changes.push(SetChange::Source {
                id,
                value,
                node,
                changed_regions,
                aspects,
            });
        }
        changes.extend(packed);
        Ok(changes)
    }
}
