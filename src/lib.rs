use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ContainerType {
    MP4,
    MKV,
    ASF,
    GXF,
    WTV,
    RCWT,
}

impl ContainerType {
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

    pub fn from_file(file: &mut File) -> Result<Self, String> {
        const START_BYTES_LENGTH: usize = 1024 * 1024;

        let mut buffer: [u8; START_BYTES_LENGTH] = [0; START_BYTES_LENGTH];
        if file.read(&mut buffer).is_err() {
            return Err("Error in reading File".to_string());
        }

        Self::from_bytes(&buffer)
    }

    pub fn from_file_path(path: &PathBuf) -> Result<Self, String> {
        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(_) => return Err("Error in Opening File".to_string()),
        };
        Self::from_file(&mut file)
    }

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
