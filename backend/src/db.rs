#[cfg(not(any(feature = "db-sqlite", feature = "db-postgres")))]
compile_error!("Enable one DB feature: db-sqlite or db-postgres");

#[cfg(all(feature = "db-sqlite", feature = "db-postgres"))]
compile_error!("Enable only one DB feature: db-sqlite or db-postgres");

use std::{env, fs, io, path::Path};

fn database_url_from_project_conf() -> Option<String> {
    // Support running from repo root or from the backend directory.
    let candidates = [Path::new("project.conf"), Path::new("../project.conf")];

    for path in candidates {
        let Ok(contents) = fs::read_to_string(path) else {
            continue;
        };

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            if key.trim() != "DATABASE_URL" {
                continue;
            }

            let value = value.trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}

#[cfg(feature = "db-sqlite")]
mod sqlite {
    use super::*;
    use refinery::embed_migrations;
    use rusqlite::Connection;

    embed_migrations!("migrations/sqlite");

    pub fn run_startup_migrations() -> io::Result<()> {
        let path = env::var("DATABASE_URL")
            .ok()
            .or_else(database_url_from_project_conf)
            .unwrap_or_else(|| "app.db".to_string());
        let mut conn = Connection::open(path).map_err(io::Error::other)?;
        migrations::runner().run(&mut conn).map_err(io::Error::other)?;
        Ok(())
    }
}

#[cfg(feature = "db-postgres")]
mod postgres_db {
    use super::*;
    use postgres::{Client, NoTls};
    use refinery::embed_migrations;

    embed_migrations!("migrations/postgres");

    pub fn run_startup_migrations() -> io::Result<()> {
        let url = env::var("DATABASE_URL")
            .ok()
            .or_else(database_url_from_project_conf)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "DATABASE_URL is required when db-postgres feature is enabled (set env var or project.conf)",
                )
            })?;
        let mut client = Client::connect(&url, NoTls).map_err(io::Error::other)?;
        migrations::runner().run(&mut client).map_err(io::Error::other)?;
        Ok(())
    }
}

#[cfg(feature = "db-sqlite")]
pub use sqlite::run_startup_migrations;

#[cfg(feature = "db-postgres")]
pub use postgres_db::run_startup_migrations;
