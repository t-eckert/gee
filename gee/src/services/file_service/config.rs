pub struct FileServiceConfig {
    /// `root_dir` is a relative or absolute path on which all relative resource
    /// lookups will be based.
    pub root_dir: String,

    /// `ignored_files` are UNIX globs defining which files will not be served.
    pub ignored_files: Option<Vec<String>>,
}
