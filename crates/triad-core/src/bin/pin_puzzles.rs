//! Regenerates `src/resources/pinned.json` from the puzzle epoch through the
//! given end date. Entries for already-published dates (through tomorrow, to
//! cover timezones ahead of UTC) are preserved verbatim; future entries are
//! regenerated so they pick up dictionary changes.
//!
//! Usage: cargo run -p triad-core --bin pin_puzzles -- 2027-12-31

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use triad_core::generator::{
    days_since_epoch, days_since_epoch_from_date, generate_daily_key_and_words, now_unix_secs,
    PinnedEntry,
};

fn main() {
    let end_date = std::env::args()
        .nth(1)
        .expect("usage: pin_puzzles <end-date YYYY-MM-DD>");
    let end = days_since_epoch_from_date(&end_date)
        .unwrap_or_else(|| panic!("invalid or pre-epoch end date: {end_date}"));

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resources/pinned.json");
    let mut pinned: BTreeMap<u32, PinnedEntry> = match fs::read_to_string(&path) {
        Ok(s) => serde_json::from_str(&s).expect("failed to parse existing pinned.json"),
        Err(_) => BTreeMap::new(),
    };

    // Published puzzles are immutable; +1 keeps "today" safe for UTC+X players
    let frozen_through = days_since_epoch(now_unix_secs()) + 1;

    let mut preserved = 0u32;
    let mut generated = 0u32;
    for n in 0..=end {
        if n <= frozen_through && pinned.contains_key(&n) {
            preserved += 1;
            continue;
        }
        let (key, words) = generate_daily_key_and_words(n);
        pinned.insert(n, PinnedEntry { key, words });
        generated += 1;
    }

    // One entry per line: readable diffs without pretty-print bloat
    let mut out = String::from("{\n");
    for (i, (n, entry)) in pinned.iter().enumerate() {
        let sep = if i + 1 < pinned.len() { "," } else { "" };
        let entry_json = serde_json::to_string(entry).expect("failed to serialize entry");
        out.push_str(&format!("  \"{n}\": {entry_json}{sep}\n"));
    }
    out.push_str("}\n");

    fs::write(&path, out).expect("failed to write pinned.json");
    println!(
        "pinned.json: {} entries through {} ({} preserved, {} generated)",
        pinned.len(),
        end_date,
        preserved,
        generated
    );
}
