# Overview
This is a simple project that allows tmux sessions to be switched, along with created and deleted from the same screen. Fuzzy searching can be used for filtering the list of sessions, also it will allow a session to be created from the text that has been entered.

If the session name that's being created matches a user defined template then a session will be created based on that template see the configuration for more information.

# Documentation
The documentation can be found [here](https://jamiecunliffe.github.io/tmux-helm/index.html)

# Installation

To install this you will first need to build the binary using cargo, from the root of this repository the following command can be run:
```
cargo build --release
```

After the binary has been built you should then launch it from your tmux config, I use the following
```
bind-key b new-window "/path/to/tmux-helm/run.sh"
```
