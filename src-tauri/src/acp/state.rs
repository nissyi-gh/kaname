use serde::{Deserialize, Serialize};

/// ACP接続の状態を表す。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// 未接続
    Disconnected,
    /// 接続中（サブプロセス起動〜initialize完了まで）
    Connecting,
    /// 接続済み（initializeが完了し、リクエスト受付可能）
    Connected,
    /// エラー発生
    Error(String),
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

/// ACP接続の全体状態を管理する。
/// PR#6でTauri Managed Stateとして使う想定。
#[derive(Debug)]
pub struct AcpState {
    /// 接続状態
    status: ConnectionStatus,
    /// 現在のセッションID（セッション未作成ならNone）
    session_id: Option<String>,
}

impl AcpState {
    pub fn new() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
            session_id: None,
        }
    }

    pub fn status(&self) -> &ConnectionStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: ConnectionStatus) {
        self.status = status;
    }

    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    pub fn set_session_id(&mut self, session_id: Option<String>) {
        self.session_id = session_id;
    }
}

impl Default for AcpState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_status_default() {
        let status = ConnectionStatus::default();
        assert_eq!(status, ConnectionStatus::Disconnected);
    }

    #[test]
    fn test_acp_state_initial() {
        let state = AcpState::new();
        assert_eq!(*state.status(), ConnectionStatus::Disconnected);
        assert!(state.session_id().is_none());
    }

    #[test]
    fn test_acp_state_set_session() {
        let mut state = AcpState::new();
        state.set_status(ConnectionStatus::Connected);
        state.set_session_id(Some("test-session-123".to_string()));

        assert_eq!(*state.status(), ConnectionStatus::Connected);
        assert_eq!(state.session_id(), Some("test-session-123"));
    }

    #[test]
    fn test_acp_state_clear_session() {
        let mut state = AcpState::new();
        state.set_session_id(Some("session-1".to_string()));
        state.set_session_id(None);
        assert!(state.session_id().is_none());
    }

    #[test]
    fn test_connection_status_error() {
        let status = ConnectionStatus::Error("connection refused".to_string());
        assert_eq!(
            status,
            ConnectionStatus::Error("connection refused".to_string())
        );
    }

    #[test]
    fn test_connection_status_serialize() {
        let status = ConnectionStatus::Connected;
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: ConnectionStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ConnectionStatus::Connected);
    }
}
