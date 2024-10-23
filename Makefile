rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run
		
python_install:
	pip install --upgrade pip && pip install -r requirements.txt

python_format:
	black *.py

python_lint:
	pylint --disable==R,C --ignore-patterns=test_.*?py *.py

python_test:
	python -m pytest -cov=main test_main.py

python_all: install python_format python_lint python_test