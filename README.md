# maturin-gettext-viewer

## Pre-requires

- gettext
  - include/gettext-po.h
  - lib/libgettextlib.so

## Usage

```bash
$ cargo build && pdm install && pdm run python sample.py
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
All packages are synced to date, nothing to do.
Installing the project as an editable package...
  ✔ Update maturin-gettext-viewer 0.1.0 -> 0.1.0 successful

🎉 All complete!

msgid:

msgstr:
Project-Id-Version: PACKAGE VERSION
POT-Creation-Date: 2024-01-27 17:27+0900
PO-Revision-Date: 2024-01-27 16:20+0900
Last-Translator: Automatically generated
Language-Team: none
Language: ja
MIME-Version: 1.0
Content-Type: text/plain; charset=UTF-8
Content-Transfer-Encoding: 8bit
Plural-Forms: nplurals=1; plural=0;
```
