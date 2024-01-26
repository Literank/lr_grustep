# lr_grustep

Example project: grep-like CLI app implemented in Rust.

## Install

```bash
make install
```

## Usage

```txt
grustep 0.1.0
literank.com
A grep-like utility in Rust

USAGE:
    grustep [FLAGS] <pattern> [file_paths]...

FLAGS:
    -c, --count           Only a count of selected lines is written to standard output
    -h, --help            Prints help information
    -i, --ignore-case     Perform case-insensitive matching
    -v, --invert-match    Selected lines are those not matching any of the specified patterns
    -n, --line-number     Each output line is preceded by its relative line number in the file, starting at line 1
    -r, --recursive       Recursively search subdirectories listed
    -V, --version         Prints version information

ARGS:
    <pattern>          The pattern to search for
    <file_paths>...    The files to search in
```

### Search

```bash
grustep pattern *txt

cat *rs | grustep pattern
```

### Recursive Search

```bash
grustep -r pattern .
```

### Search Multiple Files

```bash
grustep pattern a.txt b.rs c.cpp
```

### Show Line Numbers

```bash
grustep -n pattern *txt
```

See [project tutorial](https://literank.com/project/12/intro) here.
