# portkill

A CLI utility to terminate processes listening on a TCP port.

`portkill` finds and stops a process that is holding onto a port without needing to manually inspect process lists or copy PIDs.

---

## Why?

When a port is already in use you usually need to look up the process and kill it manually. This kills it dead with no legwork.

`portkill` reduces that to a single command

```bash
portkill 3000
```

---

## Installation

### Prebuilt binaries

Prebuilt binaries are on github releases page.

Download the binary for your platform and place it in your `$PATH`.

Example (macOS):

```bash
curl -L https://github.com/bobbysmith/portkill/releases/latest/download/portkill-macos -o portkill
chmod +x portkill
mv portkill /usr/local/bin/
```

---

### From source (Cargo)

```bash
cargo install --path . --force
```

This installs `portkill` into `~/.cargo/bin`.
Make sure that `~/.cargo/bin` is included in your `$PATH`.

---

## Usage

```bash
portkill <port>
```

### Examples

```bash
$ portkill 3000
killed port 3000 (node at pid 48291)
```

```bash
$ portkill 8000
nothing running on port 8000
```

```bash
$ portkill 80
found nginx on port 80 (pid 123) but could not kill it
```

---

## License

MIT
