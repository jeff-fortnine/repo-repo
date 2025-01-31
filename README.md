# repo-repo

`repo-repo` is a simple command line program for quickly running maintenance on Git repositories.

# Installation

### Requirements

- Rust v1.84.0 or higher

### To Install

From the root directory of this repository run the following command.

```bash
cargo install --path .
```

Note that the directory to which Cargo crates are installed (`${HOME}/.cargo/bin/`) for example, must be on your system path.

# Usage

There are two ways to run `repo-repo`.

1. Provide a comma-separated list of relative paths to the Git repositories to clean.
   ```bash
   repo-repo -r <repo1>,<repo2>,...,<repoN>
   ```
2. Specify a relative path to a text file that contains paths to the Git repositories to clean (one per line).
   ```bash
   repo-repo -f <file.txt>
   ```