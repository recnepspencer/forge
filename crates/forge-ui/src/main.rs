//! Forge Query todo showcase.
//!
//! Query declares live read surfaces and writeback artifacts; the runtime
//! bridge owns authoritative side effects.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};

use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Margin, RichText, Stroke};
use forge_query::facade::{
    declare_runtime_live_query_session, declare_writeback_from_live_session,
    DeclarativeLiveQueryRequest, DeclarativeLiveQuerySession, DeclarativeLiveViewShape,
    DeclarativeProjectionField, DeclarativeWritebackArtifact, DeclarativeWritebackIntent,
    DeclarativeWritebackValue, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
};
use forge_runtime_bridge::facade::{
    materialize_bridge_row_set, BridgeCommittedPatchItem, BridgeDeliveryReceipt,
    BridgeExecutionPolicyClass, BridgeMappingId, BridgeMappingRegistration,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgePreviewResidueClass,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgeRequestKind, BridgeRuntimePolicy,
    BridgeSignalBranchIdentity, BridgeSignalInvalidationDelivery, BridgeSourceAdapter,
    BridgeSourceCapability, BridgeSourceCapabilitySet, BridgeSpeculativePromotionRequest,
    BridgeSpeculativeSessionHandle, BridgeSpeculativeSessionRequest,
    BridgeTruthViewEvaluationRequest, BridgeTruthViewSelector, BridgeWritebackCausalityBasis,
    BridgeWritebackCausalityIdentity, BridgeWritebackEffectIdentity, CoarseRoutingMode,
    CommittedPatchSource, InvalidationSink, LoweredBridgeExecutionPolicy, MappingSelector,
    RawCommittedPatchEnvelope, RelationalBridgeSourceError, RelationalCommittedPatchRequest,
    RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
    SnapshotReadSource, SourceDeclaration, SourceDeclarationIdentity, TruthBranchHeadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader, TruthWritebackAuthority,
    TruthWritebackAuthorityError, TruthWritebackReceipt, TruthWritebackRequest,
};

const MAIN_BRANCH: &str = "main";
const PREVIEW_BRANCH: &str = "preview:sprint-next";
const SNAPSHOT_PREFIX: &str = "snapshot:";

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Forge Query Sprint")
            .with_inner_size([1380.0, 860.0])
            .with_min_inner_size([980.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Forge Query Sprint",
        native_options,
        Box::new(|cc| Ok(Box::new(TodoShowcaseApp::new(cc)))),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StageMode {
    Board,
    List,
    Compare,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    Todo,
    Doing,
    Blocked,
    Done,
}

impl Status {
    const ALL: [Self; 4] = [Self::Todo, Self::Doing, Self::Blocked, Self::Done];

    fn as_str(self) -> &'static str {
        match self {
            Self::Todo => "todo",
            Self::Doing => "doing",
            Self::Blocked => "blocked",
            Self::Done => "done",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Todo => "Todo",
            Self::Doing => "Doing",
            Self::Blocked => "Blocked",
            Self::Done => "Done",
        }
    }

    fn from_str(value: &str) -> Self {
        match value {
            "doing" => Self::Doing,
            "blocked" => Self::Blocked,
            "done" => Self::Done,
            _ => Self::Todo,
        }
    }

    fn color(self) -> Color32 {
        match self {
            Self::Todo => Color32::from_rgb(92, 111, 128),
            Self::Doing => Color32::from_rgb(24, 124, 140),
            Self::Blocked => Color32::from_rgb(188, 76, 57),
            Self::Done => Color32::from_rgb(69, 128, 85),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Task {
    id: String,
    title: String,
    status: Status,
    assignee: String,
    priority: String,
}

impl Task {
    fn field_value(&self, aspect_label: &str) -> String {
        match aspect_label {
            "identity.id" => self.id.clone(),
            "title.value" => self.title.clone(),
            "status.state" => self.status.as_str().to_string(),
            "assignee.name" => self.assignee.clone(),
            "priority.level" => self.priority.clone(),
            _ => "unknown".to_string(),
        }
    }
}

#[derive(Clone)]
struct PendingWriteback {
    branch: String,
    task_id: String,
    changes: Vec<DeclarativeChangeView>,
}

#[derive(Clone)]
struct DeclarativeChangeView {
    aspect: String,
    field: String,
    value: DeclarativeWritebackValue,
}

impl DeclarativeChangeView {
    fn apply(&self, task: &mut Task) {
        match (self.aspect.as_str(), self.field.as_str(), &self.value) {
            ("title", "value", DeclarativeWritebackValue::String(value)) => {
                task.title = value.clone();
            }
            ("status", "state", DeclarativeWritebackValue::String(value)) => {
                task.status = Status::from_str(value);
            }
            ("assignee", "name", DeclarativeWritebackValue::String(value)) => {
                task.assignee = value.clone();
            }
            ("priority", "level", DeclarativeWritebackValue::String(value)) => {
                task.priority = value.clone();
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
struct TodoTruth {
    branches: Arc<RwLock<BTreeMap<String, Vec<Task>>>>,
    versions: Arc<RwLock<BTreeMap<String, u64>>>,
    pending_writebacks: Arc<RwLock<HashMap<String, PendingWriteback>>>,
}

impl TodoTruth {
    fn seeded() -> Self {
        let tasks = vec![
            Task {
                id: "task-1".to_string(),
                title: "Cut geometry shell out of Forge UI".to_string(),
                status: Status::Doing,
                assignee: "Esther".to_string(),
                priority: "P0".to_string(),
            },
            Task {
                id: "task-2".to_string(),
                title: "Declare kanban as a live query".to_string(),
                status: Status::Todo,
                assignee: "Mara".to_string(),
                priority: "P0".to_string(),
            },
            Task {
                id: "task-3".to_string(),
                title: "Wire runtime bridge writeback authority".to_string(),
                status: Status::Doing,
                assignee: "Ari".to_string(),
                priority: "P1".to_string(),
            },
            Task {
                id: "task-4".to_string(),
                title: "Show branch compare without app diff glue".to_string(),
                status: Status::Blocked,
                assignee: "Jules".to_string(),
                priority: "P1".to_string(),
            },
            Task {
                id: "task-5".to_string(),
                title: "Explain focused inspector patching".to_string(),
                status: Status::Todo,
                assignee: "Noor".to_string(),
                priority: "P2".to_string(),
            },
            Task {
                id: "task-6".to_string(),
                title: "Make promote/discard feel like branch physics".to_string(),
                status: Status::Todo,
                assignee: "Esther".to_string(),
                priority: "P1".to_string(),
            },
            Task {
                id: "task-7".to_string(),
                title: "Polish the reality bar".to_string(),
                status: Status::Done,
                assignee: "Mara".to_string(),
                priority: "P2".to_string(),
            },
        ];

        let mut branches = BTreeMap::new();
        branches.insert(MAIN_BRANCH.to_string(), tasks);
        let mut versions = BTreeMap::new();
        versions.insert(MAIN_BRANCH.to_string(), 1);

        Self {
            branches: Arc::new(RwLock::new(branches)),
            versions: Arc::new(RwLock::new(versions)),
            pending_writebacks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn snapshot_token(&self, branch: &str) -> String {
        let version = self
            .versions
            .read()
            .expect("todo version lock poisoned")
            .get(branch)
            .copied()
            .unwrap_or_default();
        format!("{SNAPSHOT_PREFIX}{branch}:{version}")
    }

    fn branch_from_snapshot(snapshot: &str) -> String {
        snapshot
            .strip_prefix(SNAPSHOT_PREFIX)
            .and_then(|value| value.rsplit_once(':').map(|(branch, _)| branch.to_string()))
            .unwrap_or_else(|| MAIN_BRANCH.to_string())
    }

    fn tasks(&self, branch: &str) -> Vec<Task> {
        self.branches
            .read()
            .expect("todo branch lock poisoned")
            .get(branch)
            .cloned()
            .unwrap_or_default()
    }

    fn task(&self, branch: &str, id: &str) -> Option<Task> {
        self.tasks(branch).into_iter().find(|task| task.id == id)
    }

    fn ensure_preview_from_main(&self) {
        self.copy_branch(MAIN_BRANCH, PREVIEW_BRANCH);
    }

    fn copy_branch(&self, source: &str, target: &str) {
        let tasks = self.tasks(source);
        self.branches
            .write()
            .expect("todo branch lock poisoned")
            .insert(target.to_string(), tasks);
        let source_version = self
            .versions
            .read()
            .expect("todo version lock poisoned")
            .get(source)
            .copied()
            .unwrap_or_default();
        self.versions
            .write()
            .expect("todo version lock poisoned")
            .insert(target.to_string(), source_version + 1);
    }

    fn write_branch(&self, branch: &str, tasks: Vec<Task>) {
        self.branches
            .write()
            .expect("todo branch lock poisoned")
            .insert(branch.to_string(), tasks);
        self.bump(branch);
    }

    fn promote_preview(&self) {
        let preview = self.tasks(PREVIEW_BRANCH);
        self.branches
            .write()
            .expect("todo branch lock poisoned")
            .insert(MAIN_BRANCH.to_string(), preview);
        self.bump(MAIN_BRANCH);
        self.discard_preview();
    }

    fn discard_preview(&self) {
        self.remove_branch(PREVIEW_BRANCH);
    }

    fn remove_branch(&self, branch: &str) {
        self.branches
            .write()
            .expect("todo branch lock poisoned")
            .remove(branch);
        self.versions
            .write()
            .expect("todo version lock poisoned")
            .remove(branch);
    }

    fn queue_writeback(&self, effect_digest: String, pending: PendingWriteback) {
        self.pending_writebacks
            .write()
            .expect("pending writeback lock poisoned")
            .insert(effect_digest, pending);
    }

    fn apply_pending(&self, effect_digest: &str) -> bool {
        let pending = self
            .pending_writebacks
            .write()
            .expect("pending writeback lock poisoned")
            .remove(effect_digest);
        let Some(pending) = pending else {
            return false;
        };

        let mut branches = self.branches.write().expect("todo branch lock poisoned");
        let Some(tasks) = branches.get_mut(&pending.branch) else {
            return false;
        };
        let Some(task) = tasks.iter_mut().find(|task| task.id == pending.task_id) else {
            return false;
        };
        for change in &pending.changes {
            change.apply(task);
        }
        drop(branches);
        self.bump(&pending.branch);
        true
    }

    fn bump(&self, branch: &str) {
        let mut versions = self.versions.write().expect("todo version lock poisoned");
        *versions.entry(branch.to_string()).or_insert(0) += 1;
    }

    fn state_digest(&self, branch: &str) -> String {
        let version = self
            .versions
            .read()
            .expect("todo version lock poisoned")
            .get(branch)
            .copied()
            .unwrap_or_default();
        format!("todo-state:{branch}:{version}")
    }
}

impl CommittedPatchSource for TodoTruth {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
            TruthSnapshotIdentity::new(self.snapshot_token(MAIN_BRANCH)),
            TruthBranchIdentity::new(MAIN_BRANCH),
            vec![BridgeCommittedPatchItem::new("task-1", "status", "state")],
        ))
    }
}

impl SnapshotReadSource for TodoTruth {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(TodoSnapshotReader {
            truth: self.clone(),
            snapshot: identity.clone(),
        }))
    }
}

impl TruthBranchHeadSource for TodoTruth {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let branch = branch_identity.as_str();
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(format!("head-{branch}")),
            TruthPatchIdentity::new(format!("patch-{branch}")),
            TruthSnapshotIdentity::new(self.snapshot_token(branch)),
            branch_identity.clone(),
            vec![BridgeCommittedPatchItem::new("task-1", "status", "state")],
        ))
    }
}

impl BridgeSourceAdapter for TodoTruth {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ])
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        SnapshotReadSource::open_snapshot(self, identity)
    }
}

impl TruthWritebackAuthority for TodoTruth {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        let applied = self.apply_pending(request.proposed_effect_digest());
        let artifact = if applied {
            format!(
                "authoritative-artifact:{}",
                request.proposed_effect_digest()
            )
        } else {
            format!("authoritative-noop:{}", request.proposed_effect_digest())
        };
        Ok(TruthWritebackReceipt::new(
            forge_runtime_bridge::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            artifact,
            &request,
        ))
    }
}

#[derive(Clone)]
struct TodoSnapshotReader {
    truth: TodoTruth,
    snapshot: TruthSnapshotIdentity,
}

impl TruthSnapshotReader for TodoSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.snapshot.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        let branch = TodoTruth::branch_from_snapshot(self.snapshot.as_str());
        let tasks = self.truth.tasks(&branch);
        let records = request
            .reads()
            .iter()
            .map(|read| {
                let id = read.entity_identity().trim_start_matches("task:");
                let value = tasks
                    .iter()
                    .find(|task| task.id == id)
                    .map(|task| task.field_value(read.aspect_label()))
                    .unwrap_or_else(|| "unknown".to_string());
                SnapshotReadRecord::new(read.request_key(), value.into_bytes())
            })
            .collect();
        Ok(SnapshotReadPacketResult::new(
            self.snapshot.clone(),
            records,
        ))
    }
}

#[derive(Clone)]
struct TodoSignalSink;

impl InvalidationSink for TodoSignalSink {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

#[derive(Clone)]
struct HistoryEntry {
    before: Vec<Task>,
    after: Vec<Task>,
    label: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DemoBranchKind {
    Main,
    Preview,
    Undo,
    Redo,
}

#[derive(Clone)]
struct DemoBranch {
    id: String,
    label: String,
    parent: Option<String>,
    kind: DemoBranchKind,
    color: Color32,
}

impl DemoBranch {
    fn main() -> Self {
        Self {
            id: MAIN_BRANCH.to_string(),
            label: "main".to_string(),
            parent: None,
            kind: DemoBranchKind::Main,
            color: Color32::from_rgb(82, 128, 112),
        }
    }

    fn preview(parent: &str) -> Self {
        Self {
            id: PREVIEW_BRANCH.to_string(),
            label: "sprint-next".to_string(),
            parent: Some(parent.to_string()),
            kind: DemoBranchKind::Preview,
            color: Color32::from_rgb(213, 128, 68),
        }
    }

    fn undo(id: String, parent: &str) -> Self {
        Self {
            label: id.replace("undo:", "undo "),
            id,
            parent: Some(parent.to_string()),
            kind: DemoBranchKind::Undo,
            color: Color32::from_rgb(92, 147, 184),
        }
    }

    fn redo(id: String, parent: &str) -> Self {
        Self {
            label: id.replace("redo:", "redo "),
            id,
            parent: Some(parent.to_string()),
            kind: DemoBranchKind::Redo,
            color: Color32::from_rgb(166, 116, 196),
        }
    }

    fn deletable(&self) -> bool {
        self.kind != DemoBranchKind::Main
    }
}

struct TodoShowcaseApp {
    truth: TodoTruth,
    runtime: RuntimeBridge,
    policy: LoweredBridgeExecutionPolicy,
    stage: StageMode,
    active_branch: String,
    selected_task: String,
    preview: Option<BridgeSpeculativeSessionHandle>,
    branches: Vec<DemoBranch>,
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    branch_counter: usize,
    trace: Vec<String>,
    board_session: DeclarativeLiveQuerySession,
    table_session: DeclarativeLiveQuerySession,
    inspector_session: DeclarativeLiveQuerySession,
}

impl TodoShowcaseApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let truth = TodoTruth::seeded();
        let runtime = build_runtime(truth.clone());
        let policy = lowered_policy(&runtime);
        let selected_task = "task-1".to_string();
        let board_session = declare_board_session(&truth, MAIN_BRANCH);
        let table_session = declare_table_session(&truth, MAIN_BRANCH);
        let inspector_session = declare_inspector_session(&truth, MAIN_BRANCH);
        Self {
            truth,
            runtime,
            policy,
            stage: StageMode::Board,
            active_branch: MAIN_BRANCH.to_string(),
            selected_task,
            preview: None,
            branches: vec![DemoBranch::main()],
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            branch_counter: 0,
            trace: vec![
                "Forge Query declared kanban, table, and focused inspector live sessions.".to_string(),
                "RuntimeBridge is bound as source, branch-head provider, signal sink, and writeback authority.".to_string(),
            ],
            board_session,
            table_session,
            inspector_session,
        }
    }

    fn refresh_sessions(&mut self) {
        self.board_session = declare_board_session(&self.truth, &self.active_branch);
        self.table_session = declare_table_session(&self.truth, &self.active_branch);
        self.inspector_session = declare_inspector_session(&self.truth, &self.active_branch);
    }

    fn tasks(&self) -> Vec<Task> {
        runtime_tasks(&self.runtime, &self.active_branch, &self.truth)
    }

    fn selected(&self) -> Option<Task> {
        self.truth.task(&self.active_branch, &self.selected_task)
    }

    fn record_history(&mut self, before: Vec<Task>, after: Vec<Task>, label: String) {
        if before == after {
            return;
        }
        self.undo_stack.push(HistoryEntry {
            before,
            after,
            label,
        });
        self.redo_stack.clear();
    }

    fn create_timeline_branch(&mut self, prefix: &str, parent: String, tasks: Vec<Task>) -> String {
        self.branch_counter += 1;
        let branch = format!("{prefix}:{}", self.branch_counter);
        self.truth.write_branch(&branch, tasks);
        let node = match prefix {
            "undo" => DemoBranch::undo(branch.clone(), &parent),
            "redo" => DemoBranch::redo(branch.clone(), &parent),
            _ => DemoBranch::preview(&parent),
        };
        self.branches.push(node);
        self.active_branch = branch.clone();
        self.stage = StageMode::Board;
        self.refresh_sessions();
        branch
    }

    fn undo(&mut self) {
        let Some(entry) = self.undo_stack.pop() else {
            return;
        };
        let parent = self.active_branch.clone();
        let branch = self.create_timeline_branch("undo", parent, entry.before.clone());
        self.redo_stack.push(entry.clone());
        self.trace.push(format!(
            "Undo fork `{}` restored state before `{}`.",
            branch, entry.label
        ));
    }

    fn redo(&mut self) {
        let Some(entry) = self.redo_stack.pop() else {
            return;
        };
        let parent = self.active_branch.clone();
        let branch = self.create_timeline_branch("redo", parent, entry.after.clone());
        self.undo_stack.push(entry.clone());
        self.trace.push(format!(
            "Redo fork `{}` replayed `{}`.",
            branch, entry.label
        ));
    }

    fn delete_branch(&mut self, branch: &str) {
        if branch == self.active_branch || branch == MAIN_BRANCH {
            return;
        }
        if branch == PREVIEW_BRANCH {
            if let Some(handle) = self.preview.take() {
                if let Err(error) = handle.discard(vec![
                    BridgePreviewResidueClass::PreviewExecutionRetained,
                    BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
                ]) {
                    self.trace
                        .push(format!("Preview delete discard failed: {error}"));
                }
            }
        }
        self.truth.remove_branch(branch);
        self.branches.retain(|node| node.id != branch);
        self.trace.push(format!("Deleted branch `{branch}`."));
    }

    fn begin_preview(&mut self) {
        if self.preview.is_some() {
            return;
        }
        self.truth.ensure_preview_from_main();
        let request = BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("preview-session:sprint-next"),
            preview_declaration(&self.board_session),
            3,
            1,
            2,
        );
        match self.runtime.speculate(request) {
            Ok(handle) => {
                self.preview = Some(handle);
                if !self
                    .branches
                    .iter()
                    .any(|branch| branch.id == PREVIEW_BRANCH)
                {
                    self.branches.push(DemoBranch::preview(MAIN_BRANCH));
                }
                self.active_branch = PREVIEW_BRANCH.to_string();
                self.stage = StageMode::Board;
                self.trace
                    .push("RuntimeBridge activated preview session `sprint-next`.".to_string());
                self.refresh_sessions();
            }
            Err(error) => self
                .trace
                .push(format!("Preview admission failed: {error}")),
        }
    }

    fn discard_preview(&mut self) {
        if let Some(handle) = self.preview.take() {
            match handle.discard(vec![
                BridgePreviewResidueClass::PreviewExecutionRetained,
                BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            ]) {
                Ok(_) => self.trace.push(
                    "RuntimeBridge discarded preview; non-authoritative branch removed."
                        .to_string(),
                ),
                Err(error) => self.trace.push(format!("Preview discard failed: {error}")),
            }
        }
        self.truth.discard_preview();
        self.branches.retain(|branch| branch.id != PREVIEW_BRANCH);
        self.active_branch = MAIN_BRANCH.to_string();
        self.stage = StageMode::Board;
        self.refresh_sessions();
    }

    fn promote_preview(&mut self) {
        if let Some(handle) = self.preview.take() {
            let request = BridgeSpeculativePromotionRequest::new(
                self.truth.state_digest(PREVIEW_BRANCH),
                self.board_session
                    .preflight()
                    .basis()
                    .proof()
                    .digest()
                    .as_str(),
            );
            match handle.promote(request) {
                Ok(_) => {
                    self.truth.promote_preview();
                    self.branches.retain(|branch| branch.id != PREVIEW_BRANCH);
                    self.active_branch = MAIN_BRANCH.to_string();
                    self.stage = StageMode::Board;
                    self.trace.push(
                        "RuntimeBridge promoted preview; main now reflects the accepted plan."
                            .to_string(),
                    );
                }
                Err(error) => self
                    .trace
                    .push(format!("Preview promotion failed: {error}")),
            }
        }
        self.refresh_sessions();
    }

    fn write_field(
        &mut self,
        task_id: String,
        aspect: &str,
        field: &str,
        value: DeclarativeWritebackValue,
    ) {
        let before = self.truth.tasks(&self.active_branch);
        let label = format!("{aspect}.{field}");
        let intent = DeclarativeWritebackIntent::update_aspect(aspect, field, value);
        let artifact = match declare_writeback_from_live_session(&self.inspector_session, intent) {
            Ok(artifact) => artifact,
            Err(error) => {
                self.trace.push(format!(
                    "Forge Query writeback declaration failed: {error:?}"
                ));
                return;
            }
        };

        let pending = PendingWriteback {
            branch: self.active_branch.clone(),
            task_id,
            changes: artifact
                .changes()
                .iter()
                .map(|change| DeclarativeChangeView {
                    aspect: change.aspect().to_string(),
                    field: change.field().to_string(),
                    value: change.value().clone(),
                })
                .collect(),
        };
        if self.execute_writeback(artifact, pending) {
            let after = self.truth.tasks(&self.active_branch);
            self.record_history(before, after, label);
        }
        self.refresh_sessions();
    }

    fn execute_writeback(
        &mut self,
        artifact: DeclarativeWritebackArtifact,
        pending: PendingWriteback,
    ) -> bool {
        let declaration = artifact.declaration();
        let contract = match self
            .runtime
            .admit_writeback_declaration(declaration.bridge_declaration().clone(), &self.policy)
        {
            Ok(contract) => contract,
            Err(error) => {
                self.trace
                    .push(format!("RuntimeBridge writeback admission failed: {error}"));
                return false;
            }
        };
        let causality = BridgeWritebackCausalityBasis::new(
            BridgeWritebackCausalityIdentity::new(format!(
                "causality:{}",
                declaration.causality_binding().causality_digest()
            )),
            artifact.live_view_basis_digest(),
            declaration.causality_binding().binding_digest(),
            declaration.causality_binding().basis_digest(),
            self.table_session
                .preflight()
                .basis()
                .proof()
                .digest()
                .as_str(),
        );
        self.truth
            .queue_writeback(artifact.artifact_digest().to_string(), pending);
        let effect = self.runtime.lower_writeback_effect(
            &contract,
            &causality,
            BridgeWritebackEffectIdentity::new(format!("effect:{}", artifact.intent_digest())),
            artifact.artifact_digest(),
        );
        let idempotence = self.runtime.classify_writeback_idempotence(
            &effect,
            &self.policy,
            self.truth.state_digest(&self.active_branch),
            forge_runtime_bridge::facade::BridgeWritebackIdempotenceIdentity::new(format!(
                "idempotence:{}",
                artifact.intent_digest()
            )),
            declaration.bridge_declaration().idempotence_class(),
        );

        match self
            .runtime
            .execute_writeback_authority(&contract, &effect, &idempotence)
        {
            Ok((outcome, receipt)) => {
                self.trace.push(format!(
                    "Query lowered writeback `{}`; bridge authority committed `{}`.",
                    artifact.intent_digest(),
                    receipt.authoritative_artifact_digest()
                ));
                self.trace
                    .push(format!("Writeback outcome digest: {}", outcome.digest()));
                true
            }
            Err(error) => {
                self.trace
                    .push(format!("RuntimeBridge authority execution failed: {error}"));
                false
            }
        }
    }
}

fn build_runtime(truth: TodoTruth) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::development())
        .with_relational_source(truth.clone())
        .with_source_adapter(truth.clone())
        .with_truth_branch_head_source(truth.clone())
        .with_compute_sink(TodoSignalSink)
        .with_writeback_authority(truth)
        .register_source(source_declaration(MAIN_BRANCH))
        .register_source(source_declaration(PREVIEW_BRANCH))
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("todo-status"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::exact("status"),
                MappingSelector::exact("state"),
            ),
            SignalInvalidationScope::new("signal:todo-board"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("todo showcase runtime should build")
}

fn source_declaration(branch: &str) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new(format!("source:todo:{branch}")),
        BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new(branch)),
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ]),
    )
}

fn lowered_policy(runtime: &RuntimeBridge) -> LoweredBridgeExecutionPolicy {
    let contract = runtime
        .admit_policy_declaration(BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:todo-writeback"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            forge_runtime_bridge::facade::BridgeDiagnosticsTier::Standard,
            true,
            true,
        ))
        .expect("authoritative writeback policy should admit");
    runtime.lower_admitted_policy(&contract)
}

fn todo_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "forge-query-todo",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String)
                .text_predicate_queryable(),
            SchemaFieldView::new("status", "state", SchemaFieldKind::String)
                .text_predicate_queryable(),
            SchemaFieldView::new("assignee", "name", SchemaFieldKind::String)
                .text_predicate_queryable(),
            SchemaFieldView::new("priority", "level", SchemaFieldKind::String)
                .text_predicate_queryable(),
        ],
        [],
    )
}

fn task_projection(request: DeclarativeLiveQueryRequest) -> DeclarativeLiveQueryRequest {
    request
        .project(DeclarativeProjectionField::new("identity", "id"))
        .project(DeclarativeProjectionField::new("title", "value"))
        .project(DeclarativeProjectionField::new("status", "state"))
        .project(DeclarativeProjectionField::new("assignee", "name"))
        .project(DeclarativeProjectionField::new("priority", "level"))
        .order_by(DeclarativeProjectionField::new("identity", "id"))
}

fn declare_board_session(truth: &TodoTruth, branch: &str) -> DeclarativeLiveQuerySession {
    declare_runtime_live_query_session(
        task_projection(DeclarativeLiveQueryRequest::new(
            "Task",
            DeclarativeLiveViewShape::kanban_grouped("status"),
        )),
        todo_schema(),
        truth.snapshot_token(branch),
    )
    .expect("board live query should declare")
}

fn declare_table_session(truth: &TodoTruth, branch: &str) -> DeclarativeLiveQuerySession {
    declare_runtime_live_query_session(
        task_projection(DeclarativeLiveQueryRequest::new(
            "Task",
            DeclarativeLiveViewShape::table(),
        )),
        todo_schema(),
        truth.snapshot_token(branch),
    )
    .expect("table live query should declare")
}

fn declare_inspector_session(truth: &TodoTruth, branch: &str) -> DeclarativeLiveQuerySession {
    declare_runtime_live_query_session(
        task_projection(DeclarativeLiveQueryRequest::new(
            "Task",
            DeclarativeLiveViewShape::inspector_focused("status"),
        )),
        todo_schema(),
        truth.snapshot_token(branch),
    )
    .expect("inspector live query should declare")
}

fn runtime_tasks(runtime: &RuntimeBridge, branch: &str, truth: &TodoTruth) -> Vec<Task> {
    let tasks = truth.tasks(branch);
    let packet = task_read_packet(&tasks);
    let request =
        BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new(branch))
            .with_read_packet(packet);
    runtime_tasks_from_request(runtime, request).unwrap_or(tasks)
}

fn task_read_packet(tasks: &[Task]) -> SnapshotReadPacket {
    SnapshotReadPacket::new(
        tasks
            .iter()
            .flat_map(|task| {
                let entity = format!("task:{}", task.id);
                [
                    SnapshotReadRequest::for_coarse(entity.clone(), "identity.id"),
                    SnapshotReadRequest::for_coarse(entity.clone(), "title.value"),
                    SnapshotReadRequest::for_coarse(entity.clone(), "status.state"),
                    SnapshotReadRequest::for_coarse(entity.clone(), "assignee.name"),
                    SnapshotReadRequest::for_coarse(entity, "priority.level"),
                ]
            })
            .collect(),
    )
}

fn runtime_tasks_from_request(
    runtime: &RuntimeBridge,
    request: BridgeTruthViewEvaluationRequest,
) -> Option<Vec<Task>> {
    let Ok(evaluation) = runtime.evaluate(request) else {
        return None;
    };
    let Ok(row_set) = materialize_bridge_row_set(evaluation.observation()) else {
        return None;
    };

    Some(
        row_set
            .rows()
            .iter()
            .map(|row| {
                let field = |key: &str| {
                    row.fields()
                        .get(key)
                        .and_then(|value| value.value().as_str())
                        .unwrap_or_default()
                        .to_string()
                };
                Task {
                    id: field("identity.id"),
                    title: field("title.value"),
                    status: Status::from_str(&field("status.state")),
                    assignee: field("assignee.name"),
                    priority: field("priority.level"),
                }
            })
            .collect(),
    )
}

fn preview_declaration(session: &DeclarativeLiveQuerySession) -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new("preview-declaration:sprint-next"),
        BridgeRequestKind::Preview,
        forge_runtime_bridge::facade::BridgeSpeculativeBranchBinding::new(
            forge_runtime_bridge::facade::BridgeSpeculativeBranchBindingIdentity::new(
                "preview-binding:sprint-next",
            ),
            TruthBranchIdentity::new(PREVIEW_BRANCH),
            BridgeSignalBranchIdentity::new("signal:sprint-next"),
        ),
        session.preflight().basis().proof().digest().as_str(),
        "source:todo-preview",
        session.view_plan().view_plan_digest().as_str(),
        session.canonical().query().digest().as_str(),
    )
}

fn diff_task(main: &Task, preview: &Task) -> Vec<String> {
    let mut changes = Vec::new();
    if main.status != preview.status {
        changes.push(format!(
            "status: {} -> {}",
            main.status.label(),
            preview.status.label()
        ));
    }
    if main.assignee != preview.assignee {
        changes.push(format!(
            "assignee: {} -> {}",
            main.assignee, preview.assignee
        ));
    }
    if main.priority != preview.priority {
        changes.push(format!(
            "priority: {} -> {}",
            main.priority, preview.priority
        ));
    }
    if main.title != preview.title {
        changes.push(format!("title: {} -> {}", main.title, preview.title));
    }
    changes
}

fn panel_frame(fill: Color32) -> Frame {
    Frame::new()
        .fill(fill)
        .stroke(Stroke::new(1.0, Color32::from_rgb(48, 57, 57)))
        .corner_radius(CornerRadius::same(14))
        .inner_margin(Margin::same(14))
}

fn chip(ui: &mut egui::Ui, label: &str, color: Color32) {
    Frame::new()
        .fill(color)
        .corner_radius(CornerRadius::same(99))
        .inner_margin(Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.label(
                RichText::new(label)
                    .size(11.0)
                    .color(Color32::from_rgb(242, 238, 226)),
            );
        });
}

fn branch_badge(ui: &mut egui::Ui, branch: &str) {
    chip(
        ui,
        if branch == MAIN_BRANCH {
            "branch: main"
        } else {
            "branch: sprint-next"
        },
        if branch == MAIN_BRANCH {
            Color32::from_rgb(48, 73, 70)
        } else {
            Color32::from_rgb(128, 76, 45)
        },
    );
}

fn tab(ui: &mut egui::Ui, label: &str, stage: &mut StageMode, target: StageMode) {
    if ui.selectable_label(*stage == target, label).clicked() {
        *stage = target;
    }
}

fn metric(ui: &mut egui::Ui, label: &str, value: usize) {
    ui.vertical(|ui| {
        ui.label(RichText::new(value.to_string()).size(22.0).strong());
        ui.label(
            RichText::new(label)
                .size(11.0)
                .color(Color32::from_rgb(150, 160, 156)),
        );
    });
}

fn short(value: &str) -> String {
    if value.len() <= 18 {
        value.to_string()
    } else {
        format!("{}...", &value[..18])
    }
}

impl eframe::App for TodoShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::new().fill(Color32::from_rgb(14, 17, 18)))
            .show(ctx, |ui| {
                ui.add_space(12.0);
                self.reality_bar(ui);
                ui.add_space(12.0);
                self.branch_timeline(ui);
                ui.add_space(12.0);
                ui.horizontal_top(|ui| {
                    self.planning_stage(ui);
                    ui.add_space(12.0);
                    self.focus_rail(ui);
                });
            });
    }
}

impl TodoShowcaseApp {
    fn reality_bar(&mut self, ui: &mut egui::Ui) {
        panel_frame(Color32::from_rgb(25, 29, 30)).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Forge Query Sprint")
                            .size(26.0)
                            .strong()
                            .color(Color32::from_rgb(237, 232, 219)),
                    );
                    ui.label(
                        RichText::new("live query surfaces + bridge-owned writeback authority")
                            .size(12.0)
                            .color(Color32::from_rgb(156, 164, 160)),
                    );
                });
                ui.add_space(24.0);
                branch_badge(ui, &self.active_branch);
                ui.add_space(8.0);
                chip(ui, "Live", Color32::from_rgb(42, 112, 118));
                if self.preview.is_some() {
                    chip(ui, "Preview Active", Color32::from_rgb(154, 88, 50));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let redo =
                        ui.add_enabled(!self.redo_stack.is_empty(), egui::Button::new("Redo"));
                    if redo.clicked() {
                        self.redo();
                    }
                    let undo =
                        ui.add_enabled(!self.undo_stack.is_empty(), egui::Button::new("Undo"));
                    if undo.clicked() {
                        self.undo();
                    }
                    if self.preview.is_some() {
                        if ui.button("Promote").clicked() {
                            self.promote_preview();
                        }
                        if ui.button("Discard").clicked() {
                            self.discard_preview();
                        }
                        if ui.button("Compare To Main").clicked() {
                            self.stage = StageMode::Compare;
                        }
                    } else if ui.button("Plan Sprint").clicked() {
                        self.begin_preview();
                    }
                });
            });
        });
    }

    fn branch_timeline(&mut self, ui: &mut egui::Ui) {
        let mut switch_to = None;
        let mut delete = None;
        panel_frame(Color32::from_rgb(17, 22, 23)).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("Branch Graph")
                        .size(13.0)
                        .strong()
                        .color(Color32::from_rgb(237, 232, 219)),
                );
                ui.separator();
                for branch in &self.branches {
                    let current = branch.id == self.active_branch;
                    let parent_hint = branch
                        .parent
                        .as_deref()
                        .map(|parent| format!(" from {parent}"))
                        .unwrap_or_default();
                    Frame::new()
                        .fill(if current {
                            Color32::from_rgb(38, 47, 46)
                        } else {
                            Color32::from_rgb(24, 29, 30)
                        })
                        .stroke(Stroke::new(
                            1.0,
                            if current {
                                Color32::from_rgb(226, 164, 91)
                            } else {
                                Color32::from_rgb(55, 64, 64)
                            },
                        ))
                        .corner_radius(CornerRadius::same(14))
                        .inner_margin(Margin::symmetric(10, 6))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.colored_label(branch.color, "●");
                                ui.label(
                                    RichText::new("─")
                                        .monospace()
                                        .color(Color32::from_rgb(95, 108, 105)),
                                );
                                if ui
                                    .selectable_label(
                                        current,
                                        RichText::new(format!("{}{}", branch.label, parent_hint))
                                            .size(12.0),
                                    )
                                    .clicked()
                                {
                                    switch_to = Some(branch.id.clone());
                                }
                                if branch.deletable() {
                                    let can_delete = !current;
                                    if ui
                                        .add_enabled(can_delete, egui::Button::new("x"))
                                        .on_disabled_hover_text(
                                            "Cannot delete the branch you are currently on.",
                                        )
                                        .clicked()
                                    {
                                        delete = Some(branch.id.clone());
                                    }
                                }
                            });
                        });
                }
            });
        });

        if let Some(branch) = switch_to {
            self.active_branch = branch;
            self.refresh_sessions();
        }
        if let Some(branch) = delete {
            self.delete_branch(&branch);
        }
    }

    fn planning_stage(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.set_width(ui.available_width() * 0.66);
            panel_frame(Color32::from_rgb(19, 23, 24)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    tab(ui, "Board", &mut self.stage, StageMode::Board);
                    tab(ui, "List", &mut self.stage, StageMode::List);
                    tab(ui, "Compare", &mut self.stage, StageMode::Compare);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!(
                                "view plan {}",
                                short(self.board_session.view_plan().view_plan_digest().as_str())
                            ))
                            .color(Color32::from_rgb(126, 138, 134))
                            .size(11.0),
                        );
                    });
                });
                ui.add_space(10.0);
                match self.stage {
                    StageMode::Board => self.board(ui),
                    StageMode::List => self.list(ui),
                    StageMode::Compare => self.compare(ui),
                }
            });
        });
    }

    fn board(&mut self, ui: &mut egui::Ui) {
        let tasks = self.tasks();
        ui.columns(4, |columns| {
            for (index, status) in Status::ALL.into_iter().enumerate() {
                columns[index].vertical(|ui| {
                    let lane_tasks = tasks
                        .iter()
                        .filter(|task| task.status == status)
                        .cloned()
                        .collect::<Vec<_>>();
                    ui.label(
                        RichText::new(format!("{}  {}", status.label(), lane_tasks.len()))
                            .strong()
                            .color(status.color()),
                    );
                    ui.add_space(8.0);
                    for task in lane_tasks {
                        self.task_card(ui, task);
                    }
                });
            }
        });
    }

    fn task_card(&mut self, ui: &mut egui::Ui, task: Task) {
        let selected = self.selected_task == task.id;
        let fill = if selected {
            Color32::from_rgb(42, 49, 48)
        } else {
            Color32::from_rgb(28, 33, 34)
        };
        let response = panel_frame(fill)
            .stroke(Stroke::new(
                1.0,
                if selected {
                    Color32::from_rgb(216, 153, 92)
                } else {
                    Color32::from_rgb(55, 64, 64)
                },
            ))
            .show(ui, |ui| {
                ui.set_min_height(92.0);
                ui.label(
                    RichText::new(&task.title)
                        .color(Color32::from_rgb(235, 231, 219))
                        .strong(),
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    chip(ui, &task.assignee, Color32::from_rgb(58, 78, 82));
                    chip(ui, &task.priority, Color32::from_rgb(87, 68, 51));
                });
            })
            .response;
        if response.clicked() {
            self.selected_task = task.id;
            self.refresh_sessions();
        }
    }

    fn list(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("task-list").striped(true).show(ui, |ui| {
            ui.label(RichText::new("Task").strong());
            ui.label(RichText::new("Status").strong());
            ui.label(RichText::new("Assignee").strong());
            ui.label(RichText::new("Priority").strong());
            ui.end_row();
            for task in self.tasks() {
                if ui
                    .selectable_label(self.selected_task == task.id, &task.title)
                    .clicked()
                {
                    self.selected_task = task.id.clone();
                    self.refresh_sessions();
                }
                ui.label(task.status.label());
                ui.label(&task.assignee);
                ui.label(&task.priority);
                ui.end_row();
            }
        });
    }

    fn compare(&mut self, ui: &mut egui::Ui) {
        if self.active_branch == MAIN_BRANCH {
            ui.label("Switch to a preview, undo, or redo branch to compare against main.");
            return;
        }
        let main_session = declare_table_session(&self.truth, MAIN_BRANCH);
        let branch_session = declare_table_session(&self.truth, &self.active_branch);
        ui.horizontal_wrapped(|ui| {
            chip(
                ui,
                &format!(
                    "main query {}",
                    short(main_session.view_plan().view_plan_digest().as_str())
                ),
                Color32::from_rgb(48, 73, 70),
            );
            chip(
                ui,
                &format!(
                    "branch query {}",
                    short(branch_session.view_plan().view_plan_digest().as_str())
                ),
                Color32::from_rgb(91, 82, 53),
            );
            if let Some(preview) = self.preview.as_ref() {
                let comparison = preview.compare_to_main();
                chip(
                    ui,
                    &format!(
                        "bridge basis {}",
                        short(comparison.truth_view_basis_digest())
                    ),
                    Color32::from_rgb(102, 70, 45),
                );
            }
        });
        ui.add_space(10.0);

        let branch_seed = self.truth.tasks(&self.active_branch);
        let main_seed = self.truth.tasks(MAIN_BRANCH);
        let (main, preview) = if self.active_branch == PREVIEW_BRANCH {
            if let Some(handle) = self.preview.as_ref() {
                let comparison = handle.compare_to_main();
                let main = runtime_tasks_from_request(
                    &self.runtime,
                    comparison
                        .main_evaluation_request(TruthBranchIdentity::new(MAIN_BRANCH))
                        .with_read_packet(task_read_packet(&main_seed)),
                )
                .unwrap_or(main_seed);
                let preview = runtime_tasks_from_request(
                    &self.runtime,
                    comparison
                        .speculative_evaluation_request()
                        .with_read_packet(task_read_packet(&branch_seed)),
                )
                .unwrap_or(branch_seed);
                (main, preview)
            } else {
                (
                    runtime_tasks(&self.runtime, MAIN_BRANCH, &self.truth),
                    runtime_tasks(&self.runtime, &self.active_branch, &self.truth),
                )
            }
        } else {
            (
                runtime_tasks(&self.runtime, MAIN_BRANCH, &self.truth),
                runtime_tasks(&self.runtime, &self.active_branch, &self.truth),
            )
        };

        let mut rendered = 0;
        for preview_task in preview {
            let Some(main_task) = main.iter().find(|task| task.id == preview_task.id) else {
                continue;
            };
            let changes = diff_task(main_task, &preview_task);
            if changes.is_empty() {
                continue;
            }
            panel_frame(Color32::from_rgb(26, 31, 32)).show(ui, |ui| {
                ui.label(
                    RichText::new(&preview_task.title)
                        .strong()
                        .color(Color32::from_rgb(236, 229, 212)),
                );
                for change in changes {
                    ui.label(RichText::new(change).color(Color32::from_rgb(216, 153, 92)));
                }
            });
            ui.add_space(8.0);
            rendered += 1;
        }
        if rendered == 0 {
            ui.label("No deltas on this branch.");
        }
    }

    fn focus_rail(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.set_width(ui.available_width());
            self.inspector(ui);
            ui.add_space(10.0);
            self.signals(ui);
            ui.add_space(10.0);
            self.trace(ui);
        });
    }

    fn inspector(&mut self, ui: &mut egui::Ui) {
        panel_frame(Color32::from_rgb(22, 27, 28)).show(ui, |ui| {
            ui.label(
                RichText::new("Focused Inspector")
                    .size(16.0)
                    .strong()
                    .color(Color32::from_rgb(237, 232, 219)),
            );
            ui.label(
                RichText::new(format!(
                    "live {}",
                    short(
                        self.inspector_session
                            .live_view()
                            .core_live_plan()
                            .subscription_digest()
                            .as_str(),
                    )
                ))
                .size(11.0)
                .color(Color32::from_rgb(126, 138, 134)),
            );
            ui.add_space(10.0);
            let Some(task) = self.selected() else {
                ui.label("Select a task.");
                return;
            };
            ui.label(RichText::new(&task.title).size(18.0).strong());
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                for status in Status::ALL {
                    if ui
                        .selectable_label(task.status == status, status.label())
                        .clicked()
                    {
                        self.write_field(
                            task.id.clone(),
                            "status",
                            "state",
                            DeclarativeWritebackValue::String(status.as_str().to_string()),
                        );
                    }
                }
            });
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                for assignee in ["Esther", "Mara", "Ari", "Jules", "Noor"] {
                    if ui
                        .selectable_label(task.assignee == assignee, assignee)
                        .clicked()
                    {
                        self.write_field(
                            task.id.clone(),
                            "assignee",
                            "name",
                            DeclarativeWritebackValue::String(assignee.to_string()),
                        );
                    }
                }
            });
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                for priority in ["P0", "P1", "P2"] {
                    if ui
                        .selectable_label(task.priority == priority, priority)
                        .clicked()
                    {
                        self.write_field(
                            task.id.clone(),
                            "priority",
                            "level",
                            DeclarativeWritebackValue::String(priority.to_string()),
                        );
                    }
                }
            });
        });
    }

    fn signals(&self, ui: &mut egui::Ui) {
        let tasks = self.tasks();
        let open = tasks
            .iter()
            .filter(|task| task.status != Status::Done)
            .count();
        let blocked = tasks
            .iter()
            .filter(|task| task.status == Status::Blocked)
            .count();
        let high = tasks
            .iter()
            .filter(|task| task.priority == "P0" && task.status != Status::Done)
            .count();
        panel_frame(Color32::from_rgb(22, 27, 28)).show(ui, |ui| {
            ui.label(RichText::new("Signals").size(16.0).strong());
            ui.horizontal(|ui| {
                metric(ui, "Open", open);
                metric(ui, "Blocked", blocked);
                metric(ui, "P0 Open", high);
            });
        });
    }

    fn trace(&self, ui: &mut egui::Ui) {
        panel_frame(Color32::from_rgb(22, 27, 28)).show(ui, |ui| {
            ui.label(RichText::new("Why This Changed").size(16.0).strong());
            for line in self.trace.iter().rev().take(6) {
                ui.label(
                    RichText::new(line)
                        .size(11.0)
                        .color(Color32::from_rgb(172, 181, 176)),
                );
            }
        });
    }
}
