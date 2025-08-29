// Automatically generated rust module for 'wasm_host.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct HostResult<'a> {
    pub error_code: Option<i32>,
    pub error_message: Option<Cow<'a, str>>,
    pub data: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for HostResult<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.error_code = Some(r.read_int32(bytes)?),
                Ok(18) => msg.error_message = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HostResult<'a> {
    fn get_size(&self) -> usize {
        0
        + self.error_code.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error_message.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.error_code { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.error_message { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.data { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TargetRadianAction {
    pub servo_id_vec: Vec<u32>,
    pub target_rad_vec: Vec<f32>,
}

impl<'a> MessageRead<'a> for TargetRadianAction {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.servo_id_vec.push(r.read_uint32(bytes)?),
                Ok(21) => msg.target_rad_vec.push(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TargetRadianAction {
    fn get_size(&self) -> usize {
        0
        + self.servo_id_vec.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + (1 + 4) * self.target_rad_vec.len()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.servo_id_vec { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.target_rad_vec { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeltaRadianAction {
    pub servo_id_vec: Vec<u32>,
    pub delta_rad_vec: Vec<f32>,
}

impl<'a> MessageRead<'a> for DeltaRadianAction {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.servo_id_vec.push(r.read_uint32(bytes)?),
                Ok(21) => msg.delta_rad_vec.push(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeltaRadianAction {
    fn get_size(&self) -> usize {
        0
        + self.servo_id_vec.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + (1 + 4) * self.delta_rad_vec.len()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.servo_id_vec { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.delta_rad_vec { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct EndEffectorAction<'a> {
    pub delta_x: Option<f32>,
    pub delta_y: Option<f32>,
    pub delta_z: Option<f32>,
    pub urdf_file_path: Option<Cow<'a, str>>,
    pub target_link_name: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for EndEffectorAction<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.delta_x = Some(r.read_float(bytes)?),
                Ok(21) => msg.delta_y = Some(r.read_float(bytes)?),
                Ok(29) => msg.delta_z = Some(r.read_float(bytes)?),
                Ok(34) => msg.urdf_file_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.target_link_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for EndEffectorAction<'a> {
    fn get_size(&self) -> usize {
        0
        + self.delta_x.as_ref().map_or(0, |_| 1 + 4)
        + self.delta_y.as_ref().map_or(0, |_| 1 + 4)
        + self.delta_z.as_ref().map_or(0, |_| 1 + 4)
        + self.urdf_file_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.target_link_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.delta_x { w.write_with_tag(13, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.delta_y { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.delta_z { w.write_with_tag(29, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.urdf_file_path { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.target_link_name { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionResult {
    pub finish_vec: Vec<bool>,
    pub current_radian_vec: Vec<f32>,
}

impl<'a> MessageRead<'a> for ActionResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.finish_vec.push(r.read_bool(bytes)?),
                Ok(21) => msg.current_radian_vec.push(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionResult {
    fn get_size(&self) -> usize {
        0
        + self.finish_vec.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + (1 + 4) * self.current_radian_vec.len()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.finish_vec { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        for s in &self.current_radian_vec { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Pair<'a> {
    pub key: Option<Cow<'a, str>>,
    pub value: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Pair<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Pair<'a> {
    fn get_size(&self) -> usize {
        0
        + self.key.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.key { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct HttpRequest<'a> {
    pub url: Option<Cow<'a, str>>,
    pub method: Option<Cow<'a, str>>,
    pub headers: Vec<Pair<'a>>,
    pub body: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for HttpRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.method = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.headers.push(r.read_message::<Pair>(bytes)?),
                Ok(34) => msg.body = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HttpRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.method.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.headers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.body.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.url { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.method { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        for s in &self.headers { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.body { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    pub status_code: Option<i32>,
    pub headers: Vec<Pair<'a>>,
    pub body: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for HttpResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status_code = Some(r.read_int32(bytes)?),
                Ok(18) => msg.headers.push(r.read_message::<Pair>(bytes)?),
                Ok(26) => msg.body = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HttpResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.status_code.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.headers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.body.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.status_code { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        for s in &self.headers { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.body { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ServoInfo<'a> {
    pub servo_id: Option<u32>,
    pub name: Option<Cow<'a, str>>,
    pub min_rad: Option<f32>,
    pub max_rad: Option<f32>,
    pub resolution: Option<u32>,
}

impl<'a> MessageRead<'a> for ServoInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.servo_id = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(37) => msg.min_rad = Some(r.read_float(bytes)?),
                Ok(45) => msg.max_rad = Some(r.read_float(bytes)?),
                Ok(48) => msg.resolution = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ServoInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.servo_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.min_rad.as_ref().map_or(0, |_| 1 + 4)
        + self.max_rad.as_ref().map_or(0, |_| 1 + 4)
        + self.resolution.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.servo_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.min_rad { w.write_with_tag(37, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.max_rad { w.write_with_tag(45, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.resolution { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ServoInfoList<'a> {
    pub infos: Vec<ServoInfo<'a>>,
}

impl<'a> MessageRead<'a> for ServoInfoList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.infos.push(r.read_message::<ServoInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ServoInfoList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.infos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.infos { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ServoRawParam<'a> {
    pub servo_id: Option<u32>,
    pub params: Vec<ParamEntry<'a>>,
}

impl<'a> MessageRead<'a> for ServoRawParam<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.servo_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.params.push(r.read_message::<ParamEntry>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ServoRawParam<'a> {
    fn get_size(&self) -> usize {
        0
        + self.servo_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.params.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.servo_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.params { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParamEntry<'a> {
    pub key: Option<Cow<'a, str>>,
    pub value: Option<i32>,
}

impl<'a> MessageRead<'a> for ParamEntry<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.value = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ParamEntry<'a> {
    fn get_size(&self) -> usize {
        0
        + self.key.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.key { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ServoRawParamList<'a> {
    pub params: Vec<ServoRawParam<'a>>,
}

impl<'a> MessageRead<'a> for ServoRawParamList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.params.push(r.read_message::<ServoRawParam>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ServoRawParamList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.params.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.params { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct McpTool<'a> {
    pub name: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub input_schema: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for McpTool<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.description = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.input_schema = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for McpTool<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.input_schema.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.description { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.input_schema { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct McpToolList<'a> {
    pub tools: Vec<McpTool<'a>>,
}

impl<'a> MessageRead<'a> for McpToolList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tools.push(r.read_message::<McpTool>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for McpToolList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.tools.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tools { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct McpCallToolRequest<'a> {
    pub tool_name: Option<Cow<'a, str>>,
    pub arguments: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for McpCallToolRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tool_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.arguments = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for McpCallToolRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.tool_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.arguments.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.tool_name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.arguments { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct McpCallToolResponse<'a> {
    pub content: Option<Cow<'a, str>>,
    pub is_error: Option<bool>,
}

impl<'a> MessageRead<'a> for McpCallToolResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.content = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.is_error = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for McpCallToolResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.is_error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.content { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.is_error { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StringResponse<'a> {
    pub value: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for StringResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StringResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

