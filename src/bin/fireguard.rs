extern crate eyre;
extern crate fireguard;

use eyre::Result;

use fireguard::run;

fn main() -> Result<()> {
    Ok(run()?)
}
