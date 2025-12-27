# ZING

![Zing Interface](assets/interface.png)

## What is this?

Zing is a TUI (Text User Interface) designed for the execution phase of string art.

The name comes from two places:

1. The acronym: **Zing Is Not Generating**.
2. The sound a thread makes when it snaps `/zing/`.

It does not calculate algorithms. It does not turn photographs into nail patterns. It simply holds the map while you drive.

![String Art Context](assets/example.png)

## Why use it?

When you are doing string art, your hands and your mind are busy. You cannot be struggling with a complex GUI.

Zing takes a comma-separated sequence of numbers (the nail numbers) and displays them one by one on a terminal using big, readable text.

It also previews the previous and next steps so you can maintain flow.

## Installation

You need [Rust](https://rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed.

To install Zing globally on your system, run this inside the project folder:

```bash
cargo install --path .
```

Now the command zing is available from any directory in your terminal.

## Usage

Feed it a sequence.

You can feed it any plain text file. The extension does not matter, but the structure does.

```bash
zing pattern.txt
```

### Important Data Rules

1. **Single Line Input:** Your sequence of numbers must be entirely on the **first line** (separated by commas) of the file.
2. **File Modification:** Zing will write to the **second line** to save your current progress.

### Example file

```plaintext
0,105,238,104,239,106,197,99,194,54,167,47,166,46,165,45,164
```

## Controls

- `SPACE` / `RIGHT` : Next step.
- `BACKSPACE` / `LEFT` : Go back.
- `S` : Save current position.
- `q` / `ESCAPE` : Quit.

> [!IMPORTANT]
> Zing attempts to auto-save your progress when you quit.<br>If saving fails, it will warn you before closing.

## The Philosophy

Generators are for planning. Zing is for making.

The software stays out of your way. No distractions. Just the number you need to hit next.

## License

Created under the MIT License. See [LICENSE](https://github.com/000Volk000/zing/blob/main/LICENSE) for more information.

Created by [Darío Martínez Kostyuk](https://linktree.volkhost.es/) - 2025
