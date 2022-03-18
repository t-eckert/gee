# Design

## Version 0.1.0

Gee is designed to operate as a static server and Python WSGI. Users should be
able to run both functionalities independently or together. The guiding design
principles of Gee are ease of use, safety, and speed.

Gee can be started and configured using the `gee` command in the terminal.

### Configuring Gee

Gee is controlled using a command line application which can take configuration
via flag or from a file called `gee.toml`. Alternatively, `JSON` or `YAML` may 
be used. If a setting is configured in multiple places, the value set will be
chosen using the following precedence: passed in via flag, `TOML` file, `JSON`
file, then `YAML` file.

To create a template configuration file use the command

``` bash
gee init
```

By default, this will create a `TOML` config file. Other file formats may be
used by passing the `--format` flag a value of `TOML`, `JSON`, or `YAML`.

### Verifying Gee's Configuration

A user can verify that they have correctly configured Gee using the command

``` bash
gee verify
```



### Running Gee

To start Gee serving requests, use the command

``` bash
gee serve
```

By default, Gee will look for a `gee.toml` file in the current directory, if 
this is not found, it will look for `gee.json` then `gee.yaml`. If no
configuration file is found in any of these formats, Gee will default to the
following configuration

``` toml

```

#### Running a Static Server
