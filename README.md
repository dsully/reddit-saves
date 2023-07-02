# Reddit Saves

Simple tool to extract credentials from 1Password and export Reddit saved posts to a file.

Requirements:

- Rust
- 1Password CLI
- 2FA must be enabled on your Reddit account.

You must have a 1Password private vault entry named "Reddit", and two additional fields:

- "client_id"
- "client_secret"

These can be created at [Authorized Applications](https://www.reddit.com/prefs/apps) by selecting the "Scripting" type.

No OAuth URL is needed / enter a dummy one.

## Install

```shell
cargo install --git https://github.com/dsully/reddit-saves
```

## Usage

```shell
reddit-saves [--subreddit <name>] [--output <filename>]
```
