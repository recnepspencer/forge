use worth_foundational::facade::{canonicalization, CanonicalizationRuleVersion};

use super::basis::{primitive, structured, text, unsigned, DimensionBasis};
use super::dimension::{
    WorthQueryPortableConditionalDimension as Dimension,
    WorthQueryPortableConditionalOutputPart as OutputPart,
};
use super::value_basis::{
    consequence_kind, locality_values, output_kind, workflow_value_contract_name,
};
use crate::domain_operation::{
    WorthQueryConditionalConsequenceRole, WorthQueryConditionalNodeOutput,
    WorthQueryPortableConditionalNodeDeclaration,
};

pub(super) fn append_outputs(
    bases: &mut Vec<DimensionBasis>,
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    version: &CanonicalizationRuleVersion,
) {
    bases.push(primitive(
        Dimension::OutputWidth,
        unsigned(declaration.outputs().len()),
        version,
    ));
    for (index, output) in declaration.outputs().iter().enumerate() {
        let index = index as u32;
        let dimension = |part| Dimension::Output { index, part };
        bases.push(primitive(
            dimension(OutputPart::Kind),
            text(output_kind(output)),
            version,
        ));
        match output {
            WorthQueryConditionalNodeOutput::DerivedAspect {
                contract,
                locality,
                consequences,
            } => {
                let contract = canonicalization()
                    .basis()
                    .at(version.clone())
                    .from_contract(contract.clone())
                    .into_result()
                    .expect("admitted output contract has a canonical basis");
                bases.push(DimensionBasis {
                    dimension: dimension(OutputPart::Contract),
                    foundational: contract,
                });
                bases.push(structured(
                    dimension(OutputPart::Locality),
                    locality_values(locality),
                    version,
                ));
                append_consequences(bases, consequences, index, version);
            }
            WorthQueryConditionalNodeOutput::OperationOutput { projection_role } => {
                bases.push(primitive(
                    dimension(OutputPart::ProjectionRole),
                    text(projection_role.as_str()),
                    version,
                ));
            }
            WorthQueryConditionalNodeOutput::WorkflowStageOutput { contract } => {
                bases.push(primitive(
                    dimension(OutputPart::WorkflowValueContract),
                    text(workflow_value_contract_name(*contract)),
                    version,
                ));
            }
        }
    }
}

fn append_consequences(
    bases: &mut Vec<DimensionBasis>,
    consequences: &[WorthQueryConditionalConsequenceRole],
    output_index: u32,
    version: &CanonicalizationRuleVersion,
) {
    let dimension = |part| Dimension::Output {
        index: output_index,
        part,
    };
    bases.push(primitive(
        dimension(OutputPart::ConsequenceWidth),
        unsigned(consequences.len()),
        version,
    ));
    for (index, consequence) in consequences.iter().enumerate() {
        let index = index as u32;
        bases.push(primitive(
            dimension(OutputPart::ConsequenceKind(index)),
            text(consequence_kind(consequence)),
            version,
        ));
        match consequence {
            WorthQueryConditionalConsequenceRole::DerivedOnly => {}
            WorthQueryConditionalConsequenceRole::Touch(touch) => {
                bases.push(primitive(
                    dimension(OutputPart::ConsequenceTouchGraphRole(index)),
                    text(touch.graph_role()),
                    version,
                ));
                bases.push(primitive(
                    dimension(OutputPart::ConsequenceTouchScope(index)),
                    text(touch.scope()),
                    version,
                ));
            }
            WorthQueryConditionalConsequenceRole::Effect(family) => bases.push(primitive(
                dimension(OutputPart::ConsequenceEffectFamily(index)),
                text(family.as_str()),
                version,
            )),
        }
    }
}

pub(super) fn append_context(
    bases: &mut Vec<DimensionBasis>,
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    version: &CanonicalizationRuleVersion,
) {
    bases.push(primitive(
        Dimension::RequiredContextWidth,
        unsigned(declaration.required_context().len()),
        version,
    ));
    bases.extend(
        declaration
            .required_context()
            .iter()
            .enumerate()
            .map(|(index, context)| {
                primitive(
                    Dimension::RequiredContext(index as u32),
                    text(super::super::node_posture::context_name(*context)),
                    version,
                )
            }),
    );
}
