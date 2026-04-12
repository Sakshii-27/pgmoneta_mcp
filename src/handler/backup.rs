// Copyright (C) 2026 The pgmoneta community
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::borrow::Cow;
use std::sync::Arc;

use super::PgmonetaHandler;
use crate::client::PgmonetaClient;
use rmcp::ErrorData as McpError;
use rmcp::handler::server::router::tool::{AsyncTool, ToolBase};
use rmcp::model::JsonObject;
use rmcp::schemars;

#[derive(Debug, Default, serde::Deserialize, schemars::JsonSchema)]
pub struct BackupRequest {
    pub username: String,
    pub server: String,
}

/// Tool for creating a full backup of a server.
pub struct BackupServerTool;

impl ToolBase for BackupServerTool {
    type Parameter = BackupRequest;
    type Output = String;
    type Error = McpError;

    fn name() -> Cow<'static, str> {
        "backup_server".into()
    }

    fn description() -> Option<Cow<'static, str>> {
        Some(
            "Create a full backup of a server. \
            Requires a server name. \
            The username has to be one of the pgmoneta admins to be able to access pgmoneta."
                .into(),
        )
    }

    fn output_schema() -> Option<Arc<JsonObject>> {
        None
    }
}

impl AsyncTool<PgmonetaHandler> for BackupServerTool {
    async fn invoke(
        _service: &PgmonetaHandler,
        request: BackupRequest,
    ) -> Result<String, McpError> {
        let result: String = PgmonetaClient::request_backup(&request.username, &request.server)
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to create backup: {:?}", e), None)
            })?;
        PgmonetaHandler::generate_call_tool_result_string(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::PgmonetaHandler;
    use rmcp::handler::server::router::tool::ToolBase;
    use serde_json::{Map, Value};

    #[test]
    fn test_backup_server_tool_metadata() {
        assert_eq!(BackupServerTool::name(), "backup_server");
        let desc = BackupServerTool::description();
        assert!(desc.is_some());
        assert!(desc.unwrap().contains("full backup"));
    }

    #[test]
    fn test_handler_has_backup_tool() {
        let handler = PgmonetaHandler::new();
        let tools = handler.tool_router.list_all();
        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(
            tool_names.contains(&"backup_server"),
            "backup_server tool should be registered, found: {:?}",
            tool_names
        );
    }

    #[test]
    fn test_generate_call_tool_result_string_backup() {
        let response = r#"{"Outcome": {"Status": true, "Command": 1}}"#;
        let result = PgmonetaHandler::generate_call_tool_result_string(response);
        assert!(result.is_ok());
        let output = result.unwrap();
        let parsed: Map<String, Value> = serde_json::from_str(&output).unwrap();
        let outcome = parsed["Outcome"].as_object().unwrap();
        assert_eq!(outcome["Command"], "backup");
    }

    #[test]
    fn test_backup_response_with_error() {
        let response = r#"{"Outcome": {"Status": false, "Command": 1, "Error": 111}}"#;
        let result = PgmonetaHandler::generate_call_tool_result_string(response);
        assert!(result.is_ok());
        let output = result.unwrap();
        let parsed: Map<String, Value> = serde_json::from_str(&output).unwrap();
        let outcome = parsed["Outcome"].as_object().unwrap();
        assert_eq!(outcome["Error"], "Backup: server not found");
    }
}
