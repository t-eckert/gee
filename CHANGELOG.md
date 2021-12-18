# Changelog

## v0.1.0 (Unreleased) - "Return to Sender"

- FEATURES
    - Requests received on port 8080 are executed by the "application" callable in the same directory in which the server is run.
    - Requests received on port 8080 with a path prefix of "/static" will be served from the "static" directory in the same directory in which the server is run.