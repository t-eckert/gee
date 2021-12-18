use std::{collections::HashMap, fmt::Write, io::Read};

/// Environ contains values to be passed to the Python server application.
pub struct Environ {
    /// The HTTP request method, such as GET or POST.
    pub request_method: String,

    /// The initial portion of the request URL's "path" that corresponds to the application object, so that the
    /// application knows its virtual "location". This may be an empty string, if the application corresponds
    /// to the "root" of the server.
    pub script_name: String,

    /// The remainder of the URL's "path", designating the virtual "location" of the request's target within the
    /// application. This may be an empty string, if the request URL targets the application root and does not
    /// contain a trailing slash.
    pub path_info: String,

    /// The portion of the request URL that follows the "?", if any. May be empty or absent.
    pub query_string: String,

    /// The contents of any Content-Type fields in the HTTP request. May be empty or absent.
    pub content_type: String,

    /// The contents of any Content-Length fields in the HTTP request. May be empty or absent.
    pub content_length: String,

    pub server_name: String,

    pub server_port: String,

    /// The version of the protocol the client used to send the request. Typically this will be something like
    /// "HTTP/1.0" or "HTTP/1.1" and may be used by the application to determine how to treat any HTTP request headers.
    pub server_protocol: String,

    /// Variables corresponding to the client-supplied HTTP request headers (i.e., variables whose names begin with
    /// "HTTP_"). The presence or absence of these variables should correspond with the presence or absence of the appropriate HTTP header in the request.
    pub http_variables: HashMap<String, String>,

    /// The tuple (1, 0), representing WSGI version 1.0.
    wsgi_version: (u32, u32),

    /// String representing the "scheme" portion of the URL at which the application is being invoked.
    /// Normally, this will have the value "http" or "https", as appropriate.
    wsgi_url_scheme: String,

    /// Input stream (file-like object) from which the HTTP request body bytes can be read. (The server or gateway may
    /// perform reads on-demand as requested by the application, or it may pre- read the client's request body and
    /// buffer it in-memory or on disk, or use any other technique for providing such an input stream, according to its preference.)
    wsgi_input: Box<dyn Read>,

    /// An output stream (file-like object) to which error output can be written, for the purpose of recording
    /// program or other errors in a standardized and possibly centralized location. This should be a "text mode"
    /// stream; i.e., applications should use "\n" as a line ending, and assume that it will be converted to the
    /// correct line ending by the server/gateway.
    /// For many servers, wsgi.errors will be the server's main error log. Alternatively, this may be sys.stderr, or
    /// a log file of some sort. The server's documentation should include an explanation of how to configure this or
    /// where to find the recorded output. A server or gateway may supply different error streams to different
    /// applications, if this is desired.
    wsgi_errors: Box<dyn Write>,

    /// Value should evaluate true if the application object may be simultaneously invoked by another thread in the
    /// same process, and should evaluate false otherwise.
    wsgi_multithread: bool,

    /// Value should evaluate true if an equivalent application object may be simultaneously invoked by another
    /// process, and should evaluate false otherwise.
    wsgi_multiprocess: bool,

    /// Value should evaluate true if the server or gateway expects (but does not guarantee!) that the application
    /// will only be invoked this one time during the life of its containing process. Normally, this will only be
    /// true for a gateway based on CGI (or something similar).
    wsgi_run_once: bool,
}
