.SILENT:
.PHONY: default clean package build

default: package

clean:
	@echo "Cleaning up build artifacts..."
	@rm -f apodstatus *.pkg.tar.zst
	@rm -rf pkg src
	@echo "Cleanup complete."

package:
	@echo "Copying binary..."
	@cp ../target/release/apodstatus .
	@echo "Updating package checksums..."
	@updpkgsums
	@echo "Building package..."
	@makepkg -f
	@echo "Package build complete."

build:
	@echo "Building binary..."
	@cd ..
	@cargo build --release
	@echo "Package build complete."
