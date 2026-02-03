use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tracing::info;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../rusty/migrations/");

fn establish_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;
    let conn = PgConnection::establish(&database_url).map_err(|e| e.to_string())?;

    Ok(conn)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let conn = &mut establish_connection()?;

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| format!("Migration failed: {e}"))?;
    info!("Migrations applied successfully");

    Ok(())
}
