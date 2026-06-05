use crate::domain_artifacts::assignment_from_model;

pub(super) fn model_satisfies_cnf(clauses: &[Vec<i32>], model: &[i32]) -> bool {
    let assignment = assignment_from_model(model);
    clauses.iter().all(|clause| {
        clause.iter().any(|literal| {
            assignment
                .get(&literal.abs())
                .map(|value| (*literal > 0 && *value) || (*literal < 0 && !*value))
                .unwrap_or(false)
        })
    })
}
