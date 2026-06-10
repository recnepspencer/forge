use varisat::{checker::Checker, solver::Solver, ExtendFormula, Lit, ProofFormat};

use super::HadwigerColorabilityError;

pub(super) fn generate_varisat_native_proof(
    clauses: &[Vec<i32>],
) -> Result<Vec<u8>, HadwigerColorabilityError> {
    let mut proof = Vec::new();
    {
        let mut solver = Solver::new();
        solver.write_proof(&mut proof, ProofFormat::Varisat);
        for clause in clauses {
            let literals = clause
                .iter()
                .map(|literal| Lit::from_dimacs(*literal as isize))
                .collect::<Vec<_>>();
            solver.add_clause(&literals);
        }
        if solver
            .solve()
            .map_err(|error| HadwigerColorabilityError::Solver(format!("{error:?}")))?
        {
            return Err(HadwigerColorabilityError::SatisfiableFormula);
        }
        solver
            .close_proof()
            .map_err(|error| HadwigerColorabilityError::Solver(format!("{error:?}")))?;
    }
    if proof.is_empty() {
        return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
    }
    Ok(proof)
}

pub(super) fn replay_varisat_native_proof(
    clauses: &[Vec<i32>],
    proof_bytes: &[u8],
) -> Result<(), HadwigerColorabilityError> {
    if proof_bytes.is_empty() {
        return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
    }
    let dimacs = dimacs_for_clauses(clauses);
    let mut checker = Checker::new();
    checker
        .add_dimacs_cnf(&mut &dimacs[..])
        .map_err(|_| HadwigerColorabilityError::CorruptRefutationCertificate)?;
    checker
        .check_proof(&mut &proof_bytes[..])
        .map_err(|_| HadwigerColorabilityError::CorruptRefutationCertificate)
}

fn dimacs_for_clauses(clauses: &[Vec<i32>]) -> Vec<u8> {
    let variable_count = clauses
        .iter()
        .flat_map(|clause| clause.iter())
        .map(|literal| literal.unsigned_abs())
        .max()
        .unwrap_or(0);
    let mut text = format!("p cnf {variable_count} {}\n", clauses.len());
    for clause in clauses {
        for literal in clause {
            text.push_str(&literal.to_string());
            text.push(' ');
        }
        text.push_str("0\n");
    }
    text.into_bytes()
}
