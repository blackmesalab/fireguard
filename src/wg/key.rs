use color_eyre::eyre::{bail, Result};

use crate::shell::Shell;

pub struct WgKeys {
    pub public: String,
    pub private: String,
}

impl WgKeys {
    pub fn new(public: &str, private: &str) -> Self {
        WgKeys { public: public.to_string(), private: private.to_string() }
    }
    pub async fn generate() -> Result<Self> {
        let result = Shell::exec("wg", "genkey", None, true).await;
        if result.success() {
            let private = result.stdout();
            let result = Shell::exec_with_input("wg", "pubkey", None, &private, true).await;
            let public = result.stdout();
            Ok(WgKeys { private: private.trim().to_string(), public: public.trim().to_string() })
        } else {
            error!("Error generating new Wireguard keys: {}", result.stderr());
            bail!("Error generatiin Wireguard keys")
        }
    }
}
