use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub project: ProjectConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub gui: GuiConfig,
    pub admin_gui: AdminGuiConfig,
    #[serde(default)]
    pub deploy: Option<DeployConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub kind: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuiConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AdminGuiConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeployConfig {
    pub server_ip: String,
    pub ssh_user: String,
    pub domain_name: String,
    pub letsencrypt_email: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let path = Self::find_config_file().ok_or_else(|| {
            "project.toml not found. Copy project.toml.template to project.toml and fill in your values.".to_string()
        })?;

        let contents = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let mut config: Config = toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;

        config.apply_env_overrides();
        Ok(config)
    }

    fn apply_env_overrides(&mut self) {
        if let Ok(v) = std::env::var("BACKEND_HOST") {
            self.server.host = v;
        }
        if let Ok(v) = std::env::var("BACKEND_PORT") {
            if let Ok(p) = v.parse() {
                self.server.port = p;
            }
        }
        if let Ok(v) = std::env::var("DATABASE_URL") {
            self.database.url = v;
        }
        if let Ok(v) = std::env::var("GUI_PORT") {
            if let Ok(p) = v.parse() {
                self.gui.port = p;
            }
        }
        if let Ok(v) = std::env::var("ADMIN_GUI_PORT") {
            if let Ok(p) = v.parse() {
                self.admin_gui.port = p;
            }
        }
    }

    fn find_config_file() -> Option<PathBuf> {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()));

        let mut candidates = vec![
            PathBuf::from("project.toml"),
            PathBuf::from("../project.toml"),
        ];

        if let Some(ref dir) = exe_dir {
            // binary at target/release/ → project root is ../../
            candidates.push(dir.join("../../project.toml"));
            // binary deployed as sibling to project.toml
            candidates.push(dir.join("project.toml"));
        }

        candidates.into_iter().find(|p| p.exists())
    }
}
