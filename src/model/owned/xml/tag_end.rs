use model::owned::OwnedBuf;
use model::TagStart;
use byteorder::{LittleEndian, WriteBytesExt};
use chunks::*;
use errors::*;

pub struct XmlTagEndBuf;

impl XmlTagEndBuf {
    pub fn new() -> Self {
        XmlTagEndBuf
    }
}

impl OwnedBuf for XmlTagEndBuf {
    fn get_token(&self) -> u16 {
        TOKEN_XML_TAG_END
    }

    fn get_body_data(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }

    fn get_header_size(&self) -> u16 {
        8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chunks::*;

    #[test]
    fn it_can_generate_an_empty_chunk() {
        let mut tag_end = XmlTagEndBuf::new();
        let out = tag_end.to_vec().unwrap();
        let expected = vec![3, 1, 8, 0, 8, 0, 0, 0];

        assert_eq!(expected, out);
    }

    #[test]
    fn it_can_generate_a_chunk_with_the_given_data() {

    }

    #[test]
    fn identity() {

    }
}