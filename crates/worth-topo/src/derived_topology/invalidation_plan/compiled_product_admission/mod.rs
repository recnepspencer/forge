mod admitted_input;
mod denial;
mod locality_basis;
mod prior_proof_basis;
mod request;
mod source_authority_basis;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use denial::TopologyCompiledProductAdmissionErrorKind;
pub(crate) use locality_basis::TopologyCompiledProductLocalityBasis;
pub(crate) use request::TopologyCompiledProductAdmissionRequest;
pub(crate) use source_authority_basis::TopologyCompiledProductSourceAuthorityBasis;

use crate::compiled_product_family::{
    TopologyCompiledProductFamilyAdmittedInput, TopologyCompiledProductFamilyCatalog,
    TopologyCompiledProductFamilyDeclaration,
};

use self::admitted_input::TopologyCompiledProductAdmittedInput as AdmittedInput;
use self::denial::{
    TopologyCompiledProductAdmissionError as AdmissionError,
    TopologyCompiledProductAdmissionErrorKind as AdmissionErrorKind,
};
use self::prior_proof_basis::TopologyCompiledProductPriorProofBasis;
use self::request::TopologyCompiledProductAdmissionRequest as AdmissionRequest;

pub(crate) fn admit_topology_compiled_product_input(
    catalog: &TopologyCompiledProductFamilyCatalog,
    request: AdmissionRequest<'_>,
) -> Result<AdmittedInput, AdmissionError> {
    let declaration = family_declaration(catalog, request)?;
    let source_authority_basis =
        TopologyCompiledProductSourceAuthorityBasis::from_read_basis(request.read_basis())?;
    let prior_proof_basis = TopologyCompiledProductPriorProofBasis::admit(
        declaration.prior_proof(),
        request.selected_plan(),
        request.touched_closure(),
    )?;
    let locality_basis = match request.touched_closure() {
        Some(touched_closure) => TopologyCompiledProductLocalityBasis::from_selected_plan(
            request.read_basis(),
            touched_closure,
        )?,
        None => TopologyCompiledProductLocalityBasis::from_read_basis(request.read_basis()),
    };
    let family_admitted_input = TopologyCompiledProductFamilyAdmittedInput::from_admission_bases(
        request.consumer(),
        declaration.identity(),
        &source_authority_basis,
        &locality_basis,
    );
    Ok(AdmittedInput::new(
        family_admitted_input,
        source_authority_basis,
        locality_basis,
        prior_proof_basis,
    ))
}

fn family_declaration<'a>(
    catalog: &'a TopologyCompiledProductFamilyCatalog,
    request: AdmissionRequest<'a>,
) -> Result<&'a TopologyCompiledProductFamilyDeclaration, AdmissionError> {
    catalog
        .family_for_consumer(request.consumer())
        .ok_or_else(|| {
            AdmissionError::new(
                AdmissionErrorKind::NoDeclaredFamilyForConsumer,
                format!(
                    "no topology compiled-product family declaration exists for {:?}",
                    request.consumer()
                ),
            )
        })
}
