# KSH
A *NIX-like shell for Windows & Linux.

**Disclaimer**
The code in this repository is pretty bad.
I barely know rust, so yeah, expect a segfault lmao.

### Building
To build KSH, you need to have the following installed:
* The entire Rust toolchain
* The GNU C Compiler (GCC)

Once you have those installed, you can run `cargo build --release` to build KSH.

### Usage
You can run KSH by running the executable in `target/release/`.

* You can run `ksh -c <command>` to run a comman directly.
* Otherwise, you can run `ksh` to start an interactive shell.

#### Building for a different platform
To build KSH for a different platform, you need to have the following installed:
* Rustup

Once you have that installed, you can run `rustup target add <target>` to add the target to your Rust toolchain.
Then, you can run `cargo build --release --target <target>` to build KSH for that target.

* For Windows, the target is `x86_64-pc-windows-gnu`
* For Linux, the target is `x86_64-unknown-linux-gnu`
* For macOS, the target is `x86_64-apple-darwin` (untested, I don't have a Mac)

#### Code Style
KSH uses the Rustfmt code style. Please run `cargo fmt` before making a pull request.

**Note:**
This repository is mirrored from a private GitLab repository.
Pull requests will (probably) not be acknowledged.