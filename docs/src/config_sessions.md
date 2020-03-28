# Session Manager

## Session

| Field   | Required | Possible Values                                  | Default | Description                                                              |
|---------|----------|--------------------------------------------------|---------|--------------------------------------------------------------------------|
| name    | True     | A tmux session name                              | N/A     | The name of the session, when a new session is to be created when this n |
| windows | True     | A list window objects (See Window section below) | N/A     | The windows to be created inside this session                            |

## Window

| Field | Required | Possible Values        | Default | Description                            |
|-------|----------|------------------------|---------|----------------------------------------|
| pane  | True     | A list of pane objects | True    | The panes to be created in this window |


## Pane
| Field     | Required | Possible Values     | Default  | Description           |
|-----------|----------|---------------------|----------|-----------------------|
| directory | False    | Any path            | ~/       | The working director  |
| split     | False    | vertical,horizontal | vertical | How to split the pane |


## Example
```
{
  "session": [
    {
      "name": "config",
      "windows": [
        {
          "panes": [
            {
              "directory": "~/"
            },
            {
              "split": "vertical",
              "directory": "~/.cfg/"
            },
            {
              "split": "horizontal",
              "directory": "~/.config/zsh/"
            }
          ]
        }
      ]
    }
  ]
}

```
