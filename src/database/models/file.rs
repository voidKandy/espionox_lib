// ------ FILES ------ //
#[derive(sqlx::FromRow, Clone)]
pub struct FileModelSql {
    pub id: String,
    pub thread_id: String,
    pub filepath: String,
    pub parent_dir_path: String,
    pub summary: String,
    pub summary_embedding: pgvector::Vector,
}

pub struct GetFileParams {
    pub filepath: String,
}

pub struct CreateFileBody {
    pub thread_id: String,
    pub filepath: String,
    pub parent_dir_path: String,
    pub summary: String,
    pub summary_embedding: pgvector::Vector,
}

pub struct DeleteFileParams {
    pub filepath: String,
}