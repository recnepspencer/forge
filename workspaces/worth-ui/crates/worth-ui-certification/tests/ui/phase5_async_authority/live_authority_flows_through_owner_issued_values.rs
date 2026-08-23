use worth_ui_query_binding::{
    WorthUiPresentationAsyncOwner, WorthUiPresentationCorrespondenceIssuer,
    WorthUiPresentationRequestBasis,
};

fn owner_issued_values_form_the_only_lawful_path(
    owner: &mut WorthUiPresentationAsyncOwner,
    issuer: &mut WorthUiPresentationCorrespondenceIssuer,
    basis: WorthUiPresentationRequestBasis,
) {
    let correspondence = issuer.issue(basis).expect("correspondence authority");
    let pending = owner
        .admit_pending(correspondence)
        .expect("Query admission authority");
    let completion = issuer.certify_presented(&pending, 64);
    let _presented = owner
        .admit_presented(&pending, completion)
        .expect("owner-issued completion authority");
}

fn main() {}
