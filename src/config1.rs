#[derive(serde::Deserialize)]
pub struct Config {
    name: String,
    version: semver::Version,
    description: String,
    source: Source,
    target: std::path::PathBuf,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    File(std::path::PathBuf),
    Url(String),
}
