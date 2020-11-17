/// # Config Reference
///
/// Config file reference for `a_cli_tool`.
///
/// By default `a_cli_tool` looks for configuration in `./config.yaml`,
/// unless another path is specified with the `-c/--config` parameter.
///
/// `Config` details the structure of the configuration.
///
/// NOTE: Field names will appear in YAML as they appear here, unless
/// otherwise specified in the accompanying description.
///
#[derive(serde::Deserialize)]
pub struct Config {
    /// The name of the thing this CLI tool is building for you.
    ///
    name: String,

    /// The version of the thing this CLI tool will build for you.
    ///
    /// This is a [SemVer][semver] version, e.g:
    ///
    /// ```yaml
    /// version: 1.2.3
    /// ```
    ///
    /// [semver]: https://semver.org/
    ///
    version: semver::Version,

    /// A description of the thing this CLI tool is building for you.
    ///
    description: String,

    /// The configuration for the source of data for this tool.
    ///
    source: Source,

    /// A path to write the created thing to.
    ///
    target: std::path::PathBuf,
}

/// The configuration for the source of data for this tool.
///
/// This can either be set to a local file:
///
/// ```yaml
/// source:
///   file: path/to/file.yaml
/// ```
///
/// Or a URL:
///
/// ```yaml
/// source:
///   url: https://urlofsource.com/sourcedata/
/// ```
///
/// ---
///
/// Back to:
///
/// - [Configuration Reference](./struct.Config.html#structfield.source)
///
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// ```yaml
    /// file: <path to file>
    /// ```
    ///
    File(std::path::PathBuf),

    /// ```yaml
    /// url: <url>
    /// ```
    ///
    Url(String),
}
