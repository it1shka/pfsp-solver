use pfsp_solver::solver::problem::Problem;
use std::{env, fs::File, io::BufReader};

mod tui;

fn main() -> color_eyre::Result<()> {
    #[cfg(debug_assertions)]
    {
        color_eyre::install()?;
    }
    #[cfg(not(debug_assertions))]
    {
        color_eyre::config::HookBuilder::default()
            .display_env_section(false)
            .display_location_section(false)
            .panic_section("That's an unexpected error, so you should probably report it by opening a PR at: https://github.com/it1shka/pfsp-solver")
            .install()?;
    }

    let path = env::args().nth(1).ok_or_else(|| {
        color_eyre::eyre::eyre!("Please provide a path to a file with a problem definition")
    })?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let problem = Problem::parse(reader)?;
    tui::run_tui(&problem)?;
    Ok(())
}
