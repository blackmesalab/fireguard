use std::collections::HashMap;
use std::process::Stdio;

use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};

pub struct Shell {}

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
        match child.wait_with_output().await {
            Ok(output) => {
                let mut stdout = String::new();
                let mut stderr = String::new();
                if !output.stdout.is_empty() {
                    stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    if !sensitive {
                        info!("Command succeeded: {}, STDOUT:\n{}", output.status.success(), stdout.trim());
                    }
                }
                if !output.stderr.is_empty() {
                    stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    if !sensitive {
                        info!("Command succeeded: {}, STDERR:\n{}", output.status.success(), stderr.trim());
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

    async fn handle_stdin(&self, stdin: &str, child: &mut Child, sensitive: bool) -> ShellResult {
        match child.stdin.as_mut().unwrap().write_all(stdin.as_bytes()).await {
            Ok(_) => {
                if !sensitive {
                    info!("Written {} bytes into command STDIN:\n{}", stdin.len(), stdin);
                }
                ShellResult::new("Ok", "", true)
            }
            Err(e) => {
                error!("Unable to write to command STDIN: {}", e);
                ShellResult::new("", &e.to_string(), false)
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
        match Command::new(name).spawn() {
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
            Ok(child) => shell.handle_stdout_stderr(child, sensitive).await,
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
            Ok(child) => shell.handle_stdout_stderr(child, sensitive).await,
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
