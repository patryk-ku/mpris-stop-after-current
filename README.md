# mpris-stop-after-current

A small program written in Rust that adds a "stop after current" function to almost any Linux music player. It only requires the music player to support [MPRIS2](https://wiki.archlinux.org/title/MPRIS).

## How It Works

It detects the active music player, then waits for the currently playing song to change, at which point it stops playback. If the song doesn't change, the program automatically exits after 10 minutes. While running and waiting, it consumes no system resources and uses less than 4 MiB of RAM.

## Usage

Download the latest executable from the [Releases](https://github.com/patryk-ku/mpris-stop-after-current/releases) page. Then, run it in the terminal to make sure it works with your music player.

```sh
chmod +x mpris-stop-after-current
./mpris-stop-after-current
```

Use the music player's name as an argument to stop playback for the selected player only.

```sh
./mpris-stop-after-current "VLC media player"
```

Next depending on your distribution and desktop environment, you can add a keyboard shortcut or a button on the taskbar to run this command.

## Limitations

The program won't work if the next song in the queue is the same as the current one. Additionally, there may be occasional brief fragments of the next song heard, especially if the audio starts exactly from the beginning of the file.

## Compile from source

You will need to have Rust and Cargo installed.

```sh
git clone "https://github.com/patryk-ku/mpris-stop-after-current"
cd mpris-stop-after-current
cargo build --release
```

The compiled executable file will be located in: `target/release/mpris-stop-after-current`.
