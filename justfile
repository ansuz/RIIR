build: clean build-macro
	#!/usr/bin/env bash
	LIB_PATH=$(echo -n target/debug/deps/libriir_macro-*.so)
	rustc \
		--crate-name riir \
		--edition=2018 \
		README.md \
		--crate-type bin \
		--out-dir target/debug/deps \
		-L dependency=target/debug/deps \
		--extern riir_macro=$LIB_PATH
clean:
	#!/usr/bin/env bash
	rm target/debug/deps/libriir_macro-*.so || true
build-macro:
	cd riir-macro && cargo build --target-dir=../target $( (rustc --version | grep nightly >/dev/null) && echo --features span)

test:
	cd riir-macro && cargo test --target-dir=../target $( (rustc --version | grep nightly >/dev/null) && echo --features span)
