# lr_grustep

Example project: grep-like CLI app implemented in Rust.

## Install

```bash
make install
```

## Usage

```txt
```

### Search

```bash
grustep pattern *txt

cat *py | grustep pattern
```

### Recursive Search

```bash
grustep -r pattern .
```

### Search Multiple Files

```bash
grustep pattern a.txt b.py c.cpp
```

### Show Line Numbers

```bash
grustep -n pattern *txt
```

See [project tutorial](https://literank.com/project/12/intro) here.
