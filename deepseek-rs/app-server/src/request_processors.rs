use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::bespoke_event_handling::maybe_emit_hook_prompt_item_completed;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use deepseek_analytics::AnalyticsEventsClient;
use deepseek_analytics::AnalyticsJsonRpcError;
use deepseek_analytics::InputError;
use deepseek_analytics::TurnSteerRequestError;
use deepseek_app_server_protocol::Account;
use deepseek_app_server_protocol::AccountLoginCompletedNotification;
use deepseek_app_server_protocol::AccountTokenUsageDailyBucket;
use deepseek_app_server_protocol::AccountTokenUsageSummary;
use deepseek_app_server_protocol::AccountUpdatedNotification;
use deepseek_app_server_protocol::AddCreditsNudgeCreditType;
use deepseek_app_server_protocol::AddCreditsNudgeEmailStatus;
use deepseek_app_server_protocol::AdditionalContextEntry;
use deepseek_app_server_protocol::AdditionalContextKind;
use deepseek_app_server_protocol::AppListUpdatedNotification;
use deepseek_app_server_protocol::AppSummary;
use deepseek_app_server_protocol::AppTemplateSummary;
use deepseek_app_server_protocol::AppTemplateUnavailableReason;
use deepseek_app_server_protocol::AppsListParams;
use deepseek_app_server_protocol::AppsListResponse;
use deepseek_app_server_protocol::AskForApproval;
use deepseek_app_server_protocol::AuthMode;
use deepseek_app_server_protocol::CancelLoginAccountParams;
use deepseek_app_server_protocol::CancelLoginAccountResponse;
use deepseek_app_server_protocol::CancelLoginAccountStatus;
use deepseek_app_server_protocol::ClientInfo;
use deepseek_app_server_protocol::ClientRequest;
use deepseek_app_server_protocol::ClientResponsePayload;
use deepseek_app_server_protocol::DeepSeekErrorInfo;
use deepseek_app_server_protocol::CollaborationModeListParams;
use deepseek_app_server_protocol::CollaborationModeListResponse;
use deepseek_app_server_protocol::CommandExecParams;
use deepseek_app_server_protocol::CommandExecResizeParams;
use deepseek_app_server_protocol::CommandExecTerminateParams;
use deepseek_app_server_protocol::CommandExecWriteParams;
use deepseek_app_server_protocol::ConfigWarningNotification;
use deepseek_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use deepseek_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use deepseek_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use deepseek_app_server_protocol::ConversationGitInfo;
use deepseek_app_server_protocol::ConversationSummary;
use deepseek_app_server_protocol::DeprecationNoticeNotification;
use deepseek_app_server_protocol::DynamicToolFunctionSpec;
use deepseek_app_server_protocol::DynamicToolNamespaceTool;
use deepseek_app_server_protocol::DynamicToolSpec;
use deepseek_app_server_protocol::EnvironmentAddParams;
use deepseek_app_server_protocol::EnvironmentAddResponse;
use deepseek_app_server_protocol::EnvironmentInfoParams;
use deepseek_app_server_protocol::EnvironmentInfoResponse;
use deepseek_app_server_protocol::EnvironmentShellInfo;
use deepseek_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use deepseek_app_server_protocol::ExperimentalFeatureListParams;
use deepseek_app_server_protocol::ExperimentalFeatureListResponse;
use deepseek_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use deepseek_app_server_protocol::FeedbackUploadParams;
use deepseek_app_server_protocol::FeedbackUploadResponse;
use deepseek_app_server_protocol::GetAccountParams;
use deepseek_app_server_protocol::GetAccountRateLimitsResponse;
use deepseek_app_server_protocol::GetAccountResponse;
use deepseek_app_server_protocol::GetAccountTokenUsageResponse;
use deepseek_app_server_protocol::GetAuthStatusParams;
use deepseek_app_server_protocol::GetAuthStatusResponse;
use deepseek_app_server_protocol::GetConversationSummaryParams;
use deepseek_app_server_protocol::GetConversationSummaryResponse;
use deepseek_app_server_protocol::GetWorkspaceMessagesResponse;
use deepseek_app_server_protocol::GitDiffToRemoteParams;
use deepseek_app_server_protocol::GitDiffToRemoteResponse;
use deepseek_app_server_protocol::GitInfo as ApiGitInfo;
use deepseek_app_server_protocol::HookMetadata;
use deepseek_app_server_protocol::HooksListParams;
use deepseek_app_server_protocol::HooksListResponse;
use deepseek_app_server_protocol::InitializeParams;
use deepseek_app_server_protocol::InitializeResponse;
use deepseek_app_server_protocol::JSONRPCErrorError;
use deepseek_app_server_protocol::ListMcpServerStatusParams;
use deepseek_app_server_protocol::ListMcpServerStatusResponse;
use deepseek_app_server_protocol::LoginAccountParams;
use deepseek_app_server_protocol::LoginAccountResponse;
use deepseek_app_server_protocol::LoginApiKeyParams;
use deepseek_app_server_protocol::LogoutAccountResponse;
use deepseek_app_server_protocol::MarketplaceAddParams;
use deepseek_app_server_protocol::MarketplaceAddResponse;
use deepseek_app_server_protocol::MarketplaceInterface;
use deepseek_app_server_protocol::MarketplaceRemoveParams;
use deepseek_app_server_protocol::MarketplaceRemoveResponse;
use deepseek_app_server_protocol::MarketplaceUpgradeErrorInfo;
use deepseek_app_server_protocol::MarketplaceUpgradeParams;
use deepseek_app_server_protocol::MarketplaceUpgradeResponse;
use deepseek_app_server_protocol::McpResourceReadParams;
use deepseek_app_server_protocol::McpResourceReadResponse;
use deepseek_app_server_protocol::McpServerOauthLoginCompletedNotification;
use deepseek_app_server_protocol::McpServerOauthLoginParams;
use deepseek_app_server_protocol::McpServerOauthLoginResponse;
use deepseek_app_server_protocol::McpServerRefreshResponse;
use deepseek_app_server_protocol::McpServerStatus;
use deepseek_app_server_protocol::McpServerStatusDetail;
use deepseek_app_server_protocol::McpServerToolCallParams;
use deepseek_app_server_protocol::McpServerToolCallResponse;
use deepseek_app_server_protocol::MemoryResetResponse;
use deepseek_app_server_protocol::MockExperimentalMethodParams;
use deepseek_app_server_protocol::MockExperimentalMethodResponse;
use deepseek_app_server_protocol::ModelListParams;
use deepseek_app_server_protocol::ModelListResponse;
use deepseek_app_server_protocol::PermissionProfileListParams;
use deepseek_app_server_protocol::PermissionProfileListResponse;
use deepseek_app_server_protocol::PermissionProfileSummary;
use deepseek_app_server_protocol::PluginDetail;
use deepseek_app_server_protocol::PluginInstallParams;
use deepseek_app_server_protocol::PluginInstallResponse;
use deepseek_app_server_protocol::PluginInstalledParams;
use deepseek_app_server_protocol::PluginInstalledResponse;
use deepseek_app_server_protocol::PluginInterface;
use deepseek_app_server_protocol::PluginListMarketplaceKind;
use deepseek_app_server_protocol::PluginListParams;
use deepseek_app_server_protocol::PluginListResponse;
use deepseek_app_server_protocol::PluginMarketplaceEntry;
use deepseek_app_server_protocol::PluginReadParams;
use deepseek_app_server_protocol::PluginReadResponse;
use deepseek_app_server_protocol::PluginShareCheckoutParams;
use deepseek_app_server_protocol::PluginShareCheckoutResponse;
use deepseek_app_server_protocol::PluginShareContext;
use deepseek_app_server_protocol::PluginShareDeleteParams;
use deepseek_app_server_protocol::PluginShareDeleteResponse;
use deepseek_app_server_protocol::PluginShareDiscoverability;
use deepseek_app_server_protocol::PluginShareListItem;
use deepseek_app_server_protocol::PluginShareListParams;
use deepseek_app_server_protocol::PluginShareListResponse;
use deepseek_app_server_protocol::PluginSharePrincipal;
use deepseek_app_server_protocol::PluginSharePrincipalType;
use deepseek_app_server_protocol::PluginShareSaveParams;
use deepseek_app_server_protocol::PluginShareSaveResponse;
use deepseek_app_server_protocol::PluginShareTarget;
use deepseek_app_server_protocol::PluginShareUpdateDiscoverability;
use deepseek_app_server_protocol::PluginShareUpdateTargetsParams;
use deepseek_app_server_protocol::PluginShareUpdateTargetsResponse;
use deepseek_app_server_protocol::PluginSkillReadParams;
use deepseek_app_server_protocol::PluginSkillReadResponse;
use deepseek_app_server_protocol::PluginSource;
use deepseek_app_server_protocol::PluginSummary;
use deepseek_app_server_protocol::PluginUninstallParams;
use deepseek_app_server_protocol::PluginUninstallResponse;
use deepseek_app_server_protocol::RateLimitResetCreditsSummary;
use deepseek_app_server_protocol::RequestId;
use deepseek_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use deepseek_app_server_protocol::ReviewStartParams;
use deepseek_app_server_protocol::ReviewStartResponse;
use deepseek_app_server_protocol::ReviewTarget as ApiReviewTarget;
use deepseek_app_server_protocol::SandboxMode;
use deepseek_app_server_protocol::SendAddCreditsNudgeEmailParams;
use deepseek_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use deepseek_app_server_protocol::ServerNotification;
use deepseek_app_server_protocol::ServerRequestResolvedNotification;
use deepseek_app_server_protocol::SkillSummary;
use deepseek_app_server_protocol::SkillsConfigWriteParams;
use deepseek_app_server_protocol::SkillsConfigWriteResponse;
use deepseek_app_server_protocol::SkillsExtraRootsSetParams;
use deepseek_app_server_protocol::SkillsExtraRootsSetResponse;
use deepseek_app_server_protocol::SkillsListParams;
use deepseek_app_server_protocol::SkillsListResponse;
use deepseek_app_server_protocol::SortDirection;
use deepseek_app_server_protocol::Thread;
use deepseek_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use deepseek_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use deepseek_app_server_protocol::ThreadArchiveParams;
use deepseek_app_server_protocol::ThreadArchiveResponse;
use deepseek_app_server_protocol::ThreadArchivedNotification;
use deepseek_app_server_protocol::ThreadBackgroundTerminal;
use deepseek_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use deepseek_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use deepseek_app_server_protocol::ThreadBackgroundTerminalsListParams;
use deepseek_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use deepseek_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use deepseek_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use deepseek_app_server_protocol::ThreadClosedNotification;
use deepseek_app_server_protocol::ThreadCompactStartParams;
use deepseek_app_server_protocol::ThreadCompactStartResponse;
use deepseek_app_server_protocol::ThreadDecrementElicitationParams;
use deepseek_app_server_protocol::ThreadDecrementElicitationResponse;
use deepseek_app_server_protocol::ThreadDeleteParams;
use deepseek_app_server_protocol::ThreadDeleteResponse;
use deepseek_app_server_protocol::ThreadDeletedNotification;
use deepseek_app_server_protocol::ThreadForkParams;
use deepseek_app_server_protocol::ThreadForkResponse;
use deepseek_app_server_protocol::ThreadGoal;
use deepseek_app_server_protocol::ThreadGoalClearParams;
use deepseek_app_server_protocol::ThreadGoalClearResponse;
use deepseek_app_server_protocol::ThreadGoalClearedNotification;
use deepseek_app_server_protocol::ThreadGoalGetParams;
use deepseek_app_server_protocol::ThreadGoalGetResponse;
use deepseek_app_server_protocol::ThreadGoalSetParams;
use deepseek_app_server_protocol::ThreadGoalSetResponse;
use deepseek_app_server_protocol::ThreadGoalStatus;
use deepseek_app_server_protocol::ThreadGoalUpdatedNotification;
use deepseek_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use deepseek_app_server_protocol::ThreadHistoryMode;
use deepseek_app_server_protocol::ThreadIncrementElicitationParams;
use deepseek_app_server_protocol::ThreadIncrementElicitationResponse;
use deepseek_app_server_protocol::ThreadInjectItemsParams;
use deepseek_app_server_protocol::ThreadInjectItemsResponse;
use deepseek_app_server_protocol::ThreadItem;
use deepseek_app_server_protocol::ThreadItemsListParams;
use deepseek_app_server_protocol::ThreadItemsListResponse;
use deepseek_app_server_protocol::ThreadListCwdFilter;
use deepseek_app_server_protocol::ThreadListParams;
use deepseek_app_server_protocol::ThreadListResponse;
use deepseek_app_server_protocol::ThreadLoadedListParams;
use deepseek_app_server_protocol::ThreadLoadedListResponse;
use deepseek_app_server_protocol::ThreadMemoryModeSetParams;
use deepseek_app_server_protocol::ThreadMemoryModeSetResponse;
use deepseek_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use deepseek_app_server_protocol::ThreadMetadataUpdateParams;
use deepseek_app_server_protocol::ThreadMetadataUpdateResponse;
use deepseek_app_server_protocol::ThreadNameUpdatedNotification;
use deepseek_app_server_protocol::ThreadReadParams;
use deepseek_app_server_protocol::ThreadReadResponse;
use deepseek_app_server_protocol::ThreadRealtimeAppendAudioParams;
use deepseek_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use deepseek_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use deepseek_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use deepseek_app_server_protocol::ThreadRealtimeAppendTextParams;
use deepseek_app_server_protocol::ThreadRealtimeAppendTextResponse;
use deepseek_app_server_protocol::ThreadRealtimeListVoicesResponse;
use deepseek_app_server_protocol::ThreadRealtimeStartParams;
use deepseek_app_server_protocol::ThreadRealtimeStartResponse;
use deepseek_app_server_protocol::ThreadRealtimeStartTransport;
use deepseek_app_server_protocol::ThreadRealtimeStopParams;
use deepseek_app_server_protocol::ThreadRealtimeStopResponse;
use deepseek_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use deepseek_app_server_protocol::ThreadResumeParams;
use deepseek_app_server_protocol::ThreadResumeResponse;
use deepseek_app_server_protocol::ThreadRollbackParams;
use deepseek_app_server_protocol::ThreadSearchParams;
use deepseek_app_server_protocol::ThreadSearchResponse;
use deepseek_app_server_protocol::ThreadSearchResult;
use deepseek_app_server_protocol::ThreadSetNameParams;
use deepseek_app_server_protocol::ThreadSetNameResponse;
use deepseek_app_server_protocol::ThreadSettings;
use deepseek_app_server_protocol::ThreadSettingsUpdateParams;
use deepseek_app_server_protocol::ThreadSettingsUpdateResponse;
use deepseek_app_server_protocol::ThreadShellCommandParams;
use deepseek_app_server_protocol::ThreadShellCommandResponse;
use deepseek_app_server_protocol::ThreadSortKey;
use deepseek_app_server_protocol::ThreadSourceKind;
use deepseek_app_server_protocol::ThreadStartParams;
use deepseek_app_server_protocol::ThreadStartResponse;
use deepseek_app_server_protocol::ThreadStartedNotification;
use deepseek_app_server_protocol::ThreadStatus;
use deepseek_app_server_protocol::ThreadTurnsListParams;
use deepseek_app_server_protocol::ThreadTurnsListResponse;
use deepseek_app_server_protocol::ThreadUnarchiveParams;
use deepseek_app_server_protocol::ThreadUnarchiveResponse;
use deepseek_app_server_protocol::ThreadUnarchivedNotification;
use deepseek_app_server_protocol::ThreadUnsubscribeParams;
use deepseek_app_server_protocol::ThreadUnsubscribeResponse;
use deepseek_app_server_protocol::ThreadUnsubscribeStatus;
use deepseek_app_server_protocol::Turn;
use deepseek_app_server_protocol::TurnEnvironmentParams;
use deepseek_app_server_protocol::TurnError;
use deepseek_app_server_protocol::TurnInterruptParams;
use deepseek_app_server_protocol::TurnInterruptResponse;
use deepseek_app_server_protocol::TurnItemsView;
use deepseek_app_server_protocol::TurnStartParams;
use deepseek_app_server_protocol::TurnStartResponse;
use deepseek_app_server_protocol::TurnStatus;
use deepseek_app_server_protocol::TurnSteerParams;
use deepseek_app_server_protocol::TurnSteerResponse;
use deepseek_app_server_protocol::UserInput as V2UserInput;
use deepseek_app_server_protocol::WindowsSandboxReadiness;
use deepseek_app_server_protocol::WindowsSandboxReadinessResponse;
use deepseek_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use deepseek_app_server_protocol::WindowsSandboxSetupMode;
use deepseek_app_server_protocol::WindowsSandboxSetupStartParams;
use deepseek_app_server_protocol::WindowsSandboxSetupStartResponse;
use deepseek_app_server_protocol::WorkspaceMessage;
use deepseek_app_server_protocol::WorkspaceMessageType;
use deepseek_arg0::Arg0DispatchPaths;
use deepseek_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use deepseek_backend_client::Client as BackendClient;
use deepseek_backend_client::DeepSeekWorkspaceMessage as BackendWorkspaceMessage;
use deepseek_backend_client::DeepSeekWorkspaceMessageType as BackendWorkspaceMessageType;
use deepseek_backend_client::DeepSeekWorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use deepseek_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use deepseek_backend_client::RequestError as BackendRequestError;
use deepseek_backend_client::TokenUsageProfile;
use deepseek_chatgpt::connectors;
use deepseek_chatgpt::workspace_settings;
use deepseek_config::CloudConfigBundleLoadError;
use deepseek_config::CloudConfigBundleLoadErrorCode;
use deepseek_config::ConfigLayerStack;
use deepseek_config::loader::project_trust_key;
use deepseek_config::types::McpServerTransportConfig;
use deepseek_connectors::AppInfo;
use deepseek_core::DeepSeekThread;
use deepseek_core::DeepSeekThreadSettingsOverrides;
use deepseek_core::ForkSnapshot;
use deepseek_core::McpManager;
use deepseek_core::NewThread;
#[cfg(test)]
use deepseek_core::SessionMeta;
use deepseek_core::StartThreadOptions;
use deepseek_core::SteerInputError;
use deepseek_core::ThreadConfigSnapshot;
use deepseek_core::ThreadManager;
use deepseek_core::config::Config;
use deepseek_core::config::ConfigOverrides;
use deepseek_core::config::NetworkProxyAuditMetadata;
use deepseek_core::config::edit::ConfigEdit;
use deepseek_core::config::edit::ConfigEditsBuilder;
use deepseek_core::connectors::AccessibleConnectorsStatus;
use deepseek_core::exec::ExecCapturePolicy;
use deepseek_core::exec::ExecExpiration;
use deepseek_core::exec::ExecParams;
use deepseek_core::exec_env::create_env;
use deepseek_core::path_utils;
#[cfg(test)]
use deepseek_core::read_head_for_summary;
use deepseek_core::sandboxing::SandboxPermissions;
use deepseek_core::truncate_rollout_after_turn_id;
use deepseek_core::windows_sandbox::WindowsSandboxLevelExt;
use deepseek_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use deepseek_core::windows_sandbox::WindowsSandboxSetupRequest;
use deepseek_core::windows_sandbox::sandbox_setup_is_complete;
use deepseek_core_plugins::PluginInstallError as CorePluginInstallError;
use deepseek_core_plugins::PluginInstallRequest;
use deepseek_core_plugins::PluginReadRequest;
use deepseek_core_plugins::PluginUninstallError as CorePluginUninstallError;
use deepseek_core_plugins::PluginsManager;
use deepseek_core_plugins::loader::load_plugin_apps;
use deepseek_core_plugins::loader::load_plugin_mcp_servers;
use deepseek_core_plugins::manifest::PluginManifestInterface;
use deepseek_core_plugins::marketplace::MarketplaceError;
use deepseek_core_plugins::marketplace::MarketplacePluginSource;
use deepseek_core_plugins::marketplace_add::MarketplaceAddError;
use deepseek_core_plugins::marketplace_add::MarketplaceAddRequest;
use deepseek_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_deepseek_home;
use deepseek_core_plugins::marketplace_remove::MarketplaceRemoveError;
use deepseek_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use deepseek_core_plugins::marketplace_remove::remove_marketplace;
use deepseek_core_plugins::remote::RemoteMarketplace;
use deepseek_core_plugins::remote::RemoteMarketplaceSource;
use deepseek_core_plugins::remote::RemotePluginCatalogError;
use deepseek_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use deepseek_core_plugins::remote::RemotePluginServiceConfig;
use deepseek_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use deepseek_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use deepseek_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use deepseek_exec_server::EnvironmentManager;
use deepseek_exec_server::LOCAL_ENVIRONMENT_ID;
use deepseek_exec_server::LOCAL_FS;
use deepseek_features::FEATURES;
use deepseek_features::Feature;
use deepseek_features::Stage;
use deepseek_feedback::DeepSeekFeedback;
use deepseek_feedback::FeedbackAttachmentPath;
use deepseek_feedback::FeedbackUploadOptions;
use deepseek_git_utils::git_diff_to_remote;
use deepseek_git_utils::resolve_root_git_project_for_trust;
use deepseek_login::AuthManager;
use deepseek_login::DeepSeekAuth;
use deepseek_login::ServerOptions as LoginServerOptions;
use deepseek_login::ShutdownHandle;
use deepseek_login::auth::login_with_chatgpt_auth_tokens;
use deepseek_login::complete_device_code_login;
use deepseek_login::login_with_api_key;
use deepseek_login::oauth_client_id;
use deepseek_login::request_device_code;
use deepseek_login::run_login_server;
use deepseek_mcp::McpRuntimeContext;
use deepseek_mcp::McpServerStatusSnapshot;
use deepseek_mcp::McpSnapshotDetail;
use deepseek_mcp::collect_mcp_server_status_snapshot_with_detail;
use deepseek_mcp::discover_supported_scopes_with_http_client;
use deepseek_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use deepseek_mcp::resolve_oauth_scopes;
use deepseek_memories_write::clear_memory_roots_contents;
use deepseek_model_provider::create_model_provider;
use deepseek_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use deepseek_protocol::ThreadId;
use deepseek_protocol::config_types::CollaborationMode;
use deepseek_protocol::config_types::ForcedLoginMethod;
use deepseek_protocol::config_types::Personality;
use deepseek_protocol::config_types::ReasoningSummary;
use deepseek_protocol::config_types::TrustLevel;
use deepseek_protocol::config_types::WindowsSandboxLevel;
use deepseek_protocol::error::DeepSeekErr;
use deepseek_protocol::error::Result as DeepSeekResult;
#[cfg(test)]
use deepseek_protocol::items::TurnItem;
use deepseek_protocol::models::ResponseItem;
use deepseek_protocol::deepseek_models::ReasoningEffort;
#[cfg(test)]
use deepseek_protocol::permissions::FileSystemSandboxPolicy;
use deepseek_protocol::protocol::AgentStatus;
use deepseek_protocol::protocol::ConversationAudioParams;
use deepseek_protocol::protocol::ConversationSpeechParams;
use deepseek_protocol::protocol::ConversationStartParams;
use deepseek_protocol::protocol::ConversationStartTransport;
use deepseek_protocol::protocol::ConversationTextParams;
use deepseek_protocol::protocol::EventMsg;
#[cfg(test)]
use deepseek_protocol::protocol::GitInfo as CoreGitInfo;
use deepseek_protocol::protocol::InitialHistory;
use deepseek_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use deepseek_protocol::protocol::Op;
use deepseek_protocol::protocol::RealtimeVoicesList;
use deepseek_protocol::protocol::ResumedHistory;
use deepseek_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use deepseek_protocol::protocol::ReviewRequest;
use deepseek_protocol::protocol::ReviewTarget as CoreReviewTarget;
use deepseek_protocol::protocol::RolloutItem;
use deepseek_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use deepseek_protocol::protocol::SessionMetaLine;
use deepseek_protocol::protocol::TurnEnvironmentSelection;
use deepseek_protocol::protocol::TurnEnvironmentSelections;
use deepseek_protocol::protocol::USER_MESSAGE_BEGIN;
use deepseek_protocol::protocol::W3cTraceContext;
use deepseek_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use deepseek_protocol::user_input::UserInput as CoreInputItem;
use deepseek_rmcp_client::perform_oauth_login_return_url_with_http_client;
use deepseek_rollout::is_persisted_rollout_item;
use deepseek_rollout::state_db::StateDbHandle;
use deepseek_rollout::state_db::reconcile_rollout;
use deepseek_state::ThreadMetadata;
use deepseek_state::log_db::LogDbLayer;
use deepseek_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use deepseek_thread_store::DeleteThreadParams as StoreDeleteThreadParams;
use deepseek_thread_store::GitInfoPatch as StoreGitInfoPatch;
use deepseek_thread_store::ListItemsParams as StoreListItemsParams;
use deepseek_thread_store::ListThreadsParams as StoreListThreadsParams;
use deepseek_thread_store::LocalThreadStore;
use deepseek_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use deepseek_thread_store::ReadThreadParams as StoreReadThreadParams;
use deepseek_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use deepseek_thread_store::SortDirection as StoreSortDirection;
use deepseek_thread_store::StoredThread;
use deepseek_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use deepseek_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use deepseek_thread_store::ThreadSortKey as StoreThreadSortKey;
use deepseek_thread_store::ThreadStore;
use deepseek_thread_store::ThreadStoreError;
use deepseek_utils_absolute_path::AbsolutePathBuf;
use deepseek_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use deepseek_app_server_protocol::ServerRequest;

mod account_processor;
mod apps_processor;
mod catalog_processor;
mod command_exec_processor;
mod config_processor;
mod environment_processor;
mod external_agent_config_processor;
mod external_agent_session_import;
mod feedback_doctor_report;
mod feedback_processor;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_processor;
mod plugins;
mod process_exec_processor;
mod remote_control_processor;
mod search;
mod thread_processor;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use external_agent_config_processor::ExternalAgentConfigRequestProcessor;
pub(crate) use external_agent_config_processor::ExternalAgentConfigRequestProcessorArgs;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::latest_token_usage_turn_id_from_rollout_items;
use token_usage_replay::send_thread_token_usage_update_to_connection;

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_selections(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentSelection>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut selections = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        selections.push(TurnEnvironmentSelection {
            environment_id,
            cwd,
        });
    }
    thread_manager
        .validate_environment_selections(&selections)
        .map_err(environment_selection_error)?;
    Ok(Some(selections))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;
pub(crate) use self::thread_summary::thread_settings_from_core_snapshot;

pub(crate) fn build_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if is_persisted_rollout_item(item) {
            builder.handle_rollout_item(item);
        }
    }
    builder.finish()
}
