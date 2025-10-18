 Project: nhl (Rust CLI) — quick guide for AI coding agents
 
 Purpose
 - This repository is a small Rust command-line tool that fetches NHL schedule and standings data and prints compact tables to the terminal. The entrypoint is `src/main.rs` and features live API calls (via `reqwest`) and optional local fixtures (`sample_schedule.json`, `sample_standings.json`).
 
 High-level architecture
 - `src/main.rs` — CLI parsing (clap) and dispatch. Flags: `--local`, `--playoffs`, `--schedule`, `--division`, `--conference`, `--full`, `--l10`, `--save`.
 - `src/schedule.rs` — types (serde camelCase), API endpoint `SCHEDULE_URL`, read-from-API vs read-from-file logic, display helpers.
 - `src/standings.rs` — types (serde camelCase), API endpoint `STANDINGS_URL`, sorting and display logic, playoff calculation code.
 
 Important patterns and conventions
 - Serde structs use rename_all = "camelCase" (see the structs in `src/*.rs`). Map JSON keys using the defined structs rather than hand-parsing strings.
 - The project uses blocking `reqwest::blocking` calls. Code is synchronous; when changing network code, keep blocking behaviour or update all callsites.
 - String formatting: many types implement `fmt::Display` (see `Standing`, `TeamCommonName`, `PlaceName`) for terminal layout. If you change printed columns, update the Display impls and the constant widths at top of `standings.rs`.
 - Sorting key for standings: most ranking steps use tuple keys like `( -(wins-losses), games_played, -regulation_wins )`. Preserve this to match current behaviour.
 
 Developer workflows (how to build, run, and test quickly)
 - Build: `cargo build` (project root contains `Cargo.toml`).
 - Run locally against fixtures: `cargo run -- --local --schedule` or `cargo run -- --local` for standings. Use `--save` to write live API output to `sample_*.json` files.
 - Run against live APIs: `cargo run -- --schedule` or `cargo run --` (standings). Note: the program uses public NHL endpoints; no auth currently required.
 
 Files to inspect for changes
 - `Cargo.toml` — dependencies (`chrono`, `clap`, `reqwest` with the `blocking` feature, `serde`).
 - `src/main.rs` — CLI flags and dispatch.
 - `src/schedule.rs`, `src/standings.rs` — data models, IO and all display logic; most changes will touch these files.
 - `sample_schedule.json`, `sample_standings.json` — example payloads used with `--local` and for tests.
 
 Quick examples (be concrete)
 - To add a new field from the standings JSON: add the field to the `Standing` struct in `src/standings.rs` with the proper camelCase name, run `cargo build`, and update any Display impls that use column widths constants near the top of that file.
 - To change timezones in the schedule output: edit `east_timezone` in `src/schedule.rs` (currently FixedOffset::west_opt(5 * 3600)). Convert parsing/formatting uses of `DateTime::parse_from_rfc3339` accordingly.
 
 Non-obvious constraints
 - Display widths are fixed by constants (PANEL_WIDTH, TEAM_NAME_WIDTH, etc.). Tests and terminal output depend on these; coordinated changes are required when changing output layout.
 - The code intentionally uses simple sorting and removal patterns (Vec::remove(0)) in the playoffs logic — ensure safe indexing if you change the selection logic.
 
 When in doubt, prefer minimal, local changes
 - Keep changes small and well-tested locally. Run `cargo run -- --local` and inspect `sample_*.json` output before pushing changes that affect runtime behaviour.
 
 Contact / next steps
 - If more context is needed (example fixtures, CI, or preferred output formats), ask to add unit tests and a CONTRIBUTING note with the desired behaviour.
