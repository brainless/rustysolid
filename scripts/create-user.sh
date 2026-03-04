#!/usr/bin/env bash
set -euo pipefail

# Create the SSH_USER from project.conf on a fresh server using initial bootstrap credentials.
# Run this once before setup-server.sh.
#
# Usage: ./create-user.sh <INITIAL_USER> <INITIAL_PASSWORD> [path/to/project.conf]
# Example: ./create-user.sh root mypassword
# Example: ./create-user.sh ubuntu temporarypassword /path/to/project.conf

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <INITIAL_USER> <INITIAL_PASSWORD> [path/to/project.conf]"
  echo "Example: $0 root mypassword"
  exit 1
fi

INITIAL_USER="$1"
INITIAL_PASSWORD="$2"
CONFIG_FILE="${3:-${PROJECT_ROOT}/project.conf}"

if [ ! -f "$CONFIG_FILE" ]; then
  echo "Config not found: $CONFIG_FILE"
  echo "Create it from project.conf.template"
  exit 1
fi

# shellcheck disable=SC1090
source "$CONFIG_FILE"

required_vars=(SERVER_IP SSH_USER)
for v in "${required_vars[@]}"; do
  if [ -z "${!v:-}" ]; then
    echo "Missing required config key: $v"
    exit 1
  fi
done

SSH_KEY=$(cat ~/.ssh/id_ed25519.pub 2>/dev/null || cat ~/.ssh/id_rsa.pub 2>/dev/null || cat ~/.ssh/id_ecdsa.pub 2>/dev/null || true)
if [ -z "$SSH_KEY" ]; then
  echo "No SSH public key found in ~/.ssh/. Generate one with: ssh-keygen -t ed25519"
  exit 1
fi

echo "[create-user] creating ${SSH_USER} on ${SERVER_IP}..."

expect << EOF
set timeout 30
spawn ssh -o StrictHostKeyChecking=no ${INITIAL_USER}@${SERVER_IP}
expect "password:"
send "${INITIAL_PASSWORD}\r"
expect "$ "
send "id ${SSH_USER} &>/dev/null && echo USER_EXISTS || sudo useradd -m -s /bin/bash ${SSH_USER}\r"
expect "$ "
send "sudo mkdir -p /home/${SSH_USER}/.ssh\r"
expect "$ "
send "echo '${SSH_KEY}' | sudo tee /home/${SSH_USER}/.ssh/authorized_keys\r"
expect "$ "
send "sudo chmod 700 /home/${SSH_USER}/.ssh\r"
expect "$ "
send "sudo chmod 600 /home/${SSH_USER}/.ssh/authorized_keys\r"
expect "$ "
send "sudo chown -R ${SSH_USER}:${SSH_USER} /home/${SSH_USER}/.ssh\r"
expect "$ "
send "echo '${SSH_USER} ALL=(ALL) NOPASSWD:ALL' | sudo tee /etc/sudoers.d/${SSH_USER}\r"
expect "$ "
send "sudo chmod 440 /etc/sudoers.d/${SSH_USER}\r"
expect "$ "
send "exit\r"
expect eof
EOF

echo "[create-user] verifying SSH access as ${SSH_USER}..."
ssh -o StrictHostKeyChecking=no "${SSH_USER}@${SERVER_IP}" "whoami && sudo whoami"

echo "[create-user] done — ${SSH_USER} created on ${SERVER_IP}"
echo "[create-user] next step: run scripts/setup-server.sh"
