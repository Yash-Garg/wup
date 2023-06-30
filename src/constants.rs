pub const API_BASE_URL: &str = "https://api.github.com";
pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub const VALID_OS: [&str; 2] = ["windows", ".exe"];
pub const VALID_ARCH: [&str; 3] = ["64", "x64", "x86_64"];
pub const INVALID_ARCH_OS: [&str; 3] = ["arm", "apple", "linux"];
pub const VERSION_STORE: &str = ".vstore.json";
pub const CLI_CONFIG: &str = "config.yml";
