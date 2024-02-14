# Tauri Green Lantern

## Introduction

This is a menu bar application for MacOS, crafted with Tauri, this tool serves as a mouse jiggler for preventing your "status" turning yellow. The underlying logic is straightforward yet effective: it establishes a background thread to subtly shift the mouse by 1 pixel, returning it to its original position every 5 seconds.

## Pre-requisite

Please remember to add permission in System Setting -> Privacy & Security -> Accessibility -> (add button) -> select "Green Lantern" in application folder.

## How to build

```bash
$ cargo tauri build
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## References

- [Icons](https://favicon.io/)
