use worth_query::facade::domain::{
    WorthQueryDomainRebindRequest, WorthQueryInstalledDomainInspectionOutcome,
};

fn promote_diagnostic<D>(
    inspection: WorthQueryInstalledDomainInspectionOutcome<D>,
) -> WorthQueryDomainRebindRequest<D> {
    inspection
}

fn main() {}
