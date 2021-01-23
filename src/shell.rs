use std::collections::HashMap;
use std::process::Stdio;

use color_eyre::eyre::bail;
use futures_util::StreamExt;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio_stream::wrappers::LinesStream;

pub struct Shell {}

#[derive(Debug, Clone)]
pub struct ShellResult {
    stdout: String,
    stderr: String,
    success: bool,
}

impl ShellResult {
    pub fn new(stdout: &str, stderr: &str, success: bool) -> Self {
        Self { stdout: stdout.to_string(), stderr: stderr.to_string(), success }
    }
    pub fn stdout(&self) -> &str {
        &self.stdout
    }
    pub fn stderr(&self) -> &str {
        &self.stderr
    }
    pub fn success(&self) -> bool {
        self.success
    }
}

impl Shell {
    async fn handle_stdout_stderr(&self, child: Child, sensitive: bool) -> ShellResult {
        // TODO: this method is needed because we cannot stream stdout / stderr when stdin
        // handling is involved. We have 0 clues why, we are not smart enough to understand
        // tokio in its fullness.
        //
        // Ideas: https://github.com/tokio-rs/tokio/issues/2199
        //
        // So we implemented this non streaming method to overcome the limitation and thus be
        // able to generate Wireguard private and public key.
        // This might be automatically solved when we move to tokio 1 or when someone smarter
        // comes around and fix this.
        match child.wait_with_output().await {
            Ok(output) => {
                let mut stdout = String::new();
                let mut stderr = String::new();
                if !output.stdout.is_empty() {
                    stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    if !sensitive {
                        info!("Command stdout: {}", stdout.trim());
                    }
                }
                if !output.stderr.is_empty() {
                    stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    if !sensitive {
                        info!("Command stderr: {}", stderr.trim());
                    }
                }
                ShellResult::new(&stdout, &stderr, output.status.success())
            }
            Err(e) => {
                error!("Error waiting for command output: {}", e);
                ShellResult::new("", &e.to_string(), false)
            }
        }
    }

    async fn stream_stdout_stderr(&self, mut child: Child, sensitive: bool) -> ShellResult {
        let stdout = child.stdout.take().expect("Unable to take() child process stdout");
        let stderr = child.stderr.take().expect("Unable to take() child process stderr");
        let stdout_result: Vec<_> = LinesStream::new(BufReader::new(stdout).lines())
            .inspect(|l| {
                if !sensitive {
                    match l {
                        Ok(ref e) => info!("Command stdout: {}", e.trim()),
                        Err(e) => error!("Stdout error: {}", e),
                    }
                }
            })
            .map(|l| l.unwrap_or("".to_string()))
            .collect()
            .await;
        let stderr_result: Vec<_> = LinesStream::new(BufReader::new(stderr).lines())
            .inspect(|l| {
                if !sensitive {
                    match l {
                        Ok(ref e) => warn!("Command stderr: {}", e.trim()),
                        Err(e) => error!("Stderr error: {}", e),
                    }
                }
            })
            .map(|l| l.unwrap_or("".to_string()))
            .collect()
            .await;
        match child.wait_with_output().await {
            Ok(output) => {
                ShellResult::new(&stdout_result.join("\n"), &stderr_result.join("\n"), output.status.success())
            }
            Err(e) => {
                error!("Error waiting for command output: {}", e);
                ShellResult::new("", &e.to_string(), false)
            }
        }
    }

    async fn handle_stdin(&self, stdin: &str, child: &mut Child, sensitive: bool) -> ShellResult {
        match child.stdin.as_mut() {
            Some(child) => match child.write(stdin.as_bytes()).await {
                Ok(n) => {
                    if !sensitive {
                        info!("Written {} bytes into command STDIN:\n{}", n, stdin);
                    }
                    ShellResult::new("Ok", "", true)
                }
                Err(e) => {
                    error!("Unable to write to command STDIN: {}", e);
                    ShellResult::new("", &e.to_string(), false)
                }
            },
            None => {
                error!("Stdin for process is not available");
                ShellResult::new("", "Stdin for process is not available", false)
            }
        }
    }

    fn log_exec_info(
        &self,
        command: &str,
        args: &str,
        current_dir: &str,
        stdin: bool,
        env: &HashMap<&str, &str>,
        sensitive: bool,
    ) {
        info!(
            "Executing command `{} {}`, cwd: {}, stdin: {}, env: {:?}, sentitive: {}",
            command, args, current_dir, stdin, env, sensitive
        );
    }

    pub fn runnable(name: &str) -> bool {
        match Command::new(name).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
            Ok(_) => true,
            Err(_) => {
                error!("Unable to find runnable command {}", name);
                false
            }
        }
    }

    pub async fn exec(command: &str, args: &str, current_dir: Option<&str>, sensitive: bool) -> ShellResult {
        let shell = Shell {};
        let cwd = current_dir.unwrap_or(".");
        shell.log_exec_info(command, args, cwd, false, &HashMap::new(), sensitive);
        match Command::new(command)
            .current_dir(cwd)
            .args(args.split(" ").collect::<Vec<&str>>())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => shell.stream_stdout_stderr(child, sensitive).await,
            Err(e) => {
                error!("Command {} {} failed: {}", command, args, e);
                ShellResult::new("", &e.to_string(), false)
            }
        }
    }

    pub async fn exec_with_env(
        command: &str,
        args: &str,
        current_dir: Option<&str>,
        env: HashMap<&str, &str>,
        sensitive: bool,
    ) -> ShellResult {
        let shell = Shell {};
        let cwd = current_dir.unwrap_or(".");
        shell.log_exec_info(command, args, cwd, false, &HashMap::new(), sensitive);
        match Command::new(command)
            .current_dir(cwd)
            .args(args.split(" ").collect::<Vec<&str>>())
            .envs(env)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => shell.stream_stdout_stderr(child, sensitive).await,
            Err(e) => {
                error!("Command {} {} failed: {}", command, args, e);
                ShellResult::new("", &e.to_string(), false)
            }
        }
    }

    pub async fn exec_with_input(
        command: &str,
        args: &str,
        current_dir: Option<&str>,
        stdin: &str,
        sensitive: bool,
    ) -> ShellResult {
        let shell = Shell {};
        let cwd = current_dir.unwrap_or(".");
        shell.log_exec_info(command, args, cwd, true, &HashMap::new(), sensitive);
        match Command::new(command)
            .current_dir(cwd)
            .args(args.split(" ").collect::<Vec<&str>>())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                let result = shell.handle_stdin(stdin, &mut child, sensitive).await;
                if !result.success() {
                    return result;
                }
                shell.handle_stdout_stderr(child, sensitive).await
            }
            Err(e) => {
                error!("Command {} {} failed: {}", command, args, e);
                ShellResult::new("", &e.to_string(), false)
            }
        }
    }

    pub async fn exec_with_input_and_env(
        command: &str,
        args: &str,
        current_dir: Option<&str>,
        stdin: &str,
        env: HashMap<&str, &str>,
        sensitive: bool,
    ) -> ShellResult {
        let shell = Shell {};
        let cwd = current_dir.unwrap_or(".");
        shell.log_exec_info(command, args, cwd, true, &env, sensitive);
        match Command::new(command)
            .current_dir(cwd)
            .args(args.split(" ").collect::<Vec<&str>>())
            .envs(env)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                let result = shell.handle_stdin(stdin, &mut child, sensitive).await;
                if !result.success() {
                    return result;
                }
                shell.handle_stdout_stderr(child, sensitive).await
            }
            Err(e) => {
                error!("Command {} {} failed: {}", command, args, e);
                ShellResult::new("", &e.to_string(), false)
            }
        }
    }
}
