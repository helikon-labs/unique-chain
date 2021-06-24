//! TODO: I misunterstood therminology, abi IS rlp encoded, so
//! this module should be replaced with rlp crate

#![allow(dead_code)]

#[cfg(not(feature = "std"))]
use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use primitive_types::{H160, U256};

use crate::types::string;

const ABI_ALIGNMENT: usize = 32;

pub type Result<T> = core::result::Result<T, &'static str>;

#[derive(Clone)]
pub struct AbiReader<'i> {
	buf: &'i [u8],
	offset: usize,
}
impl<'i> AbiReader<'i> {
	pub fn new(buf: &'i [u8]) -> Self {
		Self { buf, offset: 0 }
	}
	pub fn new_call(buf: &'i [u8]) -> Result<(u32, Self)> {
		if buf.len() < 4 {
			return Err("missing method id");
		}
		let mut method_id = [0; 4];
		method_id.copy_from_slice(&buf[0..4]);

		Ok((u32::from_be_bytes(method_id), Self { buf, offset: 4 }))
	}

	fn read_padleft<const S: usize>(&mut self) -> Result<[u8; S]> {
		if self.buf.len() - self.offset < 32 {
			return Err("missing padding");
		}
		let mut block = [0; S];
		// Verify padding is empty
		if !self.buf[self.offset..self.offset + ABI_ALIGNMENT - S]
			.iter()
			.all(|&v| v == 0)
		{
			return Err("non zero padding (wrong types?)");
		}
		block.copy_from_slice(
			&self.buf[self.offset + ABI_ALIGNMENT - S..self.offset + ABI_ALIGNMENT],
		);
		self.offset += ABI_ALIGNMENT;
		Ok(block)
	}

	pub fn address(&mut self) -> Result<H160> {
		Ok(H160(self.read_padleft()?))
	}

	pub fn bool(&mut self) -> Result<bool> {
		let data: [u8; 1] = self.read_padleft()?;
		match data[0] {
			0 => Ok(false),
			1 => Ok(true),
			_ => Err("wrong bool value"),
		}
	}

	pub fn bytes4(&mut self) -> Result<[u8; 4]> {
		self.read_padleft()
	}

	pub fn bytes(&mut self) -> Result<Vec<u8>> {
		let mut subresult = self.subresult()?;
		let length = subresult.read_usize()?;
		if subresult.buf.len() <= subresult.offset + length {
			return Err("bytes out of bounds");
		}
		Ok(subresult.buf[subresult.offset..subresult.offset + length].into())
	}

	pub fn uint32(&mut self) -> Result<u32> {
		Ok(u32::from_be_bytes(self.read_padleft()?))
	}

	pub fn uint128(&mut self) -> Result<u128> {
		Ok(u128::from_be_bytes(self.read_padleft()?))
	}

	pub fn uint256(&mut self) -> Result<U256> {
		let buf: [u8; 32] = self.read_padleft()?;
		Ok(U256::from_big_endian(&buf))
	}

	pub fn uint64(&mut self) -> Result<u64> {
		Ok(u64::from_be_bytes(self.read_padleft()?))
	}

	pub fn read_usize(&mut self) -> Result<usize> {
		Ok(usize::from_be_bytes(self.read_padleft()?))
	}

	fn subresult(&mut self) -> Result<AbiReader<'i>> {
		let offset = self.read_usize()?;
		Ok(AbiReader {
			buf: &self.buf,
			offset: offset + self.offset,
		})
	}

	pub fn is_finished(&self) -> bool {
		self.buf.len() == self.offset
	}
}

#[derive(Default)]
pub struct AbiWriter {
	static_part: Vec<u8>,
	dynamic_part: Vec<(usize, AbiWriter)>,
}
impl AbiWriter {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn new_call(method_id: u32) -> Self {
		let mut val = Self::new();
		val.static_part.extend(&method_id.to_be_bytes());
		val
	}

	fn write_padleft(&mut self, block: &[u8]) {
		assert!(block.len() <= ABI_ALIGNMENT);
		self.static_part
			.extend(&[0; ABI_ALIGNMENT][0..ABI_ALIGNMENT - block.len()]);
		self.static_part.extend(block);
	}

	fn write_padright(&mut self, bytes: &[u8]) {
		assert!(bytes.len() <= ABI_ALIGNMENT);
		self.static_part.extend(bytes);
		self.static_part
			.extend(&[0; ABI_ALIGNMENT][0..ABI_ALIGNMENT - bytes.len()]);
	}

	pub fn address(&mut self, address: &H160) {
		self.write_padleft(&address.0)
	}

	pub fn bool(&mut self, value: &bool) {
		self.write_padleft(&[if *value { 1 } else { 0 }])
	}

	pub fn uint8(&mut self, value: &u8) {
		self.write_padleft(&[*value])
	}

	pub fn uint32(&mut self, value: &u32) {
		self.write_padleft(&u32::to_be_bytes(*value))
	}

	pub fn uint128(&mut self, value: &u128) {
		self.write_padleft(&u128::to_be_bytes(*value))
	}

	/// This method writes u128, and exists only for convenience, because there is
	/// no u256 support in rust
	pub fn uint256(&mut self, value: &U256) {
		let mut out = [0; 32];
		value.to_big_endian(&mut out);
		self.write_padleft(&out)
	}

	pub fn write_usize(&mut self, value: &usize) {
		self.write_padleft(&usize::to_be_bytes(*value))
	}

	pub fn write_subresult(&mut self, result: Self) {
		self.dynamic_part.push((self.static_part.len(), result));
		// Empty block, to be filled later
		self.write_padleft(&[]);
	}

	pub fn memory(&mut self, value: &[u8]) {
		let mut sub = Self::new();
		sub.write_usize(&value.len());
		for chunk in value.chunks(ABI_ALIGNMENT) {
			sub.write_padright(chunk);
		}
		self.write_subresult(sub);
	}

	pub fn string(&mut self, value: &str) {
		self.memory(value.as_bytes())
	}

	pub fn finish(mut self) -> Vec<u8> {
		for (static_offset, part) in self.dynamic_part {
			let part_offset = self.static_part.len();

			let encoded_dynamic_offset = usize::to_be_bytes(part_offset - static_offset);
			self.static_part[static_offset + ABI_ALIGNMENT - encoded_dynamic_offset.len()
				..static_offset + ABI_ALIGNMENT]
				.copy_from_slice(&encoded_dynamic_offset);
			self.static_part.extend(part.finish())
		}
		self.static_part
	}
}

pub trait AbiRead<T> {
	fn abi_read(&mut self) -> Result<T>;
}

macro_rules! impl_abi_readable {
	($ty:ty, $method:ident) => {
		impl AbiRead<$ty> for AbiReader<'_> {
			fn abi_read(&mut self) -> Result<$ty> {
				self.$method()
			}
		}
	};
}

impl_abi_readable!(u32, uint32);
impl_abi_readable!(u128, uint128);
impl_abi_readable!(U256, uint256);
impl_abi_readable!(H160, address);
impl_abi_readable!(Vec<u8>, bytes);
impl_abi_readable!(bool, bool);

pub trait AbiWrite {
	fn abi_write(&self, writer: &mut AbiWriter);
}

macro_rules! impl_abi_writeable {
	($ty:ty, $method:ident) => {
		impl AbiWrite for $ty {
			fn abi_write(&self, writer: &mut AbiWriter) {
				writer.$method(&self)
			}
		}
	};
}

impl_abi_writeable!(u8, uint8);
impl_abi_writeable!(u32, uint32);
impl_abi_writeable!(u128, uint128);
impl_abi_writeable!(U256, uint256);
impl_abi_writeable!(H160, address);
impl_abi_writeable!(bool, bool);
impl_abi_writeable!(&str, string);
impl AbiWrite for &string {
	fn abi_write(&self, writer: &mut AbiWriter) {
		writer.string(&self)
	}
}

impl AbiWrite for () {
	fn abi_write(&self, _writer: &mut AbiWriter) {}
}

/// Error, which can be constructed from any ToString type
/// Encoded to Abi as Error(string)
#[derive(Debug)]
pub struct StringError(String);

impl<E> From<E> for StringError
where
	E: ToString,
{
	fn from(e: E) -> Self {
		Self(e.to_string())
	}
}

impl From<StringError> for AbiWriter {
	fn from(v: StringError) -> Self {
		let mut out = AbiWriter::new_call(crate::fn_selector!(Error(string)));
		out.string(&v.0);
		out
	}
}

#[macro_export]
macro_rules! abi_decode {
	($reader:expr, $($name:ident: $typ:ident),+ $(,)?) => {
		$(
			let $name = $reader.$typ()?;
		)+
	}
}
#[macro_export]
macro_rules! abi_encode {
	($($typ:ident($value:expr)),* $(,)?) => {{
		#[allow(unused_mut)]
		let mut writer = ::evm_coder::abi::AbiWriter::new();
		$(
			writer.$typ($value);
		)*
		writer
	}};
	(call $val:expr; $($typ:ident($value:expr)),* $(,)?) => {{
		#[allow(unused_mut)]
		let mut writer = ::evm_coder::abi::AbiWriter::new_call($val);
		$(
			writer.$typ($value);
		)*
		writer
	}}
}