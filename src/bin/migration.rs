use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

/// # Panics
///
/// Will panic if `DATABASE_URL` is not set or invalide
fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn main() {
    let conn = &mut establish_connection();
    let _ = conn.run_pending_migrations(MIGRATIONS);
}
