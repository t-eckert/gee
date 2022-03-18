use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use toml;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    fs::read_to_string,
    net::{IpAddr, SocketAddr},
    path::Path,
};

use crate::hashmap;

/// `Config` is the global, immutable configuration used to construct and run
/// the Gee server.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// `address` is the IP address where the Gee server will serve content.
    pub address: IpAddr,

    /// `port` is the port where the Gee server will serve content.
    pub port: u16,

    /// `root_dir` is a relative or absolute path on which all relative resource
    /// lookups will be based.
    pub root_dir: String,

    /// `static_routes` map paths on the server to directories of static assets
    /// to be served.
    pub static_routes: Option<HashMap<String, String>>,

    /// `ignored_files` will not be served as static assets.
    pub ignored_files: Option<Vec<String>>,

    /// `application` is the relative path to a Python callable that will be
    /// invoked to handle requests.
    pub application: Option<String>,

    /// `application_name` is the name of the callable that will be invoked to
    /// handle requests.
    pub application_name: Option<String>,
}

impl Config {
    /// `new` creates a new `Config` instance.
    pub fn new(
        address: IpAddr,
        port: u16,
        root_dir: String,
        static_routes: Option<HashMap<String, String>>,
        ignored_files: Option<Vec<String>>,
        application: Option<String>,
        application_name: Option<String>,
    ) -> Self {
        Self {
            address,
            port,
            root_dir,
            static_routes,
            ignored_files,
            application,
            application_name,
        }
    }

    /// `new_default` creates a new `Config` instance with default values.
    ///
    /// ``` toml
    /// address = 127.0.0.1
    /// port = 8080
    /// root_dir = .
    /// static_routes = {
    ///    "/static" = "./static"
    /// }
    /// ```
    pub fn new_default() -> Self {
        let address = IpAddr::from([127, 0, 0, 1]);
        let port = 8080;
        let root_dir = ".".to_string();
        let static_routes = Some(hashmap!["/static".to_owned() => "./static/".to_owned()]);

        Self::new(address, port, root_dir, static_routes, None, None, None)
    }

    /// `from_file` creates a new `Config` instance from a file.
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let content = read_to_string(path)?;

        match path.extension().unwrap().to_str().unwrap() {
            "toml" => toml::from_str(&content).map_err(|e| e.into()),
            "json" => serde_json::from_str(&content).map_err(|e| e.into()),
            "yaml" | "yml" => serde_yaml::from_str(&content).map_err(|e| e.into()),
            _ => Err(format!(
                "Unsupported file format: {}. File must be a TOML, JSON, or YAML file.",
                path.extension().unwrap().to_str().unwrap()
            )
            .into()),
        }
    }

    // `to_toml` returns the TOML representation of the `Config` instance.
    pub fn to_toml(&self) -> Result<String, Box<dyn Error>> {
        toml::to_string(self).map_err(|e| e.into())
    }

    // `to_json` returns the JSON representation of the `Config` instance.
    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        serde_json::to_string(self).map_err(|e| e.into())
    }

    // `to_yaml` returns the YAML representation of the `Config` instance.
    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>> {
        serde_yaml::to_string(self).map_err(|e| e.into())
    }

    /// `socket_address` returns the `SocketAddr` that the Gee server will serve
    /// content on by joining the `address` and `port`.
    pub fn socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.address, self.port)
    }

    /// `is_static_path` returns whether the given path is a static route.
    /// This is used to determine if a request to this path should be handled
    /// by the `StaticHandler`.
    pub fn is_static_path(&self, path: &str) -> bool {
        self.static_routes.is_some() && self.static_routes.as_ref().unwrap().contains_key(path)
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
            && self.port == other.port
            && self.root_dir == other.root_dir
            && self.static_routes == other.static_routes
            && self.ignored_files == other.ignored_files
            && self.application == other.application
            && self.application_name == other.application_name
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
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: None,
            ignored_files: None,
            application: None,
            application_name: None,
        };

        let actual = Config::new(
            IpAddr::from([127, 0, 0, 1]),
            8080,
            ".".to_string(),
            None,
            None,
            None,
            None,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_default() {
        let expected = Config {
            address: IpAddr::from([127, 0, 0, 1]),
            port: 8080,
            root_dir: ".".to_string(),
            static_routes: Some(hashmap!["/static".to_owned() => "./static/".to_owned()]),
            ignored_files: None,
            application: None,
            application_name: None,
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
