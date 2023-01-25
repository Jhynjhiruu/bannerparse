mod u8;

#[binrw::binread]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct IMETHeader {
    #[br(magic = b"IMET")]
    hashsize: u32,
    unk: u32,
    sizes: [u32; 3],
    flag1: u32,
    #[br(pad_size_to(588 + 0x54 * 10))]
    #[br(map(|x: [[u16; 0x54 / 2]; 10]| x.iter().map(|e| String::from_utf16_lossy(e).trim_end_matches(char::from(0)).to_owned()).collect()))]
    names: Vec<String>,
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
    header: IMETHeader,
    #[derivative(Debug = "ignore")]
    pub content: u8::U8Archive,
}

#[binrw::binread]
#[br(big)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub enum _Banner {
    Skip40(#[br(args(0x40))] Banner),
    Skip80(#[br(args(0x80))] Banner),
}

impl From<_Banner> for Banner {
    fn from(value: _Banner) -> Self {
        match value {
            _Banner::Skip40(b) => b,
            _Banner::Skip80(b) => b,
        }
    }
}

impl Banner {
    pub fn parse<T: std::convert::AsRef<[u8]>>(
        cursor: &mut std::io::Cursor<T>,
    ) -> Result<Self, binrw::Error> {
        match binrw::BinReaderExt::read_type::<_Banner>(cursor, binrw::endian::LE) {
            Ok(b) => Ok(b.into()),
            Err(e) => Err(e),
        }
    }
}
