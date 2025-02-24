// 2025 Steven Chiacchira
use clap::Parser;
use rand::random;
use std::collections::{hash_map::HashMap, HashSet};
use std::fs::read_to_string;
use talos::matrix::ToroidalBinaryMatrix;
use talos::{automata, matrix, parse};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// CLI for testing Talos CA generation.
struct Args {
    /// Flag for testing only contiguous seeds.
    #[arg(short, long, action)]
    use_contiguous_seeds: bool,

    /// The number of seeds to test.
    #[arg(short, long, default_value_t = 1)]
    seeds: u32,

    /// The number of generations to run the [Automaton](automata::Automaton) for.
    #[arg(short, long, default_value_t = 32_000)]
    generations: u32,

    /// File to use for initializing the [Automaton](automata::Automaton) state.
    #[arg(short, long)]
    init_file: String,
}

fn main() {
    let args = Args::parse();

    let seed_gen = (0..args.seeds).map(if args.use_contiguous_seeds {
        |i| i
    } else {
        |_| random::<u32>()
    });

    println!("# Using contiguous seeds: {}", args.use_contiguous_seeds);
    println!("# Number of seeds: {}", args.seeds);
    println!("# Number of generations: {}", args.generations);
    println!("# Initial File: {}", &args.init_file);
    println!("test\ttseed\tgeneration\tn_alive");

    for (test, seed) in seed_gen.enumerate() {
        let mut char_map: HashMap<char, bool> = parse::gen_char_map(seed);
        char_map.insert('#', true);
        char_map.insert('.', false);

        let table =
            parse::parse_bool_table(&read_to_string(&args.init_file).unwrap(), &char_map).unwrap();
        let state = matrix::ToroidalBoolMatrix::new(table).unwrap();
        let rule = automata::AutomatonRule {
            born: [false, false, true, true, true, true, true, false, false],
            dies: [true, true, false, false, false, true, true, true, true],
        };

        let mut automaton = automata::Automaton::new(state, &rule);

        for generation in 0..args.generations {
            automaton.iter_rule(1);
            let n_alive = automaton.get_state().popcount();
            println!(
                "{}\t{}\t{}\t{}",
                test, seed, generation, n_alive,
            );
        }
    }
}
