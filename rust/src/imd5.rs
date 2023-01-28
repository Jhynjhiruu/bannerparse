use binrw::BinRead;

fn check_hash<R: std::io::Read + std::io::Seek>(
    reader: &mut R,
    _: &binrw::ReadOptions,
    (count, hash): (usize, &[u8; 16]),
) -> binrw::BinResult<Vec<u8>> {
    let pos = reader.stream_position()?;
    let data = <_>::read_args(reader, binrw::VecArgs { count, inner: () })?;

    let calc_hash = md5::compute(&data);
    if &calc_hash.0 == hash {
        Ok(data)
    } else {
        Err(binrw::Error::AssertFail {
            pos,
            message: format!(
                "IMD5 hash doesn't match (got {:?}, expected {calc_hash:?})",
                md5::Digest(*hash)
            ),
        })
    }
}

#[binrw::binread]
#[br(magic = b"IMD5")]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct IMD5 {
    filesize: u32,
    hash: [u8; 16],
    #[br(args(filesize as usize, &hash))]
    #[br(parse_with = check_hash)]
    #[derivative(Debug = "ignore")]
    data: Vec<u8>,
}

impl IMD5 {
    pub fn parse<T: std::convert::AsRef<[u8]>>(
        cursor: &mut std::io::Cursor<T>,
    ) -> binrw::BinResult<Self> {
        binrw::BinReaderExt::read_type::<_>(cursor, binrw::endian::LE)
    }
}
