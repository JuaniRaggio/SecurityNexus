#!/bin/bash

# ============================================
# Security Nexus - SSL Certificate Generator
# ============================================
# Generates self-signed SSL certificates for development/testing
# For production with a domain, use Let's Encrypt instead

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CERT_DIR="./ssl"
DAYS_VALID=365
COUNTRY="US"
STATE="California"
CITY="San Francisco"
ORG="Security Nexus"
OU="DevOps"

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

# Check if OpenSSL is installed
if ! command -v openssl &> /dev/null; then
    print_error "OpenSSL is not installed. Please install it first."
    exit 1
fi

# Get server IP or domain
echo ""
print_info "SSL Certificate Generator for Security Nexus"
echo ""
read -p "Enter your server IP address or domain name: " SERVER_ADDRESS

if [ -z "$SERVER_ADDRESS" ]; then
    print_error "Server address cannot be empty"
    exit 1
fi

# Create SSL directory if it doesn't exist
mkdir -p "$CERT_DIR"

print_info "Generating SSL certificates for: $SERVER_ADDRESS"
print_info "Certificate will be valid for $DAYS_VALID days"

# Generate private key
print_info "Generating private key..."
openssl genrsa -out "$CERT_DIR/privkey.pem" 2048

# Generate certificate signing request (CSR)
print_info "Generating certificate signing request..."
openssl req -new -key "$CERT_DIR/privkey.pem" -out "$CERT_DIR/csr.pem" \
    -subj "/C=$COUNTRY/ST=$STATE/L=$CITY/O=$ORG/OU=$OU/CN=$SERVER_ADDRESS"

# Generate self-signed certificate
print_info "Generating self-signed certificate..."
openssl x509 -req -days $DAYS_VALID -in "$CERT_DIR/csr.pem" \
    -signkey "$CERT_DIR/privkey.pem" -out "$CERT_DIR/fullchain.pem" \
    -extfile <(printf "subjectAltName=IP:$SERVER_ADDRESS")

# Create dhparam for better security (optional but recommended)
print_info "Generating Diffie-Hellman parameters (this may take a while)..."
openssl dhparam -out "$CERT_DIR/dhparam.pem" 2048

# Set proper permissions
chmod 600 "$CERT_DIR/privkey.pem"
chmod 644 "$CERT_DIR/fullchain.pem"
chmod 644 "$CERT_DIR/dhparam.pem"

# Clean up CSR
rm "$CERT_DIR/csr.pem"

print_info "SSL certificates generated successfully!"
echo ""
print_info "Certificate files created in: $CERT_DIR/"
echo "  - privkey.pem (private key)"
echo "  - fullchain.pem (certificate)"
echo "  - dhparam.pem (DH parameters)"
echo ""
print_warn "IMPORTANT: These are self-signed certificates for development/testing only"
print_warn "Your browser will show a security warning when accessing the site"
print_warn "For production, use Let's Encrypt with a proper domain name"
echo ""
print_info "To use these certificates:"
echo "  1. Update your .env file with SSL_ENABLED=true"
echo "  2. Run: docker-compose up -d"
echo "  3. Access your site at: https://$SERVER_ADDRESS"
echo ""
print_info "Certificate expires on: $(openssl x509 -enddate -noout -in "$CERT_DIR/fullchain.pem" | cut -d= -f2)"
