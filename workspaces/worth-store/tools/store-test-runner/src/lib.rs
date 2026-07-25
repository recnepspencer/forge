mod arguments;
#[cfg(test)]
mod c5_1_sealing_gate;
mod catalog;
mod classification;
#[cfg(feature = "physical-work-evidence")]
mod courtroom_campaign;
mod execution;
mod local_source_fingerprint;
mod mutation_campaign;
#[cfg(feature = "physical-work-evidence")]
pub mod physical_work_evidence;
#[cfg(test)]
mod physical_writer_gate;
mod plan;
mod product;
mod report;

use std::path::{Path, PathBuf};

use arguments::Arguments;
use catalog::TestCatalog;

pub fn run_from_environment() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments::help_requested(&arguments) {
        println!("{}", arguments::usage());
        return Ok(());
    }
    let arguments = Arguments::parse(arguments)?;
    run(arguments, &workspace_root())
}

fn run(arguments: Arguments, workspace_root: &Path) -> Result<(), String> {
    if matches!(arguments.product, product::TestProduct::Mutants) {
        return mutation_campaign::run(
            workspace_root,
            mutation_campaign::MutationCampaignRequest {
                scope: arguments.mutation_scope,
                list: arguments.list,
                selected: arguments.mutant,
                first: arguments.first_mutant,
                report: arguments.report.as_deref(),
            },
        );
    }
    if let product::TestProduct::Courtrooms { courtroom } = arguments.product {
        #[cfg(feature = "physical-work-evidence")]
        {
            return courtroom_campaign::run(
                workspace_root,
                courtroom_campaign::CourtroomCampaignRequest {
                    courtroom,
                    list: arguments.list,
                    target_root: arguments.target_root.as_deref(),
                    mutant_report: arguments.mutant_report.as_deref(),
                    report: arguments.report.as_deref(),
                },
            );
        }
        #[cfg(not(feature = "physical-work-evidence"))]
        {
            let _ = courtroom;
            return Err(
                "courtroom campaigns require the runner feature `physical-work-evidence`".into(),
            );
        }
    }
    let catalog = TestCatalog::load(workspace_root)?;
    let plan = plan::TestPlan::build(&arguments.product, &catalog, workspace_root)?;

    if arguments.list {
        for unit in plan.units() {
            println!("{}\t{}", unit.identity(), unit.display_command());
        }
        return Ok(());
    }

    println!("{}: {} unit(s)", plan.product_name(), plan.units().len());
    let report = execution::execute(&plan, arguments.target_root.as_deref());
    if let Some(path) = arguments.report.as_deref() {
        report::write(path, &report)?;
        println!("report: {}", path.display());
    }
    println!(
        "{}: {} in {:.2}s",
        plan.product_name(),
        if report.success { "passed" } else { "failed" },
        report.elapsed_ms as f64 / 1_000.0
    );
    if report.success {
        Ok(())
    } else {
        Err(report
            .failure
            .unwrap_or_else(|| "test execution failed".into()))
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("runner must live under tools/<crate>")
        .to_path_buf()
}
