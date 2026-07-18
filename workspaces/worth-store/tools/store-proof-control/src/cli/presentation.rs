use crate::execution::ExecutedProofRun;
use crate::selection::SelectedProofExecutionPlan;

pub fn print_plan(plan: &SelectedProofExecutionPlan) {
    println!("proof product: {}", plan.product);
    println!("plan digest: {}", plan.plan_digest);
    println!("profile/cache posture: {}", plan.cache_posture);
    println!("evidence destination: {}", plan.evidence_destination);
    println!("closeout posture: {}", plan.closeout_posture);
    println!(
        "included packages: {}",
        plan.selection.included_packages.join(", ")
    );
    println!(
        "included proof partitions: {}",
        plan.selection.included_products.join(", ")
    );
    println!(
        "included targets: {}",
        plan.selection.included_targets.join(", ")
    );
    println!(
        "included suites: {}",
        plan.selection.included_suites.join(", ")
    );
    println!(
        "included fixtures: {}",
        plan.selection.included_fixtures.join(", ")
    );
    println!(
        "declared subprocess probes: {}",
        plan.selection.subprocess_probes.join(", ")
    );
    println!("selected units:");
    for unit in &plan.units {
        println!(
            "  - {}::{} [{}; case={}; profile={}; features={}; process={}; isolation={:?}; target-root={}; timeout={}ms; retries={}; dependencies={}]",
            unit.package,
            unit.target_name,
            unit.target_selector,
            unit.case_filter.as_deref().unwrap_or("all"),
            unit.build_profile.cargo_profile(),
            unit.feature_lane.description(),
            unit.process_model,
            unit.isolation,
            unit.resources.target_root,
            unit.timeout_millis,
            unit.retry.maximum_retries,
            unit.dependencies.join(",")
        );
    }
    println!("excluded products:");
    for (product, reason) in &plan.excluded_products {
        println!("  - {product}: {reason}");
    }
    println!("excluded packages:");
    for (package, reason) in &plan.selection.excluded_packages {
        println!("  - {package}: {reason}");
    }
    println!("excluded targets:");
    for (target, reason) in &plan.selection.excluded_targets {
        println!("  - {target}: {reason}");
    }
    println!("excluded suites:");
    for (suite, reason) in &plan.selection.excluded_suites {
        println!("  - {suite}: {reason}");
    }
    println!("excluded fixtures:");
    for (fixture, reason) in &plan.selection.excluded_fixtures {
        println!("  - {fixture}: {reason}");
    }
}

pub fn print_run(plan: &SelectedProofExecutionPlan, run: &ExecutedProofRun) {
    println!("behavioral verdict: {}", run.behavioral_verdict);
    println!(
        "execution breadth: {}/{} units",
        run.executed_units, run.planned_units
    );
    println!("responsibility verdicts:");
    for verdict in &run.unit_verdicts {
        println!(
            "  - {} case={} verdict={} elapsed={}ms process={}",
            verdict.unit_identity,
            verdict.case_filter.as_deref().unwrap_or("all"),
            verdict.behavioral_verdict,
            verdict.elapsed_millis,
            verdict.process_model
        );
    }
    println!(
        "observed runner processes: cargo={} requested-test-or-check={} declared-subprocess-receipts={}",
        run.observed_cost.cargo_processes_launched,
        run.observed_cost.test_or_check_processes_requested,
        run.observed_cost.declared_subprocess_evidence
    );
    println!(
        "compiler/link/child observation: {}; {}; {}",
        run.observed_cost.compiler_process_observation,
        run.observed_cost.linker_process_observation,
        run.observed_cost.child_process_observation
    );
    println!(
        "structural build breadth: compiler-artifacts={} linked-executables={}",
        run.observed_cost.cargo_compiler_artifact_messages,
        run.observed_cost.linked_executable_artifacts.len()
    );
    println!(
        "external observer breadth: processes={} compiler-samples={} linker-samples={} peak-descendants={} authority={}",
        run.observed_cost.externally_observed_processes,
        run.observed_cost.externally_observed_compilers,
        run.observed_cost.externally_observed_linkers,
        run.observed_cost.peak_observed_descendants,
        run.observed_cost.observer_authorities.join(",")
    );
    for skipped in &run.skipped_units {
        println!(
            "  - skipped {} reason={} blockers={}",
            skipped.unit_identity,
            skipped.reason,
            skipped.blocking_units.join(",")
        );
    }
    println!(
        "evidence identity: plan={} attempt={}",
        plan.plan_digest, run.run_identity
    );
    println!(
        "rerun: cargo {}",
        plan.product.replace(':', " --partition ")
    );
    println!("milestone closeout: unavailable; this is product evidence only");
}
