use std::collections::BTreeSet;

use worth_store::physical_runtime::PhysicalWorkProcessEvidence;

use super::{C7CaseProcessRole, C7DurabilityCrashSeam};

const PROCESSES_PER_CASE: usize = C7CaseProcessRole::ALL.len();

pub(super) struct C7CrashProcessAccounting {
    processes: Box<[PhysicalWorkProcessEvidence]>,
}

impl C7CrashProcessAccounting {
    pub(super) fn bind(
        seams: &[C7DurabilityCrashSeam],
        processes: Vec<PhysicalWorkProcessEvidence>,
    ) -> Result<Self, String> {
        let expected_processes = seams.len().saturating_mul(PROCESSES_PER_CASE);
        if processes.len() != expected_processes {
            return Err(format!(
                "C7 process accounting requires {expected_processes} records, found {}",
                processes.len()
            ));
        }

        let mut campaign_roles = BTreeSet::new();
        for (case_index, seam) in seams.iter().copied().enumerate() {
            let start = case_index * PROCESSES_PER_CASE;
            let case_processes = &processes[start..start + PROCESSES_PER_CASE];
            let process_ids = case_processes
                .iter()
                .map(PhysicalWorkProcessEvidence::process)
                .collect::<BTreeSet<_>>();
            if process_ids.len() != PROCESSES_PER_CASE {
                return Err(format!(
                    "C7 process accounting found a repeated identity within case {}",
                    seam.label()
                ));
            }

            for (process, role) in case_processes.iter().zip(C7CaseProcessRole::ALL) {
                let expected_role = role.qualified(seam);
                if process.role() != expected_role {
                    return Err(format!(
                        "C7 process accounting expected role {expected_role}, found {}",
                        process.role()
                    ));
                }
                if !campaign_roles.insert(process.role()) {
                    return Err(format!(
                        "C7 process accounting found a repeated campaign role: {}",
                        process.role()
                    ));
                }
            }
        }

        Ok(Self {
            processes: processes.into_boxed_slice(),
        })
    }

    pub(super) const fn processes(&self) -> &[PhysicalWorkProcessEvidence] {
        &self.processes
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use worth_store::physical_runtime::PhysicalWorkProcessEvidence;

    use super::{C7CrashProcessAccounting, C7DurabilityCrashSeam, PROCESSES_PER_CASE};
    use crate::courtroom_campaign::bounded_residency_siege::c7_crash_campaign::C7CaseProcessRole;

    #[test]
    fn accepts_process_id_reuse_across_sequential_cases() {
        let seams = [
            C7DurabilityCrashSeam::BeforeWalAppend,
            C7DurabilityCrashSeam::DuringWalAppendPrefix,
        ];
        let processes = seams
            .into_iter()
            .flat_map(|seam| case_processes(seam, 41))
            .collect();

        let accounting = C7CrashProcessAccounting::bind(&seams, processes).unwrap();

        assert_eq!(accounting.processes().len(), 2 * PROCESSES_PER_CASE);
        assert_eq!(
            accounting.processes()[0].process(),
            accounting.processes()[5].process()
        );
    }

    #[test]
    fn exact_case_cardinality_is_required() {
        let seams = [C7DurabilityCrashSeam::BeforeWalAppend];
        let mut processes = case_processes(seams[0], 41);
        processes.pop();

        assert!(C7CrashProcessAccounting::bind(&seams, processes).is_err());
    }

    #[test]
    fn repeated_process_identity_within_one_case_is_rejected() {
        let seams = [C7DurabilityCrashSeam::BeforeWalAppend];
        let mut processes = case_processes(seams[0], 41);
        processes[1] = PhysicalWorkProcessEvidence::exited_success(
            C7CaseProcessRole::BaselineObserver.qualified(seams[0]),
            NonZeroU32::new(41).unwrap(),
        )
        .unwrap();

        if C7CrashProcessAccounting::bind(&seams, processes).is_ok() {
            panic!("MUTANT_PREDICATE:c7-termination-process-accounting-omitted");
        }
    }

    #[test]
    fn wrong_case_role_is_rejected() {
        let seams = [C7DurabilityCrashSeam::BeforeWalAppend];
        let mut processes = case_processes(seams[0], 41);
        processes[0] = PhysicalWorkProcessEvidence::exited_success(
            "c7:wrong:seed-producer",
            NonZeroU32::new(41).unwrap(),
        )
        .unwrap();

        assert!(C7CrashProcessAccounting::bind(&seams, processes).is_err());
    }

    fn case_processes(
        seam: C7DurabilityCrashSeam,
        first_process: u32,
    ) -> Vec<PhysicalWorkProcessEvidence> {
        C7CaseProcessRole::ALL
            .into_iter()
            .enumerate()
            .map(|(offset, role)| {
                PhysicalWorkProcessEvidence::exited_success(
                    role.qualified(seam),
                    NonZeroU32::new(first_process + offset as u32).unwrap(),
                )
                .unwrap()
            })
            .collect()
    }
}
