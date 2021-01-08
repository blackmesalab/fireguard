use std::process::Command;

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
    pub fn exec(command: &str, args: &str, current_dir: Option<&str>) -> ShellResult {
        debug!("Executing command `{} {}`", command, args);
        match Command::new(command)
            .current_dir(current_dir.unwrap_or("."))
            .args(args.split(" ").collect::<Vec<&str>>())
            .output()
        {
            Ok(output) => {
                info!("Command {} {} success: {:?}", command, args, output.status.success());
                let stdout = String::from_utf8_lossy(&output.stdout);
                debug!("Command {} {} stdout: {}", command, args, stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    debug!("Command {} {} stderr: {}", command, args, stderr);
                }
                ShellResult::new(&stdout, &stderr, output.status.success())
            }
            Err(e) => {
                error!("Command {} {} failed: {}", command, args, e);
                ShellResult::new("", &e.to_string(), false)
            }
        }
    }
}
