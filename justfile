# Justfile for basic-web development

# Default recipe - show available commands
default:
    @echo "Available commands:"
    @echo "  just dev         - Start SNAPPY development server (100ms response)"
    @echo "  just dev-verbose - Start development server with verbose file change tracking"
    @echo "  just stop-dev    - Stop development server safely"
    @echo "  just restart-dev - Restart development server (stop + start)"
    @echo "  just status      - Check development server status"
    @echo "  just logs        - Show development server logs"
    @echo "  just build       - Build the project"
    @echo "  just run         - Run the project in release mode"

# Start development server with optimized cargo watch
dev:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Ensure tmp directory exists
    mkdir -p tmp
    
    # Check if dev server is already running
    if [ -f tmp/dev-server.pid ]; then
        PID=$(cat tmp/dev-server.pid)
        if ps -p $PID > /dev/null 2>&1; then
            echo "Development server is already running (PID: $PID)"
            echo "Use 'just stop-dev' to stop it first"
            exit 1
        else
            echo "Removing stale PID file..."
            rm -f tmp/dev-server.pid
        fi
    fi
    
    echo "Starting optimized development server..."
    
    # Start cargo watch with SNAPPY development settings:
    # --ignore: Minimal ignores for maximum responsiveness
    # --delay: Ultra-fast 100ms delay for instant feedback
    # --why: Show changed files for debugging
    # --clear: Clean output on each rebuild
    # No problematic flags - pure speed and reliability
    cargo watch \
        --ignore "tmp/" \
        --ignore "target/" \
        --delay 0.1 \
        --why \
        --clear \
        -x run > tmp/cargo_watch.log 2>&1 &
    
    DEV_PID=$!
    
    # Save PID to file for later cleanup
    echo $DEV_PID > tmp/dev-server.pid
    
    echo "Development server started (PID: $DEV_PID)"
    echo "🚀 SNAPPY development mode enabled:"
    echo "  ⚡ 100ms delay for instant response"
    echo "  🎯 Minimal ignores (only tmp/ and target/)"
    echo "  🔍 Real-time change tracking with --why"
    echo "  🖥️  Clean terminal output with --clear"
    echo "  🔥 Native file watching for maximum speed"
    echo "  ✨ Zero deadlocks, pure performance"
    echo "Logs are being written to tmp/cargo_watch.log"
    echo "Use 'just stop-dev' to stop the server"
    echo "Use 'just logs' to follow the logs"
    
    # Wait a moment and check if process started successfully
    sleep 3
    if ! ps -p $DEV_PID > /dev/null 2>&1; then
        echo "Error: Development server failed to start"
        rm -f tmp/dev-server.pid
        exit 1
    fi
    
    echo "Ultra-responsive development server is running successfully!"

# Start development server with verbose file change tracking
dev-verbose:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Ensure tmp directory exists
    mkdir -p tmp
    
    # Check if dev server is already running
    if [ -f tmp/dev-server.pid ]; then
        PID=$(cat tmp/dev-server.pid)
        if ps -p $PID > /dev/null 2>&1; then
            echo "Development server is already running (PID: $PID)"
            echo "Use 'just stop-dev' to stop it first"
            exit 1
        else
            echo "Removing stale PID file..."
            rm -f tmp/dev-server.pid
        fi
    fi
    
    echo "Starting development server with verbose file tracking..."
    
    # ULTRA-VERBOSE mode for debugging watch issues
    # --debug: Full file watching internals
    # --why: Show exactly which files trigger rebuilds  
    # --delay: Even faster 50ms for debugging
    # Maximum information output for troubleshooting
    RUST_LOG=debug cargo watch \
        --ignore "tmp/" \
        --ignore "target/" \
        --delay 0.05 \
        --debug \
        --why \
        --clear \
        -x run > tmp/cargo_watch_verbose.log 2>&1 &
    
    DEV_PID=$!
    
    # Save PID to file for later cleanup
    echo $DEV_PID > tmp/dev-server.pid
    
    echo "Development server started (PID: $DEV_PID)"
    echo "🔍 ULTRA-VERBOSE debugging mode enabled:"
    echo "  🔬 RUST_LOG=debug for maximum detail"
    echo "  ⚡ 50ms delay for instant debugging feedback"
    echo "  📋 Complete file watching internals"
    echo "  🎯 Exact file change detection with --why"
    echo "  📝 All debug output in tmp/cargo_watch_verbose.log"
    echo "Use 'just stop-dev' to stop the server"
    echo "Use 'tail -f tmp/cargo_watch_verbose.log' to see verbose logs"
    
    # Wait a moment and check if process started successfully
    sleep 2
    if ! ps -p $DEV_PID > /dev/null 2>&1; then
        echo "Error: Development server failed to start"
        rm -f tmp/dev-server.pid
        exit 1
    fi
    
    echo "Verbose development server is running successfully!"

# Stop development server safely
stop-dev:
    #!/usr/bin/env bash
    set -euo pipefail
    
    if [ ! -f tmp/dev-server.pid ]; then
        echo "No development server PID file found. Server might not be running."
        # Clean up any potential stray processes anyway
        pkill -f "cargo watch -x run" 2>/dev/null || true
        exit 0
    fi
    
    PID=$(cat tmp/dev-server.pid)
    echo "Stopping development server (PID: $PID)..."
    
    # Check if process is still running
    if ps -p $PID > /dev/null 2>&1; then
        # Try graceful termination first
        kill $PID 2>/dev/null || true
        
        # Wait a moment for graceful shutdown
        sleep 2
        
        # If still running, force kill
        if ps -p $PID > /dev/null 2>&1; then
            echo "Process still running, forcing termination..."
            kill -9 $PID 2>/dev/null || true
        fi
        
        # Also kill any child cargo processes that might be spawned
        pkill -P $PID 2>/dev/null || true
        
        echo "Development server stopped successfully"
    else
        echo "Development server was not running"
    fi
    
    # Clean up PID file
    rm -f tmp/dev-server.pid
    
    # Clean up any remaining cargo processes (safety net)
    # This is more targeted than a general grep
    CARGO_PIDS=$(pgrep -f "cargo run" 2>/dev/null || true)
    if [ ! -z "$CARGO_PIDS" ]; then
        echo "Cleaning up remaining cargo processes..."
        echo "$CARGO_PIDS" | xargs kill 2>/dev/null || true
    fi

# Restart development server (stop + start)
restart-dev:
    #!/usr/bin/env bash
    set -euo pipefail
    
    echo "Restarting development server..."
    
    # Stop the server if it's running (using the stop-dev recipe)
    just stop-dev
    
    # Wait a moment to ensure complete cleanup
    sleep 1
    
    # Start the server again (using the dev recipe)
    just dev
    
    echo "Development server restarted successfully!"

# Check development server status
status:
    #!/usr/bin/env bash
    if [ -f tmp/dev-server.pid ]; then
        PID=$(cat tmp/dev-server.pid)
        if ps -p $PID > /dev/null 2>&1; then
            echo "Development server is running (PID: $PID)"
            echo "Server should be available at http://127.0.0.1:3000"
        else
            echo "Development server PID file exists but process is not running"
            echo "Run 'just stop-dev' to clean up, then 'just dev' to start"
        fi
    else
        echo "Development server is not running"
    fi

# Show development server logs
logs:
    #!/usr/bin/env bash
    if [ -f tmp/cargo_watch.log ]; then
        echo "=== Development Server Logs ==="
        tail -f tmp/cargo_watch.log
    else
        echo "No log file found. Start the development server with 'just dev'"
    fi

# Build the project
build:
    cargo build

# Build for release
build-release:
    cargo build --release

# Run the project (release mode)
run:
    cargo run --release

# Clean build artifacts and development files
clean:
    cargo clean
    rm -rf tmp/
    echo "Cleaned build artifacts and development files"