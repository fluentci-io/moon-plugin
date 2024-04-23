# Moon Plugin

[![ci](https://github.com/fluentci-io/moon-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/moon-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with [moon](https://moonrepo.dev/moon).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm moon setup
```

## Functions

| Name      | Description                                   |
| --------- | --------------------------------------------- |
| setup     | Install moon cli                              |
| setup_env | Setup the environment by installing all tools |
| node      | Special Node.js commands.                     |
| teardown  | Teardown the environment by uninstalling all tools and deleting temp files. |
| run       | Run all build and test related tasks for the current project.    |
| ci        | Run all affected projects and tasks in a CI environment. |
| ext       | Execute an extension plugin.  |
| clean     | Clean the workspace and delete any stale or invalid artifacts. |
| docker    | Operations for integrating with Docker and Dockerfile(s). |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/moon@v0.1.0?wasm=1", "setup", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: moon
    args: |
      setup
- name: Show Moon version
  run: |
    export PATH=${HOME}/.moon/bin:${PATH}
    type moon
    moon --version
```
