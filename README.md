# Introduction
Small Crate to infer various media containers.
Works by reading Magic Bytes.

# Supported Containers
- [X] MKV
- [X] ASF
- [X] GXF
- [X] WTV
- [X] RCWT
- [X] MP4
- [X] TS
- [X] M2TS
- [X] PS
- [X] Tivo PS
- [X] MXF

# Examples
## Get Container type from starting bytes

``` rust
let buf = [0x1a, 0x45, 0xdf, 0xa3, 0, 1];
let kind = media_infer::ContainerType::from_bytes(&buf);

assert_eq!(kind, Ok(media_infer::ContainerType::MKV));
#+end_src

** Get Container type from path to file
#+begin_src rust
use std::path::PathBuf;

let file_path = PathBuf::from("some.abc");
let kind = media_infer::ContainerType::from_file_path(&file_path);
```
## Get Container type from open file

```rust
use std::fs::File;

let mut file = File::open("some.abc").unwrap();
let kind = media_infer::ContainerType::from_file(&mut file);
```

# Resources
- [Garykessler](https://www.garykessler.net/library/file_sigs.html)
- [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures)
