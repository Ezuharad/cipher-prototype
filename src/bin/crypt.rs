// 2025 Steven Chiacchira
use clap::Parser;
use rand::random;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use talos::matrix::ToroidalBinaryMatrix;
use talos::parse::explode_u8_to_bool_vec;
use talos::{automata, encrypt, matrix, parse};

#[derive(Debug)]
enum ArgParseError {
    /// An action must be specified upon invocation of `crypt`, specifically:
    /// `--encrypt`
    /// `--decrypt`
    NoAction(),

    /// A key must be provided to decrypt a message.
    NoKeyForDecrypt(),

    /// A specified filename must exist
    NoSuchFile(),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Command line tool for encrypting and decrypting data with Talos.
/// 2025 Steven Chiacchira
struct Args {
    /// Name of the file to encrypt or decrypt
    input: String,

    /// Output file. Defaults to stdout if nothing is specified
    #[arg(short, long)]
    out: Option<String>,

    /// Encrypt data option. Mutually exclusive with --decrypt. Reads from stdin and prints encrypted data to stdout
    #[arg(short, long, action, conflicts_with = "decrypt")]
    encrypt: bool,

    /// Decrypt data option. Mutually exclusive with --encrypt. Reads from stdin and prints
    /// decrypted data to stdout
    #[arg(short, long, conflicts_with = "encrypt")]
    decrypt: bool,

    /// Key to be used, specified as a decimal unsigned integer with at most 32 bits. If left
    /// unspecified, a random key will be used.
    #[arg(short, long)]
    key: Option<u32>,
}

fn main() -> Result<(), ArgParseError> {
    let args = Args::parse();
    if args.key == None && args.decrypt {
        return Err(ArgParseError::NoKeyForDecrypt());
    }
    let seed = match args.key {
        Some(seed) => seed,
        None => random::<u32>(),
    };

    let mut char_map: HashMap<char, bool> = parse::gen_char_map(seed);

    char_map.insert('#', true);
    char_map.insert('.', false);

    let t_table = parse::parse_bool_table(T_INIT_MATRIX, &char_map).unwrap();
    let s_table = parse::parse_bool_table(S_INIT_MATRIX, &char_map).unwrap();

    let t_state = matrix::ToroidalBoolMatrix::new(t_table).unwrap();
    let s_state = matrix::ToroidalBoolMatrix::new(s_table).unwrap();

    let mut transpose_automata = automata::Automaton::new(t_state, &RULE);
    let mut shift_automata = automata::Automaton::new(s_state, &RULE);

    encrypt::temporal_seed_automata(
        &mut transpose_automata,
        seed,
        &parse::get_temporal_seed_map(T_INIT_MATRIX),
    );
    encrypt::temporal_seed_automata(
        &mut shift_automata,
        seed,
        &parse::get_temporal_seed_map(S_INIT_MATRIX),
    );

    let input_buffer = match fs::read(args.input) {
        Ok(buffer) => buffer,
        Err(_) => {
            return Err(ArgParseError::NoSuchFile());
        }
    };

    let output_bytes = if args.encrypt {
        eprintln!("Using key {}", seed);
        let bits = encrypt::encrypt_message_256(
            input_buffer,
            &mut shift_automata,
            &mut transpose_automata,
        );
        parse::concat_bool_to_u8_vec(bits)
    } else if args.decrypt {
        let bits = explode_u8_to_bool_vec(input_buffer);
        encrypt::decrypt_message_256(bits, &mut shift_automata, &mut transpose_automata)
    } else {
        return Err(ArgParseError::NoAction());
    };

    match args.out {
        Some(filename) => {
            let _ = fs::write(filename, output_bytes);
        }
        None => {
            let _ = io::stdout().write(&output_bytes);
        }
    }

    Ok(())
}

const RULE: automata::AutomatonRule = automata::AutomatonRule {
    born: [false, false, true, true, true, true, true, false, false],
    dies: [true, true, false, false, false, true, true, true, true],
};

const T_INIT_MATRIX: &str = "P#O#N#M#L#K#J#I#
#L#K.J#I.H.G#F.H
Q.D#C#B#A#7#6#E#
#M.X#W.V.U.T.5#G
R.E.H#G.F#E.S#D.
#N#Y.T#S.R.D#4.F
S.F.I#3#2.Q#R#C.
#O.Z#U.7#Z#C.3#E
T#G#J.4.6#P.Q.B#
#P#2.V#5.Y#B.2.D
U.H#K.W.X#O#P.A.
#Q.3#L.M.N.A#Z.C
V.I.4#5.6#7.O#7.
#R.J.K#L.M.N.Y#B
W.S#T.U#V#W.X.6#
#X.Y.Z.2#3.4.5.A";

const S_INIT_MATRIX: &str = ".A#3.2#Z.Y#X.W#V
7.B.4.P#O.N.M#L.
#6#C#5#Q#3.2#Z.U
E.5#D.6.R#4#7.K#
#D.4#E.7.S#5.Y.T
F.C#3.F.A#T#6#J#
#Q#B.2.G#B.U#X.S
G#P.A.Z#H.C#V.I#
.R#O.7#Y.I#D.W#R
H.E#N.6#X.J.E#H.
#S.D#M.5#W.K#F.Q
I#F.C#L.4#V#L.G.
.T.A.B#K.3#U.M.P
J#G#H#I#J#2#T#N#
.U#V.W.X.Y.Z#S.O
K#L.M#N#O#P.Q#R.";
