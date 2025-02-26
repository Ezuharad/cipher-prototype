# Talos: Cellular Automata for Encryption
This is the code and RFC for a novel encryption algorithm based on cellular automata. We name the project after the mythical bronze automaton Talos, who defended the island of Crete from invaders.

## Running the Project ⚒️
### Compiling Talos
The rust implementation of our encryption algorithm can be built using cargo, installed via [rustup](https://rustup.rs/). Once cargo has been installed, the project can be built with `cargo build --release` from the project root.

### Encryption and Decryption
The current CLI tool for encryption is called `crypt`. To encrypt a file, one would use:
```zsh
./crypt --encrypt --key <KEY> path/to/plain.txt -o encrypted.enc
```

`crypt` should automatically generate and print a key to stderr if one is not provided. Decryption is achievable with
```zsh
./crypt --decrypt --key <KEY> path/to/encrypted.enc -o plain.txt
```

### PyTorch Implementation
Additionally, we do provide a python implementation of the cellular automaton rule, although it is significantly slower than the rust implementation. The [file](script/gpu_implementation.py), as well as the other python files in the [script](script) directory can be run after installing the dependencies in [requirements.txt](script/requirements.txt). I used [uv](https://docs.astral.sh/uv/) to build my environment.

## Compiling RFCs 📝
The RFCs contained in this repository can be compiled using [typst](https://typst.app/).

## Challenge ⚔️
An [encrypted file](data/challenge.encrypted) has been provided via Git LFS. Users are free to attempt decryption via whatever means. Please let me know if you make any progress!

## Disclosure and Warning ⚠️
**It should be emphasized that this is merely a research exercise; I am not a crypanalyst or mathematician by trade, and using this tool for any serious endeavor would yield disasterous consequences.**
Multiple steps which should be taken in the development of any cryptographic protocol have been intentionally ignored:
- No prior work in adapting CAs for encryption was consulted. This was done because I want to implement something unique, and did not want to be influenced by the state of the field.
- No large scale statistical tests have been employed to verify the pseudorandomness of our automata rule.
- Multiple weaknesses identified in the protocol have not been addressed; see RFCs for details.
- Time has not been taken to assess the algorithm deeply; this was put together in a week and a half because I found it interesting, and thought it would be a good learning opportunity.

