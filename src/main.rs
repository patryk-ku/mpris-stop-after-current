use mpris::{Event, PlayerFinder};
use std::env;
use tokio::task;
use tokio::time::{timeout, Duration};

fn stop_after_current() {
    let args: Vec<String> = env::args().collect();

    let player;

    if args.len() > 1 {
        let argument = &args[1];
        println!("Player name from args: {}", argument);
        player = PlayerFinder::new()
            .expect("Could not connect to D-Bus.")
            .find_by_name(argument)
            .expect("Could not find active player.");
    } else {
        player = PlayerFinder::new()
            .expect("Could not connect to D-Bus.")
            .find_active()
            .expect("Could not find active player.");
        println!("You can specify which player to control by providing its name as an argument.")
    }

    println!("Selected player: {}", player.identity());
    println!("\"Stop after current track\" - enabled for next 10 min.");
    println!("Waiting for track change... (Exit with Ctrl-C)");

    let events = player.events().expect("Could not start event stream.");

    for event in events {
        match event {
            Ok(event) => {
                // println!("{:#?}", event);
                if matches!(event, Event::PlayerShutDown) {
                    println!("Player has ben shut down.");
                    break;
                }

                if matches!(event, Event::TrackChanged(_)) {
                    match player.pause() {
                        Ok(_) => println!("Song change detected, stopping playback."),
                        Err(_) => println!("Error: Could not pause the player."),
                    }
                    break;
                }
            }
            Err(err) => {
                println!("D-Bus error: {}. Aborting.", err);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    println!("{} v{}", name, version);
    println!("github.com/patryk-ku/mpris-stop-after-current");

    let result = timeout(
        Duration::from_secs(600),
        task::spawn_blocking(|| {
            stop_after_current();
        }),
    )
    .await;

    match result {
        Ok(_) => println!("Done."),
        Err(_) => {
            println!("Timeout reached.");
            std::process::exit(0);
        }
    }
}
