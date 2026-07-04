use anyhow::Result;
use predicates::str::contains;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::path::Path;
use tempfile::TempDir;

fn codex_command(deepseek_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(deepseek_utils_cargo_bin::cargo_bin("codex")?);
    cmd.env("DEEPSEEK_HOME", deepseek_home);
    Ok(cmd)
}

#[tokio::test]
async fn marketplace_upgrade_runs_under_plugin() -> Result<()> {
    let deepseek_home = TempDir::new()?;

    codex_command(deepseek_home.path())?
        .args(["plugin", "marketplace", "upgrade"])
        .assert()
        .success()
        .stdout(contains("No configured Git marketplaces to upgrade."));

    Ok(())
}

#[tokio::test]
async fn marketplace_upgrade_json_prints_upgrade_outcome() -> Result<()> {
    let deepseek_home = TempDir::new()?;

    let assert = codex_command(deepseek_home.path())?
        .args(["plugin", "marketplace", "upgrade", "--json"])
        .assert()
        .success();
    let stdout = assert.get_output().stdout.as_slice();
    let actual: serde_json::Value = serde_json::from_slice(stdout)?;

    assert_eq!(
        actual,
        json!({
            "selectedMarketplaces": [],
            "upgradedRoots": [],
            "errors": [],
        })
    );

    Ok(())
}

#[tokio::test]
async fn marketplace_upgrade_no_longer_runs_at_top_level() -> Result<()> {
    let deepseek_home = TempDir::new()?;

    codex_command(deepseek_home.path())?
        .args(["marketplace", "upgrade"])
        .assert()
        .failure()
        .stderr(contains("unrecognized subcommand 'upgrade'"));

    Ok(())
}
