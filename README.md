# Geode Installer for Linux, written in Rust

>> You're getting rusty, machine!


Currently supports two ways of installing Geode:

1. Under Wine
Uses the default Wine prefix and tweaks the DLL overrides. If you need to use custom prefix, you can specify it with the environment variable `WINEPREFIX=` when running the installer from the terminal.

2. For Steam
Currently only supports installing Geode to the default game library location (e.g. `~/.steam/steam/steamapps/common/`).

