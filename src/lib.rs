use extism_pdk::*;
use fluentci_pdk::dag;

use crate::helpers::setup_moon;

pub mod helpers;

#[plugin_fn]
pub fn setup() -> FnResult<String> {
    let stdout = setup_moon()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn node(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "node", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn setup_env(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "setup", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn teardown(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "teardown", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn check(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "check", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn ci(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "ci", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn run(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "run", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn ext(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "ext", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn clean(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "clean", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn docker(args: String) -> FnResult<String> {
    setup_moon()?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["moon", "docker", &args])?
        .stdout()?;
    Ok(stdout)
}
