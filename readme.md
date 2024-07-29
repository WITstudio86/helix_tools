# helix_tools

## open

- `cargo build -r`
- move to `~/.config/helix/tools/open`
- add helix config:

```toml
[keys.normal.space.t]
f = ":sh zellij run -f -x 10% -y 10% --width 80% --height 80% -- ~/.config/helix/tools/open --com yazi"
g = ":sh zellij run -f -x 10% -y 10% --width 80% --height 80% -- ~/.config/helix/tools/open --com git"
t = ":sh zellij action new-pane -f"
```
