# Configuration

The main configuration file follows XDG config specification and lives under the prefix tmux-helm with a filename of config.toml (e.g. ~/.config/tmux-helm/config.toml)

The following can be configured:

| Field          | Required | Possible Values                     | Default         | Description                                                                                                                                          |
|----------------|----------|-------------------------------------|-----------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| session_files  | False    | List of file paths                  | None            | A list of paths to session configuration files that should be checked when creating a new session, see [here](./config_sessions.md) for more details |
| session_format | False    | A valid tmux display-message string | #{session_name} | The format string to display the sessions as in the session list                                                                                     |


An example config file might look something like this:

```
session_files = [ "~/.config/tmux-helm/sessions/config.json" ]
session_format = "#{session_name}"

[theme]
selected_session_foreground = 0xFFFF00
session_foreground = 0xFFFFFF
session_list_border = 0xAAAAAA
help_binding_foreground = 0xFFFFFF
prompt_foreground = 0xFFFFFF
prompt_input_foreground = 0xAAAAAA
```
