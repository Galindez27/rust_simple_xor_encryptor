# Simple XOR Encryption with Rust!
## Author

Jonathan Galindez

## Summary
This is a simple demonstration of xor encrytion done in rust done, done
for BU CS 357 Info Security class as a demonstration.

## Usage
```cargo run [file to encrypt] [out filename] [key string]```

Example:
```cargo run test.text test.txt.enc key```

Results in the file ```test.txt.enc``` being made as the xor encrypted version of ```test.txt``` with ```"key"``` as the key.

## Requirements

Cargo >= 1.55

Rustc >= 1.55

No extrenal dependancies.