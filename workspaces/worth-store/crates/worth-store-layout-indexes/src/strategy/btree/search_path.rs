use crate::keyspace::CanonicalKeyBytes;
use crate::strategy::{BTreeLookupBranch, BTreeSeparatorLaw, StrategyDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTreeSearchPathLaw {
    separator: BTreeSeparatorLaw,
}

impl BTreeSearchPathLaw {
    pub(crate) const fn new(separator: BTreeSeparatorLaw) -> Self {
        Self { separator }
    }

    pub fn verify_search_and_insertion_path(
        self,
        probe: &CanonicalKeyBytes,
        left_max: &CanonicalKeyBytes,
        separator: &CanonicalKeyBytes,
        right_min: &CanonicalKeyBytes,
        chosen_branch: BTreeLookupBranch,
    ) -> super::BTreeSearchOutcome<()> {
        let result = self.verify_search_and_insertion_path_result(
            probe,
            left_max,
            separator,
            right_min,
            chosen_branch,
        );
        super::BTreeSearchOutcome::issue(result)
    }

    fn verify_search_and_insertion_path_result(
        self,
        probe: &CanonicalKeyBytes,
        left_max: &CanonicalKeyBytes,
        separator: &CanonicalKeyBytes,
        right_min: &CanonicalKeyBytes,
        chosen_branch: BTreeLookupBranch,
    ) -> Result<(), StrategyDenial> {
        self.separator
            .verify_separator_partition(left_max, separator, right_min)?;
        if self.separator.route_lookup(probe, separator)? == chosen_branch {
            Ok(())
        } else {
            Err(StrategyDenial::SearchPathViolation)
        }
    }

    pub(crate) fn verify_search_and_insertion_path_from_observation(
        self,
        probe_precedes_separator: bool,
        left_max_precedes_separator: bool,
        separator_precedes_right_min: bool,
        observed_branch: BTreeLookupBranch,
    ) -> Result<(), StrategyDenial> {
        if !left_max_precedes_separator || !separator_precedes_right_min {
            return Err(StrategyDenial::ComparatorOrderViolation);
        }

        let expected_branch = if probe_precedes_separator {
            BTreeLookupBranch::Left
        } else {
            BTreeLookupBranch::RightOrEqual
        };
        if observed_branch == expected_branch {
            return Ok(());
        }
        Err(StrategyDenial::SearchPathViolation)
    }
}
