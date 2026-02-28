use agent_client_protocol as acp;

/// ACPクライアント実装。
/// エージェントからのパーミッション要求とセッション通知を処理する。
pub struct KanameClient;

impl KanameClient {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait(?Send)]
impl acp::Client for KanameClient {
    async fn request_permission(
        &self,
        args: acp::RequestPermissionRequest,
    ) -> acp::Result<acp::RequestPermissionResponse> {
        // MVP: 最初のオプション（通常 AllowOnce）を自動選択して承認
        let option_id = args
            .options
            .first()
            .map(|opt| opt.option_id.clone())
            .unwrap_or_else(|| acp::PermissionOptionId::new("allow_once"));

        log::info!(
            "Auto-approving permission request (option: {:?})",
            option_id
        );

        Ok(acp::RequestPermissionResponse::new(
            acp::RequestPermissionOutcome::Selected(acp::SelectedPermissionOutcome::new(option_id)),
        ))
    }

    async fn session_notification(&self, args: acp::SessionNotification) -> acp::Result<()> {
        match &args.update {
            acp::SessionUpdate::AgentMessageChunk(chunk) => {
                if let acp::ContentBlock::Text(text) = &chunk.content {
                    log::info!("Agent: {}", text.text);
                }
            }
            acp::SessionUpdate::ToolCall(tool_call) => {
                log::info!("Tool call: {:?}", tool_call);
            }
            other => {
                log::debug!("Session update: {:?}", other);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kaname_client_creation() {
        let _client = KanameClient::new();
    }
}
