use binrw::BinRead;

#[binrw::binread]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
#[br(magic(0x55AA382Du32))]
pub struct U8Header {
    rootnode_offset: u32,
    header_size: u32,
    data_offset: u32,
}

#[binrw::binread]
#[derive(derivative::Derivative)]
#[derivative(Debug, PartialEq)]
pub enum U8NodeType {
    #[br(magic = 0u8)]
    File,
    #[br(magic = 1u8)]
    Directory,
}

const U8_NODE_SIZE: usize = 0x0C;

#[binrw::binread]
pub struct U8Node {
    node_type: U8NodeType,
    #[br(map(|x: (u8, u16)| (<u8 as Into<u32>>::into(x.0) << 16) | <u16 as Into<u32>>::into(x.1)))]
    name_offset: u32,
    data_offset: u32,
    size: u32,
}

// impl U8Node {
//     fn print(&self, f: &mut std::fmt::Formatter<'_>, ) -> std::fmt::Result {
//         f.debug_struct("U8Node")
//             .field("node_type", &self.node_type)
//             .field("name", &self.name_offset)
//             .field("size", &self.size)
//             .finish()
//     }
// }

// fn print_nodes(nodes: &Vec<U8Node>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     let nodes = nodes.iter().map(|e| )
// }

fn read_nodes<R: std::io::Read + std::io::Seek>(
    reader: &mut R,
    ro: &binrw::ReadOptions,
    _: (),
) -> binrw::BinResult<Vec<U8Node>> {
    let root_node = <U8Node>::read_options(reader, ro, ())?;
    if root_node.node_type != U8NodeType::Directory {
        return Err(binrw::Error::AssertFail {
            pos: reader.stream_position()?,
            message: "Root node is not a directory".to_string(),
        });
    }

    let root_size = root_node.size;

    let mut rv = vec![root_node];

    for _ in 1..root_size {
        rv.push(<_>::read_options(reader, ro, ())?);
    }

    Ok(rv)
}

#[binrw::binread]
pub struct U8Archive {
    #[br(restore_position)]
    header: U8Header,
    #[br(seek_before = std::io::SeekFrom::Current(header.rootnode_offset.into()))]
    #[br(parse_with = read_nodes)]
    nodes: Vec<U8Node>,
    #[br(count = header.header_size - (nodes.len() * U8_NODE_SIZE) as u32)]
    string_data: Vec<u8>,
    #[br(parse_with = binrw::until_eof)]
    #[br(pad_before = header.data_offset - (header.header_size + header.rootnode_offset))]
    data: Vec<u8>,
}

impl std::fmt::Debug for U8Archive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "U8Archive {{\n\theader: {:?}\n\tnodes: {}}}",
            self.header,
            match self.format_nodes() {
                Ok(s) => s,
                Err(_) => return Err(std::fmt::Error),
            }
        )
    }
}

pub enum U8Tree {
    File(String),
    Directory(String, Vec<U8Tree>),
}

impl std::fmt::Debug for U8Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(arg0) => f.debug_tuple("File").field(arg0).finish(),
            Self::Directory(arg0, arg1) => {
                f.debug_tuple("Directory").field(arg0).field(arg1).finish()
            }
        }
    }
}

impl U8Archive {
    fn get_string(&self, offset: u32) -> binrw::BinResult<String> {
        let mut reader = std::io::Cursor::new(&self.string_data);
        reader.set_position(offset.into());
        Ok(
            binrw::BinReaderExt::read_type::<binrw::NullString>(
                &mut reader,
                binrw::Endian::Little,
            )?
            .to_string(),
        )
    }

    fn _build_tree(&self, start: &mut u32) -> binrw::BinResult<U8Tree> {
        let root: &U8Node = self.nodes.get(*start as usize).unwrap();
        *start += 1;
        if root.node_type == U8NodeType::File {
            return Err(binrw::Error::AssertFail {
                pos: (*start).into(),
                message: "Directory node is not a directory".to_string(),
            });
        }

        let mut root_directory = vec![];
        while *start < root.size {
            let node: &U8Node = self.nodes.get(*start as usize).unwrap();
            root_directory.push(match node.node_type {
                U8NodeType::File => U8Tree::File(self.get_string(node.name_offset)?),
                U8NodeType::Directory => self._build_tree(start)?,
            });
            *start += 1;
        }

        Ok(U8Tree::Directory(
            self.get_string(root.name_offset)?,
            root_directory,
        ))
    }

    pub fn build_tree(&self) -> binrw::BinResult<U8Tree> {
        let mut idx = 0;
        self._build_tree(&mut idx)
    }

    fn format_nodes(&self) -> binrw::BinResult<String> {
        let nodes_res = self
            .nodes
            .iter()
            .map(|e| self.get_string(e.name_offset))
            .collect::<Vec<_>>();
        let nodes = {
            let mut _nodes = vec![];
            for node in nodes_res {
                _nodes.push(node?);
            }
            _nodes
        };
        Ok(format!("{:#?}", nodes))
    }
}
