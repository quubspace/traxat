use anyhow::Result;

use cli::run;

mod cli;
mod movement;
mod sqlite;

fn main() -> Result<()> {
    run()
}
