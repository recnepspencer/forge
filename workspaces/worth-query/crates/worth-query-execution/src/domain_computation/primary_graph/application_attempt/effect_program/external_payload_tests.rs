use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_declaration::facade::application_schema::{
    ApplicationEffectPayload, ApplicationExternalEffectPayload, ApplicationExternalEffectProtocol,
};
use worth_query_installation::facade::InstalledExternalEffectContract;

use super::model::{WorthQueryAdmittedApplicationEmissionBatch, WorthQueryApplicationEmission};

const EFFECT: &str = "ExternalEffect";
const EXTERNAL_PROTOCOL: ApplicationExternalEffectProtocol = ApplicationExternalEffectProtocol::new(
    BoundaryProtocolIdentity::new("test.external-payload"),
    BoundaryProtocolVersion::new(1),
);

#[derive(Clone)]
struct ExternalPayload(Vec<u8>);

impl ApplicationEffectPayload for ExternalPayload {
    fn retained_bytes(&self) -> u64 {
        u64::try_from(std::mem::size_of::<Self>() + self.0.capacity()).unwrap_or(u64::MAX)
    }
}

impl ApplicationExternalEffectPayload for ExternalPayload {
    const PROTOCOL: ApplicationExternalEffectProtocol = EXTERNAL_PROTOCOL;
    const MAX_EXTERNAL_BYTES: u64 = 8;

    fn external_effect_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

#[test]
fn exact_installed_external_emission_yields_its_projected_bytes() {
    let batch = admitted(vec![external(EFFECT, b"notice")]);

    assert_eq!(
        batch.external_payload(&contract(EFFECT)),
        Ok(Some(b"notice".to_vec()))
    );
}

#[test]
fn ordinary_emission_cannot_satisfy_an_external_contract() {
    let batch = admitted(vec![WorthQueryApplicationEmission::new(
        EFFECT,
        ExternalPayload(b"notice".to_vec()),
    )]);

    assert_eq!(
        batch.external_payload(&contract(EFFECT)),
        Err("declared external effect used an ordinary non-external emission")
    );
}

#[test]
fn missing_wrong_and_duplicate_external_emissions_are_rejected() {
    assert_eq!(
        admitted(Vec::new()).external_payload(&contract(EFFECT)),
        Err("declared external effect did not emit its installed typed payload")
    );
    assert_eq!(
        admitted(vec![external("WrongEffect", b"notice")]).external_payload(&contract(EFFECT)),
        Err("declared external effect did not emit its installed typed payload")
    );
    assert_eq!(
        admitted(vec![external(EFFECT, b"one"), external(EFFECT, b"two")])
            .external_payload(&contract(EFFECT)),
        Err("declared external effect emitted its installed payload more than once")
    );
}

#[test]
fn installed_bound_drift_is_rejected() {
    let mut contract = contract(EFFECT);
    let InstalledExternalEffectContract::Declared {
        maximum_payload_bytes,
        ..
    } = &mut contract
    else {
        unreachable!()
    };
    *maximum_payload_bytes = 7;

    assert_eq!(
        admitted(vec![external(EFFECT, b"notice")]).external_payload(&contract),
        Err("external payload projection drifted from its installed bound")
    );
}

fn contract(effect: &str) -> InstalledExternalEffectContract {
    InstalledExternalEffectContract::Declared {
        correlation_family: "external-family".to_owned(),
        effect: effect.to_owned(),
        rust_payload_type: std::any::type_name::<ExternalPayload>().to_owned(),
        protocol: EXTERNAL_PROTOCOL,
        maximum_payload_bytes: ExternalPayload::MAX_EXTERNAL_BYTES,
    }
}

fn external(effect: &'static str, bytes: &[u8]) -> WorthQueryApplicationEmission {
    WorthQueryApplicationEmission::new_external(effect, ExternalPayload(bytes.to_vec()))
        .expect("fixture projection stays within its declared bound")
}

fn admitted(
    emissions: Vec<WorthQueryApplicationEmission>,
) -> WorthQueryAdmittedApplicationEmissionBatch {
    WorthQueryAdmittedApplicationEmissionBatch::admit(emissions, 1_024)
        .expect("fixture batch stays within its retained-byte ceiling")
}
