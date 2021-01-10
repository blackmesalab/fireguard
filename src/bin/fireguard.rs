extern crate color_eyre;
extern crate fireguard;
extern crate tokio;

use color_eyre::eyre::Result;

use fireguard::run;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(run().await?)
}
