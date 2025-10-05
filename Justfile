fmt:
	# Format everything
	cargo fmt --all

check:
	# Check everything in the workspace, failing on warnings.
	RUSTFLAGS="-D warnings" cargo check --all
