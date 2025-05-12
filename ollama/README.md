# Ollama

## Setup

- Create Linode VM
   - Region: FR, Paris
   - Plan: RTX4000 Ada x1 Medium
- [Install Ollama](https://github.com/ollama/ollama/blob/main/README.md#linux)
- [Install Tailscale](https://login.tailscale.com/admin/machines/new-linux)
   - Tags: `maker`
- [Set OLLAMA_HOST](https://github.com/ollama/ollama/blob/main/docs/faq.md#setting-environment-variables-on-linux) to Tailscle IP
- Enable Linode firewall
- `sudo apt update && sudo apt upgrade && sudo apt autoremove`
- Set hostname
- Reboot
- `update-models.sh`
