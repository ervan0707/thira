# Example Scripts

This directory contains example scripts for testing Thira's parallel script execution functionality.

## Scripts

### test1.sh

A simple test script that demonstrates:

- Environment variable usage (`TEST_MODE`, `TEST_VALUE`)
- Working directory display
- Simulated work with progress output
- Multi-step execution with delays

### test2.sh

Another test script for parallel execution testing.

## Usage

These scripts are designed to be run in parallel using Thira's script management feature. You can configure them in your `hooks.yaml` file like this:

```yaml
scripts:
  test-all:
    parallel: true
    max_threads: 2
    commands:
      - command: "sh test1.sh"
        description: "Run test script 1"
        working_dir: "examples"
        env:
          TEST_VALUE: "123"
          TEST_MODE: "parallel-1"
      - command: "sh test2.sh"
        description: "Run test script 2"
        working_dir: "examples"
        env:
          TEST_VALUE: "456"
          TEST_MODE: "parallel-2"
```

Then run them with:

```sh
thira scripts run test-parallel
```

## Purpose

These scripts help demonstrate and test:

- Parallel script execution
- Environment variable passing
- Real-time output display
- Progress indication during long-running tasks
- Multi-threaded script management
