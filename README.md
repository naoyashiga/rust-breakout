# Rust Breakout Game

This is a simple implementation of the classic Breakout game using Rust and the ggez game framework. In this game, the player controls a paddle at the bottom of the screen to bounce a ball and break bricks at the top of the screen.

## Features

- Simple and clean Rust implementation
- Uses ggez for game loop and graphics
- Colorful bricks with random colors
- Basic collision detection
- Keyboard controls for the paddle

## Tested Environment

This game has been tested and verified to work on:

- Windows Subsystem for Linux (WSL)
- Ubuntu Linux

While it should work on other platforms supported by Rust and ggez, these are the environments where it has been specifically tested.

## Prerequisites

Before you can run this game, you need to have Rust and Cargo installed on your system. You can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Running the Game

To run the game, clone this repository and use Cargo:

```
git clone https://github.com/naoyashiga/rust-breakout.git
cd rust-breakout
cargo run
```

## Controls

- Left Arrow: Move paddle left
- Right Arrow: Move paddle right

## Troubleshooting

If you encounter any issues while setting up or running the game, here are some common problems and their solutions:

### ALSA lib errors

**Error message:**
```
ALSA lib confmisc.c:855:(parse_card) cannot find card '0'
ALSA lib conf.c:5178:(_snd_config_evaluate) function snd_func_card_inum returned error: No such file or directory
...
Error: AudioError("Could not initialize sound system using default output device (for some reason)")
```

**Solution:**
Install PulseAudio:
```
sudo apt-get install pulseaudio
```

### Wayland library not found

**Error message:**
```
thread 'main' panicked at /home/user/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wayland-sys-0.28.6/src/egl.rs:48:37:
Library libwayland-egl.so could not be loaded.
```

**Solution:**
Install the required Wayland development libraries:
```
sudo apt-get install libwayland-dev libwayland-egl1-mesa
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.