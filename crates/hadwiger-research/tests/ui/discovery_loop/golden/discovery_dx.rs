use hadwiger_research::facade::{
    mine_research_patterns, plan_next_experiments, propose_invariant_hypotheses,
    recompute_derived_discovery_frontier, update_discovery_frontier,
    HadwigerDiscoveryError, HadwigerResearchHandle, ResearchEvidenceCorpus,
};

fn discovery_dx(
    handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
) -> Result<(), HadwigerDiscoveryError> {
    let observations = mine_research_patterns(handle, corpus)?;
    let hypotheses = propose_invariant_hypotheses(handle, corpus, &observations)?;
    let plans = plan_next_experiments(handle, corpus, &hypotheses)?;
    let frontier = update_discovery_frontier(handle, corpus, observations, hypotheses, plans)?;
    let derived = recompute_derived_discovery_frontier(handle, corpus)?;

    assert!(!frontier.admits_theorem_authority());
    assert!(!frontier.registers_query_invariant_authority());
    assert_eq!(derived.source_corpus_digest(), corpus.corpus_digest());
    Ok(())
}

fn main() {}
