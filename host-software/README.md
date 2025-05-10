# Host software

## Deploy on a Pi

- Install Pi OS minimal
- Instlal Tailscale
   - Tag: `maker`
- `cargo build --release --target aarch64-unknown-linux-gnu`
- Copy
   - Executable to `/usr/lib/bin/`
   - Systemd unit to `/var/lib/systemd/system/llm-vn-host.service`
   - Character file to `/etc/llm-vn-characters.toml`
- `sudo mkdir /var/lib/llm-vn/`
- `sudo systemctl daemon-reload`
- Set `OLLAMA_HOST`: `systemd edit llm-vn-host.service`
- `sudo systemctl enable --now llm-vn-host.service`
