#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AuthorizedProjectionCounters {
    authorized_projection_width: usize,
    masked_projection_entry_count: usize,
    hidden_predicate_denial_count: usize,
    hidden_ordering_denial_count: usize,
    hidden_grouping_denial_count: usize,
    hidden_derived_field_denial_count: usize,
    hidden_aggregation_denial_count: usize,
    hidden_cursor_denial_count: usize,
    hidden_view_membership_denial_count: usize,
    forbidden_post_read_redaction_count: usize,
    inspected_field_reference_count: usize,
}

impl AuthorizedProjectionCounters {
    pub(crate) fn set_authorized_projection_width(&mut self, value: usize) {
        self.authorized_projection_width = value;
    }

    pub(crate) fn set_masked_projection_entry_count(&mut self, value: usize) {
        self.masked_projection_entry_count = value;
    }

    pub(crate) fn inspect_field_reference(&mut self) {
        self.inspected_field_reference_count += 1;
    }

    pub(crate) fn deny_hidden_predicate(&mut self) {
        self.hidden_predicate_denial_count += 1;
    }

    pub(crate) fn deny_hidden_ordering(&mut self) {
        self.hidden_ordering_denial_count += 1;
    }

    pub(crate) fn deny_hidden_grouping(&mut self) {
        self.hidden_grouping_denial_count += 1;
    }

    pub(crate) fn deny_hidden_derived_field(&mut self) {
        self.hidden_derived_field_denial_count += 1;
    }

    pub(crate) fn deny_hidden_aggregation(&mut self) {
        self.hidden_aggregation_denial_count += 1;
    }

    pub(crate) fn deny_hidden_cursor(&mut self) {
        self.hidden_cursor_denial_count += 1;
    }

    pub(crate) fn deny_hidden_view_membership(&mut self) {
        self.hidden_view_membership_denial_count += 1;
    }

    pub(crate) fn deny_post_read_redaction(&mut self) {
        self.forbidden_post_read_redaction_count += 1;
    }

    pub fn authorized_projection_width(&self) -> usize {
        self.authorized_projection_width
    }

    pub fn masked_projection_entry_count(&self) -> usize {
        self.masked_projection_entry_count
    }

    pub fn hidden_predicate_denial_count(&self) -> usize {
        self.hidden_predicate_denial_count
    }

    pub fn hidden_ordering_denial_count(&self) -> usize {
        self.hidden_ordering_denial_count
    }

    pub fn hidden_grouping_denial_count(&self) -> usize {
        self.hidden_grouping_denial_count
    }

    pub fn hidden_derived_field_denial_count(&self) -> usize {
        self.hidden_derived_field_denial_count
    }

    pub fn hidden_aggregation_denial_count(&self) -> usize {
        self.hidden_aggregation_denial_count
    }

    pub fn hidden_cursor_denial_count(&self) -> usize {
        self.hidden_cursor_denial_count
    }

    pub fn hidden_view_membership_denial_count(&self) -> usize {
        self.hidden_view_membership_denial_count
    }

    pub fn forbidden_post_read_redaction_count(&self) -> usize {
        self.forbidden_post_read_redaction_count
    }

    pub fn inspected_field_reference_count(&self) -> usize {
        self.inspected_field_reference_count
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        vec![
            format!("authorized_width:{}", self.authorized_projection_width),
            format!("masked_entries:{}", self.masked_projection_entry_count),
            format!("hidden_predicate:{}", self.hidden_predicate_denial_count),
            format!("hidden_ordering:{}", self.hidden_ordering_denial_count),
            format!("hidden_grouping:{}", self.hidden_grouping_denial_count),
            format!("hidden_derived:{}", self.hidden_derived_field_denial_count),
            format!(
                "hidden_aggregation:{}",
                self.hidden_aggregation_denial_count
            ),
            format!("hidden_cursor:{}", self.hidden_cursor_denial_count),
            format!(
                "hidden_view_membership:{}",
                self.hidden_view_membership_denial_count
            ),
            format!(
                "forbidden_post_read_redaction:{}",
                self.forbidden_post_read_redaction_count
            ),
            format!("inspected_fields:{}", self.inspected_field_reference_count),
        ]
    }
}
