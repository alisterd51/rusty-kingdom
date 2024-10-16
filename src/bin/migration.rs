use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rusty_kingdom::database::establish_connection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

fn main() {
    let conn = &mut establish_connection();
    let _ = conn.run_pending_migrations(MIGRATIONS);
}
