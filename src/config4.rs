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

    /// _Optional:_ A description of the thing this CLI tool is
    /// building for you.
    ///
    description: Option<String>,

    /// Application config fields.
    ///
    /// NOTE: These fields are included alongside the fields above,
    /// e.g:
    ///
    /// ```yaml
    /// name: foo
    /// version: 1.2.3
    /// source: # AppData fields
    ///   file: path/to/file.yaml
    /// target: path/to/target.yaml
    /// actions: [ foo, bar ] # End of AppData fields
    /// ```
    ///
    #[serde(flatten)]
    data: AppData,
}

/// Application config fields.
///
/// Configuration for this CLI tool that tell it where to get data
/// from, write data to, and what operations to perform on it.
///
/// ---
///
/// Back to:
///
/// - [Configuration Reference](./struct.Config.html#structfield.data)
///
#[derive(serde::Deserialize)]
pub struct AppData {
    /// The configuration for the source of data for this tool.
    ///
    source: Source,

    /// A path to write the created thing to.
    ///
    target: std::path::PathBuf,

    /// The operations to perform on the data this tool manipulates.
    ///
    /// This array of operations will be performed in order and an
    /// operation may appear more than once.
    ///
    /// E.g:
    ///
    /// ```yaml
    /// actions: [ foo, bar, baz, bar ]
    /// ```
    ///
    /// See: [possible operations](./enum.Actions.html).
    ///
    actions: Vec<Actions>,
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
/// - [App Configuration](./struct.AppData.html#structfield.source)
/// - [Configuration Reference](./struct.Config.html#structfield.data)
///
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    File(std::path::PathBuf),
    Url(String),
}

/// The possible operations to perform on the data this tool manipulates.
///
/// See each option below for what it does and how it's referenced in
/// config.
///
/// ---
///
/// Back to:
///
/// - [App Configuration](./struct.AppData.html#structfield.actions)
/// - [Configuration Reference](./struct.Config.html#structfield.data)
///
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Actions {
    /// Perform a "foo" operation on the data:
    ///
    /// ```yaml
    /// foo
    /// ```
    Foo,

    /// Perform a "bar" operation on the data:
    ///
    /// ```yaml
    /// bar
    /// ```
    Bar,

    /// Perform a "baz" operation on the data:
    ///
    /// ```yaml
    /// baz
    /// ```
    Baz,

    /// Perform a "foo bar" operation on the data:
    ///
    /// ```yaml
    /// foo_bar
    /// ```
    FooBar,

    /// Perform a "foo baz" operation on the data:
    ///
    /// ```yaml
    /// foo_baz
    /// ```
    BarBaz,

    /// Perform a "foo bar baz" operation on the data:
    ///
    /// ```yaml
    /// fbb
    /// ```
    #[serde(rename = "fbb")]
    FooBarBaz,
}
