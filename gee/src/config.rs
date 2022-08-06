use serde::{Deserialize, Serialize};
use toml;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    fs::read_to_string,
};

use crate::hashmap;

/// `Config` is the global, immutable configuration used to construct and run
/// the Gee server.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// `port` is the port where the Gee server will serve content.
    pub port: u16,

    /// `root_dir` is a relative or absolute path on which all relative resource
    /// lookups will be based.
    pub root_dir: String,
}

impl Config {
    /// `new` creates a new `Config` instance.
    pub fn new(
        port: u16,
    ) -> Self {
        Self {
            port,
            root_dir,
        }
    }

    /// `new_default` creates a new `Config` instance with default values.
    ///
    /// ``` toml
    /// port = 8080
    /// root_dir = .
    /// ```
    pub fn new_default() -> Self {
        let port = 8080;
        let root_dir = ".".to_string();

        Self::new(port, root_dir)
    }

    /// `from_file` creates a new `Config` instance from a file.
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let content = read_to_string(path)?;
        toml::from_str(&content).map_err(|e| e.into())
    }

    // `to_toml` returns the TOML representation of the `Config` instance.
    pub fn to_toml(&self) -> Result<String, Box<dyn Error>> {
        toml::to_string(self).map_err(|e| e.into())
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
            && self.port == other.port
            && self.root_dir == other.root_dir
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Have this error out properly if the config cannot be serialized.
        write!(f, "{}", self.to_toml().unwrap_or("".to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let expected = Config {
            port: 8080,
            root_dir: ".".to_string(),
        };

        let actual = Config::new(
            8080,
            ".".to_string(),
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_default() {
        let expected = Config {
            port: 8080,
            root_dir: ".".to_string(),
        };

        let actual = Config::new_default();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_from_file_with_nonexistent_file() {
        let path = Path::new("/tmp/gee_config.toml");

        assert!(Config::from_file(&path).is_err());
    }

    #[test]
    fn test_from_file_with_config_valid_00() {
        let path = Path::new("./src/fixtures/test_config_valid_00.toml");

        let expected = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: Some(hashmap!("/".to_owned() => "./".to_owned())),
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let actual = Config::from_file(&path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_from_file_with_config_valid_01() {
        let path = Path::new("./src/fixtures/test_config_valid_01.toml");

        let expected = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: Some(hashmap!("/".to_owned() => "./".to_owned())),
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let actual = Config::from_file(&path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_from_file_with_config_valid_02() {
        let path = Path::new("./src/fixtures/test_config_valid_02.json");

        let expected = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: Some(hashmap!("/".to_owned() => "./".to_owned())),
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let actual = Config::from_file(&path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_from_file_with_config_valid_03() {
        let path = Path::new("./src/fixtures/test_config_valid_03.yaml");

        let expected = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: Some(hashmap!("/".to_owned() => "./".to_owned())),
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let actual = Config::from_file(&path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_from_file_with_config_invalid_00() {
        let path = Path::new("./src/fixtures/test_config_invalid_00.toml");

        let expected = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: Some(hashmap!("/".to_owned() => "./".to_owned())),
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let actual = Config::from_file(&path);
        assert!(actual.is_err());
    }

    #[test]
    fn test_socket_address() {
        let expected = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8080);

        let config = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: None,
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let actual = config.socket_address();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_socket_path() {
        let config = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: Some(hashmap!["/static".to_owned() => "./static/".to_owned()]),
            ignored_files: None,
            application: None,
            application_name: None,
        };

        assert!(config.is_static_path("/static"));
        assert!(!config.is_static_path("/"));
        assert!(!config.is_static_path("/foo"));
    }

    #[test]
    fn test_equality() {
        let config1 = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: None,
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let config2 = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: None,
            ignored_files: None,
            application: None,
            application_name: None,
        };

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_inequality() {
        let config1 = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: None,
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let config2 = Config {
            address: IpAddr::from([126, 0, 0, 1]),
            port: 8081,
            root_dir: "..".to_string(),
            static_routes: None,
            ignored_files: None,
            application: None,
            application_name: None,
        };

        assert_ne!(config1, config2);
    }
}
