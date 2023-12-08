SHELL := /bin/bash

days := $(wildcard day*)
ua_files := $(foreach day, $(days), $(wildcard $(day)/part*.ua))

.PHONY: all test run

all: test run

test:
	@for file in $(ua_files); do \
		echo "Testing $$file"; \
		(cd $$(dirname $$file) && uiua test $$(basename $$file)); \
	done

run:
	@for file in $(ua_files); do \
		echo "Running $$file"; \
		(cd $$(dirname $$file) && uiua run $$(basename $$file)); \
	done
