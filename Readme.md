## **rhsync-diff-tool (rolling hash sync diff tool)** 
### *Rolling Hash based file diff tool coded in Rust*
***
***
#### Requirements:
    - rustc 1.63.0 stable
    - linux or macos preferably, may also run on windows

#### Compilation:
```
    make
```
if no make utility:
```
    cargo build --release
```

#### Run tests:
```
    cargo test
```

#### Usage:
```
    ./target/release/rhsync-diff-tool <file_1_path> <file_2_path> <optional chunk_size>
```

##### Default chunk size is 4 if none provided

#### Examples:
```
    ./target/release/rhsync-diff-tool abc.txt def.txt
    ./target/release/rhsync-diff-tool some.txt other.txt 4
    ./target/release/rhsync-diff-tool some.bin other.bin 7
```
***
***
##### MIT License