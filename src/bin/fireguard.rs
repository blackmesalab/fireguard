extern crate color_eyre;
extern crate fireguard;
extern crate tokio;

use color_eyre::eyre::Result;

use fireguard::run;

/// Since reqwest uses a tokio task underneath, I believe we need at
/// least 2 available threads to ensure we can run both the upgrade task
/// and the loop waiting for signals asyncronously
#[tokio::main(worker_threads = 2)]
async fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(run().await?)
}
