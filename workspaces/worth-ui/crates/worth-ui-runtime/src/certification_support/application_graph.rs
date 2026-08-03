use crate::declaration::UiDeclarationIdentity;
use crate::facade::WorthUiApp;
use crate::graph::UiRepeatedInstanceBasisKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRepeatedInstanceIdentityCertificationRow {
    graph_node_identity_digest: u64,
    repeated_instance_basis_digest: u64,
    repeated_instance_basis_kind: UiRepeatedInstanceBasisKind,
}

pub trait WorthUiApplicationGraphCertificationExt {
    fn graph_world_identity_digest(&self) -> u64;

    fn repeated_instance_identity_rows(
        &self,
        declaration: &UiDeclarationIdentity,
    ) -> Box<[UiRepeatedInstanceIdentityCertificationRow]>;
}

impl WorthUiApplicationGraphCertificationExt for WorthUiApp {
    fn graph_world_identity_digest(&self) -> u64 {
        self.graph_snapshot().world_profile().identity_digest()
    }

    fn repeated_instance_identity_rows(
        &self,
        declaration: &UiDeclarationIdentity,
    ) -> Box<[UiRepeatedInstanceIdentityCertificationRow]> {
        let graph = self.graph_snapshot();
        let mut rows = graph
            .lookup()
            .declaration_instances(declaration)
            .value()
            .iter()
            .map(|identity| {
                let node = graph
                    .lookup()
                    .graph_node(*identity)
                    .expect("issued declaration instance remains in its frozen graph")
                    .value();
                UiRepeatedInstanceIdentityCertificationRow {
                    graph_node_identity_digest: node.graph_node_identity().digest(),
                    repeated_instance_basis_digest: node
                        .repeated_instance_basis()
                        .identity_digest(),
                    repeated_instance_basis_kind: node.repeated_instance_basis().kind(),
                }
            })
            .collect::<Vec<_>>();
        rows.sort_unstable_by_key(|row| {
            (
                row.graph_node_identity_digest,
                row.repeated_instance_basis_digest,
            )
        });
        rows.into_boxed_slice()
    }
}

impl UiRepeatedInstanceIdentityCertificationRow {
    pub const fn graph_node_identity_digest(self) -> u64 {
        self.graph_node_identity_digest
    }

    pub const fn repeated_instance_basis_digest(self) -> u64 {
        self.repeated_instance_basis_digest
    }

    pub const fn repeated_instance_basis_kind(self) -> UiRepeatedInstanceBasisKind {
        self.repeated_instance_basis_kind
    }
}
