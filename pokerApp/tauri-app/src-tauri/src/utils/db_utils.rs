use crate::AppState;
use axum::extract::State;
use sea_orm::{ConnectionTrait, DatabaseBackend, DbErr, QueryResult, Statement, Value};

async fn db_stmt_execute(
    State(app_state): State<AppState>,
    query: &str,
    query_vec: Vec<Value>,
) -> Result<sea_orm::ExecResult, DbErr> {
    let db = app_state.db.clone();
    let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, query.clone(), query_vec);
    let result = db.execute(stmt).await;
    result
}
