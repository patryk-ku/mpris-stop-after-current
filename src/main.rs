use mpris::{Event, PlayerFinder};
use tokio::task;
use tokio::time::{timeout, Duration};

fn stop_after_current() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus.")
        .find_active()
        .expect("Could not find active player.");

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
    let result = timeout(
        Duration::from_secs(600),
        task::spawn_blocking(|| {
            stop_after_current();
        }),
    )
    .await;

    match result {
        Ok(_) => println!("Program exiting."),
        Err(_) => {
            println!("Timeout reached, program exiting.");
            std::process::exit(0);
        }
    }
}
