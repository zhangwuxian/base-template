TARGET = base_template
BUILD_FOLD = ./build
VERSION:=$(shell cat version.ini)
PACKAGE_FOLD_NAME = ${TARGET}-$(VERSION)

release:
	mkdir -p ${BUILD_FOLD}
	mkdir -p .cargo
	echo '[target.x86_64-unknown-linux-musl]\nlinker = "x86_64-linux-musl-gcc"' > .cargo/config.toml
	OPENSSL_DIR=/opt/homebrew/opt/openssl@3 PKG_CONFIG_SYSROOT_DIR=/ RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl
	mkdir -p $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}
	mkdir -p $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/bin
	mkdir -p $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/libs
	mkdir -p $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/config
	cp -rf target/release/sample $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/libs
	cp -rf bin/* $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/bin
	cp -rf config/* $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/config
	chmod -R 777 $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/bin/*
	cd $(BUILD_FOLD) && tar zcvf ${PACKAGE_FOLD_NAME}.tar.gz ${PACKAGE_FOLD_NAME} && rm -rf ${PACKAGE_FOLD_NAME}
	echo "build release package success. ${PACKAGE_FOLD_NAME}.tar.gz "

test:
	sh ./scripts/integration-testing.sh
clean:
	cargo clean
	rm -rf build