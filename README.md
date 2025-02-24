# Talos: Cellular Automata for Encryption
This is the code and RFC for a novel encryption algorithm based on cellular automata. We name the project after the mythical bronze automaton Talos, who defended the island of Crete from invaders.

## Running the Project
### Compiling Talos
The rust implementation of our encryption algorithm can be built using cargo, installed via [rustup](https://rustup.rs/). Once cargo has been installed, the project can be built with `cargo build --release` from the project root.

### Encryption and Decryption
The current CLI tool for encryption, `crypt`, uses stdin and stdout for all plaintext and cipher data transer. Thus, to encrypt a file, one would use:
```zsh
./crypt --encrypt < path/to/plaintext.txt > encrypted.enc
```

`crypt` should automatically generate and print a key to stderr. Decryption is achievable with
```zsh
./crypt --decrypt --key <KEY> < encrypted.enc > output.txt 
```

### PyTorch Implementation
Additionally, we do provide a python implementation of the cellular automaton rule, although it is significantly slower than the rust implementation. The [file](script/gpu_implementation.py), as well as the other python files in the [script](script) directory can be run after installing the dependencies in [requirements.txt](script/requirements.txt). I used [uv](https://docs.astral.sh/uv/) to build my environment.

## Disclosure and Warning
**It should be emphasized that this is merely a research exercise; I am not a crypanalyst or mathematician by trade, and using this code for any serious endeavor would yield disasterous consequences.**
Work has been done in using CAs for encryption schemes; however, I have consciously avoided this work, as I ultimately seek to implement something unique.
