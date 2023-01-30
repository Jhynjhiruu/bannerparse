use std::io::Read;

#[binrw::binread]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
#[br(magic = b"IMET")]
pub struct IMETHeader {
    hashsize: u32,
    unk: u32,
    sizes: [u32; 3],
    flag1: u32,
    #[br(pad_size_to(588 + 0x54 * 10))]
    #[br(map(|x: [[u16; 0x54 / 2]; 10]| x.iter().map(|e| String::from_utf16_lossy(e).trim_end_matches(char::from(0)).to_owned()).collect()))]
    pub names: Vec<String>,
    hash: [u8; 16],
}

#[binrw::binread]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
#[br(import(skip: usize))]
pub struct Banner {
    #[br(count = skip)]
    #[derivative(Debug = "ignore")]
    detritis: Vec<u8>,
    pub header: IMETHeader,
    #[br(parse_with = binrw::until_eof)]
    #[derivative(Debug = "ignore")]
    pub data: Vec<u8>,
}

#[binrw::binread]
#[br(big)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub enum _Banner {
    Skip40(#[br(args(0x40))] Banner),
    Skip80(#[br(args(0x80))] Banner),
}

impl _Banner {
    fn check_hash<T: std::convert::AsRef<[u8]>>(
        self,
        cursor: &mut std::io::Cursor<T>,
    ) -> binrw::BinResult<Banner> {
        let (banner, offset) = match self {
            _Banner::Skip40(b) => (b, 0x00),
            _Banner::Skip80(b) => (b, 0x40),
        };
        cursor.set_position(offset);
        let mut hashbuf = vec![0; banner.header.hashsize as usize];
        cursor.read_exact(&mut hashbuf)?;
        if banner.header.hashsize > 0x5F0 {
            hashbuf[0x5F0..0x600.min(banner.header.hashsize as usize)].fill(0);
        }
        let hash = md5::compute(hashbuf);
        if hash.0 == banner.header.hash {
            Ok(banner)
        } else {
            Err(binrw::Error::AssertFail {
                pos: offset,
                message: format!(
                    "Header hash doesn't match (got {:?}, expected {:?})",
                    md5::Digest(banner.header.hash),
                    hash
                ),
            })
        }
    }
}

impl Banner {
    pub fn parse<T: std::convert::AsRef<[u8]>>(
        cursor: &mut std::io::Cursor<T>,
    ) -> binrw::BinResult<Self> {
        match binrw::BinReaderExt::read_type::<_Banner>(cursor, binrw::endian::LE) {
            Ok(b) => b.check_hash(cursor),
            Err(e) => Err(e),
        }
    }
}
