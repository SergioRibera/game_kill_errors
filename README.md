# Kill Errors - 404
![Screenshot_20230527_161932](https://github.com/SergioRibera/game_kill_errors/assets/56278796/3b6a9811-d1a1-4783-a052-18a233515971)

<p align="center">
	<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/game_kill_errors/ci.yml?label=ci&style=flat-square">
	<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/game_kill_errors/build.yml?label=Build%20Native&style=flat-square">
	<a href="https://sergioribera.github.io/game_kill_errors"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/game_kill_errors/release-gh-pages.yml?label=Build%20Web&style=flat-square"></a>
    <a href="https://github.com/SergioRibera/game_kill_errors/releases"><img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/SergioRibera/game_kill_errors?label=download&style=flat-square"></a>
</p>

> ℹ️ This proyect is for [myself page](https://sergioribera.github.io/game_kill_errors)

# Platforms
- Native (MacOs, Linux & Windows)
- Web (Wasm)
- Library (Usable in other rust proyects)
- Mobile (⚠️ Soon)
  - Android
  - iOS

# Requirements
- Rust
- Cargo
- [Cargo Make](https://github.com/sagiegurari/cargo-make)
- [Trunk](https://trunkrs.dev) (Optional for web development)

# Development Guide
- Edit the `.env` file if you need
- Run `cargo make dev` for run as development mode (Native window)
- Run `cargo make --list-all-steps` for check all aviable tasks

#### Other CargoMake Tasks

* **build** - Generate release binary/lib
* **check** - Check all issues, format and code quality
* **clean** - Clean all target directory
* **clippy** - Check code quality
* **default** - Check all issues, format and code quality
* **dev** - Run native launcher with development configuration
* **fix-all** - Try fix all clippy and format issues
* **fix-clippy** - Fix code quality
* **fix-fmt** - Fix format
* **fmt** - Check format quality
* **test** - Check all unit test

# Usage as Library
> ⚠️ Check the `launchers` folders
