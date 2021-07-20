# Makefile for tauri-events-minimal

# Initialize the project after checkout
init:
	yarn install
	yarn tauri deps install
	cd www && ln -s ../node_modules modules; cd ..

# Clean the app build
clean:
	cd src-tauri/ && rm -f Cargo.lock && cargo clean; cd ..

# Remove cargo lock within the build target/
cargo-lock:
	@if [ -f src-tauri/target/debug/.cargo-lock ]; then \
	   echo ".cargo-lock exists, removing.." \
	   && rm src-tauri/target/debug/.cargo-lock; \
	else \
       echo "(No .cargo-lock anyaway)"; \
	fi

# Start development server; build debug and run
dev:  cargo-lock
	yarn tauri dev; kill $$(ps ax | awk '/http-server/ && !/awk/ {print $$1}')


.PHONY: init clean cargo-lock dev
