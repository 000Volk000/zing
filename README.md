# ZING

## What is this?

Zing is a TUI (Text User Interface) designed for the execution phase of string art.

The name comes from two places:
1. The acronym: **Zing Is Not Generating**.
2. The sound a thread makes when it snaps `/zing/`.

It does not calculate algorithms. It does not turn photographs into nail patterns. It simply holds the map while you drive.

## Why use it?

When you are doing string art, your hands and your mind are busy. You cannot be struggling with a complex GUI.

Zing takes a comma-separated sequence of numbers (the nail numbers) and displays them one by one on a high-contrast terminal.

## Usage

Feed it a sequence. 

You can feed it any plain text file. The extension does not matter, but the structure does.

```bash
zing pattern.txt
```

### Important Data Rules

1.  **Single Line Input:** Your sequence of numbers must be entirely on the **first line** (separated by commas) of the file.
2.  **File Modification:** Zing will write to the **second line** to save your current progress.

## Controls

* `SPACE` / `RIGHT` : Next step.
* `BACKSPACE` / `LEFT` : Go back.
* `q` / `ESCAPE` : Quit.

## The Philosophy

Generators are for planning. Zing is for making.

The software stays out of your way. No distractions. Just the number you need to hit next.

## License

Created under the MIT License. See [LICENSE](https://github.com/000Volk000/zing/blob/main/LICENSE) for more information.

Created by [Darío Martínez Kostyuk](https://linktree.volkhost.es/) - 2025
