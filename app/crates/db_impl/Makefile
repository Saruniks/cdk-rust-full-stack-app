build-MyMigrationLambda:
	cargo build -p db-sync --manifest-path ./db-sync/Cargo.toml --release --target x86_64-unknown-linux-musl
	cp db-sync/target/x86_64-unknown-linux-musl/release/db-sync bootstrap
	cp bootstrap $(ARTIFACTS_DIR)
