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
pub struct VerifyRequest {
    pub username: String,
    pub server: String,
    pub backup_id: String,
}

/// Tool for verifying the integrity of a specific backup.
pub struct VerifyBackupTool;

impl ToolBase for VerifyBackupTool {
    type Parameter = VerifyRequest;
    type Output = String;
    type Error = McpError;

    fn name() -> Cow<'static, str> {
        "verify_backup".into()
    }

    fn description() -> Option<Cow<'static, str>> {
        Some(
            "Verify the integrity of a backup using given backup ID and server name. \
            \"newest\", \"latest\" or \"oldest\" are also accepted as backup identifier. \
            The username has to be one of the pgmoneta admins to be able to access pgmoneta"
                .into(),
        )
    }

    // input_schema is NOT overridden — the default generates the correct JSON schema
    // automatically from `type Parameter = VerifyRequest` via its JsonSchema derive.

    // output_schema must be overridden to return None because our Output type is String
    // (dynamically-translated JSON), and the MCP spec requires output schema root type
    // to be 'object', which String does not satisfy.
    fn output_schema() -> Option<Arc<JsonObject>> {
        None
    }
}

impl AsyncTool<PgmonetaHandler> for VerifyBackupTool {
    async fn invoke(
        _service: &PgmonetaHandler,
        request: VerifyRequest,
    ) -> Result<String, McpError> {
        let result: String =
            PgmonetaClient::request_verify(&request.username, &request.server, &request.backup_id)
                .await
                .map_err(|e| {
                    McpError::internal_error(format!("Failed to verify backup: {:?}", e), None)
                })?;
        PgmonetaHandler::generate_call_tool_result_string(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::handler::server::router::tool::ToolBase;

    #[test]
    fn test_verify_backup_tool_metadata() {
        assert_eq!(VerifyBackupTool::name(), "verify_backup");
        let desc = VerifyBackupTool::description();
        assert!(desc.is_some());
        assert!(desc.unwrap().contains("Verify"));
    }
}
