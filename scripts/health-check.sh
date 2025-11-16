#!/bin/bash

# ============================================
# Security Nexus - Health Check Script
# ============================================
# Verifies all services are running correctly

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Counters
PASSED=0
FAILED=0
WARNINGS=0

# Functions
print_success() {
    echo -e "${GREEN}✓${NC} $1"
    ((PASSED++))
}

print_fail() {
    echo -e "${RED}✗${NC} $1"
    ((FAILED++))
}

print_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

check_command() {
    if command -v $1 &> /dev/null; then
        print_success "$1 is installed"
    else
        print_fail "$1 is not installed"
    fi
}

check_service() {
    if docker-compose ps | grep -q "$1.*Up"; then
        print_success "$1 service is running"
    else
        print_fail "$1 service is not running"
    fi
}

check_health() {
    SERVICE=$1
    if docker-compose ps | grep -q "$SERVICE.*(healthy)"; then
        print_success "$SERVICE is healthy"
    elif docker-compose ps | grep -q "$SERVICE.*Up"; then
        print_warn "$SERVICE is running but not healthy yet"
    else
        print_fail "$SERVICE is not running"
    fi
}

check_port() {
    PORT=$1
    DESC=$2
    if curl -sf "http://localhost:$PORT" > /dev/null 2>&1; then
        print_success "$DESC responding on port $PORT"
    else
        print_fail "$DESC not responding on port $PORT"
    fi
}

echo ""
print_info "==========================================="
print_info "Security Nexus - Health Check"
print_info "==========================================="
echo ""

# ============================================
# Check Prerequisites
# ============================================
print_info "Checking prerequisites..."
check_command docker
check_command docker-compose
check_command curl
echo ""

# ============================================
# Check Docker Daemon
# ============================================
print_info "Checking Docker daemon..."
if docker info > /dev/null 2>&1; then
    print_success "Docker daemon is running"
else
    print_fail "Docker daemon is not running"
    exit 1
fi
echo ""

# ============================================
# Check Services
# ============================================
print_info "Checking Docker Compose services..."
check_service "monitoring-engine"
check_service "dashboard"
check_service "nginx"
echo ""

# ============================================
# Check Health Status
# ============================================
print_info "Checking service health..."
check_health "monitoring-engine"
check_health "dashboard"
check_health "nginx"
echo ""

# ============================================
# Check Endpoints
# ============================================
print_info "Checking HTTP endpoints..."

# Check nginx
if curl -sf http://localhost/health > /dev/null 2>&1; then
    print_success "Nginx health check passed"
else
    print_fail "Nginx health check failed"
fi

# Check monitoring engine API
if curl -sf http://localhost:8080/api/health > /dev/null 2>&1; then
    print_success "Monitoring engine API responding"
else
    print_fail "Monitoring engine API not responding"
fi

# Check dashboard
if curl -sf http://localhost:3000 > /dev/null 2>&1; then
    print_success "Dashboard responding"
else
    print_fail "Dashboard not responding"
fi
echo ""

# ============================================
# Check Resources
# ============================================
print_info "Checking system resources..."

# Disk space
DISK_USAGE=$(df -h / | awk 'NR==2 {print $5}' | sed 's/%//')
if [ "$DISK_USAGE" -lt 80 ]; then
    print_success "Disk space OK (${DISK_USAGE}% used)"
elif [ "$DISK_USAGE" -lt 90 ]; then
    print_warn "Disk space getting low (${DISK_USAGE}% used)"
else
    print_fail "Disk space critical (${DISK_USAGE}% used)"
fi

# Memory
MEM_AVAILABLE=$(free -m | awk 'NR==2 {print $7}')
if [ "$MEM_AVAILABLE" -gt 500 ]; then
    print_success "Memory available: ${MEM_AVAILABLE}MB"
elif [ "$MEM_AVAILABLE" -gt 200 ]; then
    print_warn "Memory running low: ${MEM_AVAILABLE}MB available"
else
    print_fail "Memory critical: ${MEM_AVAILABLE}MB available"
fi

# Docker disk usage
DOCKER_DISK=$(docker system df | grep "Images" | awk '{print $4}' | sed 's/(//;s/%)//')
if [ ! -z "$DOCKER_DISK" ] && [ "$DOCKER_DISK" -lt 80 ]; then
    print_success "Docker disk usage OK"
elif [ ! -z "$DOCKER_DISK" ]; then
    print_warn "Docker disk usage high, consider: docker system prune"
fi
echo ""

# ============================================
# Check Configuration
# ============================================
print_info "Checking configuration files..."

if [ -f ".env" ]; then
    print_success ".env file exists"
else
    print_fail ".env file missing"
fi

if [ -f "docker-compose.yml" ]; then
    print_success "docker-compose.yml exists"
else
    print_fail "docker-compose.yml missing"
fi

if [ -f "nginx.conf" ]; then
    print_success "nginx.conf exists"
else
    print_fail "nginx.conf missing"
fi

# Check SSL if enabled
if [ -f ".env" ] && grep -q "SSL_ENABLED=true" .env; then
    if [ -d "ssl" ] && [ -f "ssl/fullchain.pem" ] && [ -f "ssl/privkey.pem" ]; then
        print_success "SSL certificates found"
    else
        print_fail "SSL enabled but certificates missing"
    fi
fi
echo ""

# ============================================
# Check Logs for Errors
# ============================================
print_info "Checking recent logs for errors..."

ERROR_COUNT=$(docker-compose logs --tail=100 2>&1 | grep -i "error\|fatal\|panic" | wc -l)
if [ "$ERROR_COUNT" -eq 0 ]; then
    print_success "No errors in recent logs"
elif [ "$ERROR_COUNT" -lt 5 ]; then
    print_warn "Found $ERROR_COUNT errors in recent logs"
else
    print_fail "Found $ERROR_COUNT errors in recent logs - investigate!"
fi
echo ""

# ============================================
# Network Tests
# ============================================
print_info "Checking network connectivity..."

# Test Polkadot endpoint
if grep -q "WS_ENDPOINT" .env; then
    WS_ENDPOINT=$(grep "WS_ENDPOINT" .env | cut -d'=' -f2)
    if curl -sf "https://polkadot.api.onfinality.io" > /dev/null 2>&1; then
        print_success "Can reach Polkadot RPC endpoint"
    else
        print_warn "Cannot reach Polkadot RPC endpoint"
    fi
fi
echo ""

# ============================================
# Summary
# ============================================
echo ""
print_info "==========================================="
print_info "Health Check Summary"
print_info "==========================================="
echo ""
echo -e "${GREEN}Passed:${NC}   $PASSED"
echo -e "${YELLOW}Warnings:${NC} $WARNINGS"
echo -e "${RED}Failed:${NC}   $FAILED"
echo ""

if [ "$FAILED" -eq 0 ] && [ "$WARNINGS" -eq 0 ]; then
    print_success "All checks passed! System is healthy."
    exit 0
elif [ "$FAILED" -eq 0 ]; then
    print_warn "System is running but has warnings. Review above."
    exit 0
else
    print_fail "System has issues that need attention."
    echo ""
    print_info "Troubleshooting tips:"
    echo "  - Check logs: docker-compose logs -f"
    echo "  - Restart services: docker-compose restart"
    echo "  - Rebuild if needed: docker-compose up -d --build"
    echo "  - See README_DEPLOYMENT.md for more help"
    exit 1
fi
