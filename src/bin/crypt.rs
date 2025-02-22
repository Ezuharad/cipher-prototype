// 2025 Steven Chiacchira
use clap::Parser;
use rand::random;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use talos::matrix::ToroidalBinaryMatrix;
use talos::parse::explode_u8_to_bool_vec;
use talos::{automata, encrypt, matrix, parse};

#[derive(Debug)]
enum ArgParseError {
    /// An action must be specified upon invocation of `crypt`, specifically:
    /// `--encrypt`
    /// `--decrypt`
    NoAction(),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Command line tool for encrypting and decrypting data with Talos. Data is read and written
/// through stdin and stdout.
struct Args {
    /// Encrypt data option. Mutually exclusive with --decrypt. Reads from stdin and prints encrypted data to stdout.
    #[arg(short, long, action, conflicts_with = "decrypt")]
    encrypt: bool,

    /// Decrypt data option. Mutually exclusive with --encrypt. Reads from stdin and prints
    /// decrypted data to stdout.
    #[arg(short, long, conflicts_with = "encrypt")]
    decrypt: bool,

    /// Key to be used, specified as a decimal unsigned integer with at most 32 bits. If left
    /// unspecified, a random key will be used.
    #[arg(short, long)]
    key: Option<u32>,
}

fn main() -> Result<(), ArgParseError> {
    let args = Args::parse();
    let seed = match args.key {
        Some(seed) => seed,
        None => random::<u32>(),
    };
    eprintln!("Using key {}", seed);

    let mut char_map: HashMap<char, bool> = parse::gen_char_map(seed);

    char_map.insert('#', true);
    char_map.insert('.', false);

    let t_table = parse::parse_bool_table(T_INIT_MATRIX, &char_map).unwrap();
    let s_table = parse::parse_bool_table(S_INIT_MATRIX, &char_map).unwrap();

    let t_state = matrix::ToroidalBoolMatrix::new(t_table).unwrap();
    let s_state = matrix::ToroidalBoolMatrix::new(s_table).unwrap();

    let mut shift_automata = automata::Automaton::new(s_state, &RULE);
    let mut transpose_automata = automata::Automaton::new(t_state, &RULE);
    if args.encrypt {
        let plaintext = io::read_to_string(io::stdin()).unwrap();
        let bits =
            encrypt::encrypt_message_256(&plaintext, &mut shift_automata, &mut transpose_automata);
        let encrypted = parse::concat_bool_to_u8_vec(bits);
        let mut output = io::stdout();
        let _ = output.write(&encrypted);
    } else if args.decrypt {
        let mut buffer: Vec<u8> = Vec::new();
        let _ = io::stdin().read_to_end(&mut buffer);
        let cipherstream = explode_u8_to_bool_vec(buffer);
        let decrypted = encrypt::decrypt_message_256(
            cipherstream,
            &mut shift_automata,
            &mut transpose_automata,
        );

        match decrypted {
            Ok(message) => println!("{}", message),
            Err(_) => eprintln!("Invalid key or malformed ciphertext received"),
        }
    } else {
        return Err(ArgParseError::NoAction());
    }

    //println!("Plaintext: {}", plaintext);
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
