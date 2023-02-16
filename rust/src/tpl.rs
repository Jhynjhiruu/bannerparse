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
#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
struct TplImageFormatInfo {
    block_width: usize,
    block_height: usize,
    block_size: usize,
    bpp: usize,
}

impl From<(usize, usize, usize, usize)> for TplImageFormatInfo {
    fn from(value: (usize, usize, usize, usize)) -> Self {
        Self {
            block_width: value.0,
            block_height: value.1,
            block_size: value.2,
            bpp: value.3,
        }
    }
}

impl TplImageFormat {
    fn get_info(&self) -> TplImageFormatInfo {
        match self {
            TplImageFormat::I4 => (8, 8, 32, 4),
            TplImageFormat::I8 => (8, 4, 32, 8),
            TplImageFormat::IA4 => (8, 4, 32, 8),
            TplImageFormat::IA8 => (4, 4, 32, 16),
            TplImageFormat::RGB565 => (4, 4, 32, 16),
            TplImageFormat::RGB5A3 => (4, 4, 32, 16),
            TplImageFormat::RGBA32 => (4, 4, 64, 32),
            TplImageFormat::C4 => (8, 8, 32, 4),
            TplImageFormat::C8 => (8, 4, 32, 8),
            TplImageFormat::C14x2 => (4, 4, 32, 16),
            TplImageFormat::Cmpr => (8, 8, 32, 4),
        }
        .into()
    }

    fn size(&self, height: u16, width: u16) -> usize {
        let (height, width): (usize, usize) = (height.into(), width.into());
        let TplImageFormatInfo {
            block_width,
            block_height,
            block_size,
            bpp,
        } = self.get_info();
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

    fn map_pixels(pixels: &[u8], format: &TplImageFormat) -> Vec<Vec<u8>> {
        if format == &TplImageFormat::RGBA32 {
            let (r, g, b, a) = pixels.iter().enumerate().fold(
                (vec![], vec![], vec![], vec![]),
                |(mut r, mut g, mut b, mut a), (idx, &e)| {
                    match idx {
                        x if x % 2 == 0 => match idx {
                            0..=31 => a.push(e),
                            _ => g.push(e),
                        },
                        _ => match idx {
                            0..=31 => r.push(e),
                            _ => b.push(e),
                        },
                    };
                    (r, g, b, a)
                },
            );

            itertools::izip!(r, g, b, a)
                .map(|e| vec![e.0, e.1, e.2, e.3])
                .collect()
        } else {
            let chunk_size = format.get_info().bpp;
            if chunk_size == 4 {
                pixels
                    .iter()
                    .map(|e| vec![vec![e >> 4], vec![e & 0x0F]])
                    .collect::<Vec<Vec<Vec<_>>>>()
                    .concat()
            } else {
                pixels.chunks(chunk_size / 8).map(|e| e.to_vec()).collect()
            }
        }
    }

    fn conv_pixel(pixel: &[u8], format: &TplImageFormat) -> [u8; 4] {
        match format {
            TplImageFormat::I4 => {
                let i = pixel[0] * 0x11;
                [i, i, i, 0xFF]
            }
            TplImageFormat::I8 => {
                let i = pixel[0];
                [i, i, i, 0xFF]
            }
            TplImageFormat::IA4 => todo!(),
            TplImageFormat::IA8 => {
                let i = pixel[1];
                let a = pixel[0];
                [i, i, i, a]
            }
            TplImageFormat::RGB565 => todo!(),
            TplImageFormat::RGB5A3 => todo!(),
            TplImageFormat::RGBA32 => [pixel[0], pixel[1], pixel[2], pixel[3]],
            TplImageFormat::C4 => todo!(),
            TplImageFormat::C8 => todo!(),
            TplImageFormat::C14x2 => todo!(),
            TplImageFormat::Cmpr => todo!(),
        }
    }

    pub fn get_as_rgba(&self, idx: usize) -> Option<Vec<u8>> {
        let (width, height) = self.get_image_dims(idx)?;
        println!("width {width} height {height}");
        let img = self.images.get(idx)?;
        let format_info = img.header.format.get_info();
        println!("{:?}", format_info);
        let width_in_blocks = min_of!(width as usize, format_info.block_width);

        let blocks = img.data.chunks(format_info.block_size).collect::<Vec<_>>();
        let mut pixels =
            std::vec::Vec::with_capacity(width as usize * height as usize * format_info.bpp / 8);

        for y in 0..height as usize {
            for xblock in 0..width_in_blocks {
                let block_idx = (y % format_info.block_height) * format_info.block_width;
                pixels.extend_from_slice(
                    &Self::map_pixels(
                        blocks.get(xblock + (y / format_info.block_height) * width_in_blocks)?,
                        &img.header.format,
                    )[block_idx
                        ..block_idx
                            + (width as usize - xblock * format_info.block_width)
                                .min(format_info.block_width)],
                );
            }
        }

        let rv = pixels
            .iter()
            .map(|e| Self::conv_pixel(e, &img.header.format))
            .collect::<Vec<_>>()
            .concat();

        Some(rv)
    }
}
