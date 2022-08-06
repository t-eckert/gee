pub struct PythonServiceConfig {
    /// `path` is the URL path where the application will be served.
    pub path: String,

    /// `application` is the relative path to a Python callable that will be
    /// invoked to handle requests.
    pub application: Option<String>,

    /// `application_name` is the name of the callable that will be invoked to
    /// handle requests.
    pub application_name: Option<String>,
}
