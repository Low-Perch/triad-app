#!/usr/bin/env python3
"""Filter dict.json for the Triad word puzzle game.

Removes uncommon words, plurals, inflections, and offensive content.
Usage: python3 scripts/filter_dict.py
"""

import json
import os
import sys
from collections import defaultdict
from pathlib import Path

from nltk.stem import WordNetLemmatizer
from wordfreq import zipf_frequency

# Paths
SCRIPT_DIR = Path(__file__).parent
PROJECT_ROOT = SCRIPT_DIR.parent
DICT_PATH = PROJECT_ROOT / "crates" / "triad-core" / "src" / "resources" / "dict.json"
BLOCKLIST_PATH = SCRIPT_DIR / "blocklist.txt"
REPORT_PATH = SCRIPT_DIR / "filter_report.json"

# Thresholds
UNCOMMON_ZIPF = 2.0          # Words below this are considered too obscure
INFLECTION_KEEP_ZIPF = 4.0   # Inflections above this are kept (e.g., "building")
MIN_WORDS_PER_KEY = 3

# Keys that are themselves offensive (player solves for these)
OFFENSIVE_KEYS = {"fuck", "slut", "cunt", "twat"}


def load_blocklist():
    words = set()
    if BLOCKLIST_PATH.exists():
        for line in BLOCKLIST_PATH.read_text().splitlines():
            word = line.strip().lower()
            if word and not word.startswith("#"):
                words.add(word)
    return words


def build_word_set(dictionary):
    """Build a set of all unique clue words in the dictionary."""
    words = set()
    for entry in dictionary.values():
        words.update(w.lower() for w in entry.get("prefix", []))
        words.update(w.lower() for w in entry.get("suffix", []))
    return words


def is_plural(word, lemmatizer, all_words):
    """Check if word is a plural form whose base also exists."""
    lemma = lemmatizer.lemmatize(word, pos="n")
    return lemma != word and lemma in all_words


def is_inflection(word, lemmatizer, all_words):
    """Check if word is an inflected form (verb, adjective, adverb)."""
    for pos in ("v", "a", "r"):  # verb, adjective, adverb
        lemma = lemmatizer.lemmatize(word, pos=pos)
        if lemma != word and lemma in all_words:
            return True
    return False


def filter_dictionary():
    print("Loading dictionary...")
    with open(DICT_PATH) as f:
        dictionary = json.load(f)

    blocklist = load_blocklist()
    lemmatizer = WordNetLemmatizer()
    all_words = build_word_set(dictionary)

    # Stats tracking
    stats = {
        "before": {"keys": len(dictionary), "words": 0},
        "removed": {
            "offensive_keys": [],
            "offensive_words": 0,
            "uncommon_words": 0,
            "plural_words": 0,
            "inflection_words": 0,
        },
        "keys_dropped": 0,
        "after": {"keys": 0, "words": 0},
        "sample_removals": [],
    }

    # Count initial words
    for entry in dictionary.values():
        stats["before"]["words"] += len(entry.get("prefix", [])) + len(entry.get("suffix", []))

    print(f"Before: {stats['before']['keys']} keys, {stats['before']['words']} words")
    print(f"Blocklist: {len(blocklist)} words")
    print()

    # --- Phase 1: Remove offensive keys ---
    print("Phase 1: Removing offensive keys...")
    keys_to_remove = []
    for key in dictionary:
        if key.lower() in OFFENSIVE_KEYS:
            keys_to_remove.append(key)
            stats["removed"]["offensive_keys"].append(key)

    for key in keys_to_remove:
        del dictionary[key]
    print(f"  Removed {len(keys_to_remove)} keys: {keys_to_remove}")

    # --- Phase 2-5: Filter clue words per key ---
    print("Phase 2-5: Filtering clue words...")
    filtered_dict = {}
    total_removed = 0
    sample_count = 0

    for key, entry in dictionary.items():
        prefix_words = entry.get("prefix", [])
        suffix_words = entry.get("suffix", [])
        all_clue_words = prefix_words + suffix_words

        kept_prefix = []
        kept_suffix = []
        removed_for_key = []  # (word, reason, zipf)

        for word_list, kept_list, list_type in [
            (prefix_words, kept_prefix, "prefix"),
            (suffix_words, kept_suffix, "suffix"),
        ]:
            for word in word_list:
                w = word.lower()
                freq = zipf_frequency(w, "en")
                reason = None

                # Filter 2: Offensive words
                if w in blocklist:
                    reason = "offensive"
                    stats["removed"]["offensive_words"] += 1

                # Filter 3: Uncommon words
                elif freq < UNCOMMON_ZIPF:
                    reason = "uncommon"
                    stats["removed"]["uncommon_words"] += 1

                # Filter 4: Plurals
                elif is_plural(w, lemmatizer, all_words):
                    reason = "plural"
                    stats["removed"]["plural_words"] += 1

                # Filter 5: Inflections (with frequency gate)
                elif is_inflection(w, lemmatizer, all_words):
                    if freq < INFLECTION_KEEP_ZIPF:
                        reason = "inflection"
                        stats["removed"]["inflection_words"] += 1

                if reason:
                    removed_for_key.append((word, reason, freq, list_type))
                    total_removed += 1
                    if sample_count < 30:
                        stats["sample_removals"].append({
                            "word": word, "key": key,
                            "reason": reason, "zipf": round(freq, 2)
                        })
                        sample_count += 1
                else:
                    kept_list.append(word)

        # --- Phase 6: Key retention check ---
        total_kept = len(kept_prefix) + len(kept_suffix)

        if total_kept < MIN_WORDS_PER_KEY:
            stats["keys_dropped"] += 1
            continue

        filtered_dict[key] = {}
        if kept_prefix:
            filtered_dict[key]["prefix"] = sorted(kept_prefix)
        if kept_suffix:
            filtered_dict[key]["suffix"] = sorted(kept_suffix)

    # Final stats
    after_words = 0
    for entry in filtered_dict.values():
        after_words += len(entry.get("prefix", [])) + len(entry.get("suffix", []))

    stats["after"]["keys"] = len(filtered_dict)
    stats["after"]["words"] = after_words

    # Print summary
    print()
    print("=" * 50)
    print("FILTER REPORT")
    print("=" * 50)
    print(f"BEFORE:  {stats['before']['keys']} keys, {stats['before']['words']} words")
    print(f"AFTER:   {stats['after']['keys']} keys, {stats['after']['words']} words")
    print()
    print("REMOVALS:")
    print(f"  Offensive keys:  {len(stats['removed']['offensive_keys'])} ({', '.join(stats['removed']['offensive_keys'])})")
    print(f"  Offensive words: {stats['removed']['offensive_words']}")
    print(f"  Uncommon words:  {stats['removed']['uncommon_words']}")
    print(f"  Plurals:         {stats['removed']['plural_words']}")
    print(f"  Inflections:     {stats['removed']['inflection_words']}")
    print()
    print(f"Keys dropped (< {MIN_WORDS_PER_KEY} words): {stats['keys_dropped']}")
    print()
    pct_keys = (1 - stats["after"]["keys"] / stats["before"]["keys"]) * 100
    pct_words = (1 - stats["after"]["words"] / stats["before"]["words"]) * 100
    print(f"Reduction: {pct_keys:.1f}% keys, {pct_words:.1f}% words")

    # Write report
    with open(REPORT_PATH, "w") as f:
        json.dump(stats, f, indent=2)
    print(f"\nDetailed report: {REPORT_PATH}")

    # Write filtered dictionary
    with open(DICT_PATH, "w") as f:
        json.dump(filtered_dict, f, separators=(",", ":"), sort_keys=True)
        f.write("\n")
    print(f"Filtered dictionary written to: {DICT_PATH}")


if __name__ == "__main__":
    filter_dictionary()
