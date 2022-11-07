format:
	cargo +nightly fmt # use nightly toolchain so we get group_imports
	npm run format

watch:
	systemfd --no-pid -q -s http::3000 -- cargo watch -c -w src/ -x run

test:
	cargo nextest run 

test-acceptance:
	npm run acceptance
