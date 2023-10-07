Useful watch commands

# To run the server
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# To run the quickdev
cargo watch -q -c -w examples/ -x "run --example quick_dev"
