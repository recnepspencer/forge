pub(super) const EVIDENCE_SCHEMA: &str = "worth-ui-ledger-evidence-v3";
pub(super) const FONT_PROFILE_DIGEST: &str =
    "6f140249866e6815e9284fe1c8c959a8bb1b8cab252cfbe8c7c397f9a7eb9b01";
pub(super) const NATIVE_PROFILE_DIGEST: &str =
    "1c937a22f42660267480a055e48256b25decf0c4cd5d4d7b493e5df034c6c65b";

pub(super) struct RequirementContract {
    pub(super) requirement: &'static str,
    pub(super) owner: &'static str,
    pub(super) boundary: &'static str,
    pub(super) world: &'static str,
    pub(super) proof_kind: &'static str,
    pub(super) authority: &'static str,
    pub(super) mutation_family: &'static str,
    pub(super) counter_family: &'static str,
}

impl RequirementContract {
    pub(super) fn requires_presented_source(&self) -> bool {
        matches!(
            self.requirement,
            "P2-GRAPHICS-01"
                | "P2-PRESENT-01"
                | "P2-PIXELS-01"
                | "P2-WORLD-01"
                | "P3-DAMAGE-REPLAY-01"
                | "P3-HP02-WORLD-01"
                | "P3-PHYSICAL-AMPLIFICATION-01"
        )
    }

    pub(super) fn requires_client_area(&self) -> bool {
        matches!(self.requirement, "P2-PIXELS-01" | "P2-WORLD-01")
    }
}

pub(super) fn for_requirement(requirement: &str) -> Option<&'static RequirementContract> {
    CONTRACTS
        .iter()
        .find(|contract| contract.requirement == requirement)
}

macro_rules! contract {
    ($requirement:literal, $owner:literal, $boundary:literal, $world:literal,
     $proof:literal, $authority:literal, $mutation:literal, $counter:literal) => {
        RequirementContract {
            requirement: $requirement,
            owner: $owner,
            boundary: $boundary,
            world: $world,
            proof_kind: $proof,
            authority: $authority,
            mutation_family: $mutation,
            counter_family: $counter,
        }
    };
}

pub(super) const CONTRACTS: &[RequirementContract] = &[
    contract!(
        "P1-AFFINITY-01",
        "worth-ui-runtime",
        "initial delta unchanged affinity",
        "mounted-presentation-world",
        "runtime-model",
        "worth_ui_runtime::mounting::presentation",
        "affinity",
        "work"
    ),
    contract!(
        "P1-AUTHORITY-01",
        "worth-ui-runtime",
        "native application preparation types",
        "native-application-world",
        "compile-runtime",
        "worth_ui_runtime::native_platform::application",
        "construction",
        "preparation"
    ),
    contract!(
        "P1-BACKEND-FEATURES-01",
        "worth-ui-host-native",
        "resolved native backend feature posture",
        "host-retirement-topology-world",
        "resolved-topology",
        "worth_ui_host_native::qualified_profile",
        "backend-feature",
        "resolved-feature"
    ),
    contract!(
        "P1-BASELINE-01",
        "worth-ui-runtime",
        "transparent surface baseline",
        "mounted-presentation-world",
        "compile-runtime",
        "worth_ui_runtime::mounting::host_truth",
        "baseline",
        "baseline"
    ),
    contract!(
        "P1-CLOSE-01",
        "worth-ui-certification",
        "phase one final source closure",
        "phase-one-ledger-world",
        "ledger-closure",
        "worth_ui_certification::phase_one_ledger",
        "ledger",
        "requirements"
    ),
    contract!(
        "P1-CONSUMERS-01",
        "egui-headless-certification",
        "revision 4 consumers",
        "mounted-presentation-world",
        "integration",
        "worth_ui_runtime::host_composition",
        "validated-agreement",
        "consumer"
    ),
    contract!(
        "P1-DAMAGE-01",
        "worth-ui-runtime",
        "runtime issued logical damage",
        "mounted-presentation-world",
        "runtime-model",
        "worth_ui_runtime::mounting::presentation",
        "damage",
        "damage"
    ),
    contract!(
        "P1-HEADLESS-01",
        "worth-ui-host-headless",
        "contract only headless mechanics",
        "mounted-presentation-world",
        "dependency-integration",
        "worth_ui_host_headless::adapter",
        "mechanics-substitution",
        "headless"
    ),
    contract!(
        "P1-HEADLESS-COST-01",
        "worth-ui-host-headless",
        "sparse headless carrier consumption",
        "mounted-presentation-world",
        "carrier-model",
        "worth_ui_host_headless::retained_presentation",
        "carrier-inflation",
        "carrier-cost"
    ),
    contract!(
        "P1-ORDER-01",
        "worth-ui-runtime",
        "runtime issued total paint order",
        "mounted-presentation-world",
        "runtime-model",
        "worth_ui_runtime::mounting::presentation",
        "paint-order",
        "order"
    ),
    contract!(
        "P1-ORDER-SOURCE-01",
        "worth-ui-runtime",
        "stable order identity provenance",
        "mounted-presentation-world",
        "compile-runtime",
        "worth_ui_runtime::mounting::order",
        "identity-perturbation",
        "order-source"
    ),
    contract!(
        "P1-PLATFORM-AUTHORITY-01",
        "worth-ui-runtime",
        "affine native platform binding",
        "native-application-world",
        "compile-runtime",
        "worth_ui_runtime::native_platform::binding",
        "grant-forgery",
        "grant"
    ),
    contract!(
        "P1-PREPARATION-LIFECYCLE-01",
        "worth-ui-runtime",
        "effect-free host-neutral product preparation",
        "native-application-world",
        "compile-runtime",
        "worth_ui_runtime::native_platform::preparation",
        "premature-runtime-effect",
        "effect-surface"
    ),
    contract!(
        "P1-PRESENTATION-AUTHORITY-01",
        "worth-ui-runtime",
        "runtime private presentation issuance",
        "mounted-presentation-world",
        "compile-runtime",
        "worth_ui_runtime::mounting::presentation::authority",
        "work-forgery",
        "authority"
    ),
    contract!(
        "P1-PRODUCER-01",
        "worth-ui-runtime",
        "mounted presentation work producer",
        "mounted-presentation-world",
        "integration",
        "worth_ui_runtime::mounting::presentation",
        "delta-carriage",
        "producer"
    ),
    contract!(
        "P1-PRODUCER-COST-01",
        "worth-ui-runtime",
        "sparse presentation work carrier",
        "mounted-presentation-world",
        "carrier-model",
        "worth_ui_runtime::mounting::presentation",
        "carrier-inflation",
        "carrier-cost"
    ),
    contract!(
        "P1-PROFILE-01",
        "worth-ui-host-native",
        "qualified profile manifests",
        "profile-qualification",
        "manifest-qualification",
        "worth_ui_host_native::qualified_profile",
        "manifest-field",
        "profile"
    ),
    contract!(
        "P1-PROTOCOL-01",
        "worth-ui-host-contract",
        "mounted presentation protocol",
        "mounted-presentation-world",
        "compile-runtime",
        "worth_ui_host_contract::mounted_protocol",
        "protocol-revision",
        "protocol"
    ),
    contract!(
        "P1-TOPOLOGY-01",
        "worth-ui-certification",
        "dependency and compiler enforcement",
        "host-retirement-topology-world",
        "repository-topology",
        "worth_ui_certification::host_topology",
        "hidden-edge",
        "inventory"
    ),
    contract!(
        "P1-WORLDS-01",
        "worth-ui-certification",
        "governed worlds and independent oracles",
        "mounted-presentation-world",
        "production-courtroom",
        "worth_ui_certification::mounted_world",
        "oracle-substitution",
        "world"
    ),
    contract!(
        "P2-APPLICATION-01",
        "worth-ui-runtime",
        "prepared application driver handoff",
        "windows-native-boundary-world",
        "windows-integration",
        "worth_ui_runtime::native_platform::application_driver",
        "driver-substitution",
        "application"
    ),
    contract!(
        "P2-CLOSE-01",
        "worth-ui-host-native",
        "terminal native resource cleanup",
        "windows-native-boundary-world",
        "windows-integration",
        "worth_ui_host_native::shutdown",
        "resource-leak",
        "resource-census"
    ),
    contract!(
        "P2-EVENT-LOOP-01",
        "worth-ui-host-native",
        "event loop thread ownership",
        "windows-native-boundary-world",
        "windows-integration",
        "worth_ui_host_native::event_loop",
        "thread-substitution",
        "event-loop"
    ),
    contract!(
        "P2-GRAPHICS-01",
        "worth-ui-host-native",
        "device queue and retained target ownership",
        "windows-native-boundary-world",
        "windows-integration",
        "worth_ui_host_native::graphics",
        "backend-substitution",
        "graphics"
    ),
    contract!(
        "P2-PIXELS-01",
        "worth-ui-certification",
        "independent client area pixel observation",
        "windows-native-boundary-world",
        "external-observation",
        "worth_ui_certification::windows_pixels",
        "expected-pixel-substitution",
        "pixels"
    ),
    contract!(
        "P2-PORTS-01",
        "worth-ui-host-native",
        "production external effect ports",
        "windows-native-boundary-world",
        "windows-integration",
        "worth_ui_host_native::external_ports",
        "scripted-port-substitution",
        "ports"
    ),
    contract!(
        "P2-PRESENT-01",
        "worth-ui-host-native",
        "attributed initial filled rectangle",
        "windows-native-boundary-world",
        "windows-integration",
        "worth_ui_host_native::presentation",
        "geometry-color-substitution",
        "presentation"
    ),
    contract!(
        "P2-READINESS-01",
        "worth-ui-host-native",
        "level triggered scheduling and quiescence",
        "windows-native-boundary-world",
        "lifecycle-model",
        "worth_ui_host_native::readiness",
        "wake-drop-duplicate",
        "readiness"
    ),
    contract!(
        "P2-WINDOW-01",
        "worth-ui-host-native",
        "windows window surface and dpi lifecycle",
        "windows-native-boundary-world",
        "windows-integration",
        "worth_ui_host_native::window",
        "window-substitution",
        "window"
    ),
    contract!(
        "P2-WORLD-01",
        "worth-ui-certification",
        "environment qualified windows boundary world",
        "windows-native-boundary-world",
        "production-courtroom",
        "worth_ui_certification::windows_native_world",
        "world-substitution",
        "world"
    ),
];
