#!/bin/bash
# Security Nexus - Quick Demo Runner
# This script helps you run the demo components easily

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Cleanup function for Ctrl+C
cleanup() {
    echo ""
    echo -e "${YELLOW}Caught interrupt signal, cleaning up...${NC}"
    pkill -9 -f "cargo.*run.*release" 2>/dev/null || true
    pkill -9 -f "pnpm.*dev" 2>/dev/null || true
    pkill -9 -f "next-server" 2>/dev/null || true
    pkill -9 -f "node.*next" 2>/dev/null || true
    lsof -ti:8080 | xargs kill -9 2>/dev/null || true
    lsof -ti:3000 | xargs kill -9 2>/dev/null || true
    echo -e "${GREEN}Cleanup complete${NC}"
    exit 0
}

# Set up trap for Ctrl+C
trap cleanup SIGINT SIGTERM

# Function to print colored output
print_section() {
    echo -e "\n${BLUE}===================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}===================================================${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

# Main menu
show_menu() {
    clear
    echo -e "${GREEN}"
    cat << "EOF"
   ____                      _ _           _   _
  / ___|  ___  ___ _   _ _ __(_) |_ _   _  | \ | | _____  ___   _ ___
  \___ \ / _ \/ __| | | | '__| | __| | | | |  \| |/ _ \ \/ / | | / __|
   ___) |  __/ (__| |_| | |  | | |_| |_| | | |\  |  __/>  <| |_| \__ \
  |____/ \___|\___|\__,_|_|  |_|\__|\__, | |_| \_|\___/_/\_\\__,_|___/
                                    |___/
EOF
    echo -e "${NC}"
    echo "Security Nexus - Demo Runner"
    echo ""
    echo "Select what you want to demo:"
    echo ""
    echo "  1) SAFT Analysis - Analyze vulnerable pallet (defi_vault.rs)"
    echo "  2) SAFT JSON Output - Generate JSON report for dashboard"
    echo "  3) Start Web Dashboard - Launch Next.js dashboard (port 3000)"
    echo "  4) Full Demo - Run SAFT + Start Dashboard"
    echo "  5) Build Everything - Compile all components"
    echo "  6) Local Stack with DB - Monitoring Engine + Dashboard (COMPLETE DEMO)"
    echo "  7) Stop All Services - Kill monitoring engine and dashboard"
    echo ""
    echo "  q) Quit"
    echo ""
    read -p "Enter your choice: " choice
}

# Build SAFT Enhanced
build_saft() {
    print_section "Building SAFT Enhanced"
    cd "$SCRIPT_DIR"

    if cargo build --release --package saft-enhanced; then
        print_success "SAFT Enhanced built successfully"
        if [ -f "./target/release/saft" ]; then
            print_success "Binary location: ./target/release/saft"
        fi
    else
        print_error "Failed to build SAFT Enhanced"
        return 1
    fi
}

# Run SAFT analysis
run_saft_analysis() {
    print_section "Running SAFT Analysis on defi_vault.rs"
    cd "$SCRIPT_DIR"

    if [ ! -f "./target/release/saft" ]; then
        print_error "SAFT binary not found. Building first..."
        build_saft || return 1
    fi

    if [ ! -f "./test-samples/vulnerable-pallets/defi_vault.rs" ]; then
        print_error "defi_vault.rs not found at test-samples/vulnerable-pallets/"
        return 1
    fi

    print_info "Analyzing defi_vault.rs..."
    echo ""

    ./target/release/saft analyze test-samples/vulnerable-pallets/defi_vault.rs

    echo ""
    print_success "Analysis complete!"
    print_info "You can show this output during your demo"
}

# Generate JSON report
generate_json_report() {
    print_section "Generating JSON Report"
    cd "$SCRIPT_DIR"

    if [ ! -f "./target/release/saft" ]; then
        print_error "SAFT binary not found. Building first..."
        build_saft || return 1
    fi

    local output_file="/tmp/saft-defi-vault-report.json"

    print_info "Generating JSON report..."
    ./target/release/saft analyze test-samples/vulnerable-pallets/defi_vault.rs --format json > "$output_file" 2>&1 || true

    if [ -f "$output_file" ]; then
        print_success "JSON report generated: $output_file"
        print_info "Preview:"
        echo ""
        head -20 "$output_file"
        echo ""
        echo "... (truncated)"
    else
        print_error "Failed to generate JSON report"
    fi
}

# Start dashboard
start_dashboard() {
    print_section "Starting Web Dashboard"
    cd "$SCRIPT_DIR/packages/web-dashboard"

    if [ ! -d "node_modules" ]; then
        print_info "Installing dependencies first..."
        pnpm install
    fi

    # Create .env.local if doesn't exist
    if [ ! -f ".env.local" ]; then
        print_info "Creating .env.local configuration..."
        cat > .env.local << EOF
NEXT_PUBLIC_API_URL=http://localhost:3000
MONITORING_ENGINE_URL=http://localhost:8080
SAFT_BINARY_PATH=$SCRIPT_DIR/target/release/saft
NODE_ENV=development
EOF
        print_success "Configuration created"
    fi

    print_info "Starting Next.js development server..."
    print_info "Dashboard will be available at: http://localhost:3000"
    print_info ""
    print_info "Press Ctrl+C to stop"
    echo ""

    pnpm dev
}

# Full demo
full_demo() {
    print_section "Running Full Demo"

    # Run SAFT analysis
    run_saft_analysis

    echo ""
    print_info "SAFT analysis complete. Now let's start the dashboard..."
    read -p "Press Enter to start the dashboard..."

    # Start dashboard
    start_dashboard
}

# Build everything
build_all() {
    print_section "Building All Components"

    # Build SAFT
    build_saft

    # Build dashboard
    print_section "Building Web Dashboard"
    cd "$SCRIPT_DIR/packages/web-dashboard"
    if pnpm install && pnpm build; then
        print_success "Dashboard built successfully"
    else
        print_error "Failed to build dashboard"
        return 1
    fi

    print_section "Build Complete!"
    print_success "All components built successfully"
}

# Local stack with database
run_local_stack() {
    print_section "Starting Complete Local Stack"

    # Check if database is running
    print_info "Checking database status..."
    if docker ps | grep -q "timescaledb"; then
        print_success "Database is running"
        DB_CONTAINER=$(docker ps | grep timescaledb | awk '{print $NF}')
        print_info "Container: $DB_CONTAINER"
    else
        print_error "Database is not running!"
        print_info "Starting TimescaleDB..."
        docker-compose -f docker-compose.db.yml up -d
        print_info "Waiting 30 seconds for database to be ready..."
        sleep 30
    fi

    # Verify DB connection
    print_info "Testing database connection..."
    if docker exec $(docker ps | grep timescaledb | awk '{print $NF}') psql -U nexus -d security_nexus -c "SELECT 1" > /dev/null 2>&1; then
        print_success "Database connection OK"
    else
        print_error "Cannot connect to database"
        return 1
    fi

    echo ""
    print_section "Starting Services Automatically"

    # Create log directory
    mkdir -p "$SCRIPT_DIR/logs"

    print_info "Starting Monitoring Engine in background..."
    cd "$SCRIPT_DIR/packages/monitoring-engine"

    # Start monitoring engine in background
    DATABASE_URL="postgresql://nexus:nexus_password_changeme@localhost:5432/security_nexus" \
    WS_ENDPOINT="wss://westend-rpc.polkadot.io" \
    CHAIN_NAME="Westend" \
    API_PORT="8080" \
    RUST_LOG="monitoring_engine=info,sqlx=warn" \
    cargo run --release > "$SCRIPT_DIR/logs/monitoring-engine.log" 2>&1 &

    MONITORING_PID=$!
    print_success "Monitoring Engine started (PID: $MONITORING_PID)"
    print_info "Logs: $SCRIPT_DIR/logs/monitoring-engine.log"

    # Wait a bit for monitoring engine to start
    print_info "Waiting 10 seconds for monitoring engine to initialize..."
    sleep 10

    # Check if monitoring engine is still running
    if ps -p $MONITORING_PID > /dev/null; then
        print_success "Monitoring Engine is running"
    else
        print_error "Monitoring Engine failed to start. Check logs:"
        tail -20 "$SCRIPT_DIR/logs/monitoring-engine.log"
        return 1
    fi

    echo ""
    print_section "Opening Dashboard in New Terminal"

    # Detect OS and open new terminal
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        print_info "Opening new Terminal window for Dashboard..."
        osascript -e "tell application \"Terminal\" to do script \"cd $SCRIPT_DIR/packages/web-dashboard && echo 'Starting Dashboard...' && pnpm dev\""
        print_success "Dashboard terminal opened"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux - try different terminal emulators
        if command -v gnome-terminal &> /dev/null; then
            gnome-terminal -- bash -c "cd $SCRIPT_DIR/packages/web-dashboard && pnpm dev; exec bash"
        elif command -v xterm &> /dev/null; then
            xterm -e "cd $SCRIPT_DIR/packages/web-dashboard && pnpm dev" &
        else
            print_error "No suitable terminal emulator found"
            print_info "Please open a new terminal and run:"
            echo "  cd $SCRIPT_DIR/packages/web-dashboard"
            echo "  pnpm dev"
        fi
    fi

    echo ""
    print_section "Services Started!"
    print_success "Monitoring Engine: Running (PID: $MONITORING_PID)"
    print_success "Dashboard: Starting in new terminal"
    echo ""
    print_info "View monitoring logs: tail -f $SCRIPT_DIR/logs/monitoring-engine.log"
    print_info "Dashboard will be at: http://localhost:3000"
    echo ""
    print_info "To stop monitoring engine: kill $MONITORING_PID"
    echo ""

    read -p "Press Enter to return to menu (monitoring engine will keep running)..."
}

# Stop all services
stop_all_services() {
    print_section "Stopping All Services"

    # Find and kill monitoring engine
    print_info "Looking for monitoring engine processes..."
    MONITORING_PIDS=$(pgrep -f "cargo.*run.*release" 2>/dev/null)

    if [ -n "$MONITORING_PIDS" ]; then
        for pid in $MONITORING_PIDS; do
            print_info "Killing monitoring engine (PID: $pid)..."
            kill -9 $pid 2>/dev/null || true
        done
        print_success "Monitoring engine stopped"
    else
        print_info "No monitoring engine process found"
    fi

    # Find and kill dashboard (Next.js dev server and node processes)
    print_info "Looking for dashboard processes..."

    # Kill next-server processes
    NEXT_PIDS=$(pgrep -f "next-server" 2>/dev/null)
    if [ -n "$NEXT_PIDS" ]; then
        for pid in $NEXT_PIDS; do
            print_info "Killing Next.js server (PID: $pid)..."
            kill -9 $pid 2>/dev/null || true
        done
    fi

    # Kill pnpm dev processes
    PNPM_PIDS=$(pgrep -f "pnpm.*dev" 2>/dev/null)
    if [ -n "$PNPM_PIDS" ]; then
        for pid in $PNPM_PIDS; do
            print_info "Killing pnpm dev (PID: $pid)..."
            kill -9 $pid 2>/dev/null || true
        done
    fi

    # Kill node processes running next
    NODE_PIDS=$(pgrep -f "node.*next" 2>/dev/null)
    if [ -n "$NODE_PIDS" ]; then
        for pid in $NODE_PIDS; do
            print_info "Killing node process (PID: $pid)..."
            kill -9 $pid 2>/dev/null || true
        done
        print_success "Dashboard processes stopped"
    else
        print_info "No dashboard process found"
    fi

    # Also check for processes on port 8080 and 3000
    print_info "Checking ports 8080 and 3000..."

    PORT_8080_PID=$(lsof -ti:8080 2>/dev/null)
    if [ -n "$PORT_8080_PID" ]; then
        print_info "Killing process on port 8080 (PID: $PORT_8080_PID)..."
        kill -9 $PORT_8080_PID 2>/dev/null || true
        print_success "Port 8080 cleared"
    fi

    PORT_3000_PID=$(lsof -ti:3000 2>/dev/null)
    if [ -n "$PORT_3000_PID" ]; then
        print_info "Killing process on port 3000 (PID: $PORT_3000_PID)..."
        kill -9 $PORT_3000_PID 2>/dev/null || true
        print_success "Port 3000 cleared"
    fi

    # Kill any orphaned terminal processes related to our script
    print_info "Cleaning up terminal processes..."
    sleep 1

    echo ""
    print_success "All services stopped"
    print_info "Dashboard terminal window can be closed manually"
    print_info "Database remains running (use 'docker-compose -f docker-compose.db.yml down' to stop)"

    read -p "Press Enter to continue..."
}

# Main loop
while true; do
    show_menu

    case $choice in
        1)
            run_saft_analysis
            read -p "Press Enter to continue..."
            ;;
        2)
            generate_json_report
            read -p "Press Enter to continue..."
            ;;
        3)
            start_dashboard
            ;;
        4)
            full_demo
            ;;
        5)
            build_all
            read -p "Press Enter to continue..."
            ;;
        6)
            run_local_stack
            ;;
        7)
            stop_all_services
            ;;
        q|Q)
            print_info "Cleaning up services..."
            # Silently kill any running services
            pkill -9 -f "cargo.*run.*release" 2>/dev/null || true
            pkill -9 -f "pnpm.*dev" 2>/dev/null || true
            pkill -9 -f "next-server" 2>/dev/null || true
            pkill -9 -f "node.*next" 2>/dev/null || true
            lsof -ti:8080 | xargs kill -9 2>/dev/null || true
            lsof -ti:3000 | xargs kill -9 2>/dev/null || true
            print_success "Services cleaned up"
            print_info "Exiting demo runner!"
            exit 0
            ;;
        *)
            print_error "Invalid option. Please try again."
            sleep 2
            ;;
    esac
done
