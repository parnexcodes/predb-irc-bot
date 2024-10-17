use sqlx::PgPool;
use log::{info, debug};
use uuid::Uuid;

pub async fn init_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Initializing database...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS releases (
            id UUID PRIMARY KEY,
            release_name TEXT NOT NULL,
            group_name TEXT NOT NULL,
            category TEXT NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;

    info!("Database initialized successfully.");
    Ok(())
}

pub async fn insert_release(
    pool: &PgPool,
    release_name: &str,
    group_name: &str,
    category: &str,
) -> Result<(), sqlx::Error> {
    debug!("Inserting release into database...");
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO releases (id, release_name, group_name, category) VALUES ($1, $2, $3, $4)"
    )
    .bind(id)
    .bind(release_name)
    .bind(group_name)
    .bind(category)
    .execute(pool)
    .await?;

    info!("Release inserted successfully. ID: {}", id);
    Ok(())
}
