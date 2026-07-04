//! Implements the MultiAgentV2 collaboration tool surface.

use crate::agent::AgentStatus;
use crate::agent::agent_resolver::resolve_agent_target;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::multi_agents_common::*;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use deepseek_protocol::AgentPath;
use deepseek_protocol::models::ResponseInputItem;
use deepseek_protocol::deepseek_models::ReasoningEffort;
use deepseek_protocol::protocol::CollabWaitingBeginEvent;
use deepseek_protocol::protocol::CollabWaitingEndEvent;
use deepseek_protocol::protocol::InterAgentCommunication;
use deepseek_protocol::protocol::SubAgentActivityEvent;
use deepseek_protocol::protocol::SubAgentActivityKind;
use deepseek_tools::ToolName;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;

pub(crate) use followup_task::Handler as FollowupTaskHandler;
pub(crate) use interrupt_agent::Handler as InterruptAgentHandler;
pub(crate) use list_agents::Handler as ListAgentsHandler;
pub(crate) use send_message::Handler as SendMessageHandler;
pub(crate) use spawn::Handler as SpawnAgentHandler;
pub(crate) use wait::Handler as WaitAgentHandler;

mod followup_task;
mod interrupt_agent;
mod list_agents;
mod message_tool;
mod send_message;
mod spawn;
pub(crate) mod wait;

pub(super) fn communication_from_tool_message(
    author: AgentPath,
    recipient: AgentPath,
    message: String,
) -> InterAgentCommunication {
    InterAgentCommunication::new_encrypted(
        author,
        recipient,
        Vec::new(),
        message,
        /*trigger_turn*/ true,
    )
}
