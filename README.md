# NARC BTNF Fixer (by hushrom)

**License:** LGPL v2.1

## Overview
`narc-btnf-fixer` is a Rust-based tool that fixes BTNF issues from mmodel.narc (DPPt) or 1.narc (HGSS) folder repacked by **Kiwi.ds**, ensuring compatibility with **Tinke**.

Kiwi.ds can extract/unpack and repack `.narc` files into folders, making it convenient for ROM hackers to modify **BTX** overworld sprites from the mmodel.narc. However, it slightly alters the **BTNF** section of the NARC, causing Tinke to misread the file. While this doesn't affect gameplay, it can be problematic for hacking workflows.

This tool **automatically patches** the affected bytes to restore 100% compatibility with Tinke.

## Usage

### **How to Use**
1. **Import a `mmodel.narc` file** (or `1.narc`).
2. Click **"Fix and Export"**.
3. A fixed version of the NARC file will be saved with `_fixed.narc` appended to the filename.

### **What This Fixes**
- Modifies **bytes at offset `0x1B1C - 0x1B23`**.
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
