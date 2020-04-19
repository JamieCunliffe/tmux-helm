# Installation

To install this you will first need to build the binary using cargo, from the root of this repository the following command can be run:
```
cargo build --release
```

After the binary has been built you should then launch it from your tmux config, I use the following
```
bind-key b new-window "/path/to/tmux-helm/run.sh"
```
