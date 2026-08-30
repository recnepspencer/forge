use worth_query_installation::facade::{
    WorthQueryCandidateOptimalityPosture as Optimality,
    WorthQueryCandidateSearchContract as Search,
    WorthQueryCandidateSearchEvidenceFamilies as SearchEvidence,
    WorthQueryCandidateSearchPosture as SearchPosture,
    WorthQueryConvergenceContract as Convergence,
    WorthQueryConvergenceIncumbentPosture as Incumbent,
    WorthQueryConvergenceOscillationPosture as Oscillation,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

pub(super) fn write_search(
    output: &mut dyn BinaryEncodingSink,
    contract: &Search,
) -> Result<(), Denial> {
    if let Some(universe) = contract.universe_family() {
        output.u16(2)?;
        output.text(universe)?;
        output.text(required(contract.termination_family())?)?;
        output.text(required(contract.feasibility_family())?)?;
        output.text(required(contract.comparison_family())?)?;
        output.text(required(contract.incumbent_family())?)?;
    } else {
        output.u16(1)?;
    }
    write_search_posture(output, contract.search_posture())?;
    write_optimality(output, contract.optimality_posture())
}

pub(super) fn decode_search(input: &mut BinaryInput<'_>) -> Result<Search, Denial> {
    let evidence = match input.u16()? {
        1 => None,
        2 => Some(SearchEvidence::new(
            input.text()?.to_owned(),
            input.text()?.to_owned(),
            input.text()?.to_owned(),
            input.text()?.to_owned(),
            input.text()?.to_owned(),
        )),
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    };
    let search = decode_search_posture(input)?;
    let optimality = decode_optimality(input)?;
    match evidence {
        Some(evidence) => Ok(Search::declared(evidence, search, optimality)),
        None if matches!(search, SearchPosture::NotApplicable)
            && matches!(optimality, Optimality::NotApplicable) =>
        {
            Ok(Search::not_applicable())
        }
        None => Err(Denial::new(Kind::InvalidRecordShape)),
    }
}

pub(super) fn write_convergence(
    output: &mut dyn BinaryEncodingSink,
    contract: &Convergence,
) -> Result<(), Denial> {
    match contract {
        Convergence::NotIterative => output.u16(1),
        Convergence::Iterative {
            progress_measure_family,
            comparator_family,
            repeated_state_family,
            incumbent,
            iteration_bound,
            oscillation,
        } => {
            output.u16(2)?;
            output.text(progress_measure_family)?;
            output.text(comparator_family)?;
            output.text(repeated_state_family)?;
            output.u16(incumbent_tag(*incumbent))?;
            write_usize(output, *iteration_bound)?;
            output.u16(oscillation_tag(*oscillation))
        }
    }
}

pub(super) fn decode_convergence(input: &mut BinaryInput<'_>) -> Result<Convergence, Denial> {
    match input.u16()? {
        1 => Ok(Convergence::NotIterative),
        2 => Ok(Convergence::Iterative {
            progress_measure_family: input.text()?.to_owned(),
            comparator_family: input.text()?.to_owned(),
            repeated_state_family: input.text()?.to_owned(),
            incumbent: incumbent_from_tag(input.u16()?)?,
            iteration_bound: decode_usize(input)?,
            oscillation: oscillation_from_tag(input.u16()?)?,
        }),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn required(value: Option<&str>) -> Result<&str, Denial> {
    value.ok_or_else(|| Denial::new(Kind::InvalidRecordShape))
}

fn write_search_posture(
    output: &mut dyn BinaryEncodingSink,
    posture: &SearchPosture,
) -> Result<(), Denial> {
    match posture {
        SearchPosture::NotApplicable => output.u16(1),
        SearchPosture::Exhaustive => output.u16(2),
        SearchPosture::ProvenTopK { count } => {
            output.u16(3)?;
            write_usize(output, *count)
        }
        SearchPosture::Bounded { bound_identity } => tagged_text(output, 4, bound_identity),
        SearchPosture::Sampled { sample_identity } => tagged_text(output, 5, sample_identity),
        SearchPosture::Heuristic => output.u16(6),
        SearchPosture::Incomplete => output.u16(7),
    }
}

fn decode_search_posture(input: &mut BinaryInput<'_>) -> Result<SearchPosture, Denial> {
    match input.u16()? {
        1 => Ok(SearchPosture::NotApplicable),
        2 => Ok(SearchPosture::Exhaustive),
        3 => Ok(SearchPosture::ProvenTopK {
            count: decode_usize(input)?,
        }),
        4 => Ok(SearchPosture::Bounded {
            bound_identity: input.text()?.to_owned(),
        }),
        5 => Ok(SearchPosture::Sampled {
            sample_identity: input.text()?.to_owned(),
        }),
        6 => Ok(SearchPosture::Heuristic),
        7 => Ok(SearchPosture::Incomplete),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_optimality(
    output: &mut dyn BinaryEncodingSink,
    posture: &Optimality,
) -> Result<(), Denial> {
    match posture {
        Optimality::NotApplicable => output.u16(1),
        Optimality::ProvenOptimal => output.u16(2),
        Optimality::ProvenTopK { count } => {
            output.u16(3)?;
            write_usize(output, *count)
        }
        Optimality::BoundedGap { bound_identity } => tagged_text(output, 4, bound_identity),
        Optimality::BestInDeclaredSample { sample_identity } => {
            tagged_text(output, 5, sample_identity)
        }
        Optimality::ParetoForDeclaredSet { set_identity } => tagged_text(output, 6, set_identity),
        Optimality::FeasibleOnly => output.u16(7),
        Optimality::Unknown => output.u16(8),
    }
}

fn decode_optimality(input: &mut BinaryInput<'_>) -> Result<Optimality, Denial> {
    match input.u16()? {
        1 => Ok(Optimality::NotApplicable),
        2 => Ok(Optimality::ProvenOptimal),
        3 => Ok(Optimality::ProvenTopK {
            count: decode_usize(input)?,
        }),
        4 => Ok(Optimality::BoundedGap {
            bound_identity: input.text()?.to_owned(),
        }),
        5 => Ok(Optimality::BestInDeclaredSample {
            sample_identity: input.text()?.to_owned(),
        }),
        6 => Ok(Optimality::ParetoForDeclaredSet {
            set_identity: input.text()?.to_owned(),
        }),
        7 => Ok(Optimality::FeasibleOnly),
        8 => Ok(Optimality::Unknown),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn tagged_text(output: &mut dyn BinaryEncodingSink, tag: u16, value: &str) -> Result<(), Denial> {
    output.u16(tag)?;
    output.text(value)
}

fn write_usize(output: &mut dyn BinaryEncodingSink, value: usize) -> Result<(), Denial> {
    output.u64(u64::try_from(value).map_err(|_| Denial::new(Kind::NumericWidthExceeded))?)
}

fn decode_usize(input: &mut BinaryInput<'_>) -> Result<usize, Denial> {
    usize::try_from(input.u64()?).map_err(|_| Denial::new(Kind::NumericWidthExceeded))
}

const fn incumbent_tag(value: Incumbent) -> u16 {
    match value {
        Incumbent::NoIncumbent => 1,
        Incumbent::FirstFeasible => 2,
        Incumbent::BestObserved => 3,
        Incumbent::ParetoFrontier => 4,
    }
}

fn incumbent_from_tag(tag: u16) -> Result<Incumbent, Denial> {
    match tag {
        1 => Ok(Incumbent::NoIncumbent),
        2 => Ok(Incumbent::FirstFeasible),
        3 => Ok(Incumbent::BestObserved),
        4 => Ok(Incumbent::ParetoFrontier),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn oscillation_tag(value: Oscillation) -> u16 {
    match value {
        Oscillation::Impossible => 1,
        Oscillation::DetectAndDeny => 2,
        Oscillation::DetectAndSelectIncumbent => 3,
        Oscillation::DomainClassified => 4,
    }
}

fn oscillation_from_tag(tag: u16) -> Result<Oscillation, Denial> {
    match tag {
        1 => Ok(Oscillation::Impossible),
        2 => Ok(Oscillation::DetectAndDeny),
        3 => Ok(Oscillation::DetectAndSelectIncumbent),
        4 => Ok(Oscillation::DomainClassified),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
