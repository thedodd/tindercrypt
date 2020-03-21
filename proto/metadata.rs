// This file is generated by rust-protobuf 2.10.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `proto/metadata.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_2;

#[derive(PartialEq,Clone,Default)]
pub struct KeyDerivationMetadata {
    // message fields
    pub algo: KeyDerivationAlgorithm,
    pub hash_fn: HashFunction,
    pub iterations: u64,
    pub salt: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a KeyDerivationMetadata {
    fn default() -> &'a KeyDerivationMetadata {
        <KeyDerivationMetadata as ::protobuf::Message>::default_instance()
    }
}

impl KeyDerivationMetadata {
    pub fn new() -> KeyDerivationMetadata {
        ::std::default::Default::default()
    }

    // .metadata.KeyDerivationAlgorithm algo = 1;


    pub fn get_algo(&self) -> KeyDerivationAlgorithm {
        self.algo
    }
    pub fn clear_algo(&mut self) {
        self.algo = KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_algo(&mut self, v: KeyDerivationAlgorithm) {
        self.algo = v;
    }

    // .metadata.HashFunction hash_fn = 2;


    pub fn get_hash_fn(&self) -> HashFunction {
        self.hash_fn
    }
    pub fn clear_hash_fn(&mut self) {
        self.hash_fn = HashFunction::HASH_FUNCTION_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_hash_fn(&mut self, v: HashFunction) {
        self.hash_fn = v;
    }

    // uint64 iterations = 3;


    pub fn get_iterations(&self) -> u64 {
        self.iterations
    }
    pub fn clear_iterations(&mut self) {
        self.iterations = 0;
    }

    // Param is passed by value, moved
    pub fn set_iterations(&mut self, v: u64) {
        self.iterations = v;
    }

    // bytes salt = 4;


    pub fn get_salt(&self) -> &[u8] {
        &self.salt
    }
    pub fn clear_salt(&mut self) {
        self.salt.clear();
    }

    // Param is passed by value, moved
    pub fn set_salt(&mut self, v: ::std::vec::Vec<u8>) {
        self.salt = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_salt(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.salt
    }

    // Take field
    pub fn take_salt(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.salt, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for KeyDerivationMetadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.algo, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.hash_fn, 2, &mut self.unknown_fields)?
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.iterations = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.salt)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.algo != KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.algo);
        }
        if self.hash_fn != HashFunction::HASH_FUNCTION_INVALID {
            my_size += ::protobuf::rt::enum_size(2, self.hash_fn);
        }
        if self.iterations != 0 {
            my_size += ::protobuf::rt::value_size(3, self.iterations, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.salt.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.salt);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.algo != KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_INVALID {
            os.write_enum(1, self.algo.value())?;
        }
        if self.hash_fn != HashFunction::HASH_FUNCTION_INVALID {
            os.write_enum(2, self.hash_fn.value())?;
        }
        if self.iterations != 0 {
            os.write_uint64(3, self.iterations)?;
        }
        if !self.salt.is_empty() {
            os.write_bytes(4, &self.salt)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> KeyDerivationMetadata {
        KeyDerivationMetadata::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<KeyDerivationAlgorithm>>(
                    "algo",
                    |m: &KeyDerivationMetadata| { &m.algo },
                    |m: &mut KeyDerivationMetadata| { &mut m.algo },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<HashFunction>>(
                    "hash_fn",
                    |m: &KeyDerivationMetadata| { &m.hash_fn },
                    |m: &mut KeyDerivationMetadata| { &mut m.hash_fn },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "iterations",
                    |m: &KeyDerivationMetadata| { &m.iterations },
                    |m: &mut KeyDerivationMetadata| { &mut m.iterations },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "salt",
                    |m: &KeyDerivationMetadata| { &m.salt },
                    |m: &mut KeyDerivationMetadata| { &mut m.salt },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyDerivationMetadata>(
                    "KeyDerivationMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static KeyDerivationMetadata {
        static mut instance: ::protobuf::lazy::Lazy<KeyDerivationMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyDerivationMetadata,
        };
        unsafe {
            instance.get(KeyDerivationMetadata::new)
        }
    }
}

impl ::protobuf::Clear for KeyDerivationMetadata {
    fn clear(&mut self) {
        self.algo = KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_INVALID;
        self.hash_fn = HashFunction::HASH_FUNCTION_INVALID;
        self.iterations = 0;
        self.salt.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyDerivationMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyDerivationMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EncryptionMetadata {
    // message fields
    pub algo: EncryptionAlgorithm,
    pub nonce: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EncryptionMetadata {
    fn default() -> &'a EncryptionMetadata {
        <EncryptionMetadata as ::protobuf::Message>::default_instance()
    }
}

impl EncryptionMetadata {
    pub fn new() -> EncryptionMetadata {
        ::std::default::Default::default()
    }

    // .metadata.EncryptionAlgorithm algo = 1;


    pub fn get_algo(&self) -> EncryptionAlgorithm {
        self.algo
    }
    pub fn clear_algo(&mut self) {
        self.algo = EncryptionAlgorithm::ENCRYPTION_ALGORITHM_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_algo(&mut self, v: EncryptionAlgorithm) {
        self.algo = v;
    }

    // bytes nonce = 2;


    pub fn get_nonce(&self) -> &[u8] {
        &self.nonce
    }
    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.nonce = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.nonce
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.nonce, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for EncryptionMetadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.algo, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.nonce)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.algo != EncryptionAlgorithm::ENCRYPTION_ALGORITHM_INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.algo);
        }
        if !self.nonce.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.nonce);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.algo != EncryptionAlgorithm::ENCRYPTION_ALGORITHM_INVALID {
            os.write_enum(1, self.algo.value())?;
        }
        if !self.nonce.is_empty() {
            os.write_bytes(2, &self.nonce)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EncryptionMetadata {
        EncryptionMetadata::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EncryptionAlgorithm>>(
                    "algo",
                    |m: &EncryptionMetadata| { &m.algo },
                    |m: &mut EncryptionMetadata| { &mut m.algo },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "nonce",
                    |m: &EncryptionMetadata| { &m.nonce },
                    |m: &mut EncryptionMetadata| { &mut m.nonce },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncryptionMetadata>(
                    "EncryptionMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EncryptionMetadata {
        static mut instance: ::protobuf::lazy::Lazy<EncryptionMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncryptionMetadata,
        };
        unsafe {
            instance.get(EncryptionMetadata::new)
        }
    }
}

impl ::protobuf::Clear for EncryptionMetadata {
    fn clear(&mut self) {
        self.algo = EncryptionAlgorithm::ENCRYPTION_ALGORITHM_INVALID;
        self.nonce.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EncryptionMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EncryptionMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    pub key_deriv_meta: ::protobuf::SingularPtrField<KeyDerivationMetadata>,
    pub enc_meta: ::protobuf::SingularPtrField<EncryptionMetadata>,
    pub ciphertext_size: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Metadata {
    fn default() -> &'a Metadata {
        <Metadata as ::protobuf::Message>::default_instance()
    }
}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    // .metadata.KeyDerivationMetadata key_deriv_meta = 1;


    pub fn get_key_deriv_meta(&self) -> &KeyDerivationMetadata {
        self.key_deriv_meta.as_ref().unwrap_or_else(|| KeyDerivationMetadata::default_instance())
    }
    pub fn clear_key_deriv_meta(&mut self) {
        self.key_deriv_meta.clear();
    }

    pub fn has_key_deriv_meta(&self) -> bool {
        self.key_deriv_meta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_deriv_meta(&mut self, v: KeyDerivationMetadata) {
        self.key_deriv_meta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_deriv_meta(&mut self) -> &mut KeyDerivationMetadata {
        if self.key_deriv_meta.is_none() {
            self.key_deriv_meta.set_default();
        }
        self.key_deriv_meta.as_mut().unwrap()
    }

    // Take field
    pub fn take_key_deriv_meta(&mut self) -> KeyDerivationMetadata {
        self.key_deriv_meta.take().unwrap_or_else(|| KeyDerivationMetadata::new())
    }

    // .metadata.EncryptionMetadata enc_meta = 2;


    pub fn get_enc_meta(&self) -> &EncryptionMetadata {
        self.enc_meta.as_ref().unwrap_or_else(|| EncryptionMetadata::default_instance())
    }
    pub fn clear_enc_meta(&mut self) {
        self.enc_meta.clear();
    }

    pub fn has_enc_meta(&self) -> bool {
        self.enc_meta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enc_meta(&mut self, v: EncryptionMetadata) {
        self.enc_meta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_enc_meta(&mut self) -> &mut EncryptionMetadata {
        if self.enc_meta.is_none() {
            self.enc_meta.set_default();
        }
        self.enc_meta.as_mut().unwrap()
    }

    // Take field
    pub fn take_enc_meta(&mut self) -> EncryptionMetadata {
        self.enc_meta.take().unwrap_or_else(|| EncryptionMetadata::new())
    }

    // uint64 ciphertext_size = 3;


    pub fn get_ciphertext_size(&self) -> u64 {
        self.ciphertext_size
    }
    pub fn clear_ciphertext_size(&mut self) {
        self.ciphertext_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_ciphertext_size(&mut self, v: u64) {
        self.ciphertext_size = v;
    }
}

impl ::protobuf::Message for Metadata {
    fn is_initialized(&self) -> bool {
        for v in &self.key_deriv_meta {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.enc_meta {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key_deriv_meta)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.enc_meta)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ciphertext_size = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.key_deriv_meta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.enc_meta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.ciphertext_size != 0 {
            my_size += ::protobuf::rt::value_size(3, self.ciphertext_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key_deriv_meta.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.enc_meta.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.ciphertext_size != 0 {
            os.write_uint64(3, self.ciphertext_size)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyDerivationMetadata>>(
                    "key_deriv_meta",
                    |m: &Metadata| { &m.key_deriv_meta },
                    |m: &mut Metadata| { &mut m.key_deriv_meta },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EncryptionMetadata>>(
                    "enc_meta",
                    |m: &Metadata| { &m.enc_meta },
                    |m: &mut Metadata| { &mut m.enc_meta },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ciphertext_size",
                    |m: &Metadata| { &m.ciphertext_size },
                    |m: &mut Metadata| { &mut m.ciphertext_size },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metadata>(
                    "Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Metadata {
        static mut instance: ::protobuf::lazy::Lazy<Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metadata,
        };
        unsafe {
            instance.get(Metadata::new)
        }
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.key_deriv_meta.clear();
        self.enc_meta.clear();
        self.ciphertext_size = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HashFunction {
    HASH_FUNCTION_INVALID = 0,
    HASH_FUNCTION_SHA256 = 1,
    HASH_FUNCTION_SHA384 = 2,
    HASH_FUNCTION_SHA512 = 3,
}

impl ::protobuf::ProtobufEnum for HashFunction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HashFunction> {
        match value {
            0 => ::std::option::Option::Some(HashFunction::HASH_FUNCTION_INVALID),
            1 => ::std::option::Option::Some(HashFunction::HASH_FUNCTION_SHA256),
            2 => ::std::option::Option::Some(HashFunction::HASH_FUNCTION_SHA384),
            3 => ::std::option::Option::Some(HashFunction::HASH_FUNCTION_SHA512),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HashFunction] = &[
            HashFunction::HASH_FUNCTION_INVALID,
            HashFunction::HASH_FUNCTION_SHA256,
            HashFunction::HASH_FUNCTION_SHA384,
            HashFunction::HASH_FUNCTION_SHA512,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HashFunction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HashFunction {
}

impl ::std::default::Default for HashFunction {
    fn default() -> Self {
        HashFunction::HASH_FUNCTION_INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for HashFunction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum KeyDerivationAlgorithm {
    KEY_DERIVATION_ALGORITHM_INVALID = 0,
    KEY_DERIVATION_ALGORITHM_NONE = 1,
    KEY_DERIVATION_ALGORITHM_PBKDF2 = 2,
}

impl ::protobuf::ProtobufEnum for KeyDerivationAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KeyDerivationAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_INVALID),
            1 => ::std::option::Option::Some(KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_NONE),
            2 => ::std::option::Option::Some(KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_PBKDF2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [KeyDerivationAlgorithm] = &[
            KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_INVALID,
            KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_NONE,
            KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_PBKDF2,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("KeyDerivationAlgorithm", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for KeyDerivationAlgorithm {
}

impl ::std::default::Default for KeyDerivationAlgorithm {
    fn default() -> Self {
        KeyDerivationAlgorithm::KEY_DERIVATION_ALGORITHM_INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyDerivationAlgorithm {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EncryptionAlgorithm {
    ENCRYPTION_ALGORITHM_INVALID = 0,
    ENCRYPTION_ALGORITHM_AES256GCM = 1,
    ENCRYPTION_ALGORITHM_CHACHA20_POLY1305 = 2,
}

impl ::protobuf::ProtobufEnum for EncryptionAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EncryptionAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(EncryptionAlgorithm::ENCRYPTION_ALGORITHM_INVALID),
            1 => ::std::option::Option::Some(EncryptionAlgorithm::ENCRYPTION_ALGORITHM_AES256GCM),
            2 => ::std::option::Option::Some(EncryptionAlgorithm::ENCRYPTION_ALGORITHM_CHACHA20_POLY1305),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EncryptionAlgorithm] = &[
            EncryptionAlgorithm::ENCRYPTION_ALGORITHM_INVALID,
            EncryptionAlgorithm::ENCRYPTION_ALGORITHM_AES256GCM,
            EncryptionAlgorithm::ENCRYPTION_ALGORITHM_CHACHA20_POLY1305,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EncryptionAlgorithm", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EncryptionAlgorithm {
}

impl ::std::default::Default for EncryptionAlgorithm {
    fn default() -> Self {
        EncryptionAlgorithm::ENCRYPTION_ALGORITHM_INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for EncryptionAlgorithm {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14proto/metadata.proto\x12\x08metadata\"\xb2\x01\n\x15KeyDerivationM\
    etadata\x124\n\x04algo\x18\x01\x20\x01(\x0e2\x20.metadata.KeyDerivationA\
    lgorithmR\x04algo\x12/\n\x07hash_fn\x18\x02\x20\x01(\x0e2\x16.metadata.H\
    ashFunctionR\x06hashFn\x12\x1e\n\niterations\x18\x03\x20\x01(\x04R\niter\
    ations\x12\x12\n\x04salt\x18\x04\x20\x01(\x0cR\x04salt\"]\n\x12Encryptio\
    nMetadata\x121\n\x04algo\x18\x01\x20\x01(\x0e2\x1d.metadata.EncryptionAl\
    gorithmR\x04algo\x12\x14\n\x05nonce\x18\x02\x20\x01(\x0cR\x05nonce\"\xb3\
    \x01\n\x08Metadata\x12E\n\x0ekey_deriv_meta\x18\x01\x20\x01(\x0b2\x1f.me\
    tadata.KeyDerivationMetadataR\x0ckeyDerivMeta\x127\n\x08enc_meta\x18\x02\
    \x20\x01(\x0b2\x1c.metadata.EncryptionMetadataR\x07encMeta\x12'\n\x0fcip\
    hertext_size\x18\x03\x20\x01(\x04R\x0eciphertextSize*w\n\x0cHashFunction\
    \x12\x19\n\x15HASH_FUNCTION_INVALID\x10\0\x12\x18\n\x14HASH_FUNCTION_SHA\
    256\x10\x01\x12\x18\n\x14HASH_FUNCTION_SHA384\x10\x02\x12\x18\n\x14HASH_\
    FUNCTION_SHA512\x10\x03*\x86\x01\n\x16KeyDerivationAlgorithm\x12$\n\x20K\
    EY_DERIVATION_ALGORITHM_INVALID\x10\0\x12!\n\x1dKEY_DERIVATION_ALGORITHM\
    _NONE\x10\x01\x12#\n\x1fKEY_DERIVATION_ALGORITHM_PBKDF2\x10\x02*\x87\x01\
    \n\x13EncryptionAlgorithm\x12\x20\n\x1cENCRYPTION_ALGORITHM_INVALID\x10\
    \0\x12\"\n\x1eENCRYPTION_ALGORITHM_AES256GCM\x10\x01\x12*\n&ENCRYPTION_A\
    LGORITHM_CHACHA20_POLY1305\x10\x02B+\n\x0ccom.metadataB\rMetadataProtoP\
    \x01Z\nmetadatapbb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
