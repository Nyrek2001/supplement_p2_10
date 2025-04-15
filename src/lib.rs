use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use file_tracker::{scan_directory, compare_states, save_state, load_state};
//! # File Tracker CLI
//!
//! A command-line tool that watches a directory for file changes.
//!
//! ## Features
//! - Recursively scans directory for file modification times.
//! - Prints changes in real time.
//! - Persists state on exit and resumes on relaunch.
//!
//! ## Usage
//! ```bash
//! cargo run -- path/to/directory
//! ```
fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    let state_file = "state.json";

    let old_state = load_state(state_file);
    let state = Arc::new(Mutex::new(old_state));
    let state_clone = Arc::clone(&state);
    let state_file_clone = state_file.to_string();

    ctrlc::set_handler(move || {
        let locked = state_clone.lock().unwrap();
        save_state(&state_file_clone, &locked);
        println!("\nState saved. Exiting.");
        std::process::exit(0);
    }).expect("Failed to set Ctrl+C handler");

    loop {
        let current = scan_directory(&dir);
        let mut locked = state.lock().unwrap();
        let changes = compare_states(&locked, &current);

        for (path, time) in &changes {
            println!("Modified: {:?} at {:?}", path, time);
        }

        *locked = current;
        drop(locked);

        thread::sleep(Duration::from_secs(3));
    }
}
