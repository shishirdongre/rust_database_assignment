# Define the clean target to remove any previous build artifacts
clean:
	rm -f dbfile.bin *.tab
	cargo clean


# Define the build target to compile the project
build: clean
	cargo build

build-release: clean
	cargo build --release


# Define the setup target, which depends on build
setup:
	cargo run -- "CREATE TABLE employees3 (id INT, first_name STRING(100) NOT NULL);"
	cargo run -- "INSERT INTO employees3 VALUES (1, 'John Doe');"
	cargo run -- "INSERT INTO employees3 VALUES (2, 'Jane Smith');"
	cargo run -- "INSERT INTO employees3 VALUES (3, 'Alice Johnson');"
	cargo run -- "SELECT * FROM employees3;"
