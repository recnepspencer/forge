use hadwiger_research::facade::{
    HadwigerDependencyClosureBlocker, HadwigerDependencyClosureReport,
};

fn mutate_report(report: &mut HadwigerDependencyClosureReport, blocker: HadwigerDependencyClosureBlocker) {
    report.blockers.push(blocker);
}

fn main() {
    let _ = mutate_report
        as fn(&mut HadwigerDependencyClosureReport, HadwigerDependencyClosureBlocker);
}
