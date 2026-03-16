use pgmoneta_mcp::handler::PgmonetaHandler;
use pgmoneta_mcp::handler::verify::{VerifyBackupTool, VerifyRequest};
use rmcp::handler::server::router::tool::AsyncTool;
use serde_json::Value;
mod common;
#[tokio::test]
async fn verify_test() {
    common::init_config();

    let handler = PgmonetaHandler::new();
    let verify_request = VerifyRequest {
        username: "backup_user".to_string(),
        server: "primary".to_string(),
        backup_id: "newest".to_string(),
    };

    let response = VerifyBackupTool::invoke(&handler, verify_request)
        .await
        .expect("verify_backup should succeed");

    let json: Value = serde_json::from_str(&response).expect("response should be valid json");

    if let Some(header) = json.get("Header") {
        if let Some(command) = header.get("Command") {
            assert!(command.is_string(), "Command should be a string");
            assert!(command == "verify", "Command should be 'verify'");
        } else {
            panic!("Command field missing in Header");
        }
    } else {
        panic!("Header field missing");
    };
}
