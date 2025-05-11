# Ollama

## Setup

- Create Linode VM
   - Region: FR, Paris
   - Plan: RTX4000 Ada x1 Small
- Set hostname
- `sudo apt update && sudo apt upgrade && sudo apt autoremove`
- [Install Tailscale](https://login.tailscale.com/admin/machines/new-linux)
   - Tags: `maker`
- [Install Ollama](https://github.com/ollama/ollama/blob/main/README.md#linux)
- [Set OLLAMA_HOST](https://github.com/ollama/ollama/blob/main/docs/faq.md#setting-environment-variables-on-linux) to Tailscle IP
- Enable Linode firewall
- Reboot
- `update-models.sh`
