#[path = "../db.rs"]
mod db;

fn main() -> std::io::Result<()> {
    db::run_startup_migrations()?;
    println!("Migrations applied successfully");
    Ok(())
}
