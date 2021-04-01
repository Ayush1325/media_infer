/*!
Small Crate to infer various media containers.
Designed for use with CCExtractor.

# Examples
## Get Container type from starting bytes
```rust
let buf = [0x1a, 0x45, 0xdf, 0xa3, 0, 1];
let kind = media_infer::ContainerType::from_bytes(&buf);

assert_eq!(kind, Ok(media_infer::ContainerType::MKV));
```

## Get Container type from path to file
```ignore
use std::path::PathBuf;

let file_path = PathBuf::from("some.abc");
let kind = media_infer::ContainerType::from_file_path(&file_path);
```

## Get Container type from open file
```ignore
use std::fs::File;

let mut file = File::open("some.abc").unwrap();
let kind = media_infer::ContainerType::from_file(&mut file);
```
 */
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Enum of the vairous Container Types.
/// Does not contain Unknown. Methods throw error if container cannot be identified.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ContainerType {
    MKV,
    ASF,
    GXF,
    WTV,
    RCWT,
}

impl ContainerType {
    /// Function to infer Container from a slice of bytes.
    /// Throws Error if identification fails.
    pub fn from_bytes(buffer: &[u8]) -> Result<Self, String> {
        if Self::check_asf(buffer) {
            return Ok(ContainerType::ASF);
        } else if Self::check_mkv(buffer) {
            return Ok(ContainerType::MKV);
        } else if Self::check_gxf(buffer) {
            return Ok(ContainerType::GXF);
        } else if Self::check_wtv(buffer) {
            return Ok(ContainerType::WTV);
        } else if Self::check_rcwt(buffer) {
            return Ok(ContainerType::RCWT);
        }

        Err("Could Not Identify".to_string())
    }

    /// Function to infer Container from file.
    /// Reads the starting bytes from an open file.
    /// Throws IO error + error in indentification failure
    pub fn from_file(file: &mut File) -> Result<Self, String> {
        const START_BYTES_LENGTH: usize = 1024 * 1024;

        let mut buffer: [u8; START_BYTES_LENGTH] = [0; START_BYTES_LENGTH];
        if file.read(&mut buffer).is_err() {
            return Err("Error in reading File".to_string());
        }

        Self::from_bytes(&buffer)
    }

    /// Function to infer Container from file.
    /// Takes path of file and opens it itself.
    /// Throws error in IO failure + identification failure.
    pub fn from_file_path(path: &Path) -> Result<Self, String> {
        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(_) => return Err("Error in Opening File".to_string()),
        };
        Self::from_file(&mut file)
    }

    /// Checks for ASF magic bytes
    /// Min size of buffer is 4 bytes.
    fn check_asf(buffer: &[u8]) -> bool {
        const ASF_MAGIC_BYTES: [u8; 4] = [0x30, 0x26, 0xb2, 0x75];

        if buffer.len() >= ASF_MAGIC_BYTES.len() {
            return ASF_MAGIC_BYTES
                .iter()
                .zip(buffer.iter())
                .all(|x| x.0 == x.1);
        }
        false
    }

    /// Checks for MKV Magic bytes.
    /// Contains two tests. One for EMBL bytes and other for segment bytes.
    /// Min Size of buffer is 4 bytes.
    fn check_mkv(buffer: &[u8]) -> bool {
        const MAGIC_BYTES_LEN: usize = 4;
        const MKV_EMBL_MAGIC_BYTES: [u8; MAGIC_BYTES_LEN] = [0x1a, 0x45, 0xdf, 0xa3];
        const MKV_SEGMENT_MAGIC_BYTES: [u8; MAGIC_BYTES_LEN] = [0x18, 0x53, 0x80, 0x67];

        if buffer.len() >= MAGIC_BYTES_LEN {
            return MKV_EMBL_MAGIC_BYTES
                .iter()
                .zip(buffer.iter())
                .all(|x| x.0 == x.1)
                || MKV_SEGMENT_MAGIC_BYTES
                    .iter()
                    .zip(buffer.iter())
                    .all(|x| x.0 == x.1);
        }
        false
    }

    /// Checks for GXF Magic bytes.
    /// Min Size of buffer is 6 bytes.
    fn check_gxf(buffer: &[u8]) -> bool {
        const GXF_MAGIC_BYTES: [u8; 6] = [0, 0, 0, 0, 1, 0xbc];

        if buffer.len() >= GXF_MAGIC_BYTES.len() {
            return GXF_MAGIC_BYTES
                .iter()
                .zip(buffer.iter())
                .all(|x| x.0 == x.1);
        }

        false
    }

    /// Checks for WTV Magic Bytes.
    /// Min Size of buffer is 4 bytes.
    fn check_wtv(buffer: &[u8]) -> bool {
        const WTV_MAGIC_BYTES: [u8; 4] = [0xb7, 0xd8, 0x00, 0x20];

        if buffer.len() >= WTV_MAGIC_BYTES.len() {
            return WTV_MAGIC_BYTES
                .iter()
                .zip(buffer.iter())
                .all(|x| x.0 == x.1);
        }
        false
    }

    /// Checks for CCExtractor Magic Bytes.
    /// Min Size of buffer is 11 bytes.
    fn check_rcwt(buffer: &[u8]) -> bool {
        const MIN_LEN: usize = 11;
        const RCWT_MAGIC_BYTES: [(usize, u8); 6] =
            [(0, 0xCC), (1, 0xCC), (2, 0xED), (8, 0), (9, 0), (10, 0)];

        if buffer.len() >= MIN_LEN {
            return RCWT_MAGIC_BYTES.iter().all(|x| buffer[x.0] == x.1);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let t1 = ContainerType::from_bytes(&[]);
        assert!(t1.is_err());
    }

    #[test]
    fn garbage() {
        let t = ContainerType::from_bytes(&[0; 1024 * 1024]);
        assert!(t.is_err());
    }

    #[test]
    fn asf() {
        let t = ContainerType::check_asf(&[0x30, 0x26, 0xb2, 0x75, 0x34, 0]);
        assert!(t);
    }

    #[test]
    fn mkv() {
        let t1 = ContainerType::check_mkv(&[0x1a, 0x45, 0xdf, 0xa3, 0, 1]);
        assert!(t1);
        let t2 = ContainerType::check_mkv(&[0x18, 0x53, 0x80, 0x67, 10]);
        assert!(t2);
    }

    #[test]
    fn gxf() {
        let t = ContainerType::check_gxf(&[0, 0, 0, 0, 1, 0xbc, 9]);
        assert!(t);
    }

    #[test]
    fn wtv() {
        let t = ContainerType::check_wtv(&[0xb7, 0xd8, 0x00, 0x20, 0]);
        assert!(t);
    }

    #[test]
    fn rcwt() {
        let t = ContainerType::check_rcwt(&[0xCC, 0xCC, 0xED, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(t);
    }
}
