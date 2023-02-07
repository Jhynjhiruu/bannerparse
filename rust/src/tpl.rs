use binrw::BinRead;

macro_rules! min_of {
    ($e:expr, $a:expr) => {{
        assert_eq!($a & ($a - 1), 0);
        ($e + ($a - 1)) >> $a.ilog2()
    }};
}

#[binrw::binread]
#[derive(Debug)]
pub struct TplHeader {
    num_images: u32,
    image_tbl_offset: u32,
}

#[binrw::binread]
#[derive(Debug)]
pub struct TplImageTblEntry {
    image_hdr_offset: u32,
    palette_hdr_offset: u32,
}

#[binrw::binread]
#[derive(Debug)]
pub enum TplPaletteFormat {
    #[br(magic = 0u32)]
    IA8,
    #[br(magic = 1u32)]
    RGB565,
    #[br(magic = 2u32)]
    RGB5A3,
}

impl TplPaletteFormat {
    fn size(&self, count: u16) -> usize {
        let count: usize = count.into();
        count
            * match self {
                TplPaletteFormat::IA8 => 2,
                TplPaletteFormat::RGB565 => 2,
                TplPaletteFormat::RGB5A3 => 2,
            }
    }
}

#[binrw::binread]
#[derive(Debug)]
pub struct TplPaletteHeader {
    entry_count: u16,
    unpacked: u8,
    #[br(pad_before = 1)]
    palette_format: TplPaletteFormat,
    palette_data_address: u32,
}

#[binrw::binread]
#[derive(Debug)]
pub struct TplPalette {
    header: TplPaletteHeader,
    #[br(seek_before = std::io::SeekFrom::Start(header.palette_data_address.into()))]
    #[br(count = header.palette_format.size(header.entry_count))]
    palette_data: Vec<u8>,
}

fn parse_palettes<R: std::io::Read + std::io::Seek>(
    reader: &mut R,
    ro: &binrw::ReadOptions,
    (image_tbl,): (&Vec<TplImageTblEntry>,),
) -> binrw::BinResult<Vec<Option<TplPalette>>> {
    let mut rv = vec![];
    for img in image_tbl {
        rv.push(if img.palette_hdr_offset != 0 {
            reader.seek(std::io::SeekFrom::Start(img.palette_hdr_offset.into()))?;
            Some(<_>::read_options(reader, ro, ())?)
        } else {
            None
        });
    }
    Ok(rv)
}

#[binrw::binread]
#[derive(Debug)]
pub enum TplImageFormat {
    #[br(magic = 0u32)]
    I4,
    #[br(magic = 1u32)]
    I8,
    #[br(magic = 2u32)]
    IA4,
    #[br(magic = 3u32)]
    IA8,
    #[br(magic = 4u32)]
    RGB565,
    #[br(magic = 5u32)]
    RGB5A3,
    #[br(magic = 6u32)]
    RGBA32,

    #[br(magic = 8u32)]
    C4,
    #[br(magic = 9u32)]
    C8,
    #[br(magic = 10u32)]
    C14x2,

    #[br(magic = 14u32)]
    Cmpr,
}

impl TplImageFormat {
    fn size(&self, height: u16, width: u16) -> usize {
        let (height, width): (usize, usize) = (height.into(), width.into());
        let (block_height, block_width, block_size) = match self {
            TplImageFormat::I4 => (8, 8, 32),
            TplImageFormat::I8 => (4, 8, 32),
            TplImageFormat::IA4 => (4, 8, 32),
            TplImageFormat::IA8 => (4, 4, 32),
            TplImageFormat::RGB565 => (4, 4, 32),
            TplImageFormat::RGB5A3 => (4, 4, 32),
            TplImageFormat::RGBA32 => (4, 4, 64),
            TplImageFormat::C4 => (8, 8, 32),
            TplImageFormat::C8 => (4, 8, 32),
            TplImageFormat::C14x2 => (4, 4, 32),
            TplImageFormat::Cmpr => (8, 8, 32),
        };
        let (height, width) = (min_of!(height, block_height), min_of!(width, block_width));
        height * width * block_size
    }
}

#[binrw::binread]
#[derive(Debug)]
pub struct TplImageHeader {
    height: u16,
    width: u16,
    format: TplImageFormat,
    image_data_address: u32,
    wrapS: u32,
    wrapT: u32,
    min_filter: u32,
    mag_filter: u32,
    lod_bias: f32,
    edge_lod_enable: u8,
    min_lod: u8,
    max_lod: u8,
    unpacked: u8,
}

#[binrw::binread]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct TplImage {
    header: TplImageHeader,
    #[br(seek_before = std::io::SeekFrom::Start(header.image_data_address.into()))]
    #[br(count = header.format.size(header.height, header.width))]
    #[derivative(Debug = "ignore")]
    data: Vec<u8>,
}

fn parse_images<R: std::io::Read + std::io::Seek>(
    reader: &mut R,
    ro: &binrw::ReadOptions,
    (image_tbl,): (&Vec<TplImageTblEntry>,),
) -> binrw::BinResult<Vec<TplImage>> {
    let mut rv = vec![];
    for img in image_tbl {
        reader.seek(std::io::SeekFrom::Start(img.image_hdr_offset.into()))?;
        rv.push(<_>::read_options(reader, ro, ())?);
    }
    Ok(rv)
}

#[binrw::binread]
#[br(big)]
#[br(magic = 0x0020AF30u32)]
#[derive(Debug)]
pub struct Tpl {
    header: TplHeader,
    #[br(seek_before = std::io::SeekFrom::Start(header.image_tbl_offset.into()))]
    #[br(count = header.num_images)]
    image_tbl: Vec<TplImageTblEntry>,
    #[br(args(&image_tbl))]
    #[br(parse_with = parse_palettes)]
    palettes: Vec<Option<TplPalette>>,
    #[br(args(&image_tbl))]
    #[br(parse_with = parse_images)]
    images: Vec<TplImage>,
}

impl Tpl {
    pub fn parse<T: std::convert::AsRef<[u8]>>(
        cursor: &mut std::io::Cursor<T>,
    ) -> binrw::BinResult<Self> {
        binrw::BinReaderExt::read_type::<_>(cursor, binrw::endian::LE)
    }

    pub fn get_num_imgs(&self) -> u32 {
        self.header.num_images
    }

    pub fn get_image_dims(&self, idx: usize) -> Option<(u16, u16)> {
        self.images
            .get(idx)
            .map(|i| (i.header.width, i.header.height))
    }
}
