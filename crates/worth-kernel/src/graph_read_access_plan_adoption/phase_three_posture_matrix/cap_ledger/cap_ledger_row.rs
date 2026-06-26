use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostureCapRow {
    family: &'static str,
    max_count: usize,
    owner: &'static str,
    expected_denial: &'static str,
    suggested_posture: &'static str,
    blocker: &'static str,
    removal_trigger: &'static str,
    row_digest: String,
}

impl WorthGraphReadAccessPostureCapRow {
    pub(crate) fn new(
        family: &'static str,
        max_count: usize,
        owner: &'static str,
        expected_denial: &'static str,
        suggested_posture: &'static str,
        blocker: &'static str,
        removal_trigger: &'static str,
    ) -> Self {
        Self {
            family,
            max_count,
            owner,
            expected_denial,
            suggested_posture,
            blocker,
            removal_trigger,
            row_digest: stable_digest(&[
                "worth_graph_read_access_posture_cap_row_v1".to_string(),
                format!("family:{family}"),
                format!("max_count:{max_count}"),
                format!("owner:{owner}"),
                format!("expected_denial:{expected_denial}"),
                format!("suggested_posture:{suggested_posture}"),
                format!("blocker:{blocker}"),
                format!("removal_trigger:{removal_trigger}"),
            ]),
        }
    }

    #[cfg(test)]
    pub(crate) fn for_tests(family: &'static str, max_count: usize) -> Self {
        Self::new(
            family,
            max_count,
            "worth-kernel-test",
            "test_denial",
            "test_posture",
            "test blocker",
            "test removal trigger",
        )
    }

    pub const fn family(&self) -> &'static str {
        self.family
    }

    pub const fn max_count(&self) -> usize {
        self.max_count
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn expected_denial(&self) -> &'static str {
        self.expected_denial
    }

    pub const fn suggested_posture(&self) -> &'static str {
        self.suggested_posture
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
