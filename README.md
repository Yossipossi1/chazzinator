# Chazzinator

A discord bot written in Rust for the Chabad-Lubavitch server run by ExtremeUltraHuman.

This bot is designed to run in a Linux environment, and the instructions will be for a Linux environment.

To run the bot, first [install Rust](https://www.rust-lang.org/tools/install). Then, once installed, clone this repository.

You'll want to navigate to root directory and create the `.env` file based on the `.env.example` file, and fill in the environment variable called `TOKEN`. This will be the bot's API token. Once done, you can run `cargo run --release` in the directory to start the bot.

## Contributing

If you are developing for this bot, please be aware, first and foremost:

### THIS BOT IS VERY RUSHED

I am not expecting this to be anywhere near a good example of what to do, so if you're viewing this looking for examples of good code, I have bad news. If you're here to contribute, I welcome it and appreciate your help.

The `config.toml` file contains the configuration for the bot. This contains or will contain a list of necessary IDs or variables for the bot to function. If you plan on developing with a local instance of the bot, you will need to change these values for your corresponding test server.

Of course, if you make a Pull Request, please ensure your code is idiomatic by using `cargo format` and `cargo clippy` before submitting.

## Contact

If you need to get in contact with me, please join the Chabad-Lubavitch Discord and ping/message `Yossi#1463`.
