use std::path::Path;
use crate::error::Result;
use quick_xml::de::from_str;
use serde::Deserialize;

use crate::asset_io::{AssetIO, CAIRead, CAIReadWrite, CAIReader, CAIWriter, RemoteRefEmbed, RemoteRefEmbedType, AssetPatch, HashObjectPositions};
use super::bmff_io::BmffIO;

#[derive(Debug, Deserialize)]
pub struct DashManifest {
    #[serde(rename = "@mediaPresentationDuration")]
    pub duration: String,
    #[serde(rename = "@minBufferTime")]
    pub min_buffer_time: String,
    #[serde(rename = "Period")]
    pub periods: Vec<Period>,
}

#[derive(Debug, Deserialize)]
pub struct Period {
    #[serde(rename = "AdaptationSet")]
    pub adaptation_sets: Vec<AdaptationSet>,
}

#[derive(Debug, Deserialize)]
pub struct AdaptationSet {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@mimeType")]
    pub mime_type: String,
    #[serde(rename = "Representation")]
    pub representations: Vec<Representation>,
}

#[derive(Debug, Deserialize)]
pub struct Representation {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@bandwidth")]
    pub bandwidth: u32,
    #[serde(rename = "@codecs")]
    pub codecs: String,
    #[serde(rename = "SegmentTemplate")]
    pub segment_template: Option<SegmentTemplate>,
}

#[derive(Debug, Deserialize)]
pub struct SegmentTemplate {
    #[serde(rename = "@initialization")]
    pub initialization: String,
    #[serde(rename = "@media")]
    pub media: String,
    #[serde(rename = "@timescale")]
    pub timescale: u32,
    #[serde(rename = "@duration")]
    pub duration: u32,
}

pub struct DashIO {
    bmff_io: BmffIO,
}

impl DashIO {
    pub fn new(asset_type: &str) -> Self {
        Self {
            bmff_io: BmffIO::new(asset_type),
        }
    }
}

impl AssetIO for DashIO {
    fn new(asset_type: &str) -> Self {
        Self {
            bmff_io: BmffIO::new(asset_type),
        }
    }

    fn get_handler(&self, asset_type: &str) -> Box<dyn AssetIO> {
        Box::new(Self::new(asset_type))
    }

    fn get_reader(&self) -> &dyn CAIReader {
        self
    }

    fn supported_types(&self) -> &[&str] {
        &["dash", "m4s", "cmfv", "cmfa", "cmft"]
    }

    fn asset_patch_ref(&self) -> Option<&dyn AssetPatch> {
        self.bmff_io.asset_patch_ref()
    }

    fn read_cai_store(&self, asset_path: &Path) -> Result<Vec<u8>> {
        self.bmff_io.read_cai_store(asset_path)
    }

    fn save_cai_store(&self, asset_path: &Path, store_bytes: &[u8]) -> Result<()> {
        self.bmff_io.save_cai_store(asset_path, store_bytes)
    }

    fn get_object_locations(&self, asset_path: &Path) -> Result<Vec<HashObjectPositions>> {
        self.bmff_io.get_object_locations(asset_path)
    }

    fn remove_cai_store(&self, asset_path: &Path) -> Result<()> {
        self.bmff_io.remove_cai_store(asset_path)
    }
}

impl CAIReader for DashIO {
    fn read_cai(&self, reader: &mut dyn CAIRead) -> Result<Vec<u8>> {
        self.bmff_io.read_cai(reader)
    }

    fn read_xmp(&self, reader: &mut dyn CAIRead) -> Option<String> {
        self.bmff_io.read_xmp(reader)
    }
}

impl CAIWriter for DashIO {
    fn write_cai(
        &self,
        input_stream: &mut dyn CAIRead,
        output_stream: &mut dyn CAIReadWrite,
        store_bytes: &[u8],
    ) -> Result<()> {
        self.bmff_io.write_cai(input_stream, output_stream, store_bytes)
    }

    fn get_object_locations_from_stream(
        &self,
        input_stream: &mut dyn CAIRead,
    ) -> Result<Vec<HashObjectPositions>> {
        self.bmff_io.get_object_locations_from_stream(input_stream)
    }

    fn remove_cai_store_from_stream(
        &self,
        input_stream: &mut dyn CAIRead,
        output_stream: &mut dyn CAIReadWrite,
    ) -> Result<()> {
        self.bmff_io.remove_cai_store_from_stream(input_stream, output_stream)
    }
}

impl RemoteRefEmbed for DashIO {
    fn embed_reference(
        &self,
        asset_path: &Path,
        embed_ref: RemoteRefEmbedType,
    ) -> Result<()> {
        self.bmff_io.embed_reference(asset_path, embed_ref)
    }

    fn embed_reference_to_stream(
        &self,
        input_stream: &mut dyn CAIRead,
        output_stream: &mut dyn CAIReadWrite,
        embed_ref: RemoteRefEmbedType,
    ) -> Result<()> {
        self.bmff_io.embed_reference_to_stream(input_stream, output_stream, embed_ref)
    }
} 