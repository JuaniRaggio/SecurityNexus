#!/bin/bash

# ============================================
# Security Nexus - Let's Encrypt Setup
# ============================================
# Sets up Let's Encrypt SSL certificates with automatic renewal
# Requires: A valid domain name pointing to your server

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
if [ "$EUID" -ne 0 ]; then
    print_error "This script must be run as root (use sudo)"
    exit 1
fi

# Check if certbot is installed
if ! command -v certbot &> /dev/null; then
    print_warn "Certbot is not installed. Installing now..."

    # Detect OS and install certbot
    if [ -f /etc/debian_version ]; then
        apt-get update
        apt-get install -y certbot python3-certbot-nginx
    elif [ -f /etc/redhat-release ]; then
        yum install -y certbot python3-certbot-nginx
    else
        print_error "Unsupported OS. Please install certbot manually."
        exit 1
    fi

    print_info "Certbot installed successfully"
fi

# Get domain and email
echo ""
print_info "Let's Encrypt SSL Certificate Setup"
echo ""
read -p "Enter your domain name (e.g., security-nexus.io): " DOMAIN
read -p "Enter your email address for Let's Encrypt notifications: " EMAIL

if [ -z "$DOMAIN" ] || [ -z "$EMAIL" ]; then
    print_error "Domain and email cannot be empty"
    exit 1
fi

# Validate email format
if ! echo "$EMAIL" | grep -E '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$' > /dev/null; then
    print_error "Invalid email format"
    exit 1
fi

print_step "1/4 Checking DNS configuration..."
# Check if domain resolves to current server IP
SERVER_IP=$(curl -s ifconfig.me)
DOMAIN_IP=$(dig +short "$DOMAIN" | tail -1)

if [ "$SERVER_IP" != "$DOMAIN_IP" ]; then
    print_warn "Domain $DOMAIN does not point to this server ($SERVER_IP)"
    print_warn "Domain currently points to: $DOMAIN_IP"
    read -p "Continue anyway? (y/N): " CONTINUE
    if [ "$CONTINUE" != "y" ] && [ "$CONTINUE" != "Y" ]; then
        print_error "Aborted. Please configure your DNS first."
        exit 1
    fi
fi

print_step "2/4 Stopping nginx to allow certbot to bind to port 80..."
docker-compose stop nginx || true

print_step "3/4 Obtaining SSL certificate from Let's Encrypt..."
certbot certonly --standalone \
    --preferred-challenges http \
    --email "$EMAIL" \
    --agree-tos \
    --no-eff-email \
    -d "$DOMAIN" \
    -d "www.$DOMAIN"

if [ $? -ne 0 ]; then
    print_error "Failed to obtain SSL certificate"
    exit 1
fi

print_step "4/4 Copying certificates to ssl/ directory..."
SSL_DIR="./ssl"
mkdir -p "$SSL_DIR"

# Let's Encrypt certificates are stored in /etc/letsencrypt/live/
cp "/etc/letsencrypt/live/$DOMAIN/fullchain.pem" "$SSL_DIR/fullchain.pem"
cp "/etc/letsencrypt/live/$DOMAIN/privkey.pem" "$SSL_DIR/privkey.pem"

# Set proper permissions
chmod 644 "$SSL_DIR/fullchain.pem"
chmod 600 "$SSL_DIR/privkey.pem"

# Create renewal hook to copy certificates after renewal
mkdir -p /etc/letsencrypt/renewal-hooks/deploy
cat > "/etc/letsencrypt/renewal-hooks/deploy/copy-certs.sh" <<EOF
#!/bin/bash
cp "/etc/letsencrypt/live/$DOMAIN/fullchain.pem" "$PWD/$SSL_DIR/fullchain.pem"
cp "/etc/letsencrypt/live/$DOMAIN/privkey.pem" "$PWD/$SSL_DIR/privkey.pem"
docker-compose restart nginx
EOF
chmod +x /etc/letsencrypt/renewal-hooks/deploy/copy-certs.sh

print_info "SSL certificates obtained successfully!"
echo ""
print_info "Next steps:"
echo "  1. Update your .env file:"
echo "     - Set DOMAIN=$DOMAIN"
echo "     - Set SSL_ENABLED=true"
echo "     - Set PROTOCOL=https"
echo ""
echo "  2. Update docker-compose.yml and nginx.conf to use your domain"
echo ""
echo "  3. Restart your services:"
echo "     docker-compose up -d"
echo ""
print_info "Certificate will auto-renew. Certbot renewal runs twice daily."
print_info "Certificate expires on: $(openssl x509 -enddate -noout -in "$SSL_DIR/fullchain.pem" | cut -d= -f2)"
echo ""
print_info "You can manually renew with: sudo certbot renew"
