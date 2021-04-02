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
use std::fmt;
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
    MP4,
    TS,
    PS,
    MXF,
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
        } else if Self::check_mp4(&buffer) {
            return Ok(ContainerType::MP4);
        } else if Self::check_mxf(&buffer) {
            return Ok(ContainerType::MXF);
        } else if Self::check_ts(&buffer) {
            return Ok(ContainerType::TS);
        } else if Self::check_ps(&buffer) {
            return Ok(ContainerType::PS);
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
            return ASF_MAGIC_BYTES == buffer[0..ASF_MAGIC_BYTES.len()];
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
            let buf = &buffer[0..MAGIC_BYTES_LEN];
            return MKV_EMBL_MAGIC_BYTES == buf || MKV_SEGMENT_MAGIC_BYTES == buf;
        }
        false
    }

    /// Checks for GXF Magic bytes.
    /// Min Size of buffer is 6 bytes.
    fn check_gxf(buffer: &[u8]) -> bool {
        const GXF_MAGIC_BYTES: [u8; 6] = [0, 0, 0, 0, 1, 0xbc];

        if buffer.len() >= GXF_MAGIC_BYTES.len() {
            return GXF_MAGIC_BYTES == buffer[0..GXF_MAGIC_BYTES.len()];
        }

        false
    }

    /// Checks for WTV Magic Bytes.
    /// Min Size of buffer is 4 bytes.
    fn check_wtv(buffer: &[u8]) -> bool {
        const WTV_MAGIC_BYTES: [u8; 4] = [0xb7, 0xd8, 0x00, 0x20];

        if buffer.len() >= WTV_MAGIC_BYTES.len() {
            return WTV_MAGIC_BYTES == buffer[0..WTV_MAGIC_BYTES.len()];
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

    /// Checks for MP4 magic bytes.
    /// [Magic Bytes List](https://www.garykessler.net/library/file_sigs.html)
    fn check_mp4(buffer: &[u8]) -> bool {
        const MIN_LEN: usize = 12;
        const MP4_MAGIC_BYTES_1: [u8; 8] = [0x66, 0x74, 0x79, 0x70, 0x4D, 0x53, 0x4E, 0x56];
        const MP4_MAGIC_BYTES_2: [u8; 8] = [0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x6D];

        if buffer.len() >= MIN_LEN {
            let buffer = &buffer[4..MIN_LEN];
            return MP4_MAGIC_BYTES_1 == buffer || MP4_MAGIC_BYTES_2 == buffer;
        }
        false
    }

    /// Checks for Material Exchange Format
    fn check_mxf(buffer: &[u8]) -> bool {
        const BYTES_LEN: usize = 14;
        const MXF_MAGIC_BYTES: [u8; BYTES_LEN] = [
            0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x02,
        ];

        if buffer.len() >= BYTES_LEN {
            return (0..(buffer.len() - BYTES_LEN))
                .any(|x| MXF_MAGIC_BYTES == buffer[x..(x + BYTES_LEN)]);
        }

        false
    }

    /// Checks for Transport stream.
    /// Currently checks for 8 sync bytes.
    fn check_ts(buffer: &[u8]) -> bool {
        const TS_MAGIC_POS: usize = 188;
        const M2TS_MAGIC_POS: usize = 192;
        const MAGIC_BYTE: u8 = 0x47;
        const SYNC_BYTES_TO_CHECK: usize = 8;

        // Check for TS
        if buffer.len() > TS_MAGIC_POS * SYNC_BYTES_TO_CHECK
            && (0..TS_MAGIC_POS).any(|x| {
                (0..SYNC_BYTES_TO_CHECK).all(|y| buffer[x + y * TS_MAGIC_POS] == MAGIC_BYTE)
            })
        {
            return true;
        }

        // Check for M2TS
        if buffer.len() > M2TS_MAGIC_POS * SYNC_BYTES_TO_CHECK + 4
            && (0..M2TS_MAGIC_POS).any(|x| {
                (0..SYNC_BYTES_TO_CHECK).all(|y| buffer[x + 4 + M2TS_MAGIC_POS * y] == MAGIC_BYTE)
            })
        {
            return true;
        }

        false
    }

    /// Checks for PS (Needs PACK header)
    fn check_ps(buffer: &[u8]) -> bool {
        const MAGIC_NUMBER: usize = 50000;
        const PS_MAGIC_BYTES: [u8; 4] = [0x00, 0x00, 0x01, 0xBA];

        let len = buffer.len();

        if len >= PS_MAGIC_BYTES.len() {
            let limit = if len < MAGIC_NUMBER {
                len - 3
            } else {
                MAGIC_NUMBER - 3
            };
            return (0..limit).any(|x| PS_MAGIC_BYTES == buffer[x..(x + PS_MAGIC_BYTES.len())]);
        }

        false
    }
}

impl fmt::Display for ContainerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            ContainerType::MKV => "MKV",
            ContainerType::ASF => "ASF",
            ContainerType::GXF => "GXF",
            ContainerType::WTV => "WTV",
            ContainerType::RCWT => "RCWT",
            ContainerType::MP4 => "MP4",
            ContainerType::TS => "TS",
            ContainerType::PS => "PS",
            ContainerType::MXF => "MXF",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn mp4() {
        let t1 = ContainerType::check_mp4(&[
            0, 0, 0, 32, 0x66, 0x74, 0x79, 0x70, 0x4D, 0x53, 0x4E, 0x56, 9, 34,
        ]);
        assert!(t1);
        let t2 = ContainerType::check_mp4(&[
            0, 0, 0, 32, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x6D, 87,
        ]);
        assert!(t2);
    }

    #[test]
    fn mxf() {
        let t = ContainerType::check_mxf(&[
            0, 2, 0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01,
            0x02, 9, 3,
        ]);
        assert!(t);
    }

    #[test]
    fn ts() {
        let mut buffer = [0; 192 * 9];
        for i in 0..8 {
            buffer[2 + i * 188] = 0x47;
        }
        let t = ContainerType::check_ts(&buffer);
        assert!(t);

        let mut buffer = [0; 192 * 9];
        for i in 0..8 {
            buffer[2 + 4 + i * 192] = 0x47;
        }
        let t = ContainerType::check_ts(&buffer);
        assert!(t);
    }

    #[test]
    fn ps() {
        let t = ContainerType::check_ps(&[0, 0, 0x00, 0x00, 0x01, 0xBA, 0, 0]);
        assert!(t);

        let mut buffer = [0; 50100];
        buffer[1000] = 0x01;
        buffer[1001] = 0xBA;
        let t = ContainerType::check_ps(&buffer);
        assert!(t);
    }
}
