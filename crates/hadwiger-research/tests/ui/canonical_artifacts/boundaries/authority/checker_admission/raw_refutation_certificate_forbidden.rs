use hadwiger_research::facade::ColoringRefutationCertificate;

fn main() {
    let _ = ColoringRefutationCertificate::checked_exhaustive(1, 2, vec![vec![1], vec![-1]]);
}
