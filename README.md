## Git Helper

This is a tool written in Rust to assist with the [Oh-my-zsh git plugin](https://github.com/ohmyzsh/ohmyzsh). The plugin provides useful aliases, and this is a simple lookup for them.

## Usage

Once installed, run (by default `gh`) and enter your search term. To exit, press enter or escape.

## Installation

First, install Rust (instructions from [here](https://www.rust-lang.org/tools/install)):

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Next, checkout this repo:

`git clone https://github.com/tedbennett/git-help.git`

Next, cd into the directory compile the binary. This might take a minute!

`cd git-help && cargo build --release`

Now, copy the binary to your PATH. You might need to use sudo for this.

`cp target/release/git-help /bin/usr/local/bin/gh`

This renames the binary to `gh` change the last part of the path in the second `cp` argument to change this.
