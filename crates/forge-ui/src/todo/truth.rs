use forge_query::facade::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, DeclarativeProjectionField,
    ForgeQueryAspect, ForgeQueryCollection, ForgeQueryLiveView, ForgeQueryRuntime,
    ForgeQueryRuntimeError, ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
    ForgeQueryWriteReceipt, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
};
use serde_json::{json, Value};

use super::task::{BoardColumn, BoardUser, Task};

const TASKS: &str = "Task";
const COLUMNS: &str = "Column";
const USERS: &str = "User";
const TASK_BOARD_VIEW: &str = "todo.tasks.board";
const COLUMN_LIST_VIEW: &str = "todo.columns.list";
const USER_LIST_VIEW: &str = "todo.users.list";

pub struct TodoWorkspace {
    query: ForgeQueryRuntime,
    task_view: ForgeQueryLiveView<Value>,
    column_view: ForgeQueryLiveView<Value>,
    user_view: ForgeQueryLiveView<Value>,
}

#[derive(Clone, Debug)]
pub enum TodoCommand {
    CreateTask(Task),
    AddColumn(String),
    AddUser(String),
    UpdateTaskField {
        task_id: String,
        aspect: String,
        field: String,
        value: Value,
    },
    DeleteTask(String),
}

#[derive(Clone, Debug)]
pub struct TodoView {
    pub tasks: Vec<Task>,
    pub columns: Vec<BoardColumn>,
    pub users: Vec<BoardUser>,
    pub snapshot_token: String,
}

impl TodoWorkspace {
    pub fn seeded() -> Self {
        let mut workspace = Self::new().expect("todo workspace should build");
        for name in ["Todo", "Doing", "Done"] {
            workspace
                .submit(TodoCommand::AddColumn(name.to_string()))
                .expect("default column should insert");
        }
        workspace
    }

    fn new() -> Result<Self, ForgeQueryRuntimeError> {
        let mut query = ForgeQueryRuntime::builder()
            .compatibility_in_memory_collections([
                ForgeQueryCollection::new(
                    TASKS,
                    [
                        ForgeQueryAspect::new("identity.id", "identity.id"),
                        ForgeQueryAspect::new("title.value", "title.value"),
                        ForgeQueryAspect::new("column.id", "column.id"),
                        ForgeQueryAspect::new("assignee.id", "assignee.id"),
                        ForgeQueryAspect::new("priority.level", "priority.level"),
                        ForgeQueryAspect::new("due.date", "due.date"),
                        ForgeQueryAspect::new("tag.name", "tag.name"),
                    ],
                ),
                ForgeQueryCollection::new(
                    COLUMNS,
                    [
                        ForgeQueryAspect::new("identity.id", "identity.id"),
                        ForgeQueryAspect::new("name.value", "name.value"),
                        ForgeQueryAspect::new("order.index", "order.index"),
                    ],
                ),
                ForgeQueryCollection::new(
                    USERS,
                    [
                        ForgeQueryAspect::new("identity.id", "identity.id"),
                        ForgeQueryAspect::new("name.value", "name.value"),
                    ],
                ),
            ])
            .build()?;
        let task_view =
            query.declare_live_view(TASK_BOARD_VIEW, task_live_request(), task_schema())?;
        let column_view =
            query.declare_live_view(COLUMN_LIST_VIEW, column_live_request(), column_schema())?;
        let user_view =
            query.declare_live_view(USER_LIST_VIEW, user_live_request(), user_schema())?;
        Ok(Self {
            query,
            task_view,
            column_view,
            user_view,
        })
    }

    pub fn submit(
        &mut self,
        command: TodoCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        match command {
            TodoCommand::CreateTask(task) => {
                let receipt = self.query.write(ForgeQueryWriteCommand::Insert {
                    collection: TASKS.to_string(),
                    payload: task_payload(&task),
                })?;
                self.backfill_inserted_identity(&receipt)
            }
            TodoCommand::AddColumn(name) => {
                let order = self.live_columns().len() + 1;
                let receipt = self.query.write(ForgeQueryWriteCommand::Insert {
                    collection: COLUMNS.to_string(),
                    payload: json!({
                        "identity": { "id": "" },
                        "name": { "value": name },
                        "order": { "index": order },
                    }),
                })?;
                self.backfill_inserted_identity(&receipt)
            }
            TodoCommand::AddUser(name) => {
                let receipt = self.query.write(ForgeQueryWriteCommand::Insert {
                    collection: USERS.to_string(),
                    payload: json!({
                        "identity": { "id": "" },
                        "name": { "value": name },
                    }),
                })?;
                self.backfill_inserted_identity(&receipt)
            }
            TodoCommand::UpdateTaskField {
                task_id,
                aspect,
                field,
                value,
            } => self.query.write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: task_id,
                aspect_path: format!("{aspect}.{field}"),
                value,
            }),
            TodoCommand::DeleteTask(task_id) => self.query.write(ForgeQueryWriteCommand::Delete {
                entity_identity: task_id,
            }),
        }
    }

    pub fn view(&self) -> TodoView {
        TodoView {
            tasks: self.live_tasks(),
            columns: self.live_columns(),
            users: self.live_users(),
            snapshot_token: self.snapshot_token(),
        }
    }

    pub fn snapshot_token(&self) -> String {
        self.query.snapshot_token()
    }

    pub fn live_tasks(&self) -> Vec<Task> {
        let mut tasks = self
            .query
            .read_live(&self.task_view)
            .into_iter()
            .filter_map(task_from_entity)
            .collect::<Vec<_>>();
        tasks.sort_by(|left, right| left.title.cmp(&right.title));
        tasks
    }

    pub fn live_columns(&self) -> Vec<BoardColumn> {
        let mut columns = self
            .query
            .read_live(&self.column_view)
            .into_iter()
            .filter_map(column_from_entity)
            .collect::<Vec<_>>();
        columns.sort_by_key(|column| column.order);
        columns
    }

    pub fn live_users(&self) -> Vec<BoardUser> {
        let mut users = self
            .query
            .read_live(&self.user_view)
            .into_iter()
            .filter_map(user_from_entity)
            .collect::<Vec<_>>();
        users.sort_by(|left, right| left.name.cmp(&right.name));
        users
    }

    pub fn live_task(&self, id: &str) -> Option<Task> {
        self.live_tasks().into_iter().find(|task| task.id == id)
    }

    pub fn drain_board_patches(&mut self) {
        let _ = self.query.drain_patches(&self.task_view);
        let _ = self.query.drain_patches(&self.column_view);
        let _ = self.query.drain_patches(&self.user_view);
    }

    fn backfill_inserted_identity(
        &mut self,
        receipt: &ForgeQueryWriteReceipt,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let Some(id) = receipt.deltas().first().map(|delta| &delta.entity_identity) else {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("insert did not return identity"),
            ));
        };
        self.query.write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: id.clone(),
            aspect_path: "identity.id".to_string(),
            value: Value::String(id.clone()),
        })?;
        Ok(receipt.clone())
    }
}

fn task_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new(TASKS, DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title.value"))
        .project(DeclarativeProjectionField::new("column", "id").delivered_as("column.id"))
        .project(DeclarativeProjectionField::new("assignee", "id").delivered_as("assignee.id"))
        .project(
            DeclarativeProjectionField::new("priority", "level").delivered_as("priority.level"),
        )
        .project(DeclarativeProjectionField::new("due", "date").delivered_as("due.date"))
        .project(DeclarativeProjectionField::new("tag", "name").delivered_as("tag.name"))
        .order_by(DeclarativeProjectionField::new("title", "value").delivered_as("title.value"))
}

fn column_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new(COLUMNS, DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("name", "value").delivered_as("name.value"))
        .project(DeclarativeProjectionField::new("order", "index").delivered_as("order.index"))
        .order_by(DeclarativeProjectionField::new("order", "index").delivered_as("order.index"))
}

fn user_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new(USERS, DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("name", "value").delivered_as("name.value"))
        .order_by(DeclarativeProjectionField::new("name", "value").delivered_as("name.value"))
}

fn task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "forge-query-todo-task",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String),
            SchemaFieldView::new("column", "id", SchemaFieldKind::String),
            SchemaFieldView::new("assignee", "id", SchemaFieldKind::String),
            SchemaFieldView::new("priority", "level", SchemaFieldKind::String),
            SchemaFieldView::new("due", "date", SchemaFieldKind::String),
            SchemaFieldView::new("tag", "name", SchemaFieldKind::String),
        ],
        [],
    )
}

fn column_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "forge-query-todo-column",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("name", "value", SchemaFieldKind::String),
            SchemaFieldView::new("order", "index", SchemaFieldKind::Integer),
        ],
        [],
    )
}

fn user_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "forge-query-todo-user",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("name", "value", SchemaFieldKind::String),
        ],
        [],
    )
}

fn task_from_entity(entity: forge_query::facade::ForgeQueryEntity) -> Option<Task> {
    let payload = entity.payload;
    Some(Task {
        id: string_path(&payload, &["identity", "id"]).unwrap_or(entity.identity),
        title: string_path(&payload, &["title", "value"])?,
        column_id: string_path(&payload, &["column", "id"])?,
        assignee_id: non_empty_string_path(&payload, &["assignee", "id"]),
        priority: string_path(&payload, &["priority", "level"]).unwrap_or_else(|| "P2".to_string()),
        due_date: non_empty_string_path(&payload, &["due", "date"]),
        tag: string_path(&payload, &["tag", "name"]).unwrap_or_default(),
    })
}

fn column_from_entity(entity: forge_query::facade::ForgeQueryEntity) -> Option<BoardColumn> {
    let payload = entity.payload;
    Some(BoardColumn {
        id: string_path(&payload, &["identity", "id"]).unwrap_or(entity.identity),
        name: string_path(&payload, &["name", "value"])?,
        order: payload
            .pointer("/order/index")
            .and_then(Value::as_u64)
            .unwrap_or(usize::MAX as u64) as usize,
    })
}

fn user_from_entity(entity: forge_query::facade::ForgeQueryEntity) -> Option<BoardUser> {
    let payload = entity.payload;
    Some(BoardUser {
        id: string_path(&payload, &["identity", "id"]).unwrap_or(entity.identity),
        name: string_path(&payload, &["name", "value"])?,
    })
}

fn task_payload(task: &Task) -> Value {
    json!({
        "identity": { "id": task.id.clone() },
        "title": { "value": task.title.clone() },
        "column": { "id": task.column_id.clone() },
        "assignee": { "id": task.assignee_id.clone().unwrap_or_default() },
        "priority": { "level": task.priority.clone() },
        "due": { "date": task.due_date.clone().unwrap_or_default() },
        "tag": { "name": task.tag.clone() },
    })
}

fn string_path(payload: &Value, path: &[&str]) -> Option<String> {
    let mut current = payload;
    for segment in path {
        current = current.get(*segment)?;
    }
    current.as_str().map(ToString::to_string)
}

fn non_empty_string_path(payload: &Value, path: &[&str]) -> Option<String> {
    string_path(payload, path).filter(|value| !value.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use forge_query::facade::ForgeQueryRuntimeBackendPosture;

    use super::*;

    #[test]
    fn todo_workspace_uses_explicit_compatibility_backend_through_runtime_facade() {
        let mut workspace = TodoWorkspace::seeded();

        assert_eq!(
            workspace.query.support_profile().posture(),
            ForgeQueryRuntimeBackendPosture::Compatibility
        );
        assert_eq!(workspace.live_columns().len(), 3);
        assert!(workspace.live_tasks().is_empty());

        workspace
            .submit(TodoCommand::CreateTask(Task {
                id: "task-1".to_string(),
                title: "Batch 9 audit".to_string(),
                column_id: workspace.live_columns()[0].id.clone(),
                assignee_id: None,
                priority: "P1".to_string(),
                due_date: None,
                tag: "runtime-api".to_string(),
            }))
            .expect("facade write should mutate compatibility backend");

        let tasks = workspace.live_tasks();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Batch 9 audit");
    }
}
