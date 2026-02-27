# avl_tree

Rust implementation of a self-balancing AVL tree.

## Running the easy way

I've created a github release with compiled binary for macOS silicon. 

```bash
curl -fsSL -o avl_tree "https://github.com/JBlitzar/myrust/releases/download/avl_01/avl_tree"
chmod +x avl_tree
./avl_tree
```

## See the code

it's in [src/main.rs](src/main.rs).

## Running for development

You'll need to install rust if you haven't already. `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

Make sure that `cargo` and `rustc` are available in PATH. debugging help / guide for installation: https://rust-book.cs.brown.edu/ch01-01-installation.html

Then, just clone and run.

```bash
git clone https://github.com/JBlitzar/myrust.git
cd myrust
cd avl_tree
cargo run
```

## AI declaration

I used some degree of github copilot completions in making this. Casework for AVL trees is relatively repetitive, but rest assured that I understand the algorithm itself. There's a big difference from tab-completing a few if bodies and just telling AI "make me avl tree"

Repetitive code is usually indicative of a code smell, but I guess here we are. I'm also pretty new to Rust. 