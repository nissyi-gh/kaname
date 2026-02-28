use agent_client_protocol::{self as acp, Agent as _};
use anyhow::{Context, Result};
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, oneshot};
use tokio::task::LocalSet;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

use super::client::KanameClient;

/// ACP接続の設定
pub struct AcpConnectionConfig {
    /// エージェントプログラムのパス
    pub agent_program: String,
    /// エージェントに渡す追加引数
    pub agent_args: Vec<String>,
}

impl Default for AcpConnectionConfig {
    fn default() -> Self {
        Self {
            agent_program: "claude-agent-acp".to_string(),
            agent_args: Vec::new(),
        }
    }
}

/// 外部からACP接続に送るリクエスト。PR#4以降で拡張予定。
#[derive(Debug)]
pub enum AcpRequest {
    Shutdown,
}

/// AcpRequestに対するレスポンス。
#[derive(Debug)]
pub enum AcpResponse {
    Ok,
    Error(String),
}

type RequestPair = (AcpRequest, oneshot::Sender<AcpResponse>);

/// Send + Sync なハンドル。Tauri Commands等のSendコンテキストから使える。
pub struct AcpConnectionHandle {
    request_tx: mpsc::Sender<RequestPair>,
}

/// サブプロセスを起動し、ACP接続を確立してinitialize()を完了する。
/// LocalSet内から呼ぶ必要がある。
async fn spawn_and_initialize(
    config: &AcpConnectionConfig,
) -> Result<(acp::ClientSideConnection, Child)> {
    let mut child = Command::new(&config.agent_program)
        .args(&config.agent_args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .with_context(|| format!("Failed to spawn agent process: {}", config.agent_program))?;

    let child_stdin = child
        .stdin
        .take()
        .context("Failed to capture agent stdin")?
        .compat_write();
    let child_stdout = child
        .stdout
        .take()
        .context("Failed to capture agent stdout")?
        .compat();

    let client = KanameClient::new();
    let (conn, io_future) =
        acp::ClientSideConnection::new(client, child_stdin, child_stdout, |fut| {
            tokio::task::spawn_local(fut);
        });

    tokio::task::spawn_local(async {
        if let Err(e) = io_future.await {
            log::error!("ACP I/O error: {}", e);
        }
    });

    conn.initialize(
        acp::InitializeRequest::new(acp::ProtocolVersion::V1).client_info(
            acp::Implementation::new("kaname", env!("CARGO_PKG_VERSION"))
                .title("Kaname ACP Client"),
        ),
    )
    .await
    .context("ACP initialize failed")?;

    log::info!("ACP connection initialized successfully");
    Ok((conn, child))
}

/// ACP接続を専用スレッドで起動し、Sendなハンドルを返す。
pub fn start_acp_connection(config: AcpConnectionConfig) -> Result<AcpConnectionHandle> {
    let (request_tx, mut request_rx) = mpsc::channel::<RequestPair>(32);

    std::thread::Builder::new()
        .name("acp-runtime".to_string())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build ACP tokio runtime");

            rt.block_on(async {
                let local_set = LocalSet::new();
                local_set
                    .run_until(async move {
                        match spawn_and_initialize(&config).await {
                            Ok((_conn, _child)) => {
                                log::info!("ACP connection ready, waiting for requests...");
                                while let Some((request, reply_tx)) = request_rx.recv().await {
                                    match request {
                                        AcpRequest::Shutdown => {
                                            log::info!("ACP shutdown requested");
                                            let _ = reply_tx.send(AcpResponse::Ok);
                                            break;
                                        }
                                    }
                                }
                                log::info!("ACP runtime shutting down");
                            }
                            Err(e) => {
                                log::error!("Failed to initialize ACP connection: {}", e);
                                while let Some((_, reply_tx)) = request_rx.recv().await {
                                    let _ = reply_tx.send(AcpResponse::Error(format!(
                                        "ACP not connected: {}",
                                        e
                                    )));
                                }
                            }
                        }
                    })
                    .await;
            });
        })
        .context("Failed to spawn ACP runtime thread")?;

    Ok(AcpConnectionHandle { request_tx })
}

impl AcpConnectionHandle {
    /// ACP接続を終了する。
    pub async fn shutdown(&self) -> Result<()> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.request_tx
            .send((AcpRequest::Shutdown, reply_tx))
            .await
            .context("ACP runtime is not running")?;

        match reply_rx.await.context("ACP runtime dropped")? {
            AcpResponse::Ok => Ok(()),
            AcpResponse::Error(e) => anyhow::bail!(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AcpConnectionConfig::default();
        assert_eq!(config.agent_program, "claude-agent-acp");
        assert!(config.agent_args.is_empty());
    }

    #[test]
    fn test_custom_config() {
        let config = AcpConnectionConfig {
            agent_program: "/usr/local/bin/my-agent".to_string(),
            agent_args: vec!["--verbose".to_string()],
        };
        assert_eq!(config.agent_program, "/usr/local/bin/my-agent");
        assert_eq!(config.agent_args.len(), 1);
    }
}
