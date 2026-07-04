use deepseek_config::test_support::CloudConfigBundleFixture;
use deepseek_core::config::Config;
use deepseek_core::config::ConfigBuilder;
use deepseek_exec_server::EnvironmentManager;
use deepseek_exec_server::LOCAL_ENVIRONMENT_ID;
use deepseek_extension_api::ExtensionData;
use deepseek_extension_api::ExtensionDataInit;
use deepseek_extension_api::ExtensionRegistryBuilder;
use deepseek_extension_api::McpServerContribution;
use deepseek_extension_api::McpServerContributionContext;
use deepseek_protocol::capabilities::CapabilityRootLocation;
use deepseek_protocol::capabilities::SelectedCapabilityRoot;
use deepseek_utils_path_uri::PathUri;
use pretty_assertions::assert_eq;
use std::sync::Arc;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Eq)]
struct ContributionSummary {
    name: String,
    plugin_id: String,
    plugin_display_name: String,
    selection_order: usize,
    enabled: bool,
}

#[tokio::test]
async fn selected_plugin_servers_use_managed_requirements_for_the_selected_root_id() -> TestResult {
    let deepseek_home = tempfile::tempdir()?;
    let plugin_root = tempfile::tempdir()?;
    std::fs::create_dir_all(plugin_root.path().join(".deepseek-plugin"))?;
    std::fs::write(
        plugin_root.path().join(".deepseek-plugin/plugin.json"),
        r#"{"name":"different-manifest-name","interface":{"displayName":"Selected Demo"}}"#,
    )?;
    std::fs::write(
        plugin_root.path().join(".mcp.json"),
        r#"{
  "mcpServers": {
    "allowed": {"command":"allowed-command"},
    "mismatched": {"command":"wrong-command"},
    "unlisted": {"command":"unlisted-command"}
  }
}"#,
    )?;
    let config = ConfigBuilder::default()
        .deepseek_home(deepseek_home.path().to_path_buf())
        .fallback_cwd(Some(deepseek_home.path().to_path_buf()))
        .cloud_config_bundle(
            CloudConfigBundleFixture::loader_with_enterprise_requirement(
                r#"
[plugins."selected-root".mcp_servers.allowed.identity]
command = "allowed-command"

[plugins."selected-root".mcp_servers.mismatched.identity]
command = "expected-command"
"#,
            ),
        )
        .build()
        .await?;

    let contributions = selected_plugin_contributions(&config, plugin_root.path()).await?;

    assert_eq!(
        contributions,
        vec![
            ContributionSummary {
                name: "allowed".to_string(),
                plugin_id: "selected-root".to_string(),
                plugin_display_name: "Selected Demo".to_string(),
                selection_order: 0,
                enabled: true,
            },
            ContributionSummary {
                name: "mismatched".to_string(),
                plugin_id: "selected-root".to_string(),
                plugin_display_name: "Selected Demo".to_string(),
                selection_order: 0,
                enabled: false,
            },
            ContributionSummary {
                name: "unlisted".to_string(),
                plugin_id: "selected-root".to_string(),
                plugin_display_name: "Selected Demo".to_string(),
                selection_order: 0,
                enabled: false,
            },
        ]
    );
    Ok(())
}

async fn selected_plugin_contributions(
    config: &Config,
    plugin_root: &std::path::Path,
) -> Result<Vec<ContributionSummary>, Box<dyn std::error::Error>> {
    let mut builder = ExtensionRegistryBuilder::new();
    deepseek_mcp_extension::install_executor_plugins(
        &mut builder,
        Arc::new(EnvironmentManager::default_for_tests()),
    );
    let registry = builder.build();
    let mut thread_init = ExtensionDataInit::new();
    thread_init.insert(vec![SelectedCapabilityRoot {
        id: "selected-root".to_string(),
        location: CapabilityRootLocation::Environment {
            environment_id: LOCAL_ENVIRONMENT_ID.to_string(),
            path: PathUri::from_host_native_path(plugin_root)?,
        },
    }]);
    let thread_store = ExtensionData::new_with_init("test-thread", thread_init.clone());
    let available_environment_ids = vec![LOCAL_ENVIRONMENT_ID.to_string()];

    Ok(registry.mcp_server_contributors()[0]
        .contribute(McpServerContributionContext::for_step(
            config,
            &thread_init,
            &thread_store,
            &available_environment_ids,
        ))
        .await
        .into_iter()
        .map(|contribution| match contribution {
            McpServerContribution::SelectedPlugin {
                name,
                plugin_id,
                plugin_display_name,
                selection_order,
                config,
            } => ContributionSummary {
                name,
                plugin_id,
                plugin_display_name,
                selection_order,
                enabled: config.enabled,
            },
            McpServerContribution::Set { .. }
            | McpServerContribution::SelectedPluginConnectors { .. }
            | McpServerContribution::Remove { .. } => {
                panic!("expected selected plugin contribution")
            }
        })
        .collect())
}
