include .env

generate:
	java -jar dev-tools/openapi-generator-cli.jar generate -i api.yaml --generator-name rust -o crates/codegen/api-client
	java -jar dev-tools/openapi-generator-cli.jar generate -i api.yaml -g rust-server -o crates/codegen/server

watch-backend:
	cargo watch -x 'run --bin server-app'

watch-frontend:
	cd crates/ui-app && TRUNK_SERVE_PORT=8080 TRUNK_BUILD_RELEASE=true trunk serve

build-frontend:
	cd crates/ui-app && TRUNK_SERVE_PORT=8080 TRUNK_BUILD_RELEASE=true trunk build

watch-all:
	parallel --tty --tag -j 0 ::: "$(MAKE) watch-backend" "$(MAKE) watch-frontend"

refresh-db:
	psql $(DATABASE_BASE_URL) -c 'DROP DATABASE IF EXISTS $(DATABASE) WITH (FORCE)'
	export DATABASE_URL=$(DATABASE_URL) && cd crates/db_impl && diesel setup && diesel migration run
	cargo run --bin import-files

deploy-frontend:
	aws s3 cp crates/ui-app/dist s3://$(DEST_BUCKET)/ --recursive

init-eb:
	cd dev-tools/code-deploy/vendenic-server && eb init

deploy-eb:
	cargo build --release
	cp target/release/server-app dev-tools/code-deploy/vendenic-server/server
	cd dev-tools/code-deploy/vendenic-server && eb deploy

drop-db:
	psql $(DATABASE_BASE_URL) -c 'DROP DATABASE IF EXISTS $(DATABASE) WITH (FORCE)'

fmt:
	cargo +nightly fmt
