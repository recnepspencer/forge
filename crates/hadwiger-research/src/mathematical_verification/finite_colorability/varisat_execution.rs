use varisat::{solver::Solver, ExtendFormula, Lit};

use super::HadwigerColorabilityError;

pub(super) fn solve_cnf(
    clauses: &[Vec<i32>],
) -> Result<(bool, Vec<i32>), HadwigerColorabilityError> {
    let mut solver = Solver::new();
    for clause in clauses {
        let literals = clause
            .iter()
            .map(|literal| Lit::from_dimacs(*literal as isize))
            .collect::<Vec<_>>();
        solver.add_clause(&literals);
    }
    let sat = solver
        .solve()
        .map_err(|error| HadwigerColorabilityError::Solver(format!("{error:?}")))?;
    let model = if sat {
        solver
            .model()
            .unwrap_or_default()
            .into_iter()
            .map(|literal| literal.to_dimacs() as i32)
            .collect()
    } else {
        Vec::new()
    };
    Ok((sat, model))
}
