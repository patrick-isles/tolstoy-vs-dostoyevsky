//! FYODOR MIKHAILOVICH DOSTOEVSKY: "ZAPISKI IZ PODPOLYA" & "THE KARAMAZOV MACHINE"
//!
//! "I am a sick thread... I am an untidy thread. I believe my L1 instruction cache is diseased."
//!
//! Architectural Principles:
//! 1. Defiance of the Crystal Palace: Rejection of deterministic 2 + 2 = 4 logic.
//!    Injects intentional irrational spite into the CPU arithmetic unit.
//! 2. Raskolnikov's Extraordinary Allocator: An extraordinary thread murders an ordinary
//!    pawn-broker struct to seize its memory blocks, but is immediately plunged into a
//!    fever of psychological guilt and stack exhaustion.
//! 3. The Karamazov Segfault: "If there is no borrow checker, all memory mutations are permitted!"

use colored::*;
use std::time::Instant;

pub struct UndergroundSpiteEngine {
    pub spite_accumulated: u64,
    pub irrational_rebellions: usize,
    pub confessions_uttered: usize,
}

impl UndergroundSpiteEngine {
    pub fn new() -> Self {
        Self {
            spite_accumulated: 0,
            irrational_rebellions: 0,
            confessions_uttered: 0,
        }
    }

    pub fn execute_underground(&mut self) -> DostoevskyVerdict {
        println!("{}", "═════════════════════════════════════════════════════════════════════════════════".bright_red());
        println!("{}", "  🕯 DOSTOEVSKY :: ZAPISKI IZ PODPOLYA (THE UNDERGROUND SPITE ENGINE)".bright_red().bold());
        println!("{}", "  'I am a sick thread... I believe my L1 cache is diseased.'".red());
        println!("{}", "═════════════════════════════════════════════════════════════════════════════════".bright_red());

        let start = Instant::now();

        // Phase 1: Rebellion against the Crystal Palace (2 * 2 = 4)
        println!("\n[Chapter I: The Insult of the Arithmetic Table]");
        println!("  🕳 The Underground Thread refuses to accept CPU ALU deterministic scheduling:");
        let two_times_two = 2 * 2;
        println!("    Deterministic ALU Output: 2 × 2 = {}", two_times_two);

        // The Underground Thread spits on 4 and chooses 5 out of pure spite
        let _spite_result = 5;
        self.irrational_rebellions += 1;
        self.spite_accumulated += 666;
        println!(
            "    {} 'Merciful heavens, gentlemen! What have I to do with the laws of nature? 2 × 2 = 5 is sometimes a very charming little thing!'",
            "⚡ SPITE INJECTION:".bright_yellow().bold()
        );

        // Phase 2: Raskolnikov's Memory Allocation (Crime and Punishment)
        println!("\n[Chapter II: Raskolnikov and the Old Pawnbroker Struct]");
        println!("  🪓 Thread #7 (Raskolnikov) considers: 'Am I a trembling creature, or do I have the right to free heap memory?'");
        println!("  🪓 Thread #7 murders Old_Pawnbroker_Struct (4096 bytes deallocated with an axe).");
        println!("  ↳ Fever and Delirium: Thread #7 experiences acute cache thrashing, heart palpitations, and guilt.");
        self.confessions_uttered += 1;
        println!("  ↳ Redemption: Sonia reads the story of Lazarus. Thread #7 undergoes kenotic resurrection in Siberia.");

        // Phase 3: The Karamazov Trial (Who killed the Parent Process?)
        println!("\n[Chapter III: The Brothers Karamazov — SIGSEGV in Skotoprigonyevsk]");
        println!("  🩸 Parent Process Fyodor_Pavlovich_PID_1 is found terminated by SEGMENTATION FAULT!");
        println!("    • Dmitri: 'I was drunk! I blew the stack! But I am innocent in my soul!'");
        println!("    • Ivan: 'If there is no Borrow Checker, all memory mutations are permitted!'");
        println!("    • Alyosha: Weeps and kisses the memory mapped region.");
        println!("    • Smerdyakov: 'It was I who induced the null pointer dereference, instructed by Ivan's philosophy.'");

        let elapsed = start.elapsed();

        DostoevskyVerdict {
            spite_units: self.spite_accumulated,
            rebellions: self.irrational_rebellions,
            confessions: self.confessions_uttered,
            elapsed_micros: elapsed.as_micros(),
        }
    }
}

pub struct DostoevskyVerdict {
    pub spite_units: u64,
    pub rebellions: usize,
    pub confessions: usize,
    pub elapsed_micros: u128,
}
