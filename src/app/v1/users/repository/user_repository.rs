use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::model::user::User;
use crate::app::AppState;

pub(crate) async fn create(
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

pub(crate) async fn find_by_email(email: &str, state: &AppState) -> Result<User, sqlx::Error> {
    let user = sqlx::query_file_as!(User, "queries/v1/users/find_users_by_email.sql", email)
        .fetch_one(&state.pool)
        .await?;
    Ok(user)
}
