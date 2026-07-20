use super::operand_access::string_scalar;
use super::state::LegalPredicate;

#[derive(Default)]
pub(super) struct NormalizedContainsPredicates(Vec<LegalPredicate>);

impl NormalizedContainsPredicates {
    pub(super) fn ingest(&mut self, entry: LegalPredicate) {
        let candidate = string_scalar(&entry.0);
        if self
            .0
            .iter()
            .any(|existing| string_scalar(&existing.0) == candidate)
        {
            return;
        }
        self.0
            .retain(|existing| !candidate.contains(string_scalar(&existing.0)));
        if self
            .0
            .iter()
            .any(|existing| string_scalar(&existing.0).contains(candidate))
        {
            return;
        }
        self.0.push(entry);
        self.0
            .sort_by(|left, right| string_scalar(&left.0).cmp(string_scalar(&right.0)));
    }

    pub(super) fn as_slice(&self) -> &[LegalPredicate] {
        &self.0
    }

    pub(super) fn into_entries(self) -> Vec<LegalPredicate> {
        self.0
    }
}
