#[derive(serde::Deserialize)]
pub struct Config {
    name: String,
    version: semver::Version,
    description: Option<String>,
    #[serde(flatten)]
    data: AppData,
}

#[derive(serde::Deserialize)]
pub struct AppData {
    source: Source,
    target: std::path::PathBuf,
    actions: Vec<Actions>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    File(std::path::PathBuf),
    Url(String),
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Actions {
    Foo,
    Bar,
    Baz,
    FooBar,
    BarBaz,
    #[serde(rename = "fbb")]
    FooBarBaz,
}
