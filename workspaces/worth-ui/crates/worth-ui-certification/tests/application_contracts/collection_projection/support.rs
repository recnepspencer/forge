use worth_query::facade::{foundation::WorthQueryEntityIdentity, runtime::WorthQueryWorkspace};
use worth_ui_query_binding::{
    certification::{
        insert_projection_status, seeded_collection_projection_workspace,
        update_projection_identity, update_projection_status_batch,
        WorthUiCollectionProjectionSeedPosture,
    },
    UiCollectionProjectionBindingAdmission, UiCollectionProjectionBudget,
    UiCollectionProjectionFactReceipt, UiCollectionProjectionOpenOutcome,
    UiCollectionProjectionRefreshOutcome, UiCollectionProjectionRefreshReceipt,
    UiCollectionProjectionRegistration, UiLiveCollectionProjection,
    UiLiveCollectionProjectionCloseOutcome, UiProjectionFieldRequirement, WorthUiQueryWorkspaceExt,
};

use super::oracle::ExpectedKeyedRows;

pub(super) enum WorldPosture {
    Complete,
    Partial,
    ResetOnly,
}

pub(super) struct CollectionProjectionWorld {
    workspace: WorthQueryWorkspace,
    live: Option<UiLiveCollectionProjection>,
    entities: Vec<WorthQueryEntityIdentity>,
    identities: Vec<String>,
    expected: ExpectedKeyedRows,
}

impl CollectionProjectionWorld {
    pub(super) fn open(
        row_count: usize,
        max_rows: u32,
        posture: WorldPosture,
        requires_complete: bool,
    ) -> (Self, UiCollectionProjectionFactReceipt) {
        let seed_posture = match posture {
            WorldPosture::Complete => WorthUiCollectionProjectionSeedPosture::Complete,
            WorldPosture::Partial => WorthUiCollectionProjectionSeedPosture::Partial,
            WorldPosture::ResetOnly => WorthUiCollectionProjectionSeedPosture::ResetOnly,
        };
        let authored = authored_rows(row_count);
        let (mut workspace, entities) =
            seeded_collection_projection_workspace(authored.clone(), seed_posture);
        let mut expected = ExpectedKeyedRows::default();
        let identities = entities
            .iter()
            .zip(authored.iter())
            .map(|(entity, (_, value))| {
                let identity = reporting_identity(entity);
                expected.insert(identity.clone(), value.clone());
                identity
            })
            .collect();
        let (live, fact) = open_live(&mut workspace, max_rows, requires_complete);
        (
            Self {
                workspace,
                live: Some(live),
                entities,
                identities,
                expected,
            },
            fact,
        )
    }

    pub(super) fn expected(&self) -> &ExpectedKeyedRows {
        &self.expected
    }

    pub(super) fn identities(&self) -> &[String] {
        &self.identities
    }

    pub(super) fn cardinality(&self) -> usize {
        self.entities.len()
    }

    pub(super) fn update_first(&mut self, count: usize) -> Vec<String> {
        let selected = self.identities[..count].to_vec();
        let updates = self.entities[..count]
            .iter()
            .zip(selected.iter())
            .enumerate()
            .map(|(index, (entity, identity))| {
                let value = format!("Updated {index:05}");
                self.expected.update(identity, value.clone());
                (entity.clone(), value)
            })
            .collect();
        update_projection_status_batch(&mut self.workspace, updates);
        selected
    }

    pub(super) fn insert(&mut self, authored_identity: &str, value: &str) -> String {
        let entity = insert_projection_status(&mut self.workspace, authored_identity, value);
        let identity = reporting_identity(&entity);
        self.entities.push(entity);
        self.identities.push(identity.clone());
        self.expected.insert(identity.clone(), value);
        identity
    }

    pub(super) fn remove(&mut self, index: usize) -> String {
        let entity = self.entities.remove(index);
        let identity = self.identities.remove(index);
        self.workspace
            .delete(entity)
            .expect("QP04 collection deletion");
        self.expected.remove(&identity);
        identity
    }

    pub(super) fn reorder(&mut self, index: usize, authored_identity: &str) -> String {
        let entity = self.entities[index].clone();
        let identity = self.identities[index].clone();
        update_projection_identity(&mut self.workspace, entity, authored_identity);
        identity
    }

    pub(super) fn refresh(&mut self) -> UiCollectionProjectionFactReceipt {
        self.refresh_receipt().into_fact()
    }

    pub(super) fn refresh_receipt(&mut self) -> UiCollectionProjectionRefreshReceipt {
        match self
            .live
            .as_mut()
            .expect("live collection owner")
            .refresh(&mut self.workspace)
            .expect("QP04 real Query refresh")
        {
            UiCollectionProjectionRefreshOutcome::Applied(receipt) => receipt,
            UiCollectionProjectionRefreshOutcome::NoSemanticDelivery => {
                panic!("QP04 mutation must deliver semantic collection meaning")
            }
        }
    }

    pub(super) fn close(mut self) {
        let live = self.live.take().expect("live collection owner");
        let UiLiveCollectionProjectionCloseOutcome::Closed(closed) =
            live.close(&mut self.workspace)
        else {
            panic!("QP04 live collection owner must close");
        };
        assert!(closed.owner_terminal(), "the exact Query owner must retire");
    }
}

fn authored_rows(row_count: usize) -> Vec<(String, String)> {
    (0..row_count)
        .map(|index| (format!("pulse.{index:05}"), format!("Value {index:05}")))
        .collect()
}

fn reporting_identity(entity: &WorthQueryEntityIdentity) -> String {
    entity
        .evidence_identity()
        .terminal_projection_for_reporting()
        .to_owned()
}

fn open_live(
    workspace: &mut WorthQueryWorkspace,
    max_rows: u32,
    requires_complete: bool,
) -> (
    UiLiveCollectionProjection,
    UiCollectionProjectionFactReceipt,
) {
    let installed = workspace.worth_ui().expect("WORTH UI domain installed");
    let registration = UiCollectionProjectionRegistration::text(
        installed
            .projection_view("certification.collection.qp04")
            .expect("QP04 installed collection view"),
        UiProjectionFieldRequirement::declared("identity.id").expect("row identity field"),
        [UiProjectionFieldRequirement::declared("status").expect("selected field")],
        requires_complete,
        true,
    )
    .expect("QP04 collection registration");
    let UiCollectionProjectionBindingAdmission::Ready(binding) = registration.admit(workspace)
    else {
        panic!("QP04 binding must admit");
    };
    let budget = UiCollectionProjectionBudget::new(max_rows, 131_072, 1, 8_388_608)
        .expect("QP04 collection budget");
    let UiCollectionProjectionOpenOutcome::Opened(opened) = binding.open(budget, workspace) else {
        panic!("QP04 collection projection must open");
    };
    opened.into_parts()
}
