mod arguments;
mod catalog;
mod classification;
mod execution;
mod mutation_campaign;
#[cfg(test)]
mod physical_writer_gate;
mod plan;
mod product;
mod report;

use std::path::{Path, PathBuf};

use arguments::Arguments;
use catalog::TestCatalog;

pub fn run_from_environment() -> Result<(), String> {
    let arguments = Arguments::parse(std::env::args().skip(1))?;
    run(arguments, &workspace_root())
}

fn run(arguments: Arguments, workspace_root: &Path) -> Result<(), String> {
    if matches!(arguments.product, product::TestProduct::Mutants) {
        if arguments.report.is_some() {
            return Err(
                "mutants emits one evidence record per mutation; --report is not supported".into(),
            );
        }
        return mutation_campaign::run(
            workspace_root,
            arguments.list,
            arguments.mutant,
            arguments.first_mutant,
        );
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
