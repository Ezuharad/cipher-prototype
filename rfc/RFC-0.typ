#let AUTHOR = "Steven Chiacchira"
#let TITLE = [Adapting Cellular Automata for Symmetric Encryption]

#set document(author: AUTHOR, title: [#TITLE])
#set page(numbering: "1")
#set heading(numbering: "1.1")

#align(center, text(17pt)[#TITLE])
#align(center, [#emph([#AUTHOR])])

A #emph([Cellular Automata]) (CA) is a discrete-time, deterministic process which uses a set of simple rules to simulate emergent complexities in dynamic systems. These constructs, despite their simplicity in construction, are well known to yield chaotic and diffuse results, making them intriguing as an entropy source for encryption algorithms. We seek to build a proof of concept algorithm which uses CAs for this purpose.

= Prerequisites
== Cellular Automata
We shall focus on cellular automata in the discrete 2D plane (a grid of bits), as these are well studied and allow for efficient spatial diffusion of information. CAs of this type take as input a certain grid state $G_i$ and output another grid state $G_(i+1)$, where each cell contains either a 0 or 1. We also call this process #emph([evolution]). The most famous example of such a cellular automata is John Conway's Game of Life, shown in @conway_life_example, which defines four simple rules for obtaining the next grid state:

1. Any cell containing a `1` with fewer than two adjacent `1`s becomes a `0`.
2. Any cell containing a `1` with two or three adjacent `1`s becomes a `1`.
3. Any cell containing a `1` with more than three adjacent `1`s becomes a `0`.
4. Any cell containing a `0` with exactly three adjacent neighbors becomes a `1`.

#figure([#image("asset/game-of-life-glider.png")], caption: [Three time steps from Conway's Game of Life. This particular configuration is known as a "glider".]) <conway_life_example>

Where a given cell has eight adjacent neighbors (four sharing a side, four sharing only a corner), as shown in @adjacent_cell_diagram.

#figure([#image("asset/adjacency-diagram.png", width: 20%)], caption: [A gray cell with its eight adjacent neighbors highlighted in blue.]) <adjacent_cell_diagram>

These rules illustrate the power of cellular automata in creating complex, emergent behavior through state evolution. // We now discuss certain properties of automata which will be useful in the construction of our algorithm.

// === Types of Automata  // TODO: classes
// === Attractors and Stable States
// === Complexity of Automata  // TODO: eden states, irreversible single state, etc.

= Proposed Scheme
We describe in this section our proposed encryption and decryption scheme, as well as the rationale behind many of the design decisions therein. We limit ourselves to a 32 bit key and a 256 block. We choose a 256 bit block in order to ensure a given message can be packed into a square grid ($sqrt(256) = 16$).

== Encryption and Decryption <encryption_decryption>

#figure([#text([TODO! Add encryption scheme diagram], fill: red)]) <scheme_diagram>

Our overall encryption-decryption scheme is shown in @scheme_diagram. In order to encrypt a message $P$, first split $P$ into $M$ 256-bit blocks $P_0, P_1, dots.h P_(M - 1)$. The final block may be padded with either randomly sampled noise#footnote([as is done in our reference implementation given in @reference_implementation]), or with words randomly sampled from a large text corpus.

Following splitting the message, perform the following steps on block $P_i$:
1. Use the key scheduler defined in @key_scheduler to obtain block transpose key $T_i$ and block shift key $S_i$.
2. XOR the plaintext block $P_i$ with $S_i$.
3. Scramble the result using scrambling algorithm $V$ defined in @scrambling_algorithm with key $T_i$ to obtain ciphertext block $E_i$.

$ E_i = V(P_i xor S_i, T_i) $

Decryption of ciphertext block $E_i$ can be accomplished using a reverse process:
1. Use the key scheduler defined in @key_scheduler to obtain block transpose key $T_i$ and block shift key $S_i$.
2. Unscramble the result using inverse scrambling algorithm $V^(-1)$ defined in @scrambling_algorithm with key $T_i$ to obtain ciphertext block $E_i$.
3. XOR the result with $S_i$ to obtain plaintext block $P_i$.

$
   P_i &= V^(-1)(V(P_i xor S_i, T_i), T_i) xor S_i = P_i xor S_i xor S_i = P_i \
=> P_i &= V^(-1)(E_i, T_i) xor S_i
$

The use of a scrambling algorithm in addition to the XOR is motivated by the possibility of a contiguous area of text being XORed with the same bit, allowing for reconstruction of parts of a message.

== Key Scheduling <key_scheduler>
We first use 32-bit key $K$ to initialize two 256-bit block keys: transpose key $T_0$ and shift key $S_0$ using the Key Automata rule described in @key_automata. In detail, $K$ is first used to seed the two following $16 times 16$ matrices, $I_t$ and $I_s$.

=== Block Initialization Matrices <block_initialization_matrices>

#figure(grid(
  columns: 2,
  gutter: 2mm,
  [$mat(delim: "[",
        ., A, \#, 3, ., 2, \#, Z, ., Y, \#, X, ., W, \#, V;
        7, ., B, ., 4, ., P, \#, O, ., N, ., M, \#, L, .;
        \#, 6, \#, C, \#, 5, \#, Q, \#, 3, ., 2, \#, Z, ., U;
        E, ., 5, \#, D, ., 6, ., R, \#, 4, \#, 7, ., K, \#;
        \#, D, ., 4, \#, E, ., 7, ., S, \#, 5, ., Y, ., T;
        F, ., C, \#, 3, ., F, ., A, \#, T, \#, 6, \#, J, \#;
        \#, Q, \#, B, ., 2, ., G, \#, B, ., U, \#, X, ., S;
        G, \, \#, P, ., A, ., Z, \#, H, ., C, \#, V, ., I, \#;
        ., R, \#, O, ., 7, \#, Y, ., I, \#, D, ., W, \#, R;
        H, ., E, \#, N, ., 6, \#, X, ., J, ., E, \#, H, .;
        \#, S, ., D, \#, M, ., 5, \#, W, ., K, \#, F, ., Q;
        I, \#, F, ., C, \#, L, ., 4, \#, V, \#, L, ., G, .;
        ., T, ., A, ., B, \#, K, ., 3, \#, U, ., M, ., P;
        J, \#, G, \#, H, \#, I, \#, J, \#, 2, \#, T, \#, N, \#;
        ., U, \#, V, ., W, ., X, ., Y, ., Z, \#, S, ., O;
        K, \#, L, ., M, \#, N, \#, O, \#, P, ., Q, \#, R, .;
  )$],
  [$mat(delim: "[",
    P, \#, O, \#, N, \#, M, \#, L, \#, K, \#, J, \#, I, \#;
    \#, L, \#, K, ., J, \#, I, ., H, ., G, \#, F, ., H;
    Q, ., D, \#, C, \#, B, \#, A, \#, 7, \#, 6, \#, E, \#;
    \#, M, ., X, \#, W, ., V, ., U, ., T, ., 5, \#, G;
    R, ., E, ., H, \#, G, ., F, \#, E, ., S, \#, D, .;
    \#, N, \#, Y, ., T, \#, S, ., R, ., D, \#, 4, ., F;
    S, ., F, ., I, \#, 3, \#, 2, ., Q, \#, R, \#, C, .;
    \#, O, ., Z, \#, U, ., 7, \#, Z, \#, C, ., 3, \#E;
    T, \#, G, \#, J, ., 4, ., 6, \#, P, ., Q, ., B, \#;
    \#, P, \#, 2, ., V, \#, 5, ., Y, \#, B, ., 2, ., D;
    U, ., H, \#, K, ., W, ., X, \#, O, \#, P, ., A, .;
    \#, Q, ., 3, \#, L, ., M, ., N, ., A, \#, Z, ., C;
    V, ., I, ., 4, \#, 5, ., 6, \#, 7, ., O, \#, 7, .;
    \#, R, ., J, ., K, \#, L, ., M, ., N, ., Y, \#, B;
    W, ., S, \#, T, ., U, \#, V, \#, W, ., X, ., 6, \#;
    \#, X, ., Y, ., Z, ., 2, \#, 3, ., 4, ., 5, ., A;
  )$]
)) <initialization_matrices>

Shown in @initialization_matrices are initialization matrices $I_t$ and $I_s$. The design of these matrices was intentional, and based on a few assumptions.

1. It is desirable that the initial state of each CA contains roughly the same number of `1` and `0` cells. In order to achieve this, 64 cells are initialized with a constant (independent of $K$) `1` value, and 64 cells are initialized with a constant `0` value. This applies a form of Laplacian smoothing to our initial automata states, so we are biased to obtain a state with the desired uniform property.
2. It is desirable that our key values interact with the constant values immediately, as otherwise the constant values will yield invariant sections of the early-stages of the evolved automata. Thus, the constant and seeded values are arranged in a checkerboard pattern.
3. It is desirable that no key can yield a symmetric initial state, as CAs with symmetric initial states will remain symmetric. The chosen constant values were taken from a pattern#footnote([#set text(fill: red); TODO! Decide pattern. Could be a spiral, cropped fractal]) which is not locally symmetric, guaranteeing no $K$ can yield a symmetric CA.
4. It is desirable that the key be distributed evenly through the initially seeded matrix, but not in a symmetric manner.
5. It is desirable that the two matrices not be symmetric to each other.

Following seeding, the Key Automata rule is applied 32 times to the seeded matrices $I_t$ and $I_S$ to obtain $T_0$ and $S_0$ respectively. In order to obtain the next pair of block keys $T_1$ and $S_1$, the Key Automata rule is applied an additional 32 times to $T_0$ and $S_0$. This can be repeated until enough blocks are obtained to encrypt the message.

=== Key Automata Rule <key_automata>
In choosing our CA rule for the key scheduling algorithm, a few desired traits were identified:
1. Few attractors and stable states exist. Such states would make easy guesses for decrypting patches of a message, weakening our cipher.
2. Low repetition frequency. If a CA state is repeated frequently, it is possible that some transpose key $T_i$ or shift key $S_i$ will be repeated for message blocks, allowing for a differential analysis.
3. A roughly equivalent number of `0` and `1` cells in each evolved state. This limits the capability of probabilistic attacks.
4. Few Garden of Eden states exist for the rule, as this shrinks the effective key space of each block key.
5. The rule is simple enough to process quickly, allowing for fast encryption and decryption (see @encryption_decryption).

From these maxims, we propose the following class III Cellular Automata rule:
#text([TODO! Decide class II rule], fill: red)

Where we additionally allow border cells to neighbor opposing border cells to promote faster diffusion of information. This property additionally makes all of the cells symmetric to each other; without this property border cells would behave differently. Although we do not prove any of our assumptions about this CA rule due to the undecidable nature of related problems, we empirically observe that many are approximately satisfied. Numbers obtained from these experiments are given in @empirical_results.

The decision to evolve a CA 32 times before each use was motivated by diffusion. It is clear by our rule that any given cell can only affect its eight neighboring cells. Due to this, for each cell to affect each other cell in generation, it is necessary that at least 16 iterations of our CA rule are applied. 32 rounds were chosen to allow this newly pseudorandom state to diffuse once again.  // TODO: would it be better to analyze in terms of $c$ for our rule?

=== Scrambling Algorithm <scrambling_algorithm>
We have now defined a method for generating pseudorandom noise of the same size as our message block. However, it is possible that contiguous regions of our message will be uniformly transformed by a simple XOR, meaning partial reconstruction may be possible. To mitigate this, we also introduce a scrambling algorithm $V$ to ensure the message bits are well dispersed before XORing.

$V$ takes two inputs, a 16x16 message block $P_i$ and a 16x16 block transposition key $T_i$. The process of $V$ can be described in three steps:
1. Concatenate the 1st, 5th, 9th, and 13th bits of row $j$ in $T_i$ to obtain a 4 bit unsigned integer $r_j$.
2. Iterate over the rows in $P_i$ in ascending order, swapping the $j$th row of the message with the $r_j$th row of the message.
3. Repeat this process over the columns of the resulting scramble, using the 3rd, 7th, 11th, and 15th bits of each column to yield $r_j$.

#figure([#text([TODO! Diagram ], fill: red)])

In order to reverse the scramble given ciphertext block $E_i$, calculate the column $r_j$ values and iterate over the columns of the encrypted block in descending order, swapping each column $j$ with column $r_j$. Then repeat with the rows to obtain $P_i$.

// = Strengths of Approach

== Modularity
Although it is not directly related to security, we note that our method is modular, as the proposed initial matrices, Key Automata Rule, and scrambling algorithms could all be replaced by other similar algorithms. Any other initial matrices or CA rules to be used should still satisfy the maxims given in @block_initialization_matrices and @key_automata respectively.

= Reference Implementation <reference_implementation>
A reference implementation of the defined encryption algorithm is given in @reference_code, and additionally available on GitHub to be compiled with the Rust compiler. We choose rust for our implementation to facilitate speed of execution, which is explained as a threat to vulnerability in @implementation_speed. Our reference implementation uses random noise to pad the final block of a given message. // TODO: reference rustc, give GitHub link

#figure([#text([TODO! Finish reference implementation], fill: red)]) <reference_code>

= Empirical Results of Key Automata Rule <empirical_results>
Because the Key Automata is critical in our algorithm's security, we seek to empirically show its fulfillment of the maxims given in @key_automata. We use the code given in @reference_code for our experiments and obtain our results with code available at our GitHub repository.  // TODO: GitHub repo

== Quantitative Results

// TODO: swap bits distribution
== Qualitative Results

= Threats to Validity
== Implementation Speed <implementation_speed>
Most off the shelf hardware is unable to simulate cellular automata without high-level software implementations. This means our algorithm is slower than current SOA methods of encryption. However, CA, can be implemented in hardware, meaning this is not an inherent barrier in the methodology.

== Potential Attacks
If an attacker were able to obtain two consecutive states of either the transpose key or shift key generating automata, it is possible the Key Automata rule could be reversed. We do not guard against this because consecutive states are never used in generating an encrypted message block, and we belive 32 applications of the Key Automata rule are sufficient to effectively eliminate similarities between the used key blocks (see @empirical_results).
