use media_infer::ContainerType;

#[test]
fn test_empty() {
    let t1 = ContainerType::from_bytes(&[]);
    assert!(t1.is_err());
}

#[test]
fn test_garbage() {
    let t = ContainerType::from_bytes(&[0; 1024 * 1024]);
    assert!(t.is_err());
}

#[test]
fn test_asf_bytes() {
    let buffer = [0x30, 0x26, 0xb2, 0x75, 0x34, 0];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::ASF));
}

#[test]
fn test_mkv_bytes() {
    let buffer = [0x1a, 0x45, 0xdf, 0xa3, 0, 1];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::MKV));

    let buffer = [0x18, 0x53, 0x80, 0x67, 10];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::MKV));
}

#[test]
fn test_gxf_bytes() {
    let buffer = [0, 0, 0, 0, 1, 0xbc, 9];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::GXF));
}

#[test]
fn test_wtv_bytes() {
    let buffer = [0xb7, 0xd8, 0x00, 0x20, 0];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::WTV));
}
#[test]
fn test_rcwt_bytes() {
    let buffer = [0xCC, 0xCC, 0xED, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::RCWT));
}

#[test]
fn test_mp4_bytes() {
    let buffer = [
        0, 0, 0, 32, 0x66, 0x74, 0x79, 0x70, 0x4D, 0x53, 0x4E, 0x56, 9, 34,
    ];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::MP4));

    let buffer = [
        0, 0, 0, 32, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x6D, 87,
    ];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::MP4));
}

#[test]
fn test_mxf_bytes() {
    let buffer = [
        0, 2, 0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x02,
        9, 3,
    ];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::MXF));
}

#[test]
fn test_ts_bytes() {
    let mut buffer = [0; 192 * 9];
    for i in 0..8 {
        buffer[2 + i * 188] = 0x47;
    }
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::TS));
}

#[test]
fn test_m2ts_bytes() {
    let mut buffer = [0; 192 * 9];
    for i in 0..8 {
        buffer[2 + 4 + i * 192] = 0x47;
    }
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::M2TS));
}

#[test]
fn test_ps_bytes() {
    let buffer = [0, 0, 0x00, 0x00, 0x01, 0xBA, 0, 0];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::PS));

    let mut buffer = [0; 50100];
    buffer[1000] = 0x01;
    buffer[1001] = 0xBA;
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::PS));
}

#[test]
fn test_tivo_ps() {
    let buffer = [b'T', b'i', b'V', b'o', 0];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::TivoPS));
}

#[test]
fn test_es_bytes() {
    let buffer = [0, 0, 1, 0xB3, 0, 0];
    let t = ContainerType::from_bytes(&buffer);
    assert_eq!(t, Ok(ContainerType::ES));
}
