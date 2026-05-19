-- Seed tech stack notes for a project initialized from the rustysolid template.
-- Each statement uses ?1 as the project_id parameter, bound by the caller.

INSERT INTO stack_note (project_id, tag, note, file_path, line_number, created_at, updated_at)
VALUES (?1, 'backend', 'Actix-web 4 REST API', 'backend/src/main.rs', NULL, strftime('%s', 'now'), strftime('%s', 'now'));

INSERT INTO stack_note (project_id, tag, note, file_path, line_number, created_at, updated_at)
VALUES (?1, 'database', 'SQLite via sqlx with auto-migrations', 'backend/src/db.rs', NULL, strftime('%s', 'now'), strftime('%s', 'now'));

INSERT INTO stack_note (project_id, tag, note, file_path, line_number, created_at, updated_at)
VALUES (?1, 'api_contract', 'Shared Rust types; TypeScript auto-generated from shared-types/', 'shared-types/src/', NULL, strftime('%s', 'now'), strftime('%s', 'now'));

INSERT INTO stack_note (project_id, tag, note, file_path, line_number, created_at, updated_at)
VALUES (?1, 'frontend', 'SolidJS + Vite; two apps: gui (user-facing) and admin-gui (internal)', NULL, NULL, strftime('%s', 'now'), strftime('%s', 'now'));

INSERT INTO stack_note (project_id, tag, note, file_path, line_number, created_at, updated_at)
VALUES (?1, 'auth', 'Actix-session cookie-based sessions', 'backend/src/auth/', NULL, strftime('%s', 'now'), strftime('%s', 'now'));

INSERT INTO stack_note (project_id, tag, note, file_path, line_number, created_at, updated_at)
VALUES (?1, 'config', 'project.toml for local dev; env vars override; typed Config struct', NULL, NULL, strftime('%s', 'now'), strftime('%s', 'now'));

INSERT INTO stack_note (project_id, tag, note, file_path, line_number, created_at, updated_at)
VALUES (?1, 'tooling', 'Cargo workspace + npm workspaces', 'Cargo.toml', NULL, strftime('%s', 'now'), strftime('%s', 'now'));

INSERT INTO stack_note (project_id, tag, note, file_path, line_number, created_at, updated_at)
VALUES (?1, 'deployment', 'Systemd service deployed via scripts/deploy.sh', 'scripts/', NULL, strftime('%s', 'now'), strftime('%s', 'now'));
