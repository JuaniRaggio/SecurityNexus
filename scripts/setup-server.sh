#!/bin/bash

# ============================================
# Security Nexus - Server Setup Script
# ============================================
# Automated setup for DigitalOcean/VPS servers
# Run this script on a fresh Ubuntu 22.04 server

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored messages
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    print_error "Do not run this script as root. Run as a normal user with sudo privileges."
    exit 1
fi

# Check if running on Ubuntu
if [ ! -f /etc/os-release ]; then
    print_error "This script is designed for Ubuntu. Your OS may not be supported."
    exit 1
fi

source /etc/os-release
if [ "$ID" != "ubuntu" ]; then
    print_warn "This script is optimized for Ubuntu. Your OS: $ID. Continuing anyway..."
fi

echo ""
print_info "==========================================="
print_info "Security Nexus - Server Setup"
print_info "==========================================="
echo ""

# ============================================
# Step 1: System Update
# ============================================
print_step "1/8 Updating system packages..."
sudo apt-get update
sudo apt-get upgrade -y

# ============================================
# Step 2: Install Essential Tools
# ============================================
print_step "2/8 Installing essential tools..."
sudo apt-get install -y \
    curl \
    wget \
    git \
    vim \
    htop \
    ufw \
    fail2ban \
    ca-certificates \
    gnupg \
    lsb-release

# ============================================
# Step 3: Install Docker
# ============================================
print_step "3/8 Installing Docker..."

# Remove old Docker installations
sudo apt-get remove -y docker docker-engine docker.io containerd runc || true

# Add Docker's official GPG key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

# Set up Docker repository
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker Engine
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Add current user to docker group
sudo usermod -aG docker $USER

print_info "Docker installed successfully"

# ============================================
# Step 4: Configure Firewall
# ============================================
print_step "4/8 Configuring firewall..."

# Enable UFW
sudo ufw --force enable

# Allow SSH (IMPORTANT: Do this first!)
sudo ufw allow 22/tcp comment 'SSH'

# Allow HTTP and HTTPS
sudo ufw allow 80/tcp comment 'HTTP'
sudo ufw allow 443/tcp comment 'HTTPS'

# Reload firewall
sudo ufw reload

print_info "Firewall configured: Ports 22, 80, 443 open"

# ============================================
# Step 5: Configure Fail2Ban
# ============================================
print_step "5/8 Configuring Fail2Ban..."

sudo systemctl enable fail2ban
sudo systemctl start fail2ban

print_info "Fail2Ban enabled for SSH protection"

# ============================================
# Step 6: Create Application Directory
# ============================================
print_step "6/8 Creating application directory..."

mkdir -p ~/security-nexus
cd ~/security-nexus

print_info "Application directory created: ~/security-nexus"

# ============================================
# Step 7: Configure Docker Daemon
# ============================================
print_step "7/8 Configuring Docker daemon..."

sudo mkdir -p /etc/docker
sudo tee /etc/docker/daemon.json > /dev/null <<EOF
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  },
  "live-restore": true
}
EOF

sudo systemctl restart docker

print_info "Docker daemon configured with log rotation"

# ============================================
# Step 8: System Optimization
# ============================================
print_step "8/8 Optimizing system settings..."

# Increase file descriptor limits
sudo tee -a /etc/security/limits.conf > /dev/null <<EOF
* soft nofile 65535
* hard nofile 65535
EOF

# Optimize kernel parameters for Docker
sudo tee -a /etc/sysctl.conf > /dev/null <<EOF
# Security Nexus optimizations
vm.max_map_count=262144
net.core.somaxconn=65535
net.ipv4.tcp_max_syn_backlog=8192
EOF

sudo sysctl -p

print_info "System optimization complete"

# ============================================
# Setup Complete
# ============================================
echo ""
print_info "==========================================="
print_info "Server Setup Complete!"
print_info "==========================================="
echo ""
print_info "Next steps:"
echo ""
echo "  1. Log out and log back in for Docker group changes to take effect:"
echo "     exit"
echo ""
echo "  2. Clone your repository:"
echo "     cd ~/security-nexus"
echo "     git clone https://github.com/JuaniRaggio/SecurityNexus.git ."
echo ""
echo "  3. Copy and configure environment file:"
echo "     cp .env.example .env"
echo "     vim .env  # Update with your configuration"
echo ""
echo "  4. (Optional) Generate SSL certificates:"
echo "     # For self-signed (IP only):"
echo "     bash scripts/ssl/generate-self-signed.sh"
echo "     "
echo "     # OR for Let's Encrypt (with domain):"
echo "     sudo bash scripts/ssl/setup-letsencrypt.sh"
echo ""
echo "  5. Build and start services:"
echo "     docker-compose up -d --build"
echo ""
echo "  6. Monitor logs:"
echo "     docker-compose logs -f"
echo ""
echo "  7. Check service status:"
echo "     docker-compose ps"
echo ""
print_warn "IMPORTANT: Log out and log back in before using Docker!"
echo ""
print_info "Server IP: $(curl -s ifconfig.me)"
print_info "Access your dashboard at: http://$(curl -s ifconfig.me)"
echo ""
