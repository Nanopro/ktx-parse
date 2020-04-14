#[macro_use]
extern crate nom;

use {
    std::{
        fs::{File, read},
        path::Path,
        io::{Cursor, Read},
    },
    nom::{
        IResult, error::ParseError, Err,
        bytes::complete::{tag, take, take_while, take_while_m_n, *},
        character::is_alphabetic,
        combinator::{cond, map_res, not, opt},
        error::ErrorKind,
        multi::{count, fold_many0, many0, many1, many_till},
        number::{
            complete::{},
            Endianness,
        },
        sequence::tuple,
    },
};


#[derive(Debug)]
pub enum KtxError {
    SomeError,
}

impl From<Err<KtxError>> for KtxError {
    fn from(er: Err<KtxError>) -> Self {
        Self::SomeError
    }
}

impl ParseError<&[u8]> for KtxError {
    fn from_error_kind(input: &[u8], kind: ErrorKind) -> Self {
        KtxError::SomeError
    }

    fn append(input: &[u8], kind: ErrorKind, other: Self) -> Self {
        KtxError::SomeError
    }
}

#[derive(Debug)]
pub struct Header {
    endianness: Endianness,
    pub ty: u32,
    pub type_size: u32,
    pub format: u32,
    pub internal_format: u32,
    pub base_internal_format: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_elements: u32,
    pub faces: u32,
    pub mips: u32,
    pub bytes_of_key_value_data: u32,
}

impl Header {
    /// https://docs.google.com/spreadsheets/d/1t6xFXyhlrLnSVucg6bc_Gh5yrDVKZlQLH897NnsTpTY/edit#gid=0
    pub fn vulkan_type(&self) -> u32 {
        match (self.internal_format, self.format, self.ty) {
            (37820, 0, 0) => 181,
            (36286, 0, 0) => 142,
            (35842, 0, 0) => 1000054001,
            (33336, 33320, 5121) => 20,
            (35898, 6407, 35899) => 122,
            (36209, 36248, 5125) => 104,
            (36226, 36249, 5124) => 108,
            (33778, 0, 0) => 135,
            (36759, 6408, 5120) => 38,
            (35905, 6407, 5121) => 29,
            (36168, 6401, 5121) => 127,
            (36238, 36251, 5120) => 49,
            (34837, 6407, 5126) => 106,
            (33777, 0, 0) => 133,
            (35918, 0, 0) => 136,
            (32854, 32993, 32819) => 3,
            (37818, 0, 0) => 177,
            (33326, 6403, 5126) => 100,
            (36756, 6403, 5120) => 10,
            (33325, 6403, 5131) => 76,
            (36208, 36249, 5125) => 107,
            (37842, 0, 0) => 162,
            (36194, 6407, 33635) => 4,
            (36758, 6407, 5120) => 24,
            (32855, 32993, 33638) => 8,
            (33339, 33320, 5124) => 102,
            (34836, 6408, 5126) => 109,
            (37853, 0, 0) => 184,
            (32857, 6408, 33640) => 64,
            (37872, 0, 0) => 1000054006,
            (36797, 6403, 5121) => 15,
            (33329, 36244, 5120) => 14,
            (35905, 32992, 5121) => 36,
            (36227, 36248, 5124) => 105,
            (37847, 0, 0) => 172,
            (33322, 6403, 5123) => 70,
            (33330, 36244, 5121) => 13,
            (32856, 32993, 5121) => 44,
            (37809, 0, 0) => 159,
            (37488, 0, 0) => 153,
            (36238, 36249, 5120) => 42,
            (35415, 0, 0) => 1000054005,
            (37494, 0, 0) => 149,
            (33324, 33319, 5123) => 77,
            (36232, 36249, 5122) => 96,
            (35919, 0, 0) => 138,
            (36233, 36248, 5122) => 89,
            (32859, 6408, 5123) => 91,
            (37489, 0, 0) => 154,
            (36975, 36249, 33640) => 68,
            (37808, 0, 0) => 157,
            (37840, 0, 0) => 158,
            (32849, 32992, 5121) => 30,
            (36013, 34041, 36269) => 130,
            (35843, 0, 0) => 1000054000,
            (36494, 0, 0) => 144,
            (36285, 0, 0) => 141,
            (36761, 33319, 5122) => 78,
            (33332, 36244, 5123) => 74,
            (37814, 0, 0) => 169,
            (37176, 0, 0) => 1000054003,
            (37873, 0, 0) => 1000054007,
            (36760, 6403, 5122) => 71,
            (36759, 32993, 5120) => 45,
            (33333, 36244, 5124) => 99,
            (37812, 0, 0) => 165,
            (36220, 36251, 5121) => 48,
            (33779, 0, 0) => 137,
            (37493, 0, 0) => 148,
            (37811, 0, 0) => 163,
            (37846, 0, 0) => 170,
            (37851, 0, 0) => 180,
            (37816, 0, 0) => 173,
            (33334, 36244, 5125) => 98,
            (36762, 6407, 5122) => 85,
            (36221, 36250, 5121) => 34,
            (37843, 0, 0) => 164,
            (36012, 6402, 5126) => 126,
            (37497, 0, 0) => 152,
            (33337, 33320, 5122) => 82,
            (32856, 6408, 5121) => 37,
            (33335, 33320, 5120) => 21,
            (36798, 33319, 5121) => 22,
            (32849, 6407, 5121) => 23,
            (36975, 36251, 33640) => 62,
            (36495, 0, 0) => 143,
            (37849, 0, 0) => 176,
            (37852, 0, 0) => 182,
            (35414, 0, 0) => 1000054004,
            (33323, 33319, 5121) => 16,
            (36239, 36248, 5120) => 28,
            (35907, 6408, 5121) => 43,
            (35916, 0, 0) => 132,
            (37819, 0, 0) => 179,
            (32854, 6408, 32819) => 2,
            (37821, 0, 0) => 183,
            (32852, 6407, 5123) => 84,
            (36214, 36249, 5123) => 95,
            (33328, 33319, 5126) => 103,
            (37491, 0, 0) => 156,
            (36215, 36248, 5123) => 88,
            (34843, 6407, 5131) => 90,
            (33340, 33320, 5125) => 101,
            (37175, 0, 0) => 1000054002,
            (37844, 0, 0) => 166,
            (36758, 32992, 5120) => 31,
            (37845, 0, 0) => 168,
            (33321, 6403, 5121) => 9,
            (34842, 6408, 5131) => 97,
            (33189, 6402, 5123) => 124,
            (36221, 36248, 5121) => 27,
            (35917, 0, 0) => 134,
            (37810, 0, 0) => 161,
            (33338, 33320, 5123) => 81,
            (36492, 0, 0) => 145,
            (37492, 0, 0) => 147,
            (37848, 0, 0) => 174,
            (35907, 32993, 5121) => 50,
            (33327, 33319, 5131) => 83,
            (32855, 32993, 32820) => 7,
            (36493, 0, 0) => 146,
            (36239, 36250, 5120) => 35,
            (37495, 0, 0) => 150,
            (37841, 0, 0) => 160,
            (36763, 6408, 5122) => 92,
            (33331, 36244, 5122) => 75,
            (35901, 6407, 35902) => 123,
            (35056, 34041, 34042) => 129,
            (37496, 0, 0) => 151,
            (36284, 0, 0) => 140,
            (37817, 0, 0) => 175,
            (36220, 36249, 5121) => 41,
            (33190, 6402, 5125) => 125,
            (32857, 32993, 33640) => 58,
            (36194, 6407, 33636) => 5,
            (37850, 0, 0) => 178,
            (36757, 33319, 5120) => 17,
            (33776, 0, 0) => 131,
            (36283, 0, 0) => 139,
            (37813, 0, 0) => 167,
            (37490, 0, 0) => 155,
            (32855, 6408, 32820) => 6,
            (37815, 0, 0) => 171,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug)]
pub struct Decoder {
    header: Header,
    data: Vec<u8>,
}

impl Decoder {
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}


impl Decoder {
    pub fn read(data: &[u8]) -> std::result::Result<Self, KtxError> {
        let (mut rest, header) = parse_header(data)?;
        println!("Header: {:?}", header);

        let endianness = header.endianness;
        let mut data = Vec::new();
        let is_cube = if header.faces == 6 && header.array_elements == 1 { true } else { false };
        for mip_level in 0..header.mips {
            let (r, image_size) = u32!(rest, endianness)?;
            rest = r;
            /*for array in 0..header.array_elements{
                for face in 0.. header.faces{
                    for z in 0..header.depth{
                        for y in 0..header.height{
                            for x in 0..header.width{
                                for i in 0..header.type_size*4{
                                    let (r, byte) = (&rest[1..], rest[0]);
                                    data.push(byte);
                                    rest = r;
                                }
                            }
                        }
                    }
                }
            }*/
            /*let image_size = if is_cube {
                6 * image_size
            } else {
                image_size
            };*/
            data.extend(
                &rest[0..image_size as usize]
            );
            rest = &rest[image_size as usize..];
        }


        Ok(
            Self {
                header,
                data,
            }
        )
    }
}


fn parse_header(input: &[u8]) -> IResult<&[u8], Header, KtxError> {
    let ident = [0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A];
    let (rest, t): (&[u8], &[u8]) = tag::<&[u8], &[u8], KtxError>(ident.as_ref())(input)?;
    let be = &[0x4, 0x3, 0x2, 0x1];
    let le = &[0x1, 0x2, 0x3, 0x4];
    let (rest, endianness) = if let Ok((rest, _)) = tag::<&[u8], &[u8], KtxError>(be)(rest) {
        (rest, Endianness::Big)
    } else {
        let (rest, _) = tag::<&[u8], &[u8], KtxError>(le)(rest)?;
        (rest, Endianness::Little)
    };

    let (rest, ty) = u32!(rest, endianness)?;
    let (rest, type_size) = u32!(rest, endianness)?;
    let (rest, format) = u32!(rest, endianness)?;
    let (rest, internal_format) = u32!(rest, endianness)?;
    let (rest, base_internal_format) = u32!(rest, endianness)?;
    let (rest, width) = u32!(rest, endianness).map(|(rest, x)| (rest, x.max(1)))?;
    let (rest, height) = u32!(rest, endianness).map(|(rest, x)| (rest, x.max(1)))?;
    let (rest, depth) = u32!(rest, endianness).map(|(rest, x)| (rest, x.max(1)))?;
    let (rest, array_elements) = u32!(rest, endianness).map(|(rest, x)| (rest, x.max(1)))?;
    let (rest, faces) = u32!(rest, endianness).map(|(rest, x)| (rest, x.max(1)))?;
    let (rest, mips) = u32!(rest, endianness).map(|(rest, x)| (rest, x.max(1)))?;
    let (rest, bytes_of_key_value_data) = u32!(rest, endianness)?;


    let rest = &rest[bytes_of_key_value_data as usize..];


    Ok((rest, Header {
        endianness,
        ty,
        type_size,
        format,
        internal_format,
        base_internal_format,
        width,
        height,
        depth,
        array_elements,
        faces,
        mips,
        bytes_of_key_value_data,
    }
    ))
}


#[test]
fn simple_test() {
    let data = read(r"E:\git\Vulkan\data\textures\skysphere_bc3_unorm.ktx").unwrap();
    let decoder = Decoder::read(data.as_slice()).unwrap();
    println!("Header: {:x?}", decoder.header);
    println!("Header: {:x?}", decoder.header.vulkan_type());
    assert!(false);
}



