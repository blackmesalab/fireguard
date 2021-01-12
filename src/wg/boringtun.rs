use std::collections::HashMap;

use color_eyre::eyre::{bail, Result};

use crate::shell::Shell;

const WG_QUICK_USERSPACE_IMPLEMENTATION: &str = "borintun";
const WG_QUICK_SUDO: &str = "1";

pub struct BoringTun {
    repository: String,
}

impl BoringTun {
    pub fn new(repository: &str) -> Result<Self> {
        if !Shell::runnable("boringtun") || !Shell::runnable("wg-quick") {
            bail!("Missing command dependency")
        }
        Ok(BoringTun { repository: repository.to_string() })
    }

    fn build_wg_quick_env(&self) -> HashMap<&str, &str> {
        let mut env = HashMap::new();
        env.insert("WG_QUICK_USERSPACE_IMPLEMENTATION", WG_QUICK_USERSPACE_IMPLEMENTATION);
        env.insert("WG_SUDO", WG_QUICK_SUDO);
        debug!("Injected WG_QUICK_USERSPACE_IMPLEMENTATION and WG_SUDO variables into command environment");
        env
    }

    pub async fn up(&self) -> Result<()> {
        info!("Starting new Boringtun instance for repository {}", self.repository);
        let result =
            Shell::exec_with_env("wg-quick", &format!("up {}", self.repository), None, self.build_wg_quick_env(), true)
                .await;
        if result.success() {
            info!("Boringtun instance started successfully:\n{}", result.stdout());
            Ok(())
        } else {
            bail!("Error running Boringtun instance: {}", result.stderr());
        }
    }

    pub async fn down(&self) -> Result<()> {
        info!("Stopping Boringtun instance for repository {}", self.repository);
        let result = Shell::exec_with_env(
            "wg-quick",
            &format!("down {}", self.repository),
            None,
            self.build_wg_quick_env(),
            true,
        )
        .await;
        if result.success() {
            info!("Boringtun instance stopped successfully:\n{}", result.stdout());
            Ok(())
        } else {
            bail!("Error stopping Boringtun instance: {}", result.stderr());
        }
    }
}
