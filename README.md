# NARC BTNF Fixer (by hushrom)

**Version:** 0.1  
**License:** LGPL v3

## Overview
`narc-btnf-fixer` is a Rust-based tool that fixes BTNF issues in NARC files repacked by **Kiwi.ds**, ensuring compatibility with **Tinke**.

Kiwi.ds can unpack and repack `.narc` files into folders, making it convenient for ROM hackers to modify **BTX** overworld sprites. However, it slightly alters the **BTNF** section of the NARC, causing Tinke to misread the file. While this doesn't affect gameplay, it can be problematic for hacking workflows.

This tool **automatically patches** the affected bytes to restore 100% compatibility with Tinke.

## Usage

### **How to Use**
1. **Import a `.narc` file** (usually `mmodel.narc`).
2. Click **"Fix and Export"**.
3. A fixed version of the NARC file will be saved with `_fixed.narc` appended to the filename.

### **What This Fixes**
- Modifies **bytes at offset `0x1B1C - 0x1B22`**.
- Does **not** affect gameplay, only restores compatibility with Tinke.

## Installation

### **Precompiled Binary**
You can download the latest release from the [Releases](https://github.com/YOUR_USERNAME/narc-btnf-fixer/releases) page.

### **Building from Source**
Requires **Rust** installed. To build:

```sh
git clone https://github.com/YOUR_USERNAME/narc-btnf-fixer.git
cd narc-btnf-fixer
cargo build --release
