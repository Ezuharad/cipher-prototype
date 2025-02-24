// 2025 Steven Chiacchira
use clap::Parser;
use rand::random;
use std::collections::hash_map::HashMap;
use std::fs::read_to_string;
use talos::matrix::ToroidalBinaryMatrix;
use talos::{automata, encrypt, matrix, parse};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// CLI for testing Talos CA generation.
struct Args {
    /// Flag for testing only contiguous seeds. If false random seeds will be used.
    #[arg(short, long, action)]
    use_contiguous_seeds: bool,

    /// The number of seeds to test.
    #[arg(long, default_value_t = 1)]
    seeds: u32,

    /// The number of generations to run between sampling.
    #[arg(long, default_value_t = 11)]
    inter_generations: u32,

    /// How many matrices to sample per seed
    #[arg(long, default_value_t = 1)]
    seed_samples: u32,

    /// File to use for initializing the [Automaton](automata::Automaton) state.
    #[arg(long)]
    init_file: String,
}

const RULE: automata::AutomatonRule = automata::AutomatonRule {
    born: [false, false, true, true, true, true, true, false, false],
    dies: [true, true, false, false, false, true, true, true, true],
};

fn main() {
    let args = Args::parse();

    let seed_gen = (0..args.seeds).map(if args.use_contiguous_seeds {
        |i| i
    } else {
        |_| random::<u32>()
    });

    println!("# Using contiguous seeds: {}", args.use_contiguous_seeds);
    println!("# Number of samples: {}", args.seed_samples);
    println!(
        "# Number of generations between samples: {}",
        args.inter_generations
    );
    println!("# Initial File: {}", &args.init_file);
    println!("test\tseed\tgeneration\tcol_row\tgenerated_idx");

    let seed_matrix = read_to_string(&args.init_file).unwrap();
    for (test, seed) in seed_gen.enumerate() {
        let mut char_map: HashMap<char, bool> = parse::gen_char_map(seed);
        char_map.insert('#', true);
        char_map.insert('.', false);

        let table = parse::parse_bool_table(&seed_matrix, &char_map).unwrap();
        let state = matrix::ToroidalBoolMatrix::new(table).unwrap();

        let mut automaton = automata::Automaton::new(state, &RULE);

        for iteration in 0..(args.seed_samples) {
            automaton.iter_rule(args.inter_generations);
            for row_block in 0..4 {
                // iterate over each row in the 'row block' and swap
                let block_offset: isize = 4 * row_block;
                for (row_offset, col_offset) in [0, 2, 1, 3].iter().enumerate() {
                    let (r_offset, c_offset) = (row_offset as isize, *col_offset as isize);
                    let row_swap_idx = encrypt::read_4_bits(
                        automaton.get_state(),
                        (block_offset + r_offset, c_offset),
                        (block_offset + r_offset, 4 + c_offset),
                        (block_offset + r_offset, 8 + c_offset),
                        (block_offset + r_offset, 12 + c_offset),
                    ) as isize;
                    println!(
                        "{}\t{}\t{}\tR{}\t{}",
                        test,
                        seed,
                        iteration * args.inter_generations,
                        block_offset + r_offset,
                        row_swap_idx
                    )
                }
            }
            for col_block in 0..4 {
                // iterate over each col in the 'col block' and swap
                let block_offset: isize = 4 * col_block;
                for (col_offset, row_offset) in [3, 0, 2, 1].iter().enumerate() {
                    let (r_offset, c_offset) = (*row_offset as isize, col_offset as isize);
                    let col_swap_idx = encrypt::read_4_bits(
                        automaton.get_state(),
                        (r_offset, block_offset + c_offset),
                        (4 + r_offset, block_offset + c_offset),
                        (8 + r_offset, block_offset + c_offset),
                        (12 + r_offset, block_offset + c_offset),
                    ) as isize;
                    println!(
                        "{}\t{}\t{}\tC{}\t{}",
                        test,
                        seed,
                        iteration * args.inter_generations,
                        block_offset + c_offset,
                        col_swap_idx
                    )
                }
            }
        }
    }
}
