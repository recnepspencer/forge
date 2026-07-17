use crate::ProtocolFamily;

use super::{
    ProtocolCheckBounds, ProtocolCheckStatistics, ProtocolCheckVerdict, ProtocolCounterexample,
    ProtocolCounterexampleState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolCheckerOutputDenial {
    MissingCheckerStatistics,
    UnrecognizedCheckerFailure,
}

struct ParsedCounterexampleState {
    ordinal: u64,
    action: String,
    valuations: Vec<(String, String)>,
}

pub fn interpret_tlc_output(
    protocol: ProtocolFamily,
    bounds: ProtocolCheckBounds,
    output: &str,
    process_succeeded: bool,
) -> Result<ProtocolCheckVerdict, ProtocolCheckerOutputDenial> {
    if !process_succeeded {
        let counterexample = parse_counterexample(protocol, output);
        if output.contains("Deadlock reached") {
            let statistics = parse_statistics(output)
                .ok_or(ProtocolCheckerOutputDenial::MissingCheckerStatistics)?;
            return Ok(ProtocolCheckVerdict::DeadlockFound {
                counterexample,
                statistics,
            });
        }
        if output.contains("is violated") || output.contains("Error: The behavior up to this point")
        {
            let statistics = parse_statistics(output)
                .ok_or(ProtocolCheckerOutputDenial::MissingCheckerStatistics)?;
            return Ok(ProtocolCheckVerdict::CounterexampleFound {
                counterexample,
                statistics,
            });
        }
        return Err(ProtocolCheckerOutputDenial::UnrecognizedCheckerFailure);
    }
    if output.contains("Model checking completed. No error has been found.") {
        let statistics = parse_statistics(output)
            .ok_or(ProtocolCheckerOutputDenial::MissingCheckerStatistics)?;
        if statistics.states_left_on_queue() != 0
            || statistics.distinct_states() > bounds.maximum_states().get()
            || statistics.trace_depth() > bounds.maximum_trace_depth().get()
        {
            return Ok(ProtocolCheckVerdict::BoundExhausted { bounds, statistics });
        }
        return Ok(ProtocolCheckVerdict::CheckedWithinBounds { bounds, statistics });
    }

    let counterexample = parse_counterexample(protocol, output);
    let statistics =
        parse_statistics(output).ok_or(ProtocolCheckerOutputDenial::MissingCheckerStatistics)?;
    if output.contains("Deadlock reached") {
        return Ok(ProtocolCheckVerdict::DeadlockFound {
            counterexample,
            statistics,
        });
    }
    if output.contains("is violated") || output.contains("Error: The behavior up to this point") {
        return Ok(ProtocolCheckVerdict::CounterexampleFound {
            counterexample,
            statistics,
        });
    }
    Err(ProtocolCheckerOutputDenial::MissingCheckerStatistics)
}

fn parse_statistics(output: &str) -> Option<ProtocolCheckStatistics> {
    let initial = output
        .lines()
        .find(|line| line.contains("Finished computing initial states:"))?
        .split("Finished computing initial states:")
        .nth(1)?
        .split_whitespace()
        .next()?
        .parse()
        .ok()?;
    let states = output.lines().find(|line| {
        line.contains(" states generated, ") && line.contains(" distinct states found")
    })?;
    let generated = states.split_whitespace().next()?.parse().ok()?;
    let distinct = states
        .split(" states generated, ")
        .nth(1)?
        .split_whitespace()
        .next()?
        .parse()
        .ok()?;
    let queued = states
        .split(" distinct states found, ")
        .nth(1)?
        .split_whitespace()
        .next()?
        .parse()
        .ok()?;
    let depth = output
        .lines()
        .find(|line| line.contains("depth of the complete state graph search is"))?
        .trim_end_matches('.')
        .split_whitespace()
        .last()?
        .parse()
        .ok()?;
    Some(ProtocolCheckStatistics::observed(
        initial, generated, distinct, queued, depth,
    ))
}

fn parse_counterexample(protocol: ProtocolFamily, output: &str) -> ProtocolCounterexample {
    let mut states = Vec::new();
    let mut current: Option<ParsedCounterexampleState> = None;
    for line in output.lines() {
        if let Some((ordinal, action)) = parse_state_header(line) {
            if let Some(state) = current.take() {
                states.push(ProtocolCounterexampleState::observed(
                    state.ordinal,
                    state.action,
                    state.valuations,
                ));
            }
            current = Some(ParsedCounterexampleState {
                ordinal,
                action,
                valuations: Vec::new(),
            });
            continue;
        }
        let Some(state) = current.as_mut() else {
            continue;
        };
        if let Some(valuation) = parse_valuation(line) {
            state.valuations.push(valuation);
        }
    }
    if let Some(state) = current {
        states.push(ProtocolCounterexampleState::observed(
            state.ordinal,
            state.action,
            state.valuations,
        ));
    }
    if states.is_empty() {
        return ProtocolCounterexample::diagnostic(
            protocol,
            vec!["checker-reported-illegal-transition".to_owned()],
        );
    }
    ProtocolCounterexample::from_tlc_states(protocol, states)
}

fn parse_state_header(line: &str) -> Option<(u64, String)> {
    let line = line.trim();
    let state = line.strip_prefix("State ")?;
    let (ordinal, action) = state.split_once(": <")?;
    Some((ordinal.parse().ok()?, action.strip_suffix('>')?.to_owned()))
}

fn parse_valuation(line: &str) -> Option<(String, String)> {
    let valuation = line.trim().strip_prefix("/\\ ")?;
    let (name, value) = valuation.split_once(" = ")?;
    Some((name.trim().to_owned(), value.trim().to_owned()))
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use super::*;

    #[test]
    fn checked_output_is_bound_by_observed_states_and_depth() {
        let output = "Finished computing initial states: 3 distinct states generated.\n\
                      Model checking completed. No error has been found.\n\
                      85 states generated, 12 distinct states found, 0 states left on queue.\n\
                      The depth of the complete state graph search is 9.";
        let bounds =
            ProtocolCheckBounds::new(NonZeroU64::new(20).unwrap(), NonZeroU64::new(10).unwrap());

        assert!(matches!(
            interpret_tlc_output(ProtocolFamily::SharedFrontiers, bounds, output, true),
            Ok(ProtocolCheckVerdict::CheckedWithinBounds { statistics, .. })
                if statistics.initial_states() == 3
                    && statistics.distinct_states() == 12
                    && statistics.trace_depth() == 9
        ));
    }

    #[test]
    fn nonzero_process_cannot_claim_success_with_success_shaped_text() {
        let output = "Finished computing initial states: 1 distinct state generated.\n\
                      Model checking completed. No error has been found.\n\
                      85 states generated, 12 distinct states found, 0 states left on queue.\n\
                      The depth of the complete state graph search is 9.";
        let bounds =
            ProtocolCheckBounds::new(NonZeroU64::new(20).unwrap(), NonZeroU64::new(10).unwrap());

        assert_eq!(
            interpret_tlc_output(ProtocolFamily::SharedFrontiers, bounds, output, false),
            Err(ProtocolCheckerOutputDenial::UnrecognizedCheckerFailure)
        );
    }

    #[test]
    fn unfinished_state_queue_cannot_claim_checked_execution() {
        let output = "Finished computing initial states: 1 distinct state generated.\n\
                      Model checking completed. No error has been found.\n\
                      85 states generated, 12 distinct states found, 3 states left on queue.\n\
                      The depth of the complete state graph search is 9.";
        let bounds =
            ProtocolCheckBounds::new(NonZeroU64::new(20).unwrap(), NonZeroU64::new(10).unwrap());

        assert!(matches!(
            interpret_tlc_output(ProtocolFamily::SharedFrontiers, bounds, output, true),
            Ok(ProtocolCheckVerdict::BoundExhausted { statistics, .. })
                if statistics.states_left_on_queue() == 3
        ));
    }

    #[test]
    fn counterexample_preserves_state_ordinals_actions_and_valuations() {
        let output = "Finished computing initial states: 1 distinct state generated.\n\
                      Error: Invariant DurableAckImpliesStablePrefix is violated.\n\
                      Error: The behavior up to this point is:\n\
                      State 1: <Initial predicate>\n\
                      /\\ durable = FALSE\n\
                      /\\ mutantEdge = \"none\"\n\
                      State 2: <IssueAck>\n\
                      /\\ durable = FALSE\n\
                      /\\ mutantEdge = \"ack-before-flush\"\n\
                      2 states generated, 2 distinct states found, 0 states left on queue.\n\
                      The depth of the complete state graph search is 2.";
        let bounds =
            ProtocolCheckBounds::new(NonZeroU64::new(20).unwrap(), NonZeroU64::new(10).unwrap());

        let ProtocolCheckVerdict::CounterexampleFound { counterexample, .. } =
            interpret_tlc_output(ProtocolFamily::DurabilityRecovery, bounds, output, false)
                .unwrap()
        else {
            panic!("expected structured counterexample");
        };
        assert_eq!(counterexample.states().len(), 2);
        assert_eq!(counterexample.states()[1].ordinal(), 2);
        assert_eq!(counterexample.states()[1].action(), "IssueAck");
        assert_eq!(
            counterexample.states()[1].valuation("mutantEdge"),
            Some("\"ack-before-flush\"")
        );
    }
}
