use std::path::Path;

use anyhow::Result;
use tempfile::TempDir;

fn codex_command(deepseek_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(deepseek_utils_cargo_bin::cargo_bin("codex")?);
    cmd.env("DEEPSEEK_HOME", deepseek_home);
    Ok(cmd)
}

#[test]
fn debug_models_bundled_prints_json() -> Result<()> {
    let deepseek_home = TempDir::new()?;
    let mut cmd = codex_command(deepseek_home.path())?;
    let output = cmd.args(["debug", "models", "--bundled"]).output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let value: serde_json::Value = serde_json::from_str(&stdout)?;
    assert!(value["models"].is_array());
    assert!(!value["models"].as_array().unwrap_or(&Vec::new()).is_empty());

    Ok(())
}

#[test]
fn debug_models_default_prints_json_without_auth() -> Result<()> {
    let deepseek_home = TempDir::new()?;
    let mut cmd = codex_command(deepseek_home.path())?;
    let output = cmd.args(["debug", "models"]).output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let value: serde_json::Value = serde_json::from_str(&stdout)?;
    assert!(value["models"].is_array());
    assert!(!value["models"].as_array().unwrap_or(&Vec::new()).is_empty());

    Ok(())
}
