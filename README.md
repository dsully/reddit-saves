# Reddit Saves

Simple tool to extract credentials from 1Password and export Reddit saved posts to a file.

Requirements:

* Python
* PDM
* 1Password CLI
* 2FA must be enabled on your Reddit account.

You must have a 1Password private vault entry named "Reddit", and two additional fields:

* "API ID"
* "API Secret"

These can be created at [Authorized Applications](https://www.reddit.com/prefs/apps) by selecting the "Scripting" type.

No OAuth URL is needed / enter a dummy one.

## Running

    pip install pdm (or pipx)
    pdm install
    ./reddit-saves
