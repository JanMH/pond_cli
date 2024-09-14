# Pond CLI

Pond is a command-line tool for deploying artifacts based on a manifest file. It uses a configuration file to manage deployment settings and supports different profiles for various environments.

## Features

- Deploy artifacts specified in a manifest file.
- Use different profiles for configuration.
- Automatically resolve relative paths in the manifest file.

## Prerequisites

- Rust (for building the project)
- `cargo` (Rust package manager)

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/JanMH/pond_cli.git
    cd pond
    ```

2. Install using cargo install

    ```sh
    cargo install --path=.
    ```

The project will be built and installed in the `~/.cargo/bin` directory. Make sure to add this directory to your `PATH` environment variable if you haven't already done so.

## Configuration

Pond uses a configuration file (`.pond.toml`) located in the user's home directory. The configuration file should contain the following:

```toml
host = "your_host"
access_token = "your_access_token"
```

You can also specify different profiles in the configuration file:

```toml
[default]
host = "default_host"
access_token = "default_access_token"

[production]
host = "production_host"
access_token = "production_access_token"
```

## Usage

### Deploy Command

To deploy an artifact, use the `deploy` command with the path to the manifest file:

```sh
pond deploy --manifest path/to/your/pond.toml
```

You can also specify a profile to use:

```sh
pond --profile production deploy --manifest path/to/your/pond.toml
```

### Manifest File

The manifest file (`pond.toml`) should contain the following:

```toml
name = "your_project_name"
artifact = "path/to/your/artifact"
deployment_type="static-site"
```

The `artifact` field specifies the path to the artifact that should be deployed. This can be a directory or a file. The `deployment_type` field specifies the type of deployment. Currently, only `static-site` is supported by the [server](https://github.com/JanMH/pond_server).

## Example

1. Create a configuration file in your home directory (`~/.pond.toml`):

    ```toml
    [default]
    host = "pond.your-server.com"
    access_token = "your_access_token"
    ```

2. Create a manifest file (`pond.toml`):

    ```toml
    name = "example_project"
    artifact = "./public"
    deployment_type="static-site"
    ```

3. Run the deploy command inside the directory containing the manifest file:

    ```sh
    pond deploy
    ```

## Contributing

Feel free to open a pull request or an issue if you have any suggestions or improvements. Alternatively, you can contact me at [jan@jan-herlyn.com](mailto:jan@jan-herlyn.com) if you want to get involved in the development but don't know where to start.

## License

This project is licensed under the MIT License.