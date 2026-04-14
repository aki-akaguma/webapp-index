
all: list

MAKEFILE_LIST = Makefile
# Self-documenting Makefile targets script from Stack Overflow
# Targets with comments on the same line will be listed.
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: list

clean:
	cargo clean

check:
	cargo check

test:
	cargo test --features server

apply-patch:
	cargo patch-crate

css:
	tailwindcss -i ./resources/tailwind.input.css -o ./assets/css/tailwind.css

css-watch:
	tailwindcss -i ./resources/tailwind.input.css -o ./assets/css/tailwind.css --watch

bundle-web:
	dx bundle --web --release --base-path "/app"
	cp -a resources/config.toml target/dx/index/release/web/ && chmod 640 target/dx/index/release/web/config.toml

bundle-desktop:
	dx bundle --desktop --release --package-types appimage

#	dx bundle --desktop --release --features backend_next

bundle-android-aarch64:
	@rm -fr "target/dx"
	dx bundle --android --release --target=aarch64-linux-android
	./scripts/apk-icon-assemble-r.sh broinfo aarch64 resources/android

bundle-android-x86_64:
	@rm -fr "target/dx"
	dx bundle --android --release --target=x86_64-linux-android
	./scripts/apk-icon-assemble-r.sh broinfo x86_64 resources/android

bundle-android-wv:
	@rm -fr "target/dx"
	./scripts/wv-apk-icon-assemble-r.sh broinfo resources/android ./scripts/android-webview-params.toml

bundle-android-wva:
	@rm -fr "target/dx"
	./scripts/wva-apk-icon-assemble-r.sh broinfo resources/android resources/android ./scripts/android-webview-assets-params.toml
