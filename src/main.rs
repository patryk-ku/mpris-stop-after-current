use mpris::{Event, PlayerFinder};
use std::env;
use std::error::Error;
use tokio::task;
use tokio::time::{timeout, Duration};

fn stop_after_current() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let player = if args.len() > 1 {
        let argument = &args[1];
        println!("Player name from args: {}", argument);
        PlayerFinder::new()?.find_by_name(argument)?
    } else {
        println!("You can specify which player to control by providing its name as an argument.");
        PlayerFinder::new()?.find_active()?
    };

    println!("Selected player: {}", player.identity());
    println!("\"Stop after current track\" - enabled for next 10 min.");
    println!("Waiting for track change... (Exit with Ctrl-C)");

    let events = player.events()?;
    for event in events {
        // println!("{:#?}", event);
        match event {
            Ok(Event::PlayerShutDown) => {
                println!("Player has been shut down.");
                break;
            }
            Ok(Event::TrackChanged(_)) => {
                match player.pause() {
                    Ok(_) => println!("Song change detected, stopping playback."),
                    Err(_) => println!("Error: Could not pause the player."),
                }
                break;
            }
            Ok(_) => continue,
            Err(err) => {
                println!("D-Bus error: {}. Aborting.", err);
                break;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    println!("{} v{}", name, version);
    println!("github.com/patryk-ku/mpris-stop-after-current");

    let result = timeout(
        Duration::from_secs(600),
        task::spawn_blocking(|| match stop_after_current() {
            Ok(_) => println!("Success."),
            Err(e) => println!("Error: {}", e),
        }),
    )
    .await;

    match result {
        Ok(_) => println!("Exiting program."),
        Err(_) => {
            println!("Timeout reached. Exiting program.");
            std::process::exit(0);
        }
    }
}
