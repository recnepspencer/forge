use std::collections::{BTreeMap, BTreeSet};

use crate::declaration::UiAspectName;
use crate::fact_contract::UiProducedFact;
use crate::graph::{
    UiGraphFactConsumerIdentity, UiGraphFactConsumerKey, UiGraphFactIndexBasis,
    UiGraphFactIndexEntry, UiGraphFactLookupDenial, UiGraphFactLookupReceipt,
};
use crate::runtime::observation::UiClassifiedChange;
use crate::runtime::rebind::{UiRebindBudgetInput, UiRebindLimit};

use super::{
    UiAffectedConsumer, UiAffectedFactLookup, UiAffectedScopeBasis, UiAffectedScopeCost,
    UiAffectedScopeCostInput, UiAffectedScopeDenial, UiAffectedScopeGeneration,
    UiResolvedAffectedScope, UiResolvedAffectedScopeInput,
};

pub(crate) struct UiAffectedScopeResolver;

struct ConsumerAccumulator {
    predecessor: Option<UiGraphFactConsumerIdentity>,
    candidate: Option<UiGraphFactConsumerIdentity>,
    aspects: BTreeSet<UiAspectName>,
}

struct FinishScopeInput {
    classification: crate::runtime::observation::UiChangeClassificationBasis,
    facts: Box<[UiProducedFact]>,
    source_succession: Option<crate::runtime::observation::UiAuthoredSourceSuccession>,
    predecessor_graph: UiGraphFactIndexBasis,
    candidate_generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    candidate_graph: UiGraphFactIndexBasis,
    lookups: Vec<UiAffectedFactLookup>,
    consumers: BTreeMap<UiGraphFactConsumerKey, ConsumerAccumulator>,
    aspects: BTreeSet<UiAspectName>,
}

struct DualGenerationLookupInput<'world> {
    fact_ordinal: usize,
    fact: &'world UiProducedFact,
    predecessor:
        &'world crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    candidate:
        &'world crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    predecessor_basis: UiGraphFactIndexBasis,
    candidate_basis: UiGraphFactIndexBasis,
}

struct ScopeLookupInput<'world> {
    facts: &'world [UiProducedFact],
    predecessor:
        &'world crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    candidate:
        &'world crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    predecessor_basis: UiGraphFactIndexBasis,
    candidate_basis: UiGraphFactIndexBasis,
    budget: UiRebindBudgetInput,
}

struct ScopeAccumulation {
    lookups: Vec<UiAffectedFactLookup>,
    consumers: BTreeMap<UiGraphFactConsumerKey, ConsumerAccumulator>,
    aspects: BTreeSet<UiAspectName>,
}

impl UiAffectedScopeResolver {
    pub(crate) fn resolve(
        change: UiClassifiedChange,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        predecessor: &crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationAuthority,
    ) -> Result<UiResolvedAffectedScope, UiAffectedScopeDenial> {
        let (classification, facts, source_succession) = change.into_parts();
        require_current_basis(&classification, session, predecessor)?;
        let candidate = source_succession
            .as_ref()
            .map_or(predecessor, |succession| succession.successor_authority());
        let predecessor_graph = UiGraphFactIndexBasis::from_generation(
            predecessor.graph_snapshot(),
            predecessor.capabilities(),
        );
        let candidate_graph = UiGraphFactIndexBasis::from_generation(
            candidate.graph_snapshot(),
            candidate.capabilities(),
        );
        let candidate_generation = candidate.generation_identity().clone();
        let budget = predecessor.change_profile().rebind().budget();
        enforce_limit(
            UiRebindLimit::ChangedFacts,
            budget.changed_facts,
            facts.len(),
        )?;

        let accumulation = accumulate_scope(ScopeLookupInput {
            facts: &facts,
            predecessor,
            candidate,
            predecessor_basis: predecessor_graph,
            candidate_basis: candidate_graph,
            budget,
        })?;

        finish_scope(FinishScopeInput {
            classification,
            facts,
            source_succession,
            predecessor_graph,
            candidate_generation,
            candidate_graph,
            lookups: accumulation.lookups,
            consumers: accumulation.consumers,
            aspects: accumulation.aspects,
        })
    }
}

fn accumulate_scope(
    input: ScopeLookupInput<'_>,
) -> Result<ScopeAccumulation, UiAffectedScopeDenial> {
    let mut accumulation = ScopeAccumulation {
        lookups: Vec::with_capacity(input.facts.len()),
        consumers: BTreeMap::new(),
        aspects: BTreeSet::new(),
    };
    for (fact_ordinal, fact) in input.facts.iter().enumerate() {
        let (predecessor, candidate) = lookup_both_generations(DualGenerationLookupInput {
            fact_ordinal,
            fact,
            predecessor: input.predecessor,
            candidate: input.candidate,
            predecessor_basis: input.predecessor_basis,
            candidate_basis: input.candidate_basis,
        })?;
        join_entries(
            predecessor.entries(),
            UiAffectedScopeGeneration::Predecessor,
            &mut accumulation.consumers,
            &mut accumulation.aspects,
        );
        join_entries(
            candidate.entries(),
            UiAffectedScopeGeneration::Candidate,
            &mut accumulation.consumers,
            &mut accumulation.aspects,
        );
        enforce_scope_limits(&accumulation.consumers, &accumulation.aspects, input.budget)?;
        accumulation.lookups.push(UiAffectedFactLookup::new(
            fact_ordinal,
            fact.family(),
            predecessor,
            candidate,
        ));
    }
    Ok(accumulation)
}

fn require_current_basis(
    basis: &crate::runtime::observation::UiChangeClassificationBasis,
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    predecessor: &crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationAuthority,
) -> Result<(), UiAffectedScopeDenial> {
    if basis.session() != session {
        return Err(UiAffectedScopeDenial::ForeignSession);
    }
    if basis.source_basis() != predecessor.capabilities().digest().as_u64() {
        return Err(UiAffectedScopeDenial::StaleSourceBasis);
    }
    if basis.predecessor_generation() != predecessor.generation_identity() {
        return Err(UiAffectedScopeDenial::StalePredecessorGeneration);
    }
    Ok(())
}

fn lookup_both_generations(
    input: DualGenerationLookupInput<'_>,
) -> Result<(UiGraphFactLookupReceipt, UiGraphFactLookupReceipt), UiAffectedScopeDenial> {
    let DualGenerationLookupInput {
        fact_ordinal,
        fact,
        predecessor,
        candidate,
        predecessor_basis,
        candidate_basis,
    } = input;
    let predecessor_lookup = predecessor
        .consumed_fact_index()
        .lookup(predecessor_basis, fact);
    let candidate_lookup = candidate
        .consumed_fact_index()
        .lookup(candidate_basis, fact);
    match (predecessor_lookup, candidate_lookup) {
        (Ok(predecessor), Ok(candidate)) => Ok((predecessor, candidate)),
        (Err(UiGraphFactLookupDenial::UnknownAuthoredDeclaration { .. }), Ok(candidate))
            if predecessor_basis != candidate_basis =>
        {
            Ok((
                UiGraphFactLookupReceipt::new(predecessor_basis, Box::new([])),
                candidate,
            ))
        }
        (Ok(predecessor), Err(UiGraphFactLookupDenial::UnknownAuthoredDeclaration { .. }))
            if predecessor_basis != candidate_basis =>
        {
            Ok((
                predecessor,
                UiGraphFactLookupReceipt::new(candidate_basis, Box::new([])),
            ))
        }
        (
            Err(UiGraphFactLookupDenial::UnknownAuthoredDeclaration { authored_identity }),
            Err(UiGraphFactLookupDenial::UnknownAuthoredDeclaration { .. }),
        ) => Err(
            UiAffectedScopeDenial::UnknownAuthoredSelectorInBothGenerations {
                fact_ordinal,
                authored_identity,
            },
        ),
        (Err(source), _) => Err(UiAffectedScopeDenial::Index {
            generation: UiAffectedScopeGeneration::Predecessor,
            fact_ordinal,
            source,
        }),
        (_, Err(source)) => Err(UiAffectedScopeDenial::Index {
            generation: UiAffectedScopeGeneration::Candidate,
            fact_ordinal,
            source,
        }),
    }
}

fn join_entries(
    entries: &[UiGraphFactIndexEntry],
    generation: UiAffectedScopeGeneration,
    consumers: &mut BTreeMap<UiGraphFactConsumerKey, ConsumerAccumulator>,
    aspects: &mut BTreeSet<UiAspectName>,
) {
    for entry in entries {
        let consumer = consumers
            .entry(entry.consumer_key().clone())
            .or_insert_with(|| ConsumerAccumulator {
                predecessor: None,
                candidate: None,
                aspects: BTreeSet::new(),
            });
        match generation {
            UiAffectedScopeGeneration::Predecessor => consumer.predecessor = Some(entry.consumer()),
            UiAffectedScopeGeneration::Candidate => consumer.candidate = Some(entry.consumer()),
        }
        if let Some(aspect) = entry.affected_aspect() {
            consumer.aspects.insert(aspect.clone());
            aspects.insert(aspect.clone());
        }
    }
}

fn enforce_scope_limits(
    consumers: &BTreeMap<UiGraphFactConsumerKey, ConsumerAccumulator>,
    aspects: &BTreeSet<UiAspectName>,
    budget: UiRebindBudgetInput,
) -> Result<(), UiAffectedScopeDenial> {
    enforce_limit(
        UiRebindLimit::AffectedAspects,
        budget.affected_aspects,
        aspects.len(),
    )?;
    enforce_limit(
        UiRebindLimit::DistinctConsumers,
        budget.distinct_consumers,
        consumers.len(),
    )?;
    enforce_limit(
        UiRebindLimit::GraphAndMountedEntries,
        budget.graph_and_mounted_entries,
        selected_entry_count(consumers),
    )
}

fn enforce_limit(
    limit: UiRebindLimit,
    configured: usize,
    observed: usize,
) -> Result<(), UiAffectedScopeDenial> {
    if observed > configured {
        Err(UiAffectedScopeDenial::BudgetExceeded {
            limit,
            configured,
            observed,
        })
    } else {
        Ok(())
    }
}

fn selected_entry_count(
    consumers: &BTreeMap<UiGraphFactConsumerKey, ConsumerAccumulator>,
) -> usize {
    consumers
        .values()
        .map(|consumer| {
            usize::from(consumer.predecessor.is_some()) + usize::from(consumer.candidate.is_some())
        })
        .sum()
}

fn finish_scope(input: FinishScopeInput) -> Result<UiResolvedAffectedScope, UiAffectedScopeDenial> {
    let FinishScopeInput {
        classification,
        facts,
        source_succession,
        predecessor_graph,
        candidate_generation,
        candidate_graph,
        lookups,
        consumers,
        aspects,
    } = input;
    let indexed_consumers = consumers.len();
    let graph_and_mounted_entries = selected_entry_count(&consumers);
    let (index_probes, contract_checks) = lookup_cost(&lookups);
    let affected_aspects = aspects.into_iter().collect::<Vec<_>>().into_boxed_slice();
    let consumers = materialize_consumers(consumers);
    let basis = UiAffectedScopeBasis::new(
        classification,
        predecessor_graph,
        candidate_generation,
        candidate_graph,
    );
    let cost = UiAffectedScopeCost::exact(UiAffectedScopeCostInput {
        observations: basis.classification().observation_count(),
        changed_facts: facts.len(),
        affected_aspects: affected_aspects.len(),
        indexed_consumers,
        lookup_receipts: lookups.len() * 2,
        index_probes,
        contract_checks,
        graph_and_mounted_entries,
    });
    Ok(UiResolvedAffectedScope::new(UiResolvedAffectedScopeInput {
        basis,
        facts,
        affected_aspects,
        consumers,
        lookups: lookups.into_boxed_slice(),
        cost,
        source_succession,
    }))
}

fn materialize_consumers(
    consumers: BTreeMap<UiGraphFactConsumerKey, ConsumerAccumulator>,
) -> Box<[UiAffectedConsumer]> {
    consumers
        .into_iter()
        .map(|(key, consumer)| {
            UiAffectedConsumer::new(
                key,
                consumer.predecessor,
                consumer.candidate,
                consumer
                    .aspects
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            )
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn lookup_cost(lookups: &[UiAffectedFactLookup]) -> (usize, usize) {
    lookups.iter().fold((0, 0), |cost, lookup| {
        (
            cost.0
                + lookup.predecessor().cost().index_probes()
                + lookup.candidate().cost().index_probes(),
            cost.1
                + lookup.predecessor().cost().contract_checks()
                + lookup.candidate().cost().contract_checks(),
        )
    })
}
