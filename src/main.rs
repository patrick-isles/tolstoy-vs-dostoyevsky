//! THE MOSCOW 1880 COMPETITIVE PROGRAMMING DUEL
//!
//! Leo Tolstoy vs. Fyodor Dostoevsky in the Grand Hall of the Assembly of Nobility.
//! Two colossal Russian literary egos battle for memory bandwidth, CPU scheduling supremacy,
//! and the salvation of the Slavic soul.

pub mod dostoyevsky;
pub mod tolstoy;

use colored::*;
use dostoyevsky::UndergroundSpiteEngine;
use tolstoy::HistoricalDeterminismEngine;

fn main() {
    print_duel_header();

    // ─────────────────────────────────────────────────────────────────────────
    // ROUND 1: COUNT LEV NIKOLAYEVICH TOLSTOY TAKES THE KEYBOARD
    // ─────────────────────────────────────────────────────────────────────────
    println!("\n{}", ">>> COUNT TOLSTOY APPROACHES THE TERMINAL <<<".bright_green().bold());
    println!("He arrives wearing a hand-stitched linen peasant tunic and bast shoes,");
    println!("scowling with aristocratic disgust at Dostoevsky's nervous coughing.\n");

    let mut tolstoy_engine = HistoricalDeterminismEngine::new(100_000);
    let tolstoy_result = tolstoy_engine.execute_borodino();

    // ─────────────────────────────────────────────────────────────────────────
    // ROUND 2: FYODOR MIKHAILOVICH DOSTOEVSKY TAKES THE KEYBOARD
    // ─────────────────────────────────────────────────────────────────────────
    println!("\n{}", ">>> FYODOR DOSTOEVSKY SEIZES THE TERMINAL <<<".bright_red().bold());
    println!("He stumbles forward in a rumpled black frock coat, eyes feverish,");
    println!("dropping roulette slips from Baden-Baden and muttering about suffering.\n");

    let mut dostoevsky_engine = UndergroundSpiteEngine::new();
    let dostoevsky_result = dostoevsky_engine.execute_underground();

    // ─────────────────────────────────────────────────────────────────────────
    // THE JUDGES' DELIBERATION & FINAL VERDICT
    // ─────────────────────────────────────────────────────────────────────────
    print_final_verdict(&tolstoy_result, &dostoevsky_result);
}

fn print_duel_header() {
    println!("{}", "═════════════════════════════════════════════════════════════════════════════════".bright_white());
    println!(
        "{}",
        "  ⚔ THE MOSCOW 1880 SYSTEMS HACKATHON :: THE DUEL OF THE GIANTS"
            .bright_white()
            .bold()
    );
    println!(
        "{}",
        "  Leo Tolstoy (Historical Determinism) vs. Fyodor Dostoevsky (Underground Spite)"
            .cyan()
    );
    println!("{}", "═════════════════════════════════════════════════════════════════════════════════".bright_white());
}

fn print_final_verdict(
    tolstoy: &tolstoy::TolstoyVerdict,
    dostoevsky: &dostoyevsky::DostoevskyVerdict,
) {
    println!("\n{}", "┌─ THE IMPERIAL HACKATHON JURY VERDICT ──────────────────────────────────────────".bright_yellow().bold());
    println!("│");
    println!(
        "│ {} :: VOYNA I MIR",
        "LEO TOLSTOY".bright_green().bold()
    );
    println!("│   • Architecture:        Contiguous Agrarian Array (Zero Heap Fragmentation)");
    println!("│   • Elements Processed:  {:>8} peasant infantry marching in lockstep", tolstoy.regiment_size);
    println!("│   • Napoleon Delusions:  {:>8} redundant orders ignored by reality", tolstoy.napoleon_delusions);
    println!("│   • Execution Time:      {:>8} µs (Monumental & Serene)", tolstoy.elapsed_micros);
    println!("│");
    println!(
        "│ {} :: ZAPISKI IZ PODPOLYA",
        "FYODOR DOSTOEVSKY".bright_red().bold()
    );
    println!("│   • Architecture:        Psychological Fever Loop & Karamazov Segfault");
    println!("│   • Spite Accumulated:   {:>8} spite-joules", dostoevsky.spite_units);
    println!("│   • Arithmetic Refusals: {:>8} times shouted that 2 × 2 = 5", dostoevsky.rebellions);
    println!("│   • Confessions Uttered: {:>8} soul-purging deallocations", dostoevsky.confessions);
    println!("│   • Execution Time:      {:>8} µs (Agonizing & Ecstatic)", dostoevsky.elapsed_micros);
    println!("│");
    println!("│{:-<80}", "");
    println!("│ {}", "FINAL RULING:".bright_white().bold());
    println!("│ 1. Count Tolstoy wins the Engineering Throughput Medal:");
    println!("│    His 100,000 peasant elements marched through L1 cache with absolute historical");
    println!("│    inevitability while Kutuzov slept, achieving maximum memory bandwidth.");
    println!("│");
    println!("│ 2. Fyodor Dostoevsky is disqualified by the Rust Compiler for refusing to accept");
    println!("│    that 2 + 2 = 4, but wins the Audience Choice Award because half the judges");
    println!("│    broke down in tears and confessed their embezzlements to the Linux kernel.");
    println!("{}", "└────────────────────────────────────────────────────────────────────────────────".bright_yellow().bold());
}
