use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_moon() -> Result<String, Error> {
    let path = dag().get_env("PATH")?;
    let home = dag().get_env("HOME")?;
    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.moon/bin:{}", home, path),
    )])?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type moon > /dev/null || pkgx curl -fsSL https://moonrepo.dev/install/moon.sh | bash",
        ])?
        .stdout()?;
    Ok(stdout)
}
