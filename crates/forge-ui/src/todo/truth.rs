use forge_query::facade::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, DeclarativeProjectionField,
    ForgeQueryAspect, ForgeQueryCollection, ForgeQueryMemoryApp, ForgeQueryMutationReceipt,
    ForgeQueryWorkspaceError, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
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
    app: ForgeQueryMemoryApp,
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

    fn new() -> Result<Self, ForgeQueryWorkspaceError> {
        let mut app = ForgeQueryMemoryApp::new([
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
        ])?;
        app.declare_live_view(TASK_BOARD_VIEW, task_live_request(), task_schema())?;
        app.declare_live_view(COLUMN_LIST_VIEW, column_live_request(), column_schema())?;
        app.declare_live_view(USER_LIST_VIEW, user_live_request(), user_schema())?;
        Ok(Self { app })
    }

    pub fn submit(
        &mut self,
        command: TodoCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        match command {
            TodoCommand::CreateTask(task) => {
                let receipt = self.app.insert(TASKS, task_payload(&task))?;
                self.backfill_inserted_identity(&receipt)
            }
            TodoCommand::AddColumn(name) => {
                let order = self.live_columns().len() + 1;
                let receipt = self.app.insert(
                    COLUMNS,
                    json!({
                        "identity": { "id": "" },
                        "name": { "value": name },
                        "order": { "index": order },
                    }),
                )?;
                self.backfill_inserted_identity(&receipt)
            }
            TodoCommand::AddUser(name) => {
                let receipt = self.app.insert(
                    USERS,
                    json!({
                        "identity": { "id": "" },
                        "name": { "value": name },
                    }),
                )?;
                self.backfill_inserted_identity(&receipt)
            }
            TodoCommand::UpdateTaskField {
                task_id,
                aspect,
                field,
                value,
            } => self
                .app
                .update_aspect(&task_id, &format!("{aspect}.{field}"), value),
            TodoCommand::DeleteTask(task_id) => self.app.delete(&task_id),
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
        self.app.snapshot_token()
    }

    pub fn live_tasks(&self) -> Vec<Task> {
        let mut tasks = self
            .app
            .live_entities(TASK_BOARD_VIEW)
            .into_iter()
            .filter_map(task_from_entity)
            .collect::<Vec<_>>();
        tasks.sort_by(|left, right| left.title.cmp(&right.title));
        tasks
    }

    pub fn live_columns(&self) -> Vec<BoardColumn> {
        let mut columns = self
            .app
            .live_entities(COLUMN_LIST_VIEW)
            .into_iter()
            .filter_map(column_from_entity)
            .collect::<Vec<_>>();
        columns.sort_by_key(|column| column.order);
        columns
    }

    pub fn live_users(&self) -> Vec<BoardUser> {
        let mut users = self
            .app
            .live_entities(USER_LIST_VIEW)
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
        let _ = self.app.drain_live_patches(TASK_BOARD_VIEW);
        let _ = self.app.drain_live_patches(COLUMN_LIST_VIEW);
        let _ = self.app.drain_live_patches(USER_LIST_VIEW);
    }

    fn backfill_inserted_identity(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let Some(id) = receipt.deltas.first().map(|delta| &delta.entity_identity) else {
            return Err(ForgeQueryWorkspaceError::new(
                "insert did not return identity",
            ));
        };
        self.app
            .update_aspect(id, "identity.id", Value::String(id.clone()))?;
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
