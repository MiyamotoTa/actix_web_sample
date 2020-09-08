use sqlx::query::Query;

use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::AppState;

pub(crate) async fn insert_user(
    request: &PostUsersRequest,
    state: &AppState,
) -> Result<(), sqlx::Error> {
    sqlx::query_file!(
        "queries/v1/users/insert_users.sql",
        request.name,
        request.email
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}
