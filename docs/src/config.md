# Configuration

The main configuration file follows XDG config specification and lives under the prefix tmux-session with a filename of config.toml (e.g. ~/.config/tmux-session/config.toml)

The following can be configured:

| Field          | Required | Possible Values                     | Default         | Description                                                                                                                                          |
|----------------|----------|-------------------------------------|-----------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| session_files  | False    | List of file paths                  | None            | A list of paths to session configuration files that should be checked when creating a new session, see [here](./config_sessions.md) for more details |
| session_format | False    | A valid tmux display-message string | #{session_name} | The format string to display the sessions as in the session list                                                                                     |

