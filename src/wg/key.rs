use crate::shell::Shell;
use eyre::{bail, Result};

pub struct WgKeys {
    pub public: String,
    pub private: String,
}

impl WgKeys {
    pub fn new(public: &str, private: &str) -> Self {
        WgKeys { public: public.to_string(), private: private.to_string() }
    }
    pub fn generate() -> Result<Self> {
        let result = Shell::exec("wg", "genkey", None, None, true);
        if result.success() {
            let private = result.stdout().trim();
            let result = Shell::exec("wg", "pubkey", Some(result.stdout().trim()), None, true);
            let public = result.stdout().trim();
            Ok(WgKeys { private: private.to_string(), public: public.to_string() })
        } else {
            error!("Error generating new Wireguard keys: {}", result.stderr());
            bail!("Error generatiin Wireguard keys")
        }
    }
}
