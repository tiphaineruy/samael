use crate::key_info::KeyInfo;
use crate::metadata::EncryptionMethod;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use serde::Deserialize;
use std::io::Cursor;

const NAME: &str = "md:KeyDescriptor";

#[derive(Clone, Debug, Deserialize)]
pub struct KeyDescriptor {
    #[serde(rename = "use")]
    pub key_use: Option<String>,
    #[serde(rename = "ds:KeyInfo")]
    pub key_info: KeyInfo,
    #[serde(rename = "md:EncryptionMethod")]
    pub encryption_methods: Option<Vec<EncryptionMethod>>,
}

impl KeyDescriptor {
    pub fn to_xml(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut write_buf = Vec::new();
        let mut writer = Writer::new(Cursor::new(&mut write_buf));
        let mut root = BytesStart::borrowed(NAME.as_bytes(), NAME.len());
        if let Some(key_use) = &self.key_use {
            root.push_attribute(("use", key_use.as_ref()));
        }
        writer.write_event(Event::Start(root))?;

        writer.write(self.key_info.to_xml()?.as_bytes())?;

        if let Some(encryption_methods) = &self.encryption_methods {
            for method in encryption_methods {
                writer.write(method.to_xml()?.as_bytes())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::borrowed(NAME.as_bytes())))?;
        Ok(String::from_utf8(write_buf)?)
    }
}
