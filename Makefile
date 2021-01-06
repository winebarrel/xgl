SHELL   := /bin/bash
VERSION := v$(shell cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "xjr").version')
TARGET  := x86_64-apple-darwin

.PHONY: all
all: test build

.PHONY: build
build:
	cross build --target $(TARGET) --release

.PHONY: package
package: clean build
	gzip target/$(TARGET)/release/xjr -c > xjr_$(VERSION)_$(TARGET).gz
	sha1sum xjr_$(VERSION)_$(TARGET).gz > xjr_$(VERSION)_$(TARGET).gz.sha1sum

.PHONY: clean
clean:
	rm -rf target

.PHONY: test
test:
	cargo test
