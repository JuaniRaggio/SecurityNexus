#!/bin/bash
# Emergency Cleanup Script for Security Nexus
# Use this to kill all running services and free up ports

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}=================================${NC}"
echo -e "${BLUE}Security Nexus - Emergency Cleanup${NC}"
echo -e "${BLUE}=================================${NC}\n"

# Kill monitoring engine processes
echo -e "${YELLOW}Killing monitoring engine processes...${NC}"
MONITORING_PIDS=$(pgrep -f "monitoring-engine" 2>/dev/null)
if [ -n "$MONITORING_PIDS" ]; then
    echo "$MONITORING_PIDS" | while read pid; do
        echo "  Killing PID $pid"
        kill -9 $pid 2>/dev/null || true
    done
    echo -e "${GREEN}Monitoring engine processes killed${NC}"
else
    echo "  No monitoring engine processes found"
fi

# Kill cargo run processes
echo -e "\n${YELLOW}Killing cargo processes...${NC}"
CARGO_PIDS=$(pgrep -f "cargo.*run" 2>/dev/null)
if [ -n "$CARGO_PIDS" ]; then
    echo "$CARGO_PIDS" | while read pid; do
        echo "  Killing PID $pid"
        kill -9 $pid 2>/dev/null || true
    done
    echo -e "${GREEN}Cargo processes killed${NC}"
else
    echo "  No cargo processes found"
fi

# Kill dashboard processes
echo -e "\n${YELLOW}Killing dashboard processes...${NC}"
pkill -9 -f "pnpm.*dev" 2>/dev/null && echo "  pnpm dev killed" || echo "  No pnpm dev processes"
pkill -9 -f "next-server" 2>/dev/null && echo "  next-server killed" || echo "  No next-server processes"
pkill -9 -f "node.*next" 2>/dev/null && echo "  node next killed" || echo "  No node next processes"

# Clear port 8080
echo -e "\n${YELLOW}Clearing port 8080...${NC}"
PORT_8080=$(lsof -ti:8080 2>/dev/null)
if [ -n "$PORT_8080" ]; then
    echo "$PORT_8080" | while read pid; do
        echo "  Killing PID $pid on port 8080"
        kill -9 $pid 2>/dev/null || true
    done
    echo -e "${GREEN}Port 8080 cleared${NC}"
else
    echo "  Port 8080 is free"
fi

# Clear port 3000
echo -e "\n${YELLOW}Clearing port 3000...${NC}"
PORT_3000=$(lsof -ti:3000 2>/dev/null)
if [ -n "$PORT_3000" ]; then
    echo "$PORT_3000" | while read pid; do
        echo "  Killing PID $pid on port 3000"
        kill -9 $pid 2>/dev/null || true
    done
    echo -e "${GREEN}Port 3000 cleared${NC}"
else
    echo "  Port 3000 is free"
fi

# Clean up log files
echo -e "\n${YELLOW}Cleaning up logs...${NC}"
if [ -d "logs" ]; then
    rm -f logs/*.log 2>/dev/null
    echo -e "${GREEN}Logs cleaned${NC}"
else
    echo "  No logs directory found"
fi

# Verification
echo -e "\n${BLUE}Verification:${NC}"
echo -e "${YELLOW}Port 8080:${NC} $(lsof -ti:8080 2>/dev/null || echo 'Free')"
echo -e "${YELLOW}Port 3000:${NC} $(lsof -ti:3000 2>/dev/null || echo 'Free')"
echo -e "${YELLOW}Monitoring processes:${NC} $(pgrep -f "monitoring-engine" 2>/dev/null || echo 'None')"
echo -e "${YELLOW}Dashboard processes:${NC} $(pgrep -f "next-server\|pnpm.*dev" 2>/dev/null || echo 'None')"

echo -e "\n${GREEN}=================================${NC}"
echo -e "${GREEN}Cleanup complete!${NC}"
echo -e "${GREEN}=================================${NC}"
