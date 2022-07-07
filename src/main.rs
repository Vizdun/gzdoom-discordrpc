use discord_rpc_client::Client;
use fancy_regex::Regex;
use std::{thread, time};
use window_titles::{Connection, ConnectionTrait};

// Returns the window title; for gzdoom this is "level - game", for lzdoom this is just "game"
fn info(connection: &Connection) -> Result<String, Box<dyn std::error::Error>> {
    // List of windows as vector with strings
    let windows: Vec<String> = connection.window_titles().unwrap();
    let re = Regex::new(
        "((?!(.*Firefox.*)|(.*Pale Moon.*))(DOOM)|(Project Brutality)|(Snap the Sentinel))",
    )
    .unwrap();
    let window: String = windows
        .into_iter()
        .filter(|s| re.is_match(s).unwrap())
        .collect();
    Ok(window)
}

fn main() {
    // Create the client
    let mut drpc = Client::new(729549945408979065);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Create connection so that window list may be obtained
    let connection = Connection::new().unwrap();

    loop {
        let window = info(&connection).unwrap();

        // "level - game" => [level, game]
        let game_vec: Vec<&str> = window.split(" - ").collect();

        let is_in_level = game_vec.len() > 1;

        let level = if is_in_level { game_vec[0] } else { "In Menu" };
        let game = if is_in_level {
            game_vec[1]
        } else {
            game_vec[0]
        };

        // Get the icon
        let icon = match game {
            "DOOM Registered" => "doom",
            "DOOM 2: Hell on Earth" => "doom_ii",
            "DOOM 2: Unity Edition" => "doom_ii",
            "The Ultimate DOOM" => "ultimate_doom",
            "Brutal Doom: Project Brutality" => "pb_icon",
            "Project Brutality 3.0" => "pb_icon",
            "Snap the Sentinel" => "sts",
            _ => "gz",
        };

        // Set the activity
        if let Err(why) = drpc.set_activity(|a| {
            a.details(game)
                .state(level)
                .assets(|ass| ass.large_image(icon))
        }) {
            println!("Failed to set presence: {}", why);
        }

        thread::sleep(time::Duration::from_secs(5));
    }
}
