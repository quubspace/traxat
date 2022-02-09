use crate::{
    movement::{custom_move, predefined_move},
    sqlite::{add_position, migrations, query_all_positions, remove_position},
};
use {
    anyhow::{anyhow, Result},
    clap::{
        clap_app, crate_authors, crate_description, crate_name, crate_version, values_t, ArgMatches,
    },
};

pub fn run() -> Result<()> {
    let matches: ArgMatches = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@subcommand list =>
                (about: "Lists all predefined positions")
        )
        (@subcommand move =>
                (about: "Sets the arm's position based on a triplet, three space separated values.")
                (@arg position: +required +multiple "Sets the triplet value, three space separated values.")
        )
        (@subcommand position =>
                (about: "Sets the arm to a predefined position")
                (@arg position_name: +required "Sets which position you are using")
        )
        (@subcommand add =>
                (about: "Add a predefined position to the database")
                (@arg position_name: +required "The name of your custom position")
                (@arg vector: +required +multiple "The position vector for your custom vector, three space separated values.")
        )
        (@subcommand rm =>
                (about: "Removes a predefined position from the database")
                (@arg position_name: +required "The name of the position to remove")
        )
    )
    .get_matches();

    migrations()?;

    handle_commands(matches)
}

fn handle_commands(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("move", Some(sub_m)) => custom_move(
            values_t!(sub_m, "position", usize).map_err(|_| anyhow!("Failed to create vector."))?,
        ),
        ("position", Some(sub_m)) => predefined_move(
            sub_m
                .value_of("position_name")
                .expect("Position name will never be None, as it is required by clap."),
        ),
        ("list", Some(_sub_m)) => query_all_positions(),
        ("add", Some(sub_m)) => add_position(
            sub_m
                .value_of("position_name")
                .expect("Position name will never be None, as it is required by clap."),
            values_t!(sub_m, "vector", u8).map_err(|_| anyhow!("Failed to create vector."))?,
        ),
        ("rm", Some(sub_m)) => remove_position(
            sub_m
                .value_of("position_name")
                .expect("Position name will never be None, as it is required by clap."),
        ),
        _ => Err(anyhow!(
            "This is not a valid command. Please type the help command for more info."
        )),
    }?;

    Ok(())
}
