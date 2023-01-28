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

#[binrw::binread]
#[derive(Debug)]
pub struct U8Node {
    node_type: U8NodeType,
    #[br(map(|x: (u8, u16)| (<u8 as Into<u32>>::into(x.0) << 16) | <u16 as Into<u32>>::into(x.1)))]
    name_offset: u32,
    data_offset: u32,
    size: u32,
}

#[derive(Debug)]
pub enum U8Tree {
    File(String, Vec<u8>),
    Directory(String, Vec<U8Tree>),
}

fn recurse_nodes<R: std::io::Read + std::io::Seek>(
    reader: &mut R,
    nodes: &Vec<U8Node>,
    name_offset: u64,
    file_offset: u64,
    idx: &mut u32,
) -> binrw::BinResult<U8Tree> {
    let root_node = nodes.get(*idx as usize).ok_or(binrw::Error::AssertFail {
        pos: 0,
        message: "Failed to read root node".to_string(),
    })?;
    (*idx) += 1;

    reader.seek(std::io::SeekFrom::Start(
        name_offset + root_node.name_offset as u64,
    ))?;
    let name = <binrw::NullString>::read(reader)?.to_string();

    if root_node.node_type != U8NodeType::Directory {
        return Err(binrw::Error::AssertFail {
            pos: reader.stream_position()?,
            message: "Expected root node to be a directory".to_string(),
        });
    }

    let mut files = vec![];

    while *idx < root_node.size {
        let node = nodes.get(*idx as usize).ok_or(binrw::Error::AssertFail {
            pos: (*idx).into(),
            message: format!("Failed to read node {}", *idx),
        })?;
        files.push(match node.node_type {
            U8NodeType::File => {
                reader.seek(std::io::SeekFrom::Start(
                    name_offset + node.name_offset as u64,
                ))?;
                let name = <binrw::NullString>::read(reader)?.to_string();
                reader.seek(std::io::SeekFrom::Start(
                    file_offset + node.data_offset as u64,
                ))?;
                let data = <Vec<u8>>::read_args(
                    reader,
                    binrw::VecArgs {
                        count: node.size as usize,
                        inner: (),
                    },
                )?;

                U8Tree::File(name, data)
            }
            U8NodeType::Directory => recurse_nodes(reader, nodes, name_offset, file_offset, idx)?,
        });

        (*idx) += 1;
    }

    Ok(U8Tree::Directory(name, files))
}

impl binrw::BinRead for U8Tree {
    type Args = u64;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _: &binrw::ReadOptions,
        file_offset: Self::Args,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()?;
        let root_node = <U8Node>::read_be(reader)?;
        let root_size = root_node.size;
        if root_node.node_type != U8NodeType::Directory {
            return Err(binrw::Error::AssertFail {
                pos,
                message: "Expected root node to be a directory".to_string(),
            });
        }

        reader.seek(std::io::SeekFrom::Start(pos))?;
        let nodes = <Vec<U8Node>>::read_be_args(
            reader,
            binrw::VecArgs {
                count: root_size as usize,
                inner: (),
            },
        )?;

        let name_offset = reader.stream_position()?;

        let mut idx = 0;
        recurse_nodes(reader, &nodes, name_offset, file_offset, &mut idx)
    }
}

#[derive(Debug)]
pub struct U8Archive {
    header: U8Header,
    nodes: U8Tree,
}

impl binrw::BinRead for U8Archive {
    type Args = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        options: &binrw::ReadOptions,
        _: Self::Args,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()?;
        let header = <U8Header>::read_be(reader)?;

        reader.seek(std::io::SeekFrom::Start(
            pos + header.rootnode_offset as u64,
        ))?;

        let nodes = <_>::read_options(reader, options, pos)?;
        Ok(Self { header, nodes })
    }
}

impl U8Tree {
    fn name(&self) -> &String {
        match self {
            U8Tree::File(f, _) => f,
            U8Tree::Directory(f, _) => f,
        }
    }
}

impl U8Archive {
    fn find<T: AsRef<std::path::Path>>(&self, path: T) -> binrw::BinResult<&U8Tree> {
        let mut cur_node = &self.nodes;
        if let U8Tree::File(f, _) = cur_node {
            return Err(binrw::Error::AssertFail {
                pos: 0,
                message: format!("Root node (\"{f}\") is not a directory"),
            });
        };
        for elem in path.as_ref() {
            let U8Tree::Directory(_,ref dir) = cur_node else {unreachable!()};
            let node = dir.iter().find(|&e| e.name() == elem.to_str().unwrap());
            if let Some(node) = node {
                match node {
                    U8Tree::File(_, _) => return Ok(node),
                    U8Tree::Directory(_, _) => {
                        cur_node = node;
                    }
                }
            }
        }
        Ok(cur_node)
    }

    pub fn ls<T: AsRef<std::path::Path>>(&self, path: T) -> binrw::BinResult<Vec<String>> {
        let dir = self.find(path)?;

        if let U8Tree::File(f, _) = dir {
            return Err(binrw::Error::AssertFail {
                pos: 0,
                message: format!("{f} is not a directory"),
            });
        }

        let U8Tree::Directory(_, dir) = dir else {unreachable!()};

        Ok(dir
            .iter()
            .map(|e| {
                format!(
                    "{}{}",
                    e.name(),
                    if let U8Tree::Directory(_, _) = e {
                        "/"
                    } else {
                        ""
                    }
                )
            })
            .collect())
    }

    pub fn get<T: AsRef<std::path::Path> + Clone>(&self, path: T) -> binrw::BinResult<&Vec<u8>> {
        let dir = self.find(path)?;

        match dir {
            U8Tree::File(_, p) => Ok(p),
            U8Tree::Directory(f, _) => Err(binrw::Error::AssertFail {
                pos: 0,
                message: format!("{f} is not a file"),
            }),
        }
    }
}
