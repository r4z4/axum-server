use axum::extract::Path;

pub async fn path_var(Path(id): Path<i32>) -> String {
    id.to_string()
}