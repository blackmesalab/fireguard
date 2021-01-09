use std::io::Write;
use std::process::{Command, Stdio};

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
    pub fn exec(
        command: &str,
        args: &str,
        stdin: Option<&str>,
        current_dir: Option<&str>,
        sentitive: bool,
    ) -> ShellResult {
        let cwd = current_dir.unwrap_or(".");
        if !sentitive {
            info!(
                "Executing command `{} {}`, cwd: {}, stdin: {}, sentitive: {}",
                command,
                args,
                cwd,
                stdin.is_some(),
                sentitive
            );
        }
        match Command::new(command)
            .current_dir(cwd)
            .args(args.split(" ").collect::<Vec<&str>>())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                if let Some(stdin) = stdin {
                    match child.stdin.as_mut().unwrap().write_all(stdin.as_bytes()) {
                        Ok(_) => {
                            if !sentitive {
                                info!(
                                    "Written {} bytes into command {} {} STDIN:\n{}",
                                    stdin.len(),
                                    command,
                                    args,
                                    stdin
                                )
                            }
                        }
                        Err(e) => {
                            error!("Command {} {} failed: {}", command, args, e);
                            return ShellResult::new("", &e.to_string(), false);
                        }
                    }
                }

                match child.wait_with_output() {
                    Ok(output) => {
                        let mut stdout = String::new();
                        let mut stderr = String::new();
                        if !output.stdout.is_empty() {
                            stdout = String::from_utf8_lossy(&output.stdout).to_string();
                            if !sentitive {
                                info!("Command {} {} STDOUT:\n{}", command, args, stdout.trim());
                            }
                        }
                        if !output.stderr.is_empty() {
                            stderr = String::from_utf8_lossy(&output.stderr).to_string();
                            if !sentitive {
                                info!("Command {} {} STDERR:\n{}", command, args, stderr.trim());
                            }
                        }
                        ShellResult::new(&stdout, &stderr, output.status.success())
                    }
                    Err(e) => {
                        error!("Command {} {} failed: {}", command, args, e);
                        ShellResult::new("", &e.to_string(), false)
                    }
                }
            }
            Err(e) => {
                error!("Command {} {} failed: {}", command, args, e);
                ShellResult::new("", &e.to_string(), false)
            }
        }
    }
}
