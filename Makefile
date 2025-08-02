# Design Patterns C++ Makefile
# Copyright © 2025 SHAO Liming <lmshao@163.com>

CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Wpedantic -O2
BUILD_DIR = target
BIN_DIR = $(BUILD_DIR)/cpp

# Color definitions
GREEN = \033[0;32m
BLUE = \033[0;34m
YELLOW = \033[1;33m
NC = \033[0m

# Find all main.cpp files
CPP_SOURCES = $(shell find . -name "main.cpp" -path "*/src/*")
EXECUTABLES = $(CPP_SOURCES:./%.cpp=$(BIN_DIR)/%)
EXECUTABLES := $(EXECUTABLES:/src/main=/pattern)

.PHONY: all clean help list run

all: $(BUILD_DIR) $(EXECUTABLES)
	@echo "$(GREEN)✅ All C++ examples built successfully!$(NC)"
	@echo "$(BLUE)📁 Executables are in $(BIN_DIR)/$(NC)"

$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

# Generic rule: build from category/pattern/src/main.cpp to target/cpp/category/pattern
$(BIN_DIR)/%/pattern: %/src/main.cpp | $(BUILD_DIR)
	@echo "$(BLUE)🔨 Building $*...$(NC)"
	@mkdir -p $(dir $@)
	@$(CXX) $(CXXFLAGS) $< -o $@
	@# Rename pattern to actual pattern name
	@pattern_name=$$(basename $*); \
	 target_dir=$$(dirname $@); \
	 mv $@ $$target_dir/$$pattern_name

# Run specific example: make run command, make run builder, etc.
run:
	@if [ -z "$(filter-out $@,$(MAKECMDGOALS))" ]; then \
		echo "$(YELLOW)Usage: make run <pattern_name>$(NC)"; \
		echo "$(BLUE)Examples:$(NC)"; \
		echo "  make run command      # Run command pattern"; \
		echo "  make run builder      # Run builder pattern"; \
		echo "  make run singleton    # Run singleton pattern"; \
		echo "$(BLUE)Available patterns:$(NC)"; \
		$(MAKE) list; \
		exit 1; \
	fi; \
	pattern_name="$(filter-out $@,$(MAKECMDGOALS))"; \
	found_path=$$(find $(BIN_DIR) -name "$$pattern_name" -type f 2>/dev/null | head -1); \
	if [ -n "$$found_path" ]; then \
		pattern_display=$$(echo "$$found_path" | sed 's|$(BIN_DIR)/||' | sed 's|/[^/]*$$||'); \
		echo "$(BLUE)🚀 Running $$pattern_display example...$(NC)"; \
		$$found_path; \
	else \
		echo "$(YELLOW)❌ Example not found: $$pattern_name$(NC)"; \
		echo "$(BLUE)💡 Build first with: make$(NC)"; \
		echo "$(BLUE)💡 Available examples:$(NC)"; \
		$(MAKE) list; \
	fi

# Make ignore targets passed as arguments
%:
	@:

clean:
	@echo "$(YELLOW)🧹 Cleaning C++ build directory...$(NC)"
	@rm -rf $(BIN_DIR)
	@echo "$(GREEN)✅ C++ clean completed!$(NC)"

list:
	@echo "$(BLUE)📋 Available C++ examples:$(NC)"
	@find . -name "main.cpp" -path "*/src/*" | sed 's|^\./||' | sed 's|/src/main\.cpp$$||' | sort | while read pattern; do \
		echo "  📄 $$pattern"; \
	done

help:
	@echo "Design Patterns C++ Build System"
	@echo ""
	@echo "$(BLUE)Available targets:$(NC)"
	@echo "  all             Build all C++ examples (default)"
	@echo "  clean           Clean C++ build directory"
	@echo "  list            List all available examples"
	@echo "  run <pattern>   Run a specific example"
	@echo "  help            Show this help message"
	@echo ""
	@echo "$(BLUE)Examples:$(NC)"
	@echo "  make                    # Build all examples"
	@echo "  make run command        # Run command pattern"
	@echo "  make run builder        # Run builder pattern"
	@echo "  make run singleton      # Run singleton pattern"
	@echo "  make list               # List all examples"
	@echo "  make clean              # Clean C++ build files" 