all: generate-header
.PHONY: list-types generate-header

list_types: SHELL:=/bin/bash
list-types:
	cat src/lib.rs | grep -E '^pub (struct|enum) (.*?) {$$' | cut -d ' ' -f 3 | while read x; do echo \"$$x\",; done

generate-header:
	cbindgen -v --config cbindgen.toml --crate netplan-types --output netplan-types.h