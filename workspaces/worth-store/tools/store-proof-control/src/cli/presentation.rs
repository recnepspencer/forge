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
            "  - {}::{} [{}; case={}; profile={}; features={}; process={}]",
            unit.package,
            unit.target_name,
            unit.target_selector,
            unit.case_filter.as_deref().unwrap_or("all"),
            unit.build_profile.cargo_profile(),
            unit.feature_lane.description(),
            unit.process_model
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
        run.completed_units, run.attempted_units
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
        "observed runner processes: cargo={} requested-test-or-check={} declared-subprocess-units={}",
        run.process_counts.cargo_processes_launched,
        run.process_counts.test_or_check_processes_requested,
        run.process_counts.declared_subprocess_units
    );
    println!(
        "compiler/link/child observation: {}; {}; {}",
        run.process_counts.compiler_process_observation,
        run.process_counts.linker_process_observation,
        run.process_counts.child_process_observation
    );
    println!(
        "evidence identity: plan={} attempt={}",
        plan.plan_digest, run.attempt_identity
    );
    println!(
        "rerun: cargo {}",
        plan.product.replace(':', " --partition ")
    );
    println!("milestone closeout: unavailable; this is product evidence only");
}
