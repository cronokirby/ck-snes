fmt:
	# Format everything
	cargo fmt --all

check:
	#!/usr/bin/env sh
	filter_crates() {
		cargo metadata --no-deps --format-version 1 | jq -r ".packages[] | select($1) | .name"
	}
	no_std_crates() {
		filter_crates ".metadata?.no_std"
	}
	std_crates() {
		filter_crates ".metadata?.no_std | not"
	}
	RUSTFLAGS="-D warnings"
	for crate in $(no_std_crates); do
		cargo check -p $crate --target thumbv7em-none-eabihf
	done
	for crate in $(std_crates); do
		cargo check -p $crate
	done
	cargo clippy --all-targets
