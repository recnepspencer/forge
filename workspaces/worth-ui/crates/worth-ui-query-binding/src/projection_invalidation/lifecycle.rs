use worth_query::facade::runtime::WorthQueryAsyncResultTransitionBatch;

use crate::{
    UiProjectionConsumptionBudget, UiScalarProjectionFactReceipt, UiScalarProjectionWorkCounters,
};

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub enum UiScalarProjectionBatchOutcome {
    Advanced(UiScalarProjectionTransitionReceipt),
    Unchanged(UiScalarProjectionUnchangedReceipt),
}

#[derive(Debug)]
pub enum UiScalarProjectionInitialError {
    Query(Box<worth_query::facade::runtime::WorthQueryAsyncSourceBindingError>),
    UnexpectedUnchanged,
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiScalarProjectionTransitionReceipt {
    fact: UiScalarProjectionFactReceipt,
    retained_predecessor: Option<UiScalarProjectionFactReceipt>,
    posture_trace: crate::UiProjectionPostureTrace,
    work: UiScalarProjectionWorkCounters,
    suppressed_duplicate_count: usize,
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiScalarProjectionUnchangedReceipt {
    predecessor: Option<UiScalarProjectionFactReceipt>,
    suppressed_duplicate_count: usize,
}

struct ScalarBatchProgress {
    fact: UiScalarProjectionFactReceipt,
    retained_predecessor: Option<UiScalarProjectionFactReceipt>,
    postures: Vec<crate::UiProjectionTransitionPosture>,
    work: UiScalarProjectionWorkCounters,
}

impl UiScalarProjectionTransitionReceipt {
    pub fn fact(&self) -> &UiScalarProjectionFactReceipt {
        &self.fact
    }

    pub fn retained_predecessor(&self) -> Option<&UiScalarProjectionFactReceipt> {
        self.retained_predecessor.as_ref()
    }

    pub fn work(&self) -> UiScalarProjectionWorkCounters {
        self.work
    }

    pub fn posture_trace(&self) -> &crate::UiProjectionPostureTrace {
        &self.posture_trace
    }

    pub fn suppressed_duplicate_count(&self) -> usize {
        self.suppressed_duplicate_count
    }

    pub fn into_fact_and_predecessor(
        self,
    ) -> (
        UiScalarProjectionFactReceipt,
        Option<UiScalarProjectionFactReceipt>,
    ) {
        (self.fact, self.retained_predecessor)
    }
}

impl UiScalarProjectionUnchangedReceipt {
    pub fn predecessor(&self) -> Option<&UiScalarProjectionFactReceipt> {
        self.predecessor.as_ref()
    }

    pub fn suppressed_duplicate_count(&self) -> usize {
        self.suppressed_duplicate_count
    }

    pub fn into_predecessor(self) -> Option<UiScalarProjectionFactReceipt> {
        self.predecessor
    }
}

impl crate::UiScalarProjectionBinding {
    pub fn consume_initial_async_result<T>(
        &mut self,
        workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
        view: &worth_query::facade::runtime::WorthQueryLiveView<T>,
        budget: UiProjectionConsumptionBudget,
    ) -> Result<UiScalarProjectionTransitionReceipt, UiScalarProjectionInitialError> {
        let batch = workspace
            .take_bridge_async_initial_result(view)
            .map_err(|error| UiScalarProjectionInitialError::Query(Box::new(error)))?;
        match self.consume_async_result_batch(workspace, batch, None, budget) {
            UiScalarProjectionBatchOutcome::Advanced(receipt) => Ok(receipt),
            UiScalarProjectionBatchOutcome::Unchanged(_) => {
                Err(UiScalarProjectionInitialError::UnexpectedUnchanged)
            }
        }
    }

    pub fn consume_async_result_batch(
        &mut self,
        workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
        batch: WorthQueryAsyncResultTransitionBatch,
        predecessor: Option<UiScalarProjectionFactReceipt>,
        budget: UiProjectionConsumptionBudget,
    ) -> UiScalarProjectionBatchOutcome {
        if let Some(stop) =
            super::batch_validation::validate_batch(self, &batch, predecessor.as_ref())
        {
            let posture = stop.availability().transition_posture();
            return advanced(
                stop,
                predecessor,
                vec![posture],
                UiScalarProjectionWorkCounters::default(),
                batch.suppressed_duplicate_count(),
            );
        }
        if batch.states().is_empty() {
            return UiScalarProjectionBatchOutcome::Unchanged(UiScalarProjectionUnchangedReceipt {
                predecessor,
                suppressed_duplicate_count: batch.suppressed_duplicate_count(),
            });
        }

        let progress = consume_states(self, workspace, &batch, predecessor, budget);
        advanced(
            progress.fact,
            progress.retained_predecessor,
            progress.postures,
            progress.work,
            batch.suppressed_duplicate_count(),
        )
    }
}

fn consume_states(
    binding: &mut crate::UiScalarProjectionBinding,
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    batch: &WorthQueryAsyncResultTransitionBatch,
    mut predecessor: Option<UiScalarProjectionFactReceipt>,
    budget: UiProjectionConsumptionBudget,
) -> ScalarBatchProgress {
    let mut fact = None;
    let mut postures = Vec::with_capacity(batch.states().len());
    let mut work = UiScalarProjectionWorkCounters::default();
    for (index, state) in batch.states().iter().enumerate() {
        if let Some(stop) = super::batch_validation::validate_predecessor_lineage(
            binding,
            batch,
            state,
            predecessor.as_ref(),
        ) {
            fact = Some(stop);
            postures.push(
                fact.as_ref()
                    .expect("lineage stop retained")
                    .availability()
                    .transition_posture(),
            );
            break;
        }
        let transition = super::state_translation::translate_state(
            super::state_translation::ScalarStateContext {
                binding,
                workspace,
                batch,
                state,
                budget,
            },
            predecessor,
        );
        work = transition.work;
        postures.push(transition.fact.availability().transition_posture());
        if index + 1 == batch.states().len() {
            predecessor = transition.retained_predecessor;
            fact = Some(transition.fact);
        } else {
            debug_assert!(
                transition.retained_predecessor.is_none(),
                "Query multi-state batches carry the intermediate fact, not a side predecessor"
            );
            predecessor = Some(transition.fact);
        }
    }
    ScalarBatchProgress {
        fact: fact.expect("a non-empty Query batch produces one terminal fact"),
        retained_predecessor: predecessor,
        postures,
        work,
    }
}

fn advanced(
    fact: UiScalarProjectionFactReceipt,
    retained_predecessor: Option<UiScalarProjectionFactReceipt>,
    postures: Vec<crate::UiProjectionTransitionPosture>,
    work: UiScalarProjectionWorkCounters,
    suppressed_duplicate_count: usize,
) -> UiScalarProjectionBatchOutcome {
    UiScalarProjectionBatchOutcome::Advanced(UiScalarProjectionTransitionReceipt {
        fact,
        retained_predecessor,
        posture_trace: crate::UiProjectionPostureTrace::admitted(postures),
        work,
        suppressed_duplicate_count,
    })
}
