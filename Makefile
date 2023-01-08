
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test --no-default-features --features="maximum"

clean:
	cargo clean
