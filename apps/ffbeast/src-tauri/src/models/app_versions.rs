use serde::Serialize;

#[derive(Serialize)]
pub struct AppVersions {
    pub app: String,
    pub controller: String,
}
