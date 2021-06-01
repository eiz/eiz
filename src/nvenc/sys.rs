#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/*
 * This copyright notice applies to this header file only:
 *
 * Copyright (c) 2010-2019 NVIDIA Corporation
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the software, and to permit persons to whom the
 * software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/*
 * \file nvEncodeAPI.h
 *   NVIDIA GPUs - beginning with the Kepler generation - contain a hardware-based encoder
 *   (referred to as NVENC) which provides fully-accelerated hardware-based video encoding.
 *   NvEncodeAPI provides the interface for NVIDIA video encoder (NVENC).
 * \date 2011-2019
 *  This file contains the interface constants, structure definitions and function prototypes.
 */

pub const NVENCAPI_MAJOR_VERSION: u32 = 9;
pub const NVENCAPI_MINOR_VERSION: u32 = 1;
pub const NVENCAPI_VERSION: u32 = NVENCAPI_MAJOR_VERSION | (NVENCAPI_MINOR_VERSION << 24);

pub const fn nvencapi_struct_version(ver: u32) -> u32 {
    NVENCAPI_VERSION | (ver << 16) | (0x7 << 28)
}

pub const NV_ENC_CAPS_PARAM_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_ENCODE_OUT_PARAMS_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_CREATE_INPUT_BUFFER_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_CREATE_BITSTREAM_BUFFER_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_CREATE_MV_BUFFER_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_RC_PARAMS_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_CONFIG_VER: u32 = nvencapi_struct_version(7) | (1 << 31);
pub const NV_ENC_INITIALIZE_PARAMS_VER: u32 = nvencapi_struct_version(5) | (1 << 31);
pub const NV_ENC_RECONFIGURE_PARAMS_VER: u32 = nvencapi_struct_version(1) | (1 << 31);
pub const NV_ENC_PRESET_CONFIG_VER: u32 = nvencapi_struct_version(4) | (1 << 31);
pub const NV_ENC_PIC_PARAMS_MVC_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_PIC_PARAMS_VER: u32 = nvencapi_struct_version(4) | (1 << 31);
pub const NV_ENC_MEONLY_PARAMS_VER: u32 = nvencapi_struct_version(3);
pub const NV_ENC_LOCK_BITSTREAM_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_LOCK_INPUT_BUFFER_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_MAP_INPUT_RESOURCE_VER: u32 = nvencapi_struct_version(4);
pub const NV_ENC_REGISTER_RESOURCE_VER: u32 = nvencapi_struct_version(3);
pub const NV_ENC_STAT_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_SEQUENCE_PARAM_PAYLOAD_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_EVENT_PARAMS_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER: u32 = nvencapi_struct_version(1);
pub const NV_ENCODE_API_FUNCTION_LIST_VER: u32 = nvencapi_struct_version(2);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}

#[doc = " Abstracts the GUID structure for non-windows platforms."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GUID {
    #[doc = "Specifies the first 8 hexadecimal digits of the GUID."]
    pub Data1: u32,
    #[doc = "Specifies the first group of 4 hexadecimal digits."]
    pub Data2: u16,
    #[doc = "Specifies the second group of 4 hexadecimal digits."]
    pub Data3: u16,
    #[doc = "Array of 8 bytes. The first 2 bytes contain the third group of 4 hexadecimal digits."]
    #[doc = "The remaining 6 bytes contain the final 12 hexadecimal digits."]
    pub Data4: [u8; 8usize],
}

impl GUID {
    pub const fn new(
        d1: u32,
        d2: u16,
        d3: u16,
        d4: u8,
        d5: u8,
        d6: u8,
        d7: u8,
        d8: u8,
        d9: u8,
        d10: u8,
        d11: u8,
    ) -> Self {
        GUID {
            Data1: d1,
            Data2: d2,
            Data3: d3,
            Data4: [d4, d5, d6, d7, d8, d9, d10, d11],
        }
    }
}

#[doc = " Defines a Rectangle. Used in ::NV_ENC_PREPROCESS_FRAME."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NVENC_RECT {
    #[doc = "X coordinate of the upper left corner of rectangular area to be specified."]
    pub left: u32,
    #[doc = "Y coordinate of the upper left corner of the rectangular area to be specified."]
    pub top: u32,
    #[doc = "X coordinate of the bottom right corner of the rectangular area to be specified."]
    pub right: u32,
    #[doc = "Y coordinate of the bottom right corner of the rectangular area to be specified."]
    pub bottom: u32,
}
pub type NVENC_RECT = _NVENC_RECT;
pub type NV_ENC_INPUT_PTR = *mut ::core::ffi::c_void;
pub type NV_ENC_OUTPUT_PTR = *mut ::core::ffi::c_void;
pub type NV_ENC_REGISTERED_PTR = *mut ::core::ffi::c_void;
pub type NV_ENC_CUSTREAM_PTR = *mut ::core::ffi::c_void;

macro_rules! define_guids {
    ($(static const GUID $name:ident = {
        $d1:literal, $d2:literal, $d3:literal, {
            $d4:literal, $d5:literal, $d6:literal, $d7:literal,
            $d8:literal, $d9:literal, $d10:literal, $d11:literal }});*;) => {
        $(pub const $name: GUID = GUID::new($d1, $d2, $d3, $d4, $d5, $d6, $d7, $d8, $d9, $d10, $d11));*;
    };
}

define_guids! {
    // =========================================================================================
    // Encode Codec GUIDS supported by the NvEncodeAPI interface.
    // =========================================================================================

    // {6BC82762-4E63-4ca4-AA85-1E50F321F6BF}
    static const GUID NV_ENC_CODEC_H264_GUID =
    { 0x6bc82762, 0x4e63, 0x4ca4, { 0xaa, 0x85, 0x1e, 0x50, 0xf3, 0x21, 0xf6, 0xbf } };

    // {790CDC88-4522-4d7b-9425-BDA9975F7603}
    static const GUID NV_ENC_CODEC_HEVC_GUID =
    { 0x790cdc88, 0x4522, 0x4d7b, { 0x94, 0x25, 0xbd, 0xa9, 0x97, 0x5f, 0x76, 0x3 } };



    // =========================================================================================
    // *   Encode Profile GUIDS supported by the NvEncodeAPI interface.
    // =========================================================================================

    // {BFD6F8E7-233C-4341-8B3E-4818523803F4}
    static const GUID NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID =
    { 0xbfd6f8e7, 0x233c, 0x4341, { 0x8b, 0x3e, 0x48, 0x18, 0x52, 0x38, 0x3, 0xf4 } };

    // {0727BCAA-78C4-4c83-8C2F-EF3DFF267C6A}
    static const GUID  NV_ENC_H264_PROFILE_BASELINE_GUID =
    { 0x727bcaa, 0x78c4, 0x4c83, { 0x8c, 0x2f, 0xef, 0x3d, 0xff, 0x26, 0x7c, 0x6a } };

    // {60B5C1D4-67FE-4790-94D5-C4726D7B6E6D}
    static const GUID  NV_ENC_H264_PROFILE_MAIN_GUID =
    { 0x60b5c1d4, 0x67fe, 0x4790, { 0x94, 0xd5, 0xc4, 0x72, 0x6d, 0x7b, 0x6e, 0x6d } };

    // {E7CBC309-4F7A-4b89-AF2A-D537C92BE310}
    static const GUID NV_ENC_H264_PROFILE_HIGH_GUID =
    { 0xe7cbc309, 0x4f7a, 0x4b89, { 0xaf, 0x2a, 0xd5, 0x37, 0xc9, 0x2b, 0xe3, 0x10 } };

    // {7AC663CB-A598-4960-B844-339B261A7D52}
    static const GUID  NV_ENC_H264_PROFILE_HIGH_444_GUID =
    { 0x7ac663cb, 0xa598, 0x4960, { 0xb8, 0x44, 0x33, 0x9b, 0x26, 0x1a, 0x7d, 0x52 } };

    // {40847BF5-33F7-4601-9084-E8FE3C1DB8B7}
    static const GUID NV_ENC_H264_PROFILE_STEREO_GUID =
    { 0x40847bf5, 0x33f7, 0x4601, { 0x90, 0x84, 0xe8, 0xfe, 0x3c, 0x1d, 0xb8, 0xb7 } };

    // {CE788D20-AAA9-4318-92BB-AC7E858C8D36}
    static const GUID NV_ENC_H264_PROFILE_SVC_TEMPORAL_SCALABILTY =
    { 0xce788d20, 0xaaa9, 0x4318, { 0x92, 0xbb, 0xac, 0x7e, 0x85, 0x8c, 0x8d, 0x36 } };

    // {B405AFAC-F32B-417B-89C4-9ABEED3E5978}
    static const GUID NV_ENC_H264_PROFILE_PROGRESSIVE_HIGH_GUID =
    { 0xb405afac, 0xf32b, 0x417b, { 0x89, 0xc4, 0x9a, 0xbe, 0xed, 0x3e, 0x59, 0x78 } };

    // {AEC1BD87-E85B-48f2-84C3-98BCA6285072}
    static const GUID NV_ENC_H264_PROFILE_CONSTRAINED_HIGH_GUID =
    { 0xaec1bd87, 0xe85b, 0x48f2, { 0x84, 0xc3, 0x98, 0xbc, 0xa6, 0x28, 0x50, 0x72 } };

    // {B514C39A-B55B-40fa-878F-F1253B4DFDEC}
    static const GUID NV_ENC_HEVC_PROFILE_MAIN_GUID =
    { 0xb514c39a, 0xb55b, 0x40fa, { 0x87, 0x8f, 0xf1, 0x25, 0x3b, 0x4d, 0xfd, 0xec } };

    // {fa4d2b6c-3a5b-411a-8018-0a3f5e3c9be5}
    static const GUID NV_ENC_HEVC_PROFILE_MAIN10_GUID =
    { 0xfa4d2b6c, 0x3a5b, 0x411a, { 0x80, 0x18, 0x0a, 0x3f, 0x5e, 0x3c, 0x9b, 0xe5 } };

    // For HEVC Main 444 8 bit and HEVC Main 444 10 bit profiles only
    // {51ec32b5-1b4c-453c-9cbd-b616bd621341}
    static const GUID NV_ENC_HEVC_PROFILE_FREXT_GUID =
    { 0x51ec32b5, 0x1b4c, 0x453c, { 0x9c, 0xbd, 0xb6, 0x16, 0xbd, 0x62, 0x13, 0x41 } };

    // =========================================================================================
    // *   Preset GUIDS supported by the NvEncodeAPI interface.
    // =========================================================================================
    // {B2DFB705-4EBD-4C49-9B5F-24A777D3E587}
    static const GUID NV_ENC_PRESET_DEFAULT_GUID =
    { 0xb2dfb705, 0x4ebd, 0x4c49, { 0x9b, 0x5f, 0x24, 0xa7, 0x77, 0xd3, 0xe5, 0x87 } };

    // {60E4C59F-E846-4484-A56D-CD45BE9FDDF6}
    static const GUID NV_ENC_PRESET_HP_GUID =
    { 0x60e4c59f, 0xe846, 0x4484, { 0xa5, 0x6d, 0xcd, 0x45, 0xbe, 0x9f, 0xdd, 0xf6 } };

    // {34DBA71D-A77B-4B8F-9C3E-B6D5DA24C012}
    static const GUID NV_ENC_PRESET_HQ_GUID =
    { 0x34dba71d, 0xa77b, 0x4b8f, { 0x9c, 0x3e, 0xb6, 0xd5, 0xda, 0x24, 0xc0, 0x12 } };

    // {82E3E450-BDBB-4e40-989C-82A90DF9EF32}
    static const GUID NV_ENC_PRESET_BD_GUID  =
    { 0x82e3e450, 0xbdbb, 0x4e40, { 0x98, 0x9c, 0x82, 0xa9, 0xd, 0xf9, 0xef, 0x32 } };

    // {49DF21C5-6DFA-4feb-9787-6ACC9EFFB726}
    static const GUID NV_ENC_PRESET_LOW_LATENCY_DEFAULT_GUID  =
    { 0x49df21c5, 0x6dfa, 0x4feb, { 0x97, 0x87, 0x6a, 0xcc, 0x9e, 0xff, 0xb7, 0x26 } };

    // {C5F733B9-EA97-4cf9-BEC2-BF78A74FD105}
    static const GUID NV_ENC_PRESET_LOW_LATENCY_HQ_GUID  =
    { 0xc5f733b9, 0xea97, 0x4cf9, { 0xbe, 0xc2, 0xbf, 0x78, 0xa7, 0x4f, 0xd1, 0x5 } };

    // {67082A44-4BAD-48FA-98EA-93056D150A58}
    static const GUID NV_ENC_PRESET_LOW_LATENCY_HP_GUID =
    { 0x67082a44, 0x4bad, 0x48fa, { 0x98, 0xea, 0x93, 0x5, 0x6d, 0x15, 0xa, 0x58 } };

    // {D5BFB716-C604-44e7-9BB8-DEA5510FC3AC}
    static const GUID NV_ENC_PRESET_LOSSLESS_DEFAULT_GUID =
    { 0xd5bfb716, 0xc604, 0x44e7, { 0x9b, 0xb8, 0xde, 0xa5, 0x51, 0xf, 0xc3, 0xac } };

    // {149998E7-2364-411d-82EF-179888093409}
    static const GUID NV_ENC_PRESET_LOSSLESS_HP_GUID =
    { 0x149998e7, 0x2364, 0x411d, { 0x82, 0xef, 0x17, 0x98, 0x88, 0x9, 0x34, 0x9 } };
}

#[doc = " Frame mode"]
pub const _NV_ENC_PARAMS_FRAME_FIELD_MODE_NV_ENC_PARAMS_FRAME_FIELD_MODE_FRAME:
    _NV_ENC_PARAMS_FRAME_FIELD_MODE = 1;
#[doc = " Field mode"]
pub const _NV_ENC_PARAMS_FRAME_FIELD_MODE_NV_ENC_PARAMS_FRAME_FIELD_MODE_FIELD:
    _NV_ENC_PARAMS_FRAME_FIELD_MODE = 2;
#[doc = " MB adaptive frame/field"]
pub const _NV_ENC_PARAMS_FRAME_FIELD_MODE_NV_ENC_PARAMS_FRAME_FIELD_MODE_MBAFF:
    _NV_ENC_PARAMS_FRAME_FIELD_MODE = 3;
#[doc = " Input frame encode modes"]
pub type _NV_ENC_PARAMS_FRAME_FIELD_MODE = i32;
pub use self::_NV_ENC_PARAMS_FRAME_FIELD_MODE as NV_ENC_PARAMS_FRAME_FIELD_MODE;
#[doc = " Constant QP mode"]
pub const _NV_ENC_PARAMS_RC_MODE_NV_ENC_PARAMS_RC_CONSTQP: _NV_ENC_PARAMS_RC_MODE = 0;
#[doc = " Variable bitrate mode"]
pub const _NV_ENC_PARAMS_RC_MODE_NV_ENC_PARAMS_RC_VBR: _NV_ENC_PARAMS_RC_MODE = 1;
#[doc = " Constant bitrate mode"]
pub const _NV_ENC_PARAMS_RC_MODE_NV_ENC_PARAMS_RC_CBR: _NV_ENC_PARAMS_RC_MODE = 2;
#[doc = " low-delay CBR, high quality"]
pub const _NV_ENC_PARAMS_RC_MODE_NV_ENC_PARAMS_RC_CBR_LOWDELAY_HQ: _NV_ENC_PARAMS_RC_MODE = 8;
#[doc = " CBR, high quality (slower)"]
pub const _NV_ENC_PARAMS_RC_MODE_NV_ENC_PARAMS_RC_CBR_HQ: _NV_ENC_PARAMS_RC_MODE = 16;
#[doc = " VBR, high quality (slower)"]
pub const _NV_ENC_PARAMS_RC_MODE_NV_ENC_PARAMS_RC_VBR_HQ: _NV_ENC_PARAMS_RC_MODE = 32;
#[doc = " Rate Control Modes"]
pub type _NV_ENC_PARAMS_RC_MODE = i32;
pub use self::_NV_ENC_PARAMS_RC_MODE as NV_ENC_PARAMS_RC_MODE;
#[doc = " Emphasis Map Level 0, for zero Delta QP value"]
pub const _NV_ENC_EMPHASIS_MAP_LEVEL_NV_ENC_EMPHASIS_MAP_LEVEL_0: _NV_ENC_EMPHASIS_MAP_LEVEL = 0;
#[doc = " Emphasis Map Level 1, for very low Delta QP value"]
pub const _NV_ENC_EMPHASIS_MAP_LEVEL_NV_ENC_EMPHASIS_MAP_LEVEL_1: _NV_ENC_EMPHASIS_MAP_LEVEL = 1;
#[doc = " Emphasis Map Level 2, for low Delta QP value"]
pub const _NV_ENC_EMPHASIS_MAP_LEVEL_NV_ENC_EMPHASIS_MAP_LEVEL_2: _NV_ENC_EMPHASIS_MAP_LEVEL = 2;
#[doc = " Emphasis Map Level 3, for medium Delta QP value"]
pub const _NV_ENC_EMPHASIS_MAP_LEVEL_NV_ENC_EMPHASIS_MAP_LEVEL_3: _NV_ENC_EMPHASIS_MAP_LEVEL = 3;
#[doc = " Emphasis Map Level 4, for high Delta QP value"]
pub const _NV_ENC_EMPHASIS_MAP_LEVEL_NV_ENC_EMPHASIS_MAP_LEVEL_4: _NV_ENC_EMPHASIS_MAP_LEVEL = 4;
#[doc = " Emphasis Map Level 5, for very high Delta QP value"]
pub const _NV_ENC_EMPHASIS_MAP_LEVEL_NV_ENC_EMPHASIS_MAP_LEVEL_5: _NV_ENC_EMPHASIS_MAP_LEVEL = 5;
#[doc = " Emphasis Levels"]
pub type _NV_ENC_EMPHASIS_MAP_LEVEL = i32;
pub use self::_NV_ENC_EMPHASIS_MAP_LEVEL as NV_ENC_EMPHASIS_MAP_LEVEL;
#[doc = " Value in NV_ENC_PIC_PARAMS::qpDeltaMap have no effect."]
pub const _NV_ENC_QP_MAP_MODE_NV_ENC_QP_MAP_DISABLED: _NV_ENC_QP_MAP_MODE = 0;
#[doc = " Value in NV_ENC_PIC_PARAMS::qpDeltaMap will be treated as Empasis level. Currently this is only supported for H264"]
pub const _NV_ENC_QP_MAP_MODE_NV_ENC_QP_MAP_EMPHASIS: _NV_ENC_QP_MAP_MODE = 1;
#[doc = " Value in NV_ENC_PIC_PARAMS::qpDeltaMap will be treated as QP delta map."]
pub const _NV_ENC_QP_MAP_MODE_NV_ENC_QP_MAP_DELTA: _NV_ENC_QP_MAP_MODE = 2;
#[doc = " Currently This is not supported. Value in NV_ENC_PIC_PARAMS::qpDeltaMap will be treated as QP value."]
pub const _NV_ENC_QP_MAP_MODE_NV_ENC_QP_MAP: _NV_ENC_QP_MAP_MODE = 3;
#[doc = " QP MAP MODE"]
pub type _NV_ENC_QP_MAP_MODE = i32;
pub use self::_NV_ENC_QP_MAP_MODE as NV_ENC_QP_MAP_MODE;
#[doc = " Progressive frame"]
pub const NV_ENC_PIC_STRUCT_FRAME: _NV_ENC_PIC_STRUCT = 1;
#[doc = " Field encoding top field first"]
pub const NV_ENC_PIC_STRUCT_FIELD_TOP_BOTTOM: _NV_ENC_PIC_STRUCT = 2;
#[doc = " Field encoding bottom field first"]
pub const NV_ENC_PIC_STRUCT_FIELD_BOTTOM_TOP: _NV_ENC_PIC_STRUCT = 3;
#[doc = " Input picture structure"]
pub type _NV_ENC_PIC_STRUCT = i32;
pub use self::_NV_ENC_PIC_STRUCT as NV_ENC_PIC_STRUCT;
#[doc = " Forward predicted"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_P: _NV_ENC_PIC_TYPE = 0;
#[doc = " Bi-directionally predicted picture"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_B: _NV_ENC_PIC_TYPE = 1;
#[doc = " Intra predicted picture"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_I: _NV_ENC_PIC_TYPE = 2;
#[doc = " IDR picture"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_IDR: _NV_ENC_PIC_TYPE = 3;
#[doc = " Bi-directionally predicted with only Intra MBs"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_BI: _NV_ENC_PIC_TYPE = 4;
#[doc = " Picture is skipped"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_SKIPPED: _NV_ENC_PIC_TYPE = 5;
#[doc = " First picture in intra refresh cycle"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_INTRA_REFRESH: _NV_ENC_PIC_TYPE = 6;
#[doc = " Non reference P picture"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_NONREF_P: _NV_ENC_PIC_TYPE = 7;
#[doc = " Picture type unknown"]
pub const _NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_UNKNOWN: _NV_ENC_PIC_TYPE = 255;
#[doc = " Input picture type"]
pub type _NV_ENC_PIC_TYPE = i32;
pub use self::_NV_ENC_PIC_TYPE as NV_ENC_PIC_TYPE;
#[doc = "Driver selects QuarterPel motion vector precision by default"]
pub const _NV_ENC_MV_PRECISION_NV_ENC_MV_PRECISION_DEFAULT: _NV_ENC_MV_PRECISION = 0;
#[doc = " FullPel  motion vector precision"]
pub const _NV_ENC_MV_PRECISION_NV_ENC_MV_PRECISION_FULL_PEL: _NV_ENC_MV_PRECISION = 1;
#[doc = " HalfPel motion vector precision"]
pub const _NV_ENC_MV_PRECISION_NV_ENC_MV_PRECISION_HALF_PEL: _NV_ENC_MV_PRECISION = 2;
#[doc = " QuarterPel motion vector precision"]
pub const _NV_ENC_MV_PRECISION_NV_ENC_MV_PRECISION_QUARTER_PEL: _NV_ENC_MV_PRECISION = 3;
#[doc = " Motion vector precisions"]
pub type _NV_ENC_MV_PRECISION = i32;
pub use self::_NV_ENC_MV_PRECISION as NV_ENC_MV_PRECISION;
#[doc = " Undefined buffer format"]
pub const NV_ENC_BUFFER_FORMAT_UNDEFINED: _NV_ENC_BUFFER_FORMAT = 0;
#[doc = " Semi-Planar YUV [Y plane followed by interleaved UV plane]"]
pub const NV_ENC_BUFFER_FORMAT_NV12: _NV_ENC_BUFFER_FORMAT = 1;
#[doc = " Planar YUV [Y plane followed by V and U planes]"]
pub const NV_ENC_BUFFER_FORMAT_YV12: _NV_ENC_BUFFER_FORMAT = 16;
#[doc = " Planar YUV [Y plane followed by U and V planes]"]
pub const NV_ENC_BUFFER_FORMAT_IYUV: _NV_ENC_BUFFER_FORMAT = 256;
#[doc = " Planar YUV [Y plane followed by U and V planes]"]
pub const NV_ENC_BUFFER_FORMAT_YUV444: _NV_ENC_BUFFER_FORMAT = 4096;
#[doc = " 10 bit Semi-Planar YUV [Y plane followed by interleaved UV plane]. Each pixel of size 2 bytes. Most Significant 10 bits contain pixel data."]
pub const NV_ENC_BUFFER_FORMAT_YUV420_10BIT: _NV_ENC_BUFFER_FORMAT = 65536;
#[doc = " 10 bit Planar YUV444 [Y plane followed by U and V planes]. Each pixel of size 2 bytes. Most Significant 10 bits contain pixel data."]
pub const NV_ENC_BUFFER_FORMAT_YUV444_10BIT: _NV_ENC_BUFFER_FORMAT = 1048576;
#[doc = " 8 bit Packed A8R8G8B8. This is a word-ordered format"]
#[doc = "where a pixel is represented by a 32-bit word with B"]
#[doc = "in the lowest 8 bits, G in the next 8 bits, R in the"]
#[doc = "8 bits after that and A in the highest 8 bits."]
pub const NV_ENC_BUFFER_FORMAT_ARGB: _NV_ENC_BUFFER_FORMAT = 16777216;
#[doc = " 10 bit Packed A2R10G10B10. This is a word-ordered format"]
#[doc = "where a pixel is represented by a 32-bit word with B"]
#[doc = "in the lowest 10 bits, G in the next 10 bits, R in the"]
#[doc = "10 bits after that and A in the highest 2 bits."]
pub const NV_ENC_BUFFER_FORMAT_ARGB10: _NV_ENC_BUFFER_FORMAT = 33554432;
#[doc = " 8 bit Packed A8Y8U8V8. This is a word-ordered format"]
#[doc = "where a pixel is represented by a 32-bit word with V"]
#[doc = "in the lowest 8 bits, U in the next 8 bits, Y in the"]
#[doc = "8 bits after that and A in the highest 8 bits."]
pub const NV_ENC_BUFFER_FORMAT_AYUV: _NV_ENC_BUFFER_FORMAT = 67108864;
#[doc = " 8 bit Packed A8B8G8R8. This is a word-ordered format"]
#[doc = "where a pixel is represented by a 32-bit word with R"]
#[doc = "in the lowest 8 bits, G in the next 8 bits, B in the"]
#[doc = "8 bits after that and A in the highest 8 bits."]
pub const NV_ENC_BUFFER_FORMAT_ABGR: _NV_ENC_BUFFER_FORMAT = 268435456;
#[doc = " 10 bit Packed A2B10G10R10. This is a word-ordered format"]
#[doc = "where a pixel is represented by a 32-bit word with R"]
#[doc = "in the lowest 10 bits, G in the next 10 bits, B in the"]
#[doc = "10 bits after that and A in the highest 2 bits."]
pub const NV_ENC_BUFFER_FORMAT_ABGR10: _NV_ENC_BUFFER_FORMAT = 536870912;
#[doc = " Buffer format representing one-dimensional buffer."]
#[doc = "This format should be used only when registering the"]
#[doc = "resource as output buffer, which will be used to write"]
#[doc = "the encoded bit stream or H.264 ME only mode output."]
pub const NV_ENC_BUFFER_FORMAT_U8: _NV_ENC_BUFFER_FORMAT = 1073741824;
#[doc = " Input buffer formats"]
pub type _NV_ENC_BUFFER_FORMAT = i32;
pub use self::_NV_ENC_BUFFER_FORMAT as NV_ENC_BUFFER_FORMAT;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_AUTOSELECT: _NV_ENC_LEVEL = 0;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_1: _NV_ENC_LEVEL = 10;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_1b: _NV_ENC_LEVEL = 9;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_11: _NV_ENC_LEVEL = 11;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_12: _NV_ENC_LEVEL = 12;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_13: _NV_ENC_LEVEL = 13;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_2: _NV_ENC_LEVEL = 20;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_21: _NV_ENC_LEVEL = 21;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_22: _NV_ENC_LEVEL = 22;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_3: _NV_ENC_LEVEL = 30;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_31: _NV_ENC_LEVEL = 31;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_32: _NV_ENC_LEVEL = 32;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_4: _NV_ENC_LEVEL = 40;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_41: _NV_ENC_LEVEL = 41;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_42: _NV_ENC_LEVEL = 42;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_5: _NV_ENC_LEVEL = 50;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_51: _NV_ENC_LEVEL = 51;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_H264_52: _NV_ENC_LEVEL = 52;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_1: _NV_ENC_LEVEL = 30;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_2: _NV_ENC_LEVEL = 60;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_21: _NV_ENC_LEVEL = 63;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_3: _NV_ENC_LEVEL = 90;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_31: _NV_ENC_LEVEL = 93;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_4: _NV_ENC_LEVEL = 120;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_41: _NV_ENC_LEVEL = 123;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_5: _NV_ENC_LEVEL = 150;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_51: _NV_ENC_LEVEL = 153;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_52: _NV_ENC_LEVEL = 156;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_6: _NV_ENC_LEVEL = 180;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_61: _NV_ENC_LEVEL = 183;
pub const _NV_ENC_LEVEL_NV_ENC_LEVEL_HEVC_62: _NV_ENC_LEVEL = 186;
pub const _NV_ENC_LEVEL_NV_ENC_TIER_HEVC_MAIN: _NV_ENC_LEVEL = 0;
pub const _NV_ENC_LEVEL_NV_ENC_TIER_HEVC_HIGH: _NV_ENC_LEVEL = 1;
#[doc = " Encoding levels"]
pub type _NV_ENC_LEVEL = i32;
pub use self::_NV_ENC_LEVEL as NV_ENC_LEVEL;
#[doc = " This indicates that API call returned with no errors."]
pub const NV_ENC_SUCCESS: _NVENCSTATUS = 0;
#[doc = " This indicates that no encode capable devices were detected."]
pub const NV_ENC_ERR_NO_ENCODE_DEVICE: _NVENCSTATUS = 1;
#[doc = " This indicates that devices pass by the client is not supported."]
pub const NV_ENC_ERR_UNSUPPORTED_DEVICE: _NVENCSTATUS = 2;
#[doc = " This indicates that the encoder device supplied by the client is not"]
#[doc = " valid."]
pub const NV_ENC_ERR_INVALID_ENCODERDEVICE: _NVENCSTATUS = 3;
#[doc = " This indicates that device passed to the API call is invalid."]
pub const NV_ENC_ERR_INVALID_DEVICE: _NVENCSTATUS = 4;
#[doc = " This indicates that device passed to the API call is no longer available and"]
#[doc = " needs to be reinitialized. The clients need to destroy the current encoder"]
#[doc = " session by freeing the allocated input output buffers and destroying the device"]
#[doc = " and create a new encoding session."]
pub const NV_ENC_ERR_DEVICE_NOT_EXIST: _NVENCSTATUS = 5;
#[doc = " This indicates that one or more of the pointers passed to the API call"]
#[doc = " is invalid."]
pub const NV_ENC_ERR_INVALID_PTR: _NVENCSTATUS = 6;
#[doc = " This indicates that completion event passed in ::NvEncEncodePicture() call"]
#[doc = " is invalid."]
pub const NV_ENC_ERR_INVALID_EVENT: _NVENCSTATUS = 7;
#[doc = " This indicates that one or more of the parameter passed to the API call"]
#[doc = " is invalid."]
pub const NV_ENC_ERR_INVALID_PARAM: _NVENCSTATUS = 8;
#[doc = " This indicates that an API call was made in wrong sequence/order."]
pub const NV_ENC_ERR_INVALID_CALL: _NVENCSTATUS = 9;
#[doc = " This indicates that the API call failed because it was unable to allocate"]
#[doc = " enough memory to perform the requested operation."]
pub const NV_ENC_ERR_OUT_OF_MEMORY: _NVENCSTATUS = 10;
#[doc = " This indicates that the encoder has not been initialized with"]
#[doc = " ::NvEncInitializeEncoder() or that initialization has failed."]
#[doc = " The client cannot allocate input or output buffers or do any encoding"]
#[doc = " related operation before successfully initializing the encoder."]
pub const NV_ENC_ERR_ENCODER_NOT_INITIALIZED: _NVENCSTATUS = 11;
#[doc = " This indicates that an unsupported parameter was passed by the client."]
pub const NV_ENC_ERR_UNSUPPORTED_PARAM: _NVENCSTATUS = 12;
#[doc = " This indicates that the ::NvEncLockBitstream() failed to lock the output"]
#[doc = " buffer. This happens when the client makes a non blocking lock call to"]
#[doc = " access the output bitstream by passing NV_ENC_LOCK_BITSTREAM::doNotWait flag."]
#[doc = " This is not a fatal error and client should retry the same operation after"]
#[doc = " few milliseconds."]
pub const NV_ENC_ERR_LOCK_BUSY: _NVENCSTATUS = 13;
#[doc = " This indicates that the size of the user buffer passed by the client is"]
#[doc = " insufficient for the requested operation."]
pub const NV_ENC_ERR_NOT_ENOUGH_BUFFER: _NVENCSTATUS = 14;
#[doc = " This indicates that an invalid struct version was used by the client."]
pub const NV_ENC_ERR_INVALID_VERSION: _NVENCSTATUS = 15;
#[doc = " This indicates that ::NvEncMapInputResource() API failed to map the client"]
#[doc = " provided input resource."]
pub const NV_ENC_ERR_MAP_FAILED: _NVENCSTATUS = 16;
#[doc = " This indicates encode driver requires more input buffers to produce an output"]
#[doc = " bitstream. If this error is returned from ::NvEncEncodePicture() API, this"]
#[doc = " is not a fatal error. If the client is encoding with B frames then,"]
#[doc = " ::NvEncEncodePicture() API might be buffering the input frame for re-ordering."]
#[doc = ""]
#[doc = " A client operating in synchronous mode cannot call ::NvEncLockBitstream()"]
#[doc = " API on the output bitstream buffer if ::NvEncEncodePicture() returned the"]
#[doc = " ::NV_ENC_ERR_NEED_MORE_INPUT error code."]
#[doc = " The client must continue providing input frames until encode driver returns"]
#[doc = " ::NV_ENC_SUCCESS. After receiving ::NV_ENC_SUCCESS status the client can call"]
#[doc = " ::NvEncLockBitstream() API on the output buffers in the same order in which"]
#[doc = " it has called ::NvEncEncodePicture()."]
pub const NV_ENC_ERR_NEED_MORE_INPUT: _NVENCSTATUS = 17;
#[doc = " This indicates that the HW encoder is busy encoding and is unable to encode"]
#[doc = " the input. The client should call ::NvEncEncodePicture() again after few"]
#[doc = " milliseconds."]
pub const NV_ENC_ERR_ENCODER_BUSY: _NVENCSTATUS = 18;
#[doc = " This indicates that the completion event passed in ::NvEncEncodePicture()"]
#[doc = " API has not been registered with encoder driver using ::NvEncRegisterAsyncEvent()."]
pub const NV_ENC_ERR_EVENT_NOT_REGISTERD: _NVENCSTATUS = 19;
#[doc = " This indicates that an unknown internal error has occurred."]
pub const NV_ENC_ERR_GENERIC: _NVENCSTATUS = 20;
#[doc = " This indicates that the client is attempting to use a feature"]
#[doc = " that is not available for the license type for the current system."]
pub const NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY: _NVENCSTATUS = 21;
#[doc = " This indicates that the client is attempting to use a feature"]
#[doc = " that is not implemented for the current version."]
pub const NV_ENC_ERR_UNIMPLEMENTED: _NVENCSTATUS = 22;
#[doc = " This indicates that the ::NvEncRegisterResource API failed to register the resource."]
pub const NV_ENC_ERR_RESOURCE_REGISTER_FAILED: _NVENCSTATUS = 23;
#[doc = " This indicates that the client is attempting to unregister a resource"]
#[doc = " that has not been successfully registered."]
pub const NV_ENC_ERR_RESOURCE_NOT_REGISTERED: _NVENCSTATUS = 24;
#[doc = " This indicates that the client is attempting to unmap a resource"]
#[doc = " that has not been successfully mapped."]
pub const NV_ENC_ERR_RESOURCE_NOT_MAPPED: _NVENCSTATUS = 25;
#[doc = " Error Codes"]
pub type _NVENCSTATUS = i32;
pub use self::_NVENCSTATUS as NVENCSTATUS;
#[doc = " Encode the current picture as an Intra picture"]
pub const _NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_FORCEINTRA: _NV_ENC_PIC_FLAGS = 1;
#[doc = " Encode the current picture as an IDR picture."]
#[doc = "This flag is only valid when Picture type decision is taken by the Encoder"]
#[doc = "[_NV_ENC_INITIALIZE_PARAMS::enablePTD == 1]."]
pub const _NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_FORCEIDR: _NV_ENC_PIC_FLAGS = 2;
#[doc = " Write the sequence and picture header in encoded bitstream of the current picture"]
pub const _NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_OUTPUT_SPSPPS: _NV_ENC_PIC_FLAGS = 4;
#[doc = " Indicates end of the input stream"]
pub const _NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_EOS: _NV_ENC_PIC_FLAGS = 8;
#[doc = " Encode Picture encode flags."]
pub type _NV_ENC_PIC_FLAGS = i32;
pub use self::_NV_ENC_PIC_FLAGS as NV_ENC_PIC_FLAGS;
#[doc = " Memory heap to be decided by the encoder driver based on the usage"]
pub const _NV_ENC_MEMORY_HEAP_NV_ENC_MEMORY_HEAP_AUTOSELECT: _NV_ENC_MEMORY_HEAP = 0;
#[doc = " Memory heap is in local video memory"]
pub const _NV_ENC_MEMORY_HEAP_NV_ENC_MEMORY_HEAP_VID: _NV_ENC_MEMORY_HEAP = 1;
#[doc = " Memory heap is in cached system memory"]
pub const _NV_ENC_MEMORY_HEAP_NV_ENC_MEMORY_HEAP_SYSMEM_CACHED: _NV_ENC_MEMORY_HEAP = 2;
#[doc = " Memory heap is in uncached system memory"]
pub const _NV_ENC_MEMORY_HEAP_NV_ENC_MEMORY_HEAP_SYSMEM_UNCACHED: _NV_ENC_MEMORY_HEAP = 3;
#[doc = " Memory heap to allocate input and output buffers."]
pub type _NV_ENC_MEMORY_HEAP = i32;
pub use self::_NV_ENC_MEMORY_HEAP as NV_ENC_MEMORY_HEAP;
#[doc = " B frame is not used for reference"]
pub const _NV_ENC_BFRAME_REF_MODE_NV_ENC_BFRAME_REF_MODE_DISABLED: _NV_ENC_BFRAME_REF_MODE = 0;
#[doc = " Each B-frame will be used for reference. currently not supported for H.264"]
pub const _NV_ENC_BFRAME_REF_MODE_NV_ENC_BFRAME_REF_MODE_EACH: _NV_ENC_BFRAME_REF_MODE = 1;
#[doc = " Only(Number of B-frame)/2 th B-frame will be used for reference"]
pub const _NV_ENC_BFRAME_REF_MODE_NV_ENC_BFRAME_REF_MODE_MIDDLE: _NV_ENC_BFRAME_REF_MODE = 2;
#[doc = " B-frame used as reference modes"]
pub type _NV_ENC_BFRAME_REF_MODE = i32;
pub use self::_NV_ENC_BFRAME_REF_MODE as NV_ENC_BFRAME_REF_MODE;
#[doc = " Entropy coding mode is auto selected by the encoder driver"]
pub const _NV_ENC_H264_ENTROPY_CODING_MODE_NV_ENC_H264_ENTROPY_CODING_MODE_AUTOSELECT:
    _NV_ENC_H264_ENTROPY_CODING_MODE = 0;
#[doc = " Entropy coding mode is CABAC"]
pub const _NV_ENC_H264_ENTROPY_CODING_MODE_NV_ENC_H264_ENTROPY_CODING_MODE_CABAC:
    _NV_ENC_H264_ENTROPY_CODING_MODE = 1;
#[doc = " Entropy coding mode is CAVLC"]
pub const _NV_ENC_H264_ENTROPY_CODING_MODE_NV_ENC_H264_ENTROPY_CODING_MODE_CAVLC:
    _NV_ENC_H264_ENTROPY_CODING_MODE = 2;
#[doc = " H.264 entropy coding modes."]
pub type _NV_ENC_H264_ENTROPY_CODING_MODE = i32;
pub use self::_NV_ENC_H264_ENTROPY_CODING_MODE as NV_ENC_H264_ENTROPY_CODING_MODE;
#[doc = " BDirect mode is auto selected by the encoder driver"]
pub const _NV_ENC_H264_BDIRECT_MODE_NV_ENC_H264_BDIRECT_MODE_AUTOSELECT: _NV_ENC_H264_BDIRECT_MODE =
    0;
#[doc = " Disable BDirect mode"]
pub const _NV_ENC_H264_BDIRECT_MODE_NV_ENC_H264_BDIRECT_MODE_DISABLE: _NV_ENC_H264_BDIRECT_MODE = 1;
#[doc = " Temporal BDirect mode"]
pub const _NV_ENC_H264_BDIRECT_MODE_NV_ENC_H264_BDIRECT_MODE_TEMPORAL: _NV_ENC_H264_BDIRECT_MODE =
    2;
#[doc = " Spatial BDirect mode"]
pub const _NV_ENC_H264_BDIRECT_MODE_NV_ENC_H264_BDIRECT_MODE_SPATIAL: _NV_ENC_H264_BDIRECT_MODE = 3;
#[doc = " H.264 specific Bdirect modes"]
pub type _NV_ENC_H264_BDIRECT_MODE = i32;
pub use self::_NV_ENC_H264_BDIRECT_MODE as NV_ENC_H264_BDIRECT_MODE;
#[doc = " FMO usage is auto selected by the encoder driver"]
pub const _NV_ENC_H264_FMO_MODE_NV_ENC_H264_FMO_AUTOSELECT: _NV_ENC_H264_FMO_MODE = 0;
#[doc = " Enable FMO"]
pub const _NV_ENC_H264_FMO_MODE_NV_ENC_H264_FMO_ENABLE: _NV_ENC_H264_FMO_MODE = 1;
#[doc = " Disble FMO"]
pub const _NV_ENC_H264_FMO_MODE_NV_ENC_H264_FMO_DISABLE: _NV_ENC_H264_FMO_MODE = 2;
#[doc = " H.264 specific FMO usage"]
pub type _NV_ENC_H264_FMO_MODE = i32;
pub use self::_NV_ENC_H264_FMO_MODE as NV_ENC_H264_FMO_MODE;
#[doc = " Adaptive Transform 8x8 mode is auto selected by the encoder driver"]
pub const _NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE_NV_ENC_H264_ADAPTIVE_TRANSFORM_AUTOSELECT:
    _NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE = 0;
#[doc = " Adaptive Transform 8x8 mode disabled"]
pub const _NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE_NV_ENC_H264_ADAPTIVE_TRANSFORM_DISABLE:
    _NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE = 1;
#[doc = " Adaptive Transform 8x8 mode should be used"]
pub const _NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE_NV_ENC_H264_ADAPTIVE_TRANSFORM_ENABLE:
    _NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE = 2;
#[doc = " H.264 specific Adaptive Transform modes"]
pub type _NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE = i32;
pub use self::_NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE as NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE;
#[doc = " No Stereo packing required"]
pub const _NV_ENC_STEREO_PACKING_MODE_NV_ENC_STEREO_PACKING_MODE_NONE: _NV_ENC_STEREO_PACKING_MODE =
    0;
#[doc = " Checkerboard mode for packing stereo frames"]
pub const _NV_ENC_STEREO_PACKING_MODE_NV_ENC_STEREO_PACKING_MODE_CHECKERBOARD:
    _NV_ENC_STEREO_PACKING_MODE = 1;
#[doc = " Column Interleave mode for packing stereo frames"]
pub const _NV_ENC_STEREO_PACKING_MODE_NV_ENC_STEREO_PACKING_MODE_COLINTERLEAVE:
    _NV_ENC_STEREO_PACKING_MODE = 2;
#[doc = " Row Interleave mode for packing stereo frames"]
pub const _NV_ENC_STEREO_PACKING_MODE_NV_ENC_STEREO_PACKING_MODE_ROWINTERLEAVE:
    _NV_ENC_STEREO_PACKING_MODE = 3;
#[doc = " Side-by-side mode for packing stereo frames"]
pub const _NV_ENC_STEREO_PACKING_MODE_NV_ENC_STEREO_PACKING_MODE_SIDEBYSIDE:
    _NV_ENC_STEREO_PACKING_MODE = 4;
#[doc = " Top-Bottom mode for packing stereo frames"]
pub const _NV_ENC_STEREO_PACKING_MODE_NV_ENC_STEREO_PACKING_MODE_TOPBOTTOM:
    _NV_ENC_STEREO_PACKING_MODE = 5;
#[doc = " Frame Sequential mode for packing stereo frames"]
pub const _NV_ENC_STEREO_PACKING_MODE_NV_ENC_STEREO_PACKING_MODE_FRAMESEQ:
    _NV_ENC_STEREO_PACKING_MODE = 6;
#[doc = " Stereo frame packing modes."]
pub type _NV_ENC_STEREO_PACKING_MODE = i32;
pub use self::_NV_ENC_STEREO_PACKING_MODE as NV_ENC_STEREO_PACKING_MODE;
#[doc = " input resource type is a directx9 surface"]
pub const NV_ENC_INPUT_RESOURCE_TYPE_DIRECTX: _NV_ENC_INPUT_RESOURCE_TYPE = 0;
#[doc = " input resource type is a cuda device pointer surface"]
pub const NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR: _NV_ENC_INPUT_RESOURCE_TYPE = 1;
#[doc = " input resource type is a cuda array surface."]
#[doc = "This array must be a 2D array and the CUDA_ARRAY3D_SURFACE_LDST"]
#[doc = "flag must have been specified when creating it."]
pub const NV_ENC_INPUT_RESOURCE_TYPE_CUDAARRAY: _NV_ENC_INPUT_RESOURCE_TYPE = 2;
#[doc = " input resource type is an OpenGL texture"]
pub const NV_ENC_INPUT_RESOURCE_TYPE_OPENGL_TEX: _NV_ENC_INPUT_RESOURCE_TYPE = 3;
#[doc = "  Input Resource type"]
pub type _NV_ENC_INPUT_RESOURCE_TYPE = i32;
pub use self::_NV_ENC_INPUT_RESOURCE_TYPE as NV_ENC_INPUT_RESOURCE_TYPE;
#[doc = " Registered surface will be used for input image"]
pub const NV_ENC_INPUT_IMAGE: _NV_ENC_BUFFER_USAGE = 0;
#[doc = " Registered surface will be used for output of H.264 ME only mode."]
#[doc = "This buffer usage type is not supported for HEVC ME only mode."]
pub const NV_ENC_OUTPUT_MOTION_VECTOR: _NV_ENC_BUFFER_USAGE = 1;
#[doc = " Registered surface will be used for output bitstream in encoding"]
pub const NV_ENC_OUTPUT_BITSTREAM: _NV_ENC_BUFFER_USAGE = 2;
#[doc = "  Buffer usage"]
pub type _NV_ENC_BUFFER_USAGE = i32;
pub use self::_NV_ENC_BUFFER_USAGE as NV_ENC_BUFFER_USAGE;
#[doc = " encode device type is a directx9 device"]
pub const NV_ENC_DEVICE_TYPE_DIRECTX: _NV_ENC_DEVICE_TYPE = 0;
#[doc = " encode device type is a cuda device"]
pub const NV_ENC_DEVICE_TYPE_CUDA: _NV_ENC_DEVICE_TYPE = 1;
#[doc = " encode device type is an OpenGL device."]
#[doc = "Use of this device type is supported only on Linux"]
pub const NV_ENC_DEVICE_TYPE_OPENGL: _NV_ENC_DEVICE_TYPE = 2;
#[doc = "  Encoder Device type"]
pub type _NV_ENC_DEVICE_TYPE = i32;
pub use self::_NV_ENC_DEVICE_TYPE as NV_ENC_DEVICE_TYPE;
#[doc = " Number of reference frames is auto selected by the encoder driver"]
pub const _NV_ENC_NUM_REF_FRAMES_NV_ENC_NUM_REF_FRAMES_AUTOSELECT: _NV_ENC_NUM_REF_FRAMES = 0;
#[doc = " Number of reference frames equal to 1"]
pub const _NV_ENC_NUM_REF_FRAMES_NV_ENC_NUM_REF_FRAMES_1: _NV_ENC_NUM_REF_FRAMES = 1;
#[doc = " Number of reference frames equal to 2"]
pub const _NV_ENC_NUM_REF_FRAMES_NV_ENC_NUM_REF_FRAMES_2: _NV_ENC_NUM_REF_FRAMES = 2;
#[doc = " Number of reference frames equal to 3"]
pub const _NV_ENC_NUM_REF_FRAMES_NV_ENC_NUM_REF_FRAMES_3: _NV_ENC_NUM_REF_FRAMES = 3;
#[doc = " Number of reference frames equal to 4"]
pub const _NV_ENC_NUM_REF_FRAMES_NV_ENC_NUM_REF_FRAMES_4: _NV_ENC_NUM_REF_FRAMES = 4;
#[doc = " Number of reference frames equal to 5"]
pub const _NV_ENC_NUM_REF_FRAMES_NV_ENC_NUM_REF_FRAMES_5: _NV_ENC_NUM_REF_FRAMES = 5;
#[doc = " Number of reference frames equal to 6"]
pub const _NV_ENC_NUM_REF_FRAMES_NV_ENC_NUM_REF_FRAMES_6: _NV_ENC_NUM_REF_FRAMES = 6;
#[doc = " Number of reference frames equal to 7"]
pub const _NV_ENC_NUM_REF_FRAMES_NV_ENC_NUM_REF_FRAMES_7: _NV_ENC_NUM_REF_FRAMES = 7;
#[doc = " Number of reference frames"]
pub type _NV_ENC_NUM_REF_FRAMES = i32;
pub use self::_NV_ENC_NUM_REF_FRAMES as NV_ENC_NUM_REF_FRAMES;
#[doc = " Maximum number of B-Frames supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_NUM_MAX_BFRAMES: _NV_ENC_CAPS = 0;
#[doc = " Rate control modes supported."]
#[doc = " \\n The API return value is a bitmask of the values in NV_ENC_PARAMS_RC_MODE."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORTED_RATECONTROL_MODES: _NV_ENC_CAPS = 1;
#[doc = " Indicates HW support for field mode encoding."]
#[doc = " \\n 0 : Interlaced mode encoding is not supported."]
#[doc = " \\n 1 : Interlaced field mode encoding is supported."]
#[doc = " \\n 2 : Interlaced frame encoding and field mode encoding are both supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_FIELD_ENCODING: _NV_ENC_CAPS = 2;
#[doc = " Indicates HW support for monochrome mode encoding."]
#[doc = " \\n 0 : Monochrome mode not supported."]
#[doc = " \\n 1 : Monochrome mode supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_MONOCHROME: _NV_ENC_CAPS = 3;
#[doc = " Indicates HW support for FMO."]
#[doc = " \\n 0 : FMO not supported."]
#[doc = " \\n 1 : FMO supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_FMO: _NV_ENC_CAPS = 4;
#[doc = " Indicates HW capability for Quarter pel motion estimation."]
#[doc = " \\n 0 : QuarterPel Motion Estimation not supported."]
#[doc = " \\n 1 : QuarterPel Motion Estimation supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_QPELMV: _NV_ENC_CAPS = 5;
#[doc = " H.264 specific. Indicates HW support for BDirect modes."]
#[doc = " \\n 0 : BDirect mode encoding not supported."]
#[doc = " \\n 1 : BDirect mode encoding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_BDIRECT_MODE: _NV_ENC_CAPS = 6;
#[doc = " H264 specific. Indicates HW support for CABAC entropy coding mode."]
#[doc = " \\n 0 : CABAC entropy coding not supported."]
#[doc = " \\n 1 : CABAC entropy coding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_CABAC: _NV_ENC_CAPS = 7;
#[doc = " Indicates HW support for Adaptive Transform."]
#[doc = " \\n 0 : Adaptive Transform not supported."]
#[doc = " \\n 1 : Adaptive Transform supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_ADAPTIVE_TRANSFORM: _NV_ENC_CAPS = 8;
#[doc = " Indicates HW support for Multi View Coding."]
#[doc = " \\n 0 : Multi View Coding not supported."]
#[doc = " \\n 1 : Multi View Coding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_STEREO_MVC: _NV_ENC_CAPS = 9;
#[doc = " Indicates HW support for encoding Temporal layers."]
#[doc = " \\n 0 : Encoding Temporal layers not supported."]
#[doc = " \\n 1 : Encoding Temporal layers supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_NUM_MAX_TEMPORAL_LAYERS: _NV_ENC_CAPS = 10;
#[doc = " Indicates HW support for Hierarchical P frames."]
#[doc = " \\n 0 : Hierarchical P frames not supported."]
#[doc = " \\n 1 : Hierarchical P frames supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_HIERARCHICAL_PFRAMES: _NV_ENC_CAPS = 11;
#[doc = " Indicates HW support for Hierarchical B frames."]
#[doc = " \\n 0 : Hierarchical B frames not supported."]
#[doc = " \\n 1 : Hierarchical B frames supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_HIERARCHICAL_BFRAMES: _NV_ENC_CAPS = 12;
#[doc = " Maximum Encoding level supported (See ::NV_ENC_LEVEL for details)."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_LEVEL_MAX: _NV_ENC_CAPS = 13;
#[doc = " Minimum Encoding level supported (See ::NV_ENC_LEVEL for details)."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_LEVEL_MIN: _NV_ENC_CAPS = 14;
#[doc = " Indicates HW support for separate colour plane encoding."]
#[doc = " \\n 0 : Separate colour plane encoding not supported."]
#[doc = " \\n 1 : Separate colour plane encoding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SEPARATE_COLOUR_PLANE: _NV_ENC_CAPS = 15;
#[doc = " Maximum output width supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MAX: _NV_ENC_CAPS = 16;
#[doc = " Maximum output height supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_HEIGHT_MAX: _NV_ENC_CAPS = 17;
#[doc = " Indicates Temporal Scalability Support."]
#[doc = " \\n 0 : Temporal SVC encoding not supported."]
#[doc = " \\n 1 : Temporal SVC encoding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_TEMPORAL_SVC: _NV_ENC_CAPS = 18;
#[doc = " Indicates Dynamic Encode Resolution Change Support."]
#[doc = " Support added from NvEncodeAPI version 2.0."]
#[doc = " \\n 0 : Dynamic Encode Resolution Change not supported."]
#[doc = " \\n 1 : Dynamic Encode Resolution Change supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_DYN_RES_CHANGE: _NV_ENC_CAPS = 19;
#[doc = " Indicates Dynamic Encode Bitrate Change Support."]
#[doc = " Support added from NvEncodeAPI version 2.0."]
#[doc = " \\n 0 : Dynamic Encode bitrate change not supported."]
#[doc = " \\n 1 : Dynamic Encode bitrate change supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_DYN_BITRATE_CHANGE: _NV_ENC_CAPS = 20;
#[doc = " Indicates Forcing Constant QP On The Fly Support."]
#[doc = " Support added from NvEncodeAPI version 2.0."]
#[doc = " \\n 0 : Forcing constant QP on the fly not supported."]
#[doc = " \\n 1 : Forcing constant QP on the fly supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_DYN_FORCE_CONSTQP: _NV_ENC_CAPS = 21;
#[doc = " Indicates Dynamic rate control mode Change Support."]
#[doc = " \\n 0 : Dynamic rate control mode change not supported."]
#[doc = " \\n 1 : Dynamic rate control mode change supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_DYN_RCMODE_CHANGE: _NV_ENC_CAPS = 22;
#[doc = " Indicates Subframe readback support for slice-based encoding."]
#[doc = " \\n 0 : Subframe readback not supported."]
#[doc = " \\n 1 : Subframe readback supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_SUBFRAME_READBACK: _NV_ENC_CAPS = 23;
#[doc = " Indicates Constrained Encoding mode support."]
#[doc = " Support added from NvEncodeAPI version 2.0."]
#[doc = " \\n 0 : Constrained encoding mode not supported."]
#[doc = " \\n 1 : Constarined encoding mode supported."]
#[doc = " If this mode is supported client can enable this during initialisation."]
#[doc = " Client can then force a picture to be coded as constrained picture where"]
#[doc = " each slice in a constrained picture will have constrained_intra_pred_flag set to 1"]
#[doc = " and disable_deblocking_filter_idc will be set to 2 and prediction vectors for inter"]
#[doc = " macroblocks in each slice will be restricted to the slice region."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_CONSTRAINED_ENCODING: _NV_ENC_CAPS = 24;
#[doc = " Indicates Intra Refresh Mode Support."]
#[doc = " Support added from NvEncodeAPI version 2.0."]
#[doc = " \\n 0 : Intra Refresh Mode not supported."]
#[doc = " \\n 1 : Intra Refresh Mode supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_INTRA_REFRESH: _NV_ENC_CAPS = 25;
#[doc = " Indicates Custom VBV Bufer Size support. It can be used for capping frame size."]
#[doc = " Support added from NvEncodeAPI version 2.0."]
#[doc = " \\n 0 : Custom VBV buffer size specification from client, not supported."]
#[doc = " \\n 1 : Custom VBV buffer size specification from client, supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_CUSTOM_VBV_BUF_SIZE: _NV_ENC_CAPS = 26;
#[doc = " Indicates Dynamic Slice Mode Support."]
#[doc = " Support added from NvEncodeAPI version 2.0."]
#[doc = " \\n 0 : Dynamic Slice Mode not supported."]
#[doc = " \\n 1 : Dynamic Slice Mode supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_DYNAMIC_SLICE_MODE: _NV_ENC_CAPS = 27;
#[doc = " Indicates Reference Picture Invalidation Support."]
#[doc = " Support added from NvEncodeAPI version 2.0."]
#[doc = " \\n 0 : Reference Picture Invalidation not supported."]
#[doc = " \\n 1 : Reference Picture Invalidation supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_REF_PIC_INVALIDATION: _NV_ENC_CAPS = 28;
#[doc = " Indicates support for PreProcessing."]
#[doc = " The API return value is a bitmask of the values defined in ::NV_ENC_PREPROC_FLAGS"]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_PREPROC_SUPPORT: _NV_ENC_CAPS = 29;
#[doc = " Indicates support Async mode."]
#[doc = " \\n 0 : Async Encode mode not supported."]
#[doc = " \\n 1 : Async Encode mode supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_ASYNC_ENCODE_SUPPORT: _NV_ENC_CAPS = 30;
#[doc = " Maximum MBs per frame supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_MB_NUM_MAX: _NV_ENC_CAPS = 31;
#[doc = " Maximum aggregate throughput in MBs per sec."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_MB_PER_SEC_MAX: _NV_ENC_CAPS = 32;
#[doc = " Indicates HW support for YUV444 mode encoding."]
#[doc = " \\n 0 : YUV444 mode encoding not supported."]
#[doc = " \\n 1 : YUV444 mode encoding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_YUV444_ENCODE: _NV_ENC_CAPS = 33;
#[doc = " Indicates HW support for lossless encoding."]
#[doc = " \\n 0 : lossless encoding not supported."]
#[doc = " \\n 1 : lossless encoding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_LOSSLESS_ENCODE: _NV_ENC_CAPS = 34;
#[doc = " Indicates HW support for Sample Adaptive Offset."]
#[doc = " \\n 0 : SAO not supported."]
#[doc = " \\n 1 : SAO encoding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_SAO: _NV_ENC_CAPS = 35;
#[doc = " Indicates HW support for MEOnly Mode."]
#[doc = " \\n 0 : MEOnly Mode not supported."]
#[doc = " \\n 1 : MEOnly Mode supported for I and P frames."]
#[doc = " \\n 2 : MEOnly Mode supported for I, P and B frames."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_MEONLY_MODE: _NV_ENC_CAPS = 36;
#[doc = " Indicates HW support for lookahead encoding (enableLookahead=1)."]
#[doc = " \\n 0 : Lookahead not supported."]
#[doc = " \\n 1 : Lookahead supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_LOOKAHEAD: _NV_ENC_CAPS = 37;
#[doc = " Indicates HW support for temporal AQ encoding (enableTemporalAQ=1)."]
#[doc = " \\n 0 : Temporal AQ not supported."]
#[doc = " \\n 1 : Temporal AQ supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_TEMPORAL_AQ: _NV_ENC_CAPS = 38;
#[doc = " Indicates HW support for 10 bit encoding."]
#[doc = " \\n 0 : 10 bit encoding not supported."]
#[doc = " \\n 1 : 10 bit encoding supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_10BIT_ENCODE: _NV_ENC_CAPS = 39;
#[doc = " Maximum number of Long Term Reference frames supported"]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_NUM_MAX_LTR_FRAMES: _NV_ENC_CAPS = 40;
#[doc = " Indicates HW support for Weighted Predicition."]
#[doc = " \\n 0 : Weighted Predicition not supported."]
#[doc = " \\n 1 : Weighted Predicition supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_WEIGHTED_PREDICTION: _NV_ENC_CAPS = 41;
#[doc = " On managed (vGPU) platforms (Windows only), this API, in conjunction with other GRID Management APIs, can be used"]
#[doc = " to estimate the residual capacity of the hardware encoder on the GPU as a percentage of the total available encoder capacity."]
#[doc = " This API can be called at any time; i.e. during the encode session or before opening the encode session."]
#[doc = " If the available encoder capacity is returned as zero, applications may choose to switch to software encoding"]
#[doc = " and continue to call this API (e.g. polling once per second) until capacity becomes available."]
#[doc = ""]
#[doc = " On baremetal (non-virtualized GPU) and linux platforms, this API always returns 100."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_DYNAMIC_QUERY_ENCODER_CAPACITY: _NV_ENC_CAPS = 42;
#[doc = " Indicates B as refererence support."]
#[doc = " \\n 0 : B as reference is not supported."]
#[doc = " \\n 1 : each B-Frame as reference is supported."]
#[doc = " \\n 2 : only Middle B-frame as reference is supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_BFRAME_REF_MODE: _NV_ENC_CAPS = 43;
#[doc = " Indicates HW support for Emphasis Level Map based delta QP computation."]
#[doc = " \\n 0 : Emphasis Level Map based delta QP not supported."]
#[doc = " \\n 1 : Emphasis Level Map based delta QP is supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_EMPHASIS_LEVEL_MAP: _NV_ENC_CAPS = 44;
#[doc = " Minimum input width supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MIN: _NV_ENC_CAPS = 45;
#[doc = " Minimum input height supported."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_HEIGHT_MIN: _NV_ENC_CAPS = 46;
#[doc = " Indicates HW support for multiple reference frames."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_MULTIPLE_REF_FRAMES: _NV_ENC_CAPS = 47;
#[doc = " Reserved - Not to be used by clients."]
pub const _NV_ENC_CAPS_NV_ENC_CAPS_EXPOSED_COUNT: _NV_ENC_CAPS = 48;
#[doc = " Encoder capabilities enumeration."]
pub type _NV_ENC_CAPS = i32;
pub use self::_NV_ENC_CAPS as NV_ENC_CAPS;
pub const _NV_ENC_HEVC_CUSIZE_NV_ENC_HEVC_CUSIZE_AUTOSELECT: _NV_ENC_HEVC_CUSIZE = 0;
pub const _NV_ENC_HEVC_CUSIZE_NV_ENC_HEVC_CUSIZE_8x8: _NV_ENC_HEVC_CUSIZE = 1;
pub const _NV_ENC_HEVC_CUSIZE_NV_ENC_HEVC_CUSIZE_16x16: _NV_ENC_HEVC_CUSIZE = 2;
pub const _NV_ENC_HEVC_CUSIZE_NV_ENC_HEVC_CUSIZE_32x32: _NV_ENC_HEVC_CUSIZE = 3;
pub const _NV_ENC_HEVC_CUSIZE_NV_ENC_HEVC_CUSIZE_64x64: _NV_ENC_HEVC_CUSIZE = 4;
#[doc = "  HEVC CU SIZE"]
pub type _NV_ENC_HEVC_CUSIZE = i32;
pub use self::_NV_ENC_HEVC_CUSIZE as NV_ENC_HEVC_CUSIZE;
#[doc = " Input struct for querying Encoding capabilities."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CAPS_PARAM {
    #[doc = "Struct version. Must be set to ::NV_ENC_CAPS_PARAM_VER"]
    pub version: u32,
    #[doc = "Specifies the encode capability to be queried. Client should pass a member for ::NV_ENC_CAPS enum."]
    pub capsToQuery: NV_ENC_CAPS,
    #[doc = "Reserved and must be set to 0"]
    pub reserved: [u32; 62usize],
}
pub type NV_ENC_CAPS_PARAM = _NV_ENC_CAPS_PARAM;
#[doc = " Encoder Output parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_ENCODE_OUT_PARAMS {
    #[doc = "Struct version."]
    pub version: u32,
    #[doc = "Encoded bitstream size in bytes"]
    pub bitstreamSizeInBytes: u32,
    #[doc = "Reserved and must be set to 0"]
    pub reserved: [u32; 62usize],
}
pub type NV_ENC_ENCODE_OUT_PARAMS = _NV_ENC_ENCODE_OUT_PARAMS;
#[doc = " Creation parameters for input buffer."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CREATE_INPUT_BUFFER {
    #[doc = "Struct version. Must be set to ::NV_ENC_CREATE_INPUT_BUFFER_VER"]
    pub version: u32,
    #[doc = "Input buffer width"]
    pub width: u32,
    #[doc = "Input buffer width"]
    pub height: u32,
    #[doc = "Deprecated. Do not use"]
    pub memoryHeap: NV_ENC_MEMORY_HEAP,
    #[doc = "Input buffer format"]
    pub bufferFmt: NV_ENC_BUFFER_FORMAT,
    #[doc = "Reserved and must be set to 0"]
    pub reserved: u32,
    #[doc = "Pointer to input buffer"]
    pub inputBuffer: NV_ENC_INPUT_PTR,
    #[doc = "Pointer to existing sysmem buffer"]
    pub pSysMemBuffer: *mut ::core::ffi::c_void,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 57usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 63usize],
}
pub type NV_ENC_CREATE_INPUT_BUFFER = _NV_ENC_CREATE_INPUT_BUFFER;
#[doc = " Creation parameters for output bitstream buffer."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CREATE_BITSTREAM_BUFFER {
    #[doc = "Struct version. Must be set to ::NV_ENC_CREATE_BITSTREAM_BUFFER_VER"]
    pub version: u32,
    #[doc = "Deprecated. Do not use"]
    pub size: u32,
    #[doc = "Deprecated. Do not use"]
    pub memoryHeap: NV_ENC_MEMORY_HEAP,
    #[doc = "Reserved and must be set to 0"]
    pub reserved: u32,
    #[doc = "Pointer to the output bitstream buffer"]
    pub bitstreamBuffer: NV_ENC_OUTPUT_PTR,
    #[doc = "Reserved and should not be used"]
    pub bitstreamBufferPtr: *mut ::core::ffi::c_void,
    #[doc = "Reserved and should be set to 0"]
    pub reserved1: [u32; 58usize],
    #[doc = "Reserved and should be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
pub type NV_ENC_CREATE_BITSTREAM_BUFFER = _NV_ENC_CREATE_BITSTREAM_BUFFER;
#[doc = " Structs needed for ME only mode."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_MVECTOR {
    #[doc = " the x component of MV in qpel units"]
    pub mvx: i16,
    #[doc = " the y component of MV in qpel units"]
    pub mvy: i16,
}
pub type NV_ENC_MVECTOR = _NV_ENC_MVECTOR;
#[doc = " Motion vector structure per macroblock for H264 motion estimation."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_H264_MV_DATA {
    #[doc = " up to 4 vectors for 8x8 partition"]
    pub mv: [NV_ENC_MVECTOR; 4usize],
    #[doc = " 0 (I), 1 (P), 2 (IPCM), 3 (B)"]
    pub mbType: u8,
    #[doc = " Specifies the block partition type. 0:16x16, 1:8x8, 2:16x8, 3:8x16"]
    pub partitionType: u8,
    #[doc = " reserved padding for alignment"]
    pub reserved: u16,
    pub mbCost: u32,
}
pub type NV_ENC_H264_MV_DATA = _NV_ENC_H264_MV_DATA;
#[doc = " Motion vector structure per CU for HEVC motion estimation."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_HEVC_MV_DATA {
    #[doc = " up to 4 vectors within a CU"]
    pub mv: [NV_ENC_MVECTOR; 4usize],
    #[doc = " 0 (I), 1(P)"]
    pub cuType: u8,
    #[doc = " 0: 8x8, 1: 16x16, 2: 32x32, 3: 64x64"]
    pub cuSize: u8,
    #[doc = " The CU partition mode"]
    #[doc = "0 (2Nx2N), 1 (2NxN), 2(Nx2N), 3 (NxN),"]
    #[doc = "4 (2NxnU), 5 (2NxnD), 6(nLx2N), 7 (nRx2N)"]
    pub partitionMode: u8,
    #[doc = " Marker to separate CUs in the current CTB from CUs in the next CTB"]
    pub lastCUInCTB: u8,
}
pub type NV_ENC_HEVC_MV_DATA = _NV_ENC_HEVC_MV_DATA;
#[doc = " Creation parameters for output motion vector buffer for ME only mode."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CREATE_MV_BUFFER {
    #[doc = "Struct version. Must be set to NV_ENC_CREATE_MV_BUFFER_VER"]
    pub version: u32,
    #[doc = "Pointer to the output motion vector buffer"]
    pub mvBuffer: NV_ENC_OUTPUT_PTR,
    #[doc = "Reserved and should be set to 0"]
    pub reserved1: [u32; 255usize],
    #[doc = "Reserved and should be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 63usize],
}
pub type NV_ENC_CREATE_MV_BUFFER = _NV_ENC_CREATE_MV_BUFFER;
#[doc = " QP value for frames"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_QP {
    #[doc = "Specifies QP value for P-frame. Even though this field is uint32_t for legacy reasons, the client should treat this as a signed parameter(int32_t) for cases in which negative QP values are to be specified."]
    pub qpInterP: u32,
    #[doc = "Specifies QP value for B-frame. Even though this field is uint32_t for legacy reasons, the client should treat this as a signed parameter(int32_t) for cases in which negative QP values are to be specified."]
    pub qpInterB: u32,
    #[doc = "Specifies QP value for Intra Frame. Even though this field is uint32_t for legacy reasons, the client should treat this as a signed parameter(int32_t) for cases in which negative QP values are to be specified."]
    pub qpIntra: u32,
}
pub type NV_ENC_QP = _NV_ENC_QP;
#[doc = " Rate Control Configuration Paramters"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_RC_PARAMS {
    pub version: u32,
    #[doc = "Specifies the rate control mode. Check support for various rate control modes using ::NV_ENC_CAPS_SUPPORTED_RATECONTROL_MODES caps."]
    pub rateControlMode: NV_ENC_PARAMS_RC_MODE,
    #[doc = "Specifies the initial QP to be used for encoding, these values would be used for all frames if in Constant QP mode."]
    pub constQP: NV_ENC_QP,
    #[doc = "Specifies the average bitrate(in bits/sec) used for encoding."]
    pub averageBitRate: u32,
    #[doc = "Specifies the maximum bitrate for the encoded output. This is used for VBR and ignored for CBR mode."]
    pub maxBitRate: u32,
    #[doc = "Specifies the VBV(HRD) buffer size. in bits. Set 0 to use the default VBV  buffer size."]
    pub vbvBufferSize: u32,
    #[doc = "Specifies the VBV(HRD) initial delay in bits. Set 0 to use the default VBV  initial delay ."]
    pub vbvInitialDelay: u32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
    #[doc = "Specifies the minimum QP used for rate control. Client must set NV_ENC_CONFIG::enableMinQP to 1."]
    pub minQP: NV_ENC_QP,
    #[doc = "Specifies the maximum QP used for rate control. Client must set NV_ENC_CONFIG::enableMaxQP to 1."]
    pub maxQP: NV_ENC_QP,
    #[doc = "Specifies the initial QP used for rate control. Client must set NV_ENC_CONFIG::enableInitialRCQP to 1."]
    pub initialRCQP: NV_ENC_QP,
    #[doc = "Specifies the temporal layers (as a bitmask) whose QPs have changed. Valid max bitmask is [2^NV_ENC_CAPS_NUM_MAX_TEMPORAL_LAYERS - 1]"]
    pub temporallayerIdxMask: u32,
    #[doc = "Specifies the temporal layer QPs used for rate control. Temporal layer index is used as as the array index"]
    pub temporalLayerQP: [u8; 8usize],
    #[doc = "Target CQ (Constant Quality) level for VBR mode (range 0-51 with 0-automatic)"]
    pub targetQuality: u8,
    #[doc = "Fractional part of target quality (as 8.8 fixed point format)"]
    pub targetQualityLSB: u8,
    #[doc = "Maximum depth of lookahead with range 0-32 (only used if enableLookahead=1)"]
    pub lookaheadDepth: u16,
    pub reserved1: u32,
    #[doc = "This flag is used to interpret values in array specified by NV_ENC_PIC_PARAMS::qpDeltaMap."]
    #[doc = "Set this to NV_ENC_QP_MAP_EMPHASIS to treat values specified by NV_ENC_PIC_PARAMS::qpDeltaMap as Emphasis Level Map."]
    #[doc = "Emphasis Level can be assigned any value specified in enum NV_ENC_EMPHASIS_MAP_LEVEL."]
    #[doc = "Emphasis Level Map is used to specify regions to be encoded at varying levels of quality."]
    #[doc = "The hardware encoder adjusts the quantization within the image as per the provided emphasis map,"]
    #[doc = "by adjusting the quantization parameter (QP) assigned to each macroblock. This adjustment is commonly called “Delta QP”."]
    #[doc = "The adjustment depends on the absolute QP decided by the rate control algorithm, and is applied after the rate control has decided each macroblock’s QP."]
    #[doc = "Since the Delta QP overrides rate control, enabling Emphasis Level Map may violate bitrate and VBV buffer size constraints."]
    #[doc = "Emphasis Level Map is useful in situations where client has a priori knowledge of the image complexity (e.g. via use of NVFBC's Classification feature) and encoding those high-complexity areas at higher quality (lower QP) is important, even at the possible cost of violating bitrate/VBV buffer size constraints"]
    #[doc = "This feature is not supported when AQ( Spatial/Temporal) is enabled."]
    #[doc = "This feature is only supported for H264 codec currently."]
    #[doc = ""]
    #[doc = "Set this to NV_ENC_QP_MAP_DELTA to treat values specified by NV_ENC_PIC_PARAMS::qpDeltaMap as QPDelta. This specifies QP modifier to be applied on top of the QP chosen by rate control"]
    #[doc = ""]
    #[doc = "Set this to NV_ENC_QP_MAP_DISABLED to ignore NV_ENC_PIC_PARAMS::qpDeltaMap values. In this case, qpDeltaMap should be set to NULL."]
    #[doc = ""]
    #[doc = "Other values are reserved for future use."]
    pub qpMapMode: NV_ENC_QP_MAP_MODE,
    pub reserved: [u32; 7usize],
}
impl _NV_ENC_RC_PARAMS {
    #[inline]
    pub fn enableMinQP(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableMinQP(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableMaxQP(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableMaxQP(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableInitialRCQP(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableInitialRCQP(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableAQ(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableAQ(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reservedBitField1(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reservedBitField1(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableLookahead(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableLookahead(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disableIadapt(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disableIadapt(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disableBadapt(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disableBadapt(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableTemporalAQ(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableTemporalAQ(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn zeroReorderDelay(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_zeroReorderDelay(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableNonRefP(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableNonRefP(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn strictGOPTarget(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_strictGOPTarget(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn aqStrength(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_aqStrength(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn reservedBitFields(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_reservedBitFields(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        enableMinQP: u32,
        enableMaxQP: u32,
        enableInitialRCQP: u32,
        enableAQ: u32,
        reservedBitField1: u32,
        enableLookahead: u32,
        disableIadapt: u32,
        disableBadapt: u32,
        enableTemporalAQ: u32,
        zeroReorderDelay: u32,
        enableNonRefP: u32,
        strictGOPTarget: u32,
        aqStrength: u32,
        reservedBitFields: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let enableMinQP: u32 = unsafe { ::core::mem::transmute(enableMinQP) };
            enableMinQP as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let enableMaxQP: u32 = unsafe { ::core::mem::transmute(enableMaxQP) };
            enableMaxQP as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let enableInitialRCQP: u32 = unsafe { ::core::mem::transmute(enableInitialRCQP) };
            enableInitialRCQP as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let enableAQ: u32 = unsafe { ::core::mem::transmute(enableAQ) };
            enableAQ as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let reservedBitField1: u32 = unsafe { ::core::mem::transmute(reservedBitField1) };
            reservedBitField1 as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let enableLookahead: u32 = unsafe { ::core::mem::transmute(enableLookahead) };
            enableLookahead as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let disableIadapt: u32 = unsafe { ::core::mem::transmute(disableIadapt) };
            disableIadapt as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let disableBadapt: u32 = unsafe { ::core::mem::transmute(disableBadapt) };
            disableBadapt as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let enableTemporalAQ: u32 = unsafe { ::core::mem::transmute(enableTemporalAQ) };
            enableTemporalAQ as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let zeroReorderDelay: u32 = unsafe { ::core::mem::transmute(zeroReorderDelay) };
            zeroReorderDelay as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let enableNonRefP: u32 = unsafe { ::core::mem::transmute(enableNonRefP) };
            enableNonRefP as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let strictGOPTarget: u32 = unsafe { ::core::mem::transmute(strictGOPTarget) };
            strictGOPTarget as u64
        });
        __bindgen_bitfield_unit.set(12usize, 4u8, {
            let aqStrength: u32 = unsafe { ::core::mem::transmute(aqStrength) };
            aqStrength as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let reservedBitFields: u32 = unsafe { ::core::mem::transmute(reservedBitFields) };
            reservedBitFields as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_RC_PARAMS = _NV_ENC_RC_PARAMS;
#[doc = " H264 Video Usability Info parameters"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_CONFIG_H264_VUI_PARAMETERS {
    #[doc = "if set to 1 , it specifies that the overscanInfo is present"]
    pub overscanInfoPresentFlag: u32,
    #[doc = "Specifies the overscan info(as defined in Annex E of the ITU-T Specification)."]
    pub overscanInfo: u32,
    #[doc = "If set to 1, it specifies  that the videoFormat, videoFullRangeFlag and colourDescriptionPresentFlag are present."]
    pub videoSignalTypePresentFlag: u32,
    #[doc = "Specifies the source video format(as defined in Annex E of the ITU-T Specification)."]
    pub videoFormat: u32,
    #[doc = "Specifies the output range of the luma and chroma samples(as defined in Annex E of the ITU-T Specification)."]
    pub videoFullRangeFlag: u32,
    #[doc = "If set to 1, it specifies that the colourPrimaries, transferCharacteristics and colourMatrix are present."]
    pub colourDescriptionPresentFlag: u32,
    #[doc = "Specifies color primaries for converting to RGB(as defined in Annex E of the ITU-T Specification)"]
    pub colourPrimaries: u32,
    #[doc = "Specifies the opto-electronic transfer characteristics to use (as defined in Annex E of the ITU-T Specification)"]
    pub transferCharacteristics: u32,
    #[doc = "Specifies the matrix coefficients used in deriving the luma and chroma from the RGB primaries (as defined in Annex E of the ITU-T Specification)."]
    pub colourMatrix: u32,
    #[doc = "if set to 1 , it specifies that the chromaSampleLocationTop and chromaSampleLocationBot are present."]
    pub chromaSampleLocationFlag: u32,
    #[doc = "Specifies the chroma sample location for top field(as defined in Annex E of the ITU-T Specification)"]
    pub chromaSampleLocationTop: u32,
    #[doc = "Specifies the chroma sample location for bottom field(as defined in Annex E of the ITU-T Specification)"]
    pub chromaSampleLocationBot: u32,
    #[doc = "if set to 1, it specifies the bitstream restriction parameters are present in the bitstream."]
    pub bitstreamRestrictionFlag: u32,
    pub reserved: [u32; 15usize],
}
pub type NV_ENC_CONFIG_H264_VUI_PARAMETERS = _NV_ENC_CONFIG_H264_VUI_PARAMETERS;
pub type NV_ENC_CONFIG_HEVC_VUI_PARAMETERS = NV_ENC_CONFIG_H264_VUI_PARAMETERS;
#[doc = " External motion vector hint counts per block type."]
#[doc = " H264 supports multiple hint while HEVC supports one hint for each valid candidate."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NVENC_EXTERNAL_ME_HINT_COUNTS_PER_BLOCKTYPE {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
    #[doc = "Reserved for future use."]
    pub reserved1: [u32; 3usize],
}
impl _NVENC_EXTERNAL_ME_HINT_COUNTS_PER_BLOCKTYPE {
    #[inline]
    pub fn numCandsPerBlk16x16(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_numCandsPerBlk16x16(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn numCandsPerBlk16x8(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_numCandsPerBlk16x8(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn numCandsPerBlk8x16(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_numCandsPerBlk8x16(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn numCandsPerBlk8x8(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_numCandsPerBlk8x8(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        numCandsPerBlk16x16: u32,
        numCandsPerBlk16x8: u32,
        numCandsPerBlk8x16: u32,
        numCandsPerBlk8x8: u32,
        reserved: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let numCandsPerBlk16x16: u32 = unsafe { ::core::mem::transmute(numCandsPerBlk16x16) };
            numCandsPerBlk16x16 as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let numCandsPerBlk16x8: u32 = unsafe { ::core::mem::transmute(numCandsPerBlk16x8) };
            numCandsPerBlk16x8 as u64
        });
        __bindgen_bitfield_unit.set(8usize, 4u8, {
            let numCandsPerBlk8x16: u32 = unsafe { ::core::mem::transmute(numCandsPerBlk8x16) };
            numCandsPerBlk8x16 as u64
        });
        __bindgen_bitfield_unit.set(12usize, 4u8, {
            let numCandsPerBlk8x8: u32 = unsafe { ::core::mem::transmute(numCandsPerBlk8x8) };
            numCandsPerBlk8x8 as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let reserved: u32 = unsafe { ::core::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NVENC_EXTERNAL_ME_HINT_COUNTS_PER_BLOCKTYPE = _NVENC_EXTERNAL_ME_HINT_COUNTS_PER_BLOCKTYPE;
#[doc = " External Motion Vector hint structure."]
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct _NVENC_EXTERNAL_ME_HINT {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
}
impl _NVENC_EXTERNAL_ME_HINT {
    #[inline]
    pub fn mvx(&self) -> i32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 12u8) as u32) }
    }
    #[inline]
    pub fn set_mvx(&mut self, val: i32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn mvy(&self) -> i32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 10u8) as u32) }
    }
    #[inline]
    pub fn set_mvy(&mut self, val: i32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn refidx(&self) -> i32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(22usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set_refidx(&mut self, val: i32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(22usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn dir(&self) -> i32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(27usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_dir(&mut self, val: i32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(27usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn partType(&self) -> i32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(28usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_partType(&mut self, val: i32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(28usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn lastofPart(&self) -> i32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_lastofPart(&mut self, val: i32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn lastOfMB(&self) -> i32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_lastOfMB(&mut self, val: i32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mvx: i32,
        mvy: i32,
        refidx: i32,
        dir: i32,
        partType: i32,
        lastofPart: i32,
        lastOfMB: i32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 12u8, {
            let mvx: u32 = unsafe { ::core::mem::transmute(mvx) };
            mvx as u64
        });
        __bindgen_bitfield_unit.set(12usize, 10u8, {
            let mvy: u32 = unsafe { ::core::mem::transmute(mvy) };
            mvy as u64
        });
        __bindgen_bitfield_unit.set(22usize, 5u8, {
            let refidx: u32 = unsafe { ::core::mem::transmute(refidx) };
            refidx as u64
        });
        __bindgen_bitfield_unit.set(27usize, 1u8, {
            let dir: u32 = unsafe { ::core::mem::transmute(dir) };
            dir as u64
        });
        __bindgen_bitfield_unit.set(28usize, 2u8, {
            let partType: u32 = unsafe { ::core::mem::transmute(partType) };
            partType as u64
        });
        __bindgen_bitfield_unit.set(30usize, 1u8, {
            let lastofPart: u32 = unsafe { ::core::mem::transmute(lastofPart) };
            lastofPart as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let lastOfMB: u32 = unsafe { ::core::mem::transmute(lastOfMB) };
            lastOfMB as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NVENC_EXTERNAL_ME_HINT = _NVENC_EXTERNAL_ME_HINT;
#[doc = " H264 encoder configuration parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CONFIG_H264 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
    #[doc = "Specifies the encoding level. Client is recommended to set this to NV_ENC_LEVEL_AUTOSELECT in order to enable the NvEncodeAPI interface to select the correct level."]
    pub level: u32,
    #[doc = "Specifies the IDR interval. If not set, this is made equal to gopLength in NV_ENC_CONFIG.Low latency application client can set IDR interval to NVENC_INFINITE_GOPLENGTH so that IDR frames are not inserted automatically."]
    pub idrPeriod: u32,
    #[doc = "Set to 1 to enable 4:4:4 separate colour planes"]
    pub separateColourPlaneFlag: u32,
    #[doc = "Specifies the deblocking filter mode. Permissible value range: [0,2]"]
    pub disableDeblockingFilterIDC: u32,
    #[doc = "Specifies max temporal layers to be used for hierarchical coding. Valid value range is [1,::NV_ENC_CAPS_NUM_MAX_TEMPORAL_LAYERS]"]
    pub numTemporalLayers: u32,
    #[doc = "Specifies the SPS id of the sequence header"]
    pub spsId: u32,
    #[doc = "Specifies the PPS id of the picture header"]
    pub ppsId: u32,
    #[doc = "Specifies the AdaptiveTransform Mode. Check support for AdaptiveTransform mode using ::NV_ENC_CAPS_SUPPORT_ADAPTIVE_TRANSFORM caps."]
    pub adaptiveTransformMode: NV_ENC_H264_ADAPTIVE_TRANSFORM_MODE,
    #[doc = "Specified the FMO Mode. Check support for FMO using ::NV_ENC_CAPS_SUPPORT_FMO caps."]
    pub fmoMode: NV_ENC_H264_FMO_MODE,
    #[doc = "Specifies the BDirect mode. Check support for BDirect mode using ::NV_ENC_CAPS_SUPPORT_BDIRECT_MODE caps."]
    pub bdirectMode: NV_ENC_H264_BDIRECT_MODE,
    #[doc = "Specifies the entropy coding mode. Check support for CABAC mode using ::NV_ENC_CAPS_SUPPORT_CABAC caps."]
    pub entropyCodingMode: NV_ENC_H264_ENTROPY_CODING_MODE,
    #[doc = "Specifies the stereo frame packing mode which is to be signalled in frame packing arrangement SEI"]
    pub stereoMode: NV_ENC_STEREO_PACKING_MODE,
    #[doc = "Specifies the interval between successive intra refresh if enableIntrarefresh is set. Requires enableIntraRefresh to be set."]
    #[doc = "Will be disabled if NV_ENC_CONFIG::gopLength is not set to NVENC_INFINITE_GOPLENGTH."]
    pub intraRefreshPeriod: u32,
    #[doc = "Specifies the length of intra refresh in number of frames for periodic intra refresh. This value should be smaller than intraRefreshPeriod"]
    pub intraRefreshCnt: u32,
    #[doc = "Specifies the DPB size used for encoding. Setting it to 0 will let driver use the default dpb size."]
    #[doc = "The low latency application which wants to invalidate reference frame as an error resilience tool"]
    #[doc = "is recommended to use a large DPB size so that the encoder can keep old reference frames which can be used if recent"]
    #[doc = "frames are invalidated."]
    pub maxNumRefFrames: u32,
    #[doc = "This parameter in conjunction with sliceModeData specifies the way in which the picture is divided into slices"]
    #[doc = "sliceMode = 0 MB based slices, sliceMode = 1 Byte based slices, sliceMode = 2 MB row based slices, sliceMode = 3 numSlices in Picture."]
    #[doc = "When forceIntraRefreshWithFrameCnt is set it will have priority over sliceMode setting"]
    #[doc = "When sliceMode == 0 and sliceModeData == 0 whole picture will be coded with one slice"]
    pub sliceMode: u32,
    #[doc = "Specifies the parameter needed for sliceMode. For:"]
    #[doc = "sliceMode = 0, sliceModeData specifies # of MBs in each slice (except last slice)"]
    #[doc = "sliceMode = 1, sliceModeData specifies maximum # of bytes in each slice (except last slice)"]
    #[doc = "sliceMode = 2, sliceModeData specifies # of MB rows in each slice (except last slice)"]
    #[doc = "sliceMode = 3, sliceModeData specifies number of slices in the picture. Driver will divide picture into slices optimally"]
    pub sliceModeData: u32,
    #[doc = "Specifies the H264 video usability info pamameters"]
    pub h264VUIParameters: NV_ENC_CONFIG_H264_VUI_PARAMETERS,
    #[doc = "Specifies the number of LTR frames. This parameter has different meaning in two LTR modes."]
    #[doc = "In \"LTR Trust\" mode (ltrTrustMode = 1), encoder will mark the first ltrNumFrames base layer reference frames within each IDR interval as LTR."]
    #[doc = "In \"LTR Per Picture\" mode (ltrTrustMode = 0 and ltrMarkFrame = 1), ltrNumFrames specifies maximum number of LTR frames in DPB."]
    pub ltrNumFrames: u32,
    #[doc = "Specifies the LTR operating mode. See comments near NV_ENC_CONFIG_H264::enableLTR for description of the two modes."]
    #[doc = "Set to 1 to use \"LTR Trust\" mode of LTR operation. Clients are discouraged to use \"LTR Trust\" mode as this mode may"]
    #[doc = "be deprecated in future releases."]
    #[doc = "Set to 0 when using \"LTR Per Picture\" mode of LTR operation."]
    pub ltrTrustMode: u32,
    #[doc = "Specifies the chroma format. Should be set to 1 for yuv420 input, 3 for yuv444 input."]
    #[doc = "Check support for YUV444 encoding using ::NV_ENC_CAPS_SUPPORT_YUV444_ENCODE caps."]
    pub chromaFormatIDC: u32,
    #[doc = "Specifies the max temporal layer used for hierarchical coding."]
    pub maxTemporalLayers: u32,
    #[doc = "Specifies the B-Frame as reference mode. Check support for useBFramesAsRef mode using ::NV_ENC_CAPS_SUPPORT_BFRAME_REF_MODE caps."]
    pub useBFramesAsRef: NV_ENC_BFRAME_REF_MODE,
    #[doc = "Specifies max number of reference frames in reference picture list L0, that can be used by hardware for prediction of a frame."]
    #[doc = "Check support for numRefL0 using ::NV_ENC_CAPS_SUPPORT_MULTIPLE_REF_FRAMES caps."]
    pub numRefL0: NV_ENC_NUM_REF_FRAMES,
    #[doc = "Specifies max number of reference frames in reference picture list L1, that can be used by hardware for prediction of a frame."]
    #[doc = "Check support for numRefL1 using ::NV_ENC_CAPS_SUPPORT_MULTIPLE_REF_FRAMES caps."]
    pub numRefL1: NV_ENC_NUM_REF_FRAMES,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 267usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
impl _NV_ENC_CONFIG_H264 {
    #[inline]
    pub fn reserved(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableStereoMVC(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableStereoMVC(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hierarchicalPFrames(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hierarchicalPFrames(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hierarchicalBFrames(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hierarchicalBFrames(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outputBufferingPeriodSEI(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outputBufferingPeriodSEI(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outputPictureTimingSEI(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outputPictureTimingSEI(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outputAUD(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outputAUD(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disableSPSPPS(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disableSPSPPS(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outputFramePackingSEI(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outputFramePackingSEI(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outputRecoveryPointSEI(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outputRecoveryPointSEI(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableIntraRefresh(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableIntraRefresh(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableConstrainedEncoding(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableConstrainedEncoding(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn repeatSPSPPS(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_repeatSPSPPS(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableVFR(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableVFR(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableLTR(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableLTR(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn qpPrimeYZeroTransformBypassFlag(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_qpPrimeYZeroTransformBypassFlag(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn useConstrainedIntraPred(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_useConstrainedIntraPred(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableFillerDataInsertion(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableFillerDataInsertion(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reservedBitFields(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(18usize, 14u8) as u32) }
    }
    #[inline]
    pub fn set_reservedBitFields(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(18usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        reserved: u32,
        enableStereoMVC: u32,
        hierarchicalPFrames: u32,
        hierarchicalBFrames: u32,
        outputBufferingPeriodSEI: u32,
        outputPictureTimingSEI: u32,
        outputAUD: u32,
        disableSPSPPS: u32,
        outputFramePackingSEI: u32,
        outputRecoveryPointSEI: u32,
        enableIntraRefresh: u32,
        enableConstrainedEncoding: u32,
        repeatSPSPPS: u32,
        enableVFR: u32,
        enableLTR: u32,
        qpPrimeYZeroTransformBypassFlag: u32,
        useConstrainedIntraPred: u32,
        enableFillerDataInsertion: u32,
        reservedBitFields: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let reserved: u32 = unsafe { ::core::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let enableStereoMVC: u32 = unsafe { ::core::mem::transmute(enableStereoMVC) };
            enableStereoMVC as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let hierarchicalPFrames: u32 = unsafe { ::core::mem::transmute(hierarchicalPFrames) };
            hierarchicalPFrames as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let hierarchicalBFrames: u32 = unsafe { ::core::mem::transmute(hierarchicalBFrames) };
            hierarchicalBFrames as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let outputBufferingPeriodSEI: u32 =
                unsafe { ::core::mem::transmute(outputBufferingPeriodSEI) };
            outputBufferingPeriodSEI as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let outputPictureTimingSEI: u32 =
                unsafe { ::core::mem::transmute(outputPictureTimingSEI) };
            outputPictureTimingSEI as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let outputAUD: u32 = unsafe { ::core::mem::transmute(outputAUD) };
            outputAUD as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let disableSPSPPS: u32 = unsafe { ::core::mem::transmute(disableSPSPPS) };
            disableSPSPPS as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let outputFramePackingSEI: u32 =
                unsafe { ::core::mem::transmute(outputFramePackingSEI) };
            outputFramePackingSEI as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let outputRecoveryPointSEI: u32 =
                unsafe { ::core::mem::transmute(outputRecoveryPointSEI) };
            outputRecoveryPointSEI as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let enableIntraRefresh: u32 = unsafe { ::core::mem::transmute(enableIntraRefresh) };
            enableIntraRefresh as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let enableConstrainedEncoding: u32 =
                unsafe { ::core::mem::transmute(enableConstrainedEncoding) };
            enableConstrainedEncoding as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let repeatSPSPPS: u32 = unsafe { ::core::mem::transmute(repeatSPSPPS) };
            repeatSPSPPS as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let enableVFR: u32 = unsafe { ::core::mem::transmute(enableVFR) };
            enableVFR as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let enableLTR: u32 = unsafe { ::core::mem::transmute(enableLTR) };
            enableLTR as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let qpPrimeYZeroTransformBypassFlag: u32 =
                unsafe { ::core::mem::transmute(qpPrimeYZeroTransformBypassFlag) };
            qpPrimeYZeroTransformBypassFlag as u64
        });
        __bindgen_bitfield_unit.set(16usize, 1u8, {
            let useConstrainedIntraPred: u32 =
                unsafe { ::core::mem::transmute(useConstrainedIntraPred) };
            useConstrainedIntraPred as u64
        });
        __bindgen_bitfield_unit.set(17usize, 1u8, {
            let enableFillerDataInsertion: u32 =
                unsafe { ::core::mem::transmute(enableFillerDataInsertion) };
            enableFillerDataInsertion as u64
        });
        __bindgen_bitfield_unit.set(18usize, 14u8, {
            let reservedBitFields: u32 = unsafe { ::core::mem::transmute(reservedBitFields) };
            reservedBitFields as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_CONFIG_H264 = _NV_ENC_CONFIG_H264;
#[doc = " HEVC encoder configuration parameters to be set during initialization."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CONFIG_HEVC {
    #[doc = "Specifies the level of the encoded bitstream."]
    pub level: u32,
    #[doc = "Specifies the level tier of the encoded bitstream."]
    pub tier: u32,
    #[doc = "Specifies the minimum size of luma coding unit."]
    pub minCUSize: NV_ENC_HEVC_CUSIZE,
    #[doc = "Specifies the maximum size of luma coding unit. Currently NVENC SDK only supports maxCUSize equal to NV_ENC_HEVC_CUSIZE_32x32."]
    pub maxCUSize: NV_ENC_HEVC_CUSIZE,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    #[doc = "Specifies the IDR interval. If not set, this is made equal to gopLength in NV_ENC_CONFIG.Low latency application client can set IDR interval to NVENC_INFINITE_GOPLENGTH so that IDR frames are not inserted automatically."]
    pub idrPeriod: u32,
    #[doc = "Specifies the interval between successive intra refresh if enableIntrarefresh is set. Requires enableIntraRefresh to be set."]
    #[doc = "Will be disabled if NV_ENC_CONFIG::gopLength is not set to NVENC_INFINITE_GOPLENGTH."]
    pub intraRefreshPeriod: u32,
    #[doc = "Specifies the length of intra refresh in number of frames for periodic intra refresh. This value should be smaller than intraRefreshPeriod"]
    pub intraRefreshCnt: u32,
    #[doc = "Specifies the maximum number of references frames in the DPB."]
    pub maxNumRefFramesInDPB: u32,
    #[doc = "This parameter has different meaning in two LTR modes."]
    #[doc = "In \"LTR Trust\" mode (ltrTrustMode = 1), encoder will mark the first ltrNumFrames base layer reference frames within each IDR interval as LTR."]
    #[doc = "In \"LTR Per Picture\" mode (ltrTrustMode = 0 and ltrMarkFrame = 1), ltrNumFrames specifies maximum number of LTR frames in DPB."]
    pub ltrNumFrames: u32,
    #[doc = "Specifies the VPS id of the video parameter set"]
    pub vpsId: u32,
    #[doc = "Specifies the SPS id of the sequence header"]
    pub spsId: u32,
    #[doc = "Specifies the PPS id of the picture header"]
    pub ppsId: u32,
    #[doc = "This parameter in conjunction with sliceModeData specifies the way in which the picture is divided into slices"]
    #[doc = "sliceMode = 0 CTU based slices, sliceMode = 1 Byte based slices, sliceMode = 2 CTU row based slices, sliceMode = 3, numSlices in Picture"]
    #[doc = "When sliceMode == 0 and sliceModeData == 0 whole picture will be coded with one slice"]
    pub sliceMode: u32,
    #[doc = "Specifies the parameter needed for sliceMode. For:"]
    #[doc = "sliceMode = 0, sliceModeData specifies # of CTUs in each slice (except last slice)"]
    #[doc = "sliceMode = 1, sliceModeData specifies maximum # of bytes in each slice (except last slice)"]
    #[doc = "sliceMode = 2, sliceModeData specifies # of CTU rows in each slice (except last slice)"]
    #[doc = "sliceMode = 3, sliceModeData specifies number of slices in the picture. Driver will divide picture into slices optimally"]
    pub sliceModeData: u32,
    #[doc = "Specifies the max temporal layer used for hierarchical coding."]
    pub maxTemporalLayersMinus1: u32,
    #[doc = "Specifies the HEVC video usability info pamameters"]
    pub hevcVUIParameters: NV_ENC_CONFIG_HEVC_VUI_PARAMETERS,
    #[doc = "Specifies the LTR operating mode. See comments near NV_ENC_CONFIG_HEVC::enableLTR for description of the two modes."]
    #[doc = "Set to 1 to use \"LTR Trust\" mode of LTR operation. Clients are discouraged to use \"LTR Trust\" mode as this mode may"]
    #[doc = "be deprecated in future releases."]
    #[doc = "Set to 0 when using \"LTR Per Picture\" mode of LTR operation."]
    pub ltrTrustMode: u32,
    #[doc = "Specifies the B-Frame as reference mode. Check support for useBFramesAsRef mode using  ::NV_ENC_CAPS_SUPPORT_BFRAME_REF_MODE caps."]
    pub useBFramesAsRef: NV_ENC_BFRAME_REF_MODE,
    #[doc = "Specifies max number of reference frames in reference picture list L0, that can be used by hardware for prediction of a frame."]
    #[doc = "Check support for numRefL0 using ::NV_ENC_CAPS_SUPPORT_MULTIPLE_REF_FRAMES caps."]
    pub numRefL0: NV_ENC_NUM_REF_FRAMES,
    #[doc = "Specifies max number of reference frames in reference picture list L1, that can be used by hardware for prediction of a frame."]
    #[doc = "Check support for numRefL1 using ::NV_ENC_CAPS_SUPPORT_MULTIPLE_REF_FRAMES caps."]
    pub numRefL1: NV_ENC_NUM_REF_FRAMES,
    #[doc = "Reserved and must be set to 0."]
    pub reserved1: [u32; 214usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
impl _NV_ENC_CONFIG_HEVC {
    #[inline]
    pub fn useConstrainedIntraPred(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_useConstrainedIntraPred(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disableDeblockAcrossSliceBoundary(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disableDeblockAcrossSliceBoundary(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outputBufferingPeriodSEI(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outputBufferingPeriodSEI(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outputPictureTimingSEI(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outputPictureTimingSEI(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outputAUD(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outputAUD(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableLTR(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableLTR(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disableSPSPPS(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disableSPSPPS(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn repeatSPSPPS(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_repeatSPSPPS(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableIntraRefresh(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableIntraRefresh(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn chromaFormatIDC(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(9usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_chromaFormatIDC(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(9usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn pixelBitDepthMinus8(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(11usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_pixelBitDepthMinus8(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(11usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn enableFillerDataInsertion(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableFillerDataInsertion(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(15usize, 17u8) as u32) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(15usize, 17u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        useConstrainedIntraPred: u32,
        disableDeblockAcrossSliceBoundary: u32,
        outputBufferingPeriodSEI: u32,
        outputPictureTimingSEI: u32,
        outputAUD: u32,
        enableLTR: u32,
        disableSPSPPS: u32,
        repeatSPSPPS: u32,
        enableIntraRefresh: u32,
        chromaFormatIDC: u32,
        pixelBitDepthMinus8: u32,
        enableFillerDataInsertion: u32,
        reserved: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let useConstrainedIntraPred: u32 =
                unsafe { ::core::mem::transmute(useConstrainedIntraPred) };
            useConstrainedIntraPred as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let disableDeblockAcrossSliceBoundary: u32 =
                unsafe { ::core::mem::transmute(disableDeblockAcrossSliceBoundary) };
            disableDeblockAcrossSliceBoundary as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let outputBufferingPeriodSEI: u32 =
                unsafe { ::core::mem::transmute(outputBufferingPeriodSEI) };
            outputBufferingPeriodSEI as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let outputPictureTimingSEI: u32 =
                unsafe { ::core::mem::transmute(outputPictureTimingSEI) };
            outputPictureTimingSEI as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let outputAUD: u32 = unsafe { ::core::mem::transmute(outputAUD) };
            outputAUD as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let enableLTR: u32 = unsafe { ::core::mem::transmute(enableLTR) };
            enableLTR as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let disableSPSPPS: u32 = unsafe { ::core::mem::transmute(disableSPSPPS) };
            disableSPSPPS as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let repeatSPSPPS: u32 = unsafe { ::core::mem::transmute(repeatSPSPPS) };
            repeatSPSPPS as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let enableIntraRefresh: u32 = unsafe { ::core::mem::transmute(enableIntraRefresh) };
            enableIntraRefresh as u64
        });
        __bindgen_bitfield_unit.set(9usize, 2u8, {
            let chromaFormatIDC: u32 = unsafe { ::core::mem::transmute(chromaFormatIDC) };
            chromaFormatIDC as u64
        });
        __bindgen_bitfield_unit.set(11usize, 3u8, {
            let pixelBitDepthMinus8: u32 = unsafe { ::core::mem::transmute(pixelBitDepthMinus8) };
            pixelBitDepthMinus8 as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let enableFillerDataInsertion: u32 =
                unsafe { ::core::mem::transmute(enableFillerDataInsertion) };
            enableFillerDataInsertion as u64
        });
        __bindgen_bitfield_unit.set(15usize, 17u8, {
            let reserved: u32 = unsafe { ::core::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_CONFIG_HEVC = _NV_ENC_CONFIG_HEVC;
#[doc = " H264 encoder configuration parameters for ME only Mode"]
#[doc = ""]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CONFIG_H264_MEONLY {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 255usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
impl _NV_ENC_CONFIG_H264_MEONLY {
    #[inline]
    pub fn disablePartition16x16(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disablePartition16x16(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disablePartition8x16(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disablePartition8x16(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disablePartition16x8(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disablePartition16x8(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disablePartition8x8(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disablePartition8x8(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn disableIntraSearch(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_disableIntraSearch(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bStereoEnable(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bStereoEnable(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 26u8) as u32) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 26u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        disablePartition16x16: u32,
        disablePartition8x16: u32,
        disablePartition16x8: u32,
        disablePartition8x8: u32,
        disableIntraSearch: u32,
        bStereoEnable: u32,
        reserved: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let disablePartition16x16: u32 =
                unsafe { ::core::mem::transmute(disablePartition16x16) };
            disablePartition16x16 as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let disablePartition8x16: u32 = unsafe { ::core::mem::transmute(disablePartition8x16) };
            disablePartition8x16 as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let disablePartition16x8: u32 = unsafe { ::core::mem::transmute(disablePartition16x8) };
            disablePartition16x8 as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let disablePartition8x8: u32 = unsafe { ::core::mem::transmute(disablePartition8x8) };
            disablePartition8x8 as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let disableIntraSearch: u32 = unsafe { ::core::mem::transmute(disableIntraSearch) };
            disableIntraSearch as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let bStereoEnable: u32 = unsafe { ::core::mem::transmute(bStereoEnable) };
            bStereoEnable as u64
        });
        __bindgen_bitfield_unit.set(6usize, 26u8, {
            let reserved: u32 = unsafe { ::core::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_CONFIG_H264_MEONLY = _NV_ENC_CONFIG_H264_MEONLY;
#[doc = " HEVC encoder configuration parameters for ME only Mode"]
#[doc = ""]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CONFIG_HEVC_MEONLY {
    #[doc = "Reserved and must be set to 0"]
    pub reserved: [u32; 256usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved1: [*mut ::core::ffi::c_void; 64usize],
}
pub type NV_ENC_CONFIG_HEVC_MEONLY = _NV_ENC_CONFIG_HEVC_MEONLY;
#[doc = " Codec-specific encoder configuration parameters to be set during initialization."]
#[repr(C)]
#[derive(Copy, Clone)]
pub union _NV_ENC_CODEC_CONFIG {
    #[doc = "Specifies the H.264-specific encoder configuration."]
    pub h264Config: NV_ENC_CONFIG_H264,
    #[doc = "Specifies the HEVC-specific encoder configuration."]
    pub hevcConfig: NV_ENC_CONFIG_HEVC,
    #[doc = "Specifies the H.264-specific ME only encoder configuration."]
    pub h264MeOnlyConfig: NV_ENC_CONFIG_H264_MEONLY,
    #[doc = "Specifies the HEVC-specific ME only encoder configuration."]
    pub hevcMeOnlyConfig: NV_ENC_CONFIG_HEVC_MEONLY,
    #[doc = "Reserved and must be set to 0"]
    pub reserved: [u32; 320usize],
    _bindgen_union_align: [u64; 224usize],
}
pub type NV_ENC_CODEC_CONFIG = _NV_ENC_CODEC_CONFIG;
#[doc = " Encoder configuration parameters to be set during initialization."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_CONFIG {
    #[doc = "Struct version. Must be set to ::NV_ENC_CONFIG_VER."]
    pub version: u32,
    #[doc = "Specifies the codec profile guid. If client specifies \\p NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID the NvEncodeAPI interface will select the appropriate codec profile."]
    pub profileGUID: GUID,
    #[doc = "Specifies the number of pictures in one GOP. Low latency application client can set goplength to NVENC_INFINITE_GOPLENGTH so that keyframes are not inserted automatically."]
    pub gopLength: u32,
    #[doc = "Specifies the GOP pattern as follows: \\p frameIntervalP = 0: I, 1: IPP, 2: IBP, 3: IBBP  If goplength is set to NVENC_INFINITE_GOPLENGTH \\p frameIntervalP should be set to 1."]
    pub frameIntervalP: i32,
    #[doc = "Set this to 1 to enable monochrome encoding for this session."]
    pub monoChromeEncoding: u32,
    #[doc = "Specifies the frame/field mode."]
    #[doc = "Check support for field encoding using ::NV_ENC_CAPS_SUPPORT_FIELD_ENCODING caps."]
    #[doc = "Using a frameFieldMode other than NV_ENC_PARAMS_FRAME_FIELD_MODE_FRAME for RGB input is not supported."]
    pub frameFieldMode: NV_ENC_PARAMS_FRAME_FIELD_MODE,
    #[doc = "Specifies the desired motion vector prediction precision."]
    pub mvPrecision: NV_ENC_MV_PRECISION,
    #[doc = "Specifies the rate control parameters for the current encoding session."]
    pub rcParams: NV_ENC_RC_PARAMS,
    #[doc = "Specifies the codec specific config parameters through this union."]
    pub encodeCodecConfig: NV_ENC_CODEC_CONFIG,
    #[doc = "Reserved and must be set to 0"]
    pub reserved: [u32; 278usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
pub type NV_ENC_CONFIG = _NV_ENC_CONFIG;
#[doc = " Encode Session Initialization parameters."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_INITIALIZE_PARAMS {
    #[doc = "Struct version. Must be set to ::NV_ENC_INITIALIZE_PARAMS_VER."]
    pub version: u32,
    #[doc = "Specifies the Encode GUID for which the encoder is being created. ::NvEncInitializeEncoder() API will fail if this is not set, or set to unsupported value."]
    pub encodeGUID: GUID,
    #[doc = "Specifies the preset for encoding. If the preset GUID is set then , the preset configuration will be applied before any other parameter."]
    pub presetGUID: GUID,
    #[doc = "Specifies the encode width. If not set ::NvEncInitializeEncoder() API will fail."]
    pub encodeWidth: u32,
    #[doc = "Specifies the encode height. If not set ::NvEncInitializeEncoder() API will fail."]
    pub encodeHeight: u32,
    #[doc = "Specifies the display aspect ratio Width."]
    pub darWidth: u32,
    #[doc = "Specifies the display aspect ratio height."]
    pub darHeight: u32,
    #[doc = "Specifies the numerator for frame rate used for encoding in frames per second ( Frame rate = frameRateNum / frameRateDen )."]
    pub frameRateNum: u32,
    #[doc = "Specifies the denominator for frame rate used for encoding in frames per second ( Frame rate = frameRateNum / frameRateDen )."]
    pub frameRateDen: u32,
    #[doc = "Set this to 1 to enable asynchronous mode and is expected to use events to get picture completion notification."]
    pub enableEncodeAsync: u32,
    #[doc = "Set this to 1 to enable the Picture Type Decision is be taken by the NvEncodeAPI interface."]
    pub enablePTD: u32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    #[doc = "Reserved private data buffer size and must be set to 0"]
    pub privDataSize: u32,
    #[doc = "Reserved private data buffer and must be set to NULL"]
    pub privData: *mut ::core::ffi::c_void,
    #[doc = "Specifies the advanced codec specific structure. If client has sent a valid codec config structure, it will override parameters set by the NV_ENC_INITIALIZE_PARAMS::presetGUID parameter. If set to NULL the NvEncodeAPI interface will use the NV_ENC_INITIALIZE_PARAMS::presetGUID to set the codec specific parameters."]
    #[doc = "Client can also optionally query the NvEncodeAPI interface to get codec specific parameters for a presetGUID using ::NvEncGetEncodePresetConfig() API. It can then modify (if required) some of the codec config parameters and send down a custom config structure as part of ::_NV_ENC_INITIALIZE_PARAMS."]
    #[doc = "Even in this case client is recommended to pass the same preset guid it has used in ::NvEncGetEncodePresetConfig() API to query the config structure; as NV_ENC_INITIALIZE_PARAMS::presetGUID. This will not override the custom config structure but will be used to determine other Encoder HW specific parameters not exposed in the API."]
    pub encodeConfig: *mut NV_ENC_CONFIG,
    #[doc = "Maximum encode width to be used for current Encode session."]
    #[doc = "Client should allocate output buffers according to this dimension for dynamic resolution change. If set to 0, Encoder will not allow dynamic resolution change."]
    pub maxEncodeWidth: u32,
    #[doc = "Maximum encode height to be allowed for current Encode session."]
    #[doc = "Client should allocate output buffers according to this dimension for dynamic resolution change. If set to 0, Encode will not allow dynamic resolution change."]
    pub maxEncodeHeight: u32,
    #[doc = "If Client wants to pass external motion vectors in NV_ENC_PIC_PARAMS::meExternalHints buffer it must specify the maximum number of hint candidates per block per direction for the encode session."]
    #[doc = "The `NV_ENC_INITIALIZE_PARAMS::maxMEHintCountsPerBlock[0]` is for L0 predictors and `NV_ENC_INITIALIZE_PARAMS::maxMEHintCountsPerBlock[1]` is for L1 predictors."]
    #[doc = "This client must also set NV_ENC_INITIALIZE_PARAMS::enableExternalMEHints to 1."]
    pub maxMEHintCountsPerBlock: [NVENC_EXTERNAL_ME_HINT_COUNTS_PER_BLOCKTYPE; 2usize],
    #[doc = "Reserved and must be set to 0"]
    pub reserved: [u32; 289usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
impl _NV_ENC_INITIALIZE_PARAMS {
    #[inline]
    pub fn reportSliceOffsets(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reportSliceOffsets(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableSubFrameWrite(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableSubFrameWrite(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableExternalMEHints(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableExternalMEHints(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableMEOnlyMode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableMEOnlyMode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableWeightedPrediction(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableWeightedPrediction(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn enableOutputInVidmem(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableOutputInVidmem(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reservedBitFields(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 26u8) as u32) }
    }
    #[inline]
    pub fn set_reservedBitFields(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 26u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        reportSliceOffsets: u32,
        enableSubFrameWrite: u32,
        enableExternalMEHints: u32,
        enableMEOnlyMode: u32,
        enableWeightedPrediction: u32,
        enableOutputInVidmem: u32,
        reservedBitFields: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let reportSliceOffsets: u32 = unsafe { ::core::mem::transmute(reportSliceOffsets) };
            reportSliceOffsets as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let enableSubFrameWrite: u32 = unsafe { ::core::mem::transmute(enableSubFrameWrite) };
            enableSubFrameWrite as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let enableExternalMEHints: u32 =
                unsafe { ::core::mem::transmute(enableExternalMEHints) };
            enableExternalMEHints as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let enableMEOnlyMode: u32 = unsafe { ::core::mem::transmute(enableMEOnlyMode) };
            enableMEOnlyMode as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let enableWeightedPrediction: u32 =
                unsafe { ::core::mem::transmute(enableWeightedPrediction) };
            enableWeightedPrediction as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let enableOutputInVidmem: u32 = unsafe { ::core::mem::transmute(enableOutputInVidmem) };
            enableOutputInVidmem as u64
        });
        __bindgen_bitfield_unit.set(6usize, 26u8, {
            let reservedBitFields: u32 = unsafe { ::core::mem::transmute(reservedBitFields) };
            reservedBitFields as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_INITIALIZE_PARAMS = _NV_ENC_INITIALIZE_PARAMS;
#[doc = " Encode Session Reconfigured parameters."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_RECONFIGURE_PARAMS {
    #[doc = "Struct version. Must be set to ::NV_ENC_RECONFIGURE_PARAMS_VER."]
    pub version: u32,
    #[doc = "Encoder session re-initialization parameters."]
    #[doc = "If reInitEncodeParams.encodeConfig is NULL and"]
    #[doc = "reInitEncodeParams.presetGUID is the same as the preset"]
    #[doc = "GUID specified on the call to NvEncInitializeEncoder(),"]
    #[doc = "EncodeAPI will continue to use the existing encode"]
    #[doc = "configuration."]
    #[doc = "If reInitEncodeParams.encodeConfig is NULL and"]
    #[doc = "reInitEncodeParams.presetGUID is different from the preset"]
    #[doc = "GUID specified on the call to NvEncInitializeEncoder(),"]
    #[doc = "EncodeAPI will try to use the default configuration for"]
    #[doc = "the preset specified by reInitEncodeParams.presetGUID."]
    #[doc = "In this case, reconfiguration may fail if the new"]
    #[doc = "configuration is incompatible with the existing"]
    #[doc = "configuration (e.g. the new configuration results in"]
    #[doc = "a change in the GOP structure)."]
    pub reInitEncodeParams: NV_ENC_INITIALIZE_PARAMS,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    pub __bindgen_padding_0: u32,
}
impl _NV_ENC_RECONFIGURE_PARAMS {
    #[inline]
    pub fn resetEncoder(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_resetEncoder(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn forceIDR(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_forceIDR(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 30u8) as u32) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 30u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        resetEncoder: u32,
        forceIDR: u32,
        reserved: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let resetEncoder: u32 = unsafe { ::core::mem::transmute(resetEncoder) };
            resetEncoder as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let forceIDR: u32 = unsafe { ::core::mem::transmute(forceIDR) };
            forceIDR as u64
        });
        __bindgen_bitfield_unit.set(2usize, 30u8, {
            let reserved: u32 = unsafe { ::core::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_RECONFIGURE_PARAMS = _NV_ENC_RECONFIGURE_PARAMS;
#[doc = " Encoder preset config"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_PRESET_CONFIG {
    #[doc = " Struct version. Must be set to ::NV_ENC_PRESET_CONFIG_VER."]
    pub version: u32,
    #[doc = "preset config returned by the Nvidia Video Encoder interface."]
    pub presetCfg: NV_ENC_CONFIG,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 255usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
pub type NV_ENC_PRESET_CONFIG = _NV_ENC_PRESET_CONFIG;
#[doc = " MVC-specific parameters to be sent on a per-frame basis."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_PIC_PARAMS_MVC {
    #[doc = "Struct version. Must be set to ::NV_ENC_PIC_PARAMS_MVC_VER."]
    pub version: u32,
    #[doc = "Specifies the view ID associated with the current input view."]
    pub viewID: u32,
    #[doc = "Specifies the temporal ID associated with the current input view."]
    pub temporalID: u32,
    #[doc = "Specifies the priority ID associated with the current input view. Reserved and ignored by the NvEncodeAPI interface."]
    pub priorityID: u32,
    #[doc = "Reserved and must be set to 0."]
    pub reserved1: [u32; 12usize],
    #[doc = "Reserved and must be set to NULL."]
    pub reserved2: [*mut ::core::ffi::c_void; 8usize],
}
pub type NV_ENC_PIC_PARAMS_MVC = _NV_ENC_PIC_PARAMS_MVC;
#[doc = " H264 extension  picture parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union _NV_ENC_PIC_PARAMS_H264_EXT {
    #[doc = "Specifies the MVC picture parameters."]
    pub mvcPicParams: NV_ENC_PIC_PARAMS_MVC,
    #[doc = "Reserved and must be set to 0."]
    pub reserved1: [u32; 32usize],
    _bindgen_union_align: [u64; 16usize],
}
pub type NV_ENC_PIC_PARAMS_H264_EXT = _NV_ENC_PIC_PARAMS_H264_EXT;
#[doc = "  User SEI message"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_SEI_PAYLOAD {
    #[doc = "SEI payload size in bytes. SEI payload must be byte aligned, as described in Annex D"]
    pub payloadSize: u32,
    #[doc = "SEI payload types and syntax can be found in Annex D of the H.264 Specification."]
    pub payloadType: u32,
    #[doc = "pointer to user data"]
    pub payload: *mut u8,
}
pub type NV_ENC_SEI_PAYLOAD = _NV_ENC_SEI_PAYLOAD;
#[doc = " H264 specific enc pic params. sent on a per frame basis."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_PIC_PARAMS_H264 {
    #[doc = "Specifies the display POC syntax This is required to be set if client is handling the picture type decision."]
    pub displayPOCSyntax: u32,
    #[doc = "Reserved and must be set to 0"]
    pub reserved3: u32,
    #[doc = "Set to 1 for a reference picture. This is ignored if NV_ENC_INITIALIZE_PARAMS::enablePTD is set to 1."]
    pub refPicFlag: u32,
    #[doc = "Specifies the colour plane ID associated with the current input."]
    pub colourPlaneId: u32,
    #[doc = "Forces an intra refresh with duration equal to intraRefreshFrameCnt."]
    #[doc = "When outputRecoveryPointSEI is set this is value is used for recovery_frame_cnt in recovery point SEI message"]
    #[doc = "forceIntraRefreshWithFrameCnt cannot be used if B frames are used in the GOP structure specified"]
    pub forceIntraRefreshWithFrameCnt: u32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    #[doc = "Deprecated."]
    pub sliceTypeData: *mut u8,
    #[doc = "Deprecated."]
    pub sliceTypeArrayCnt: u32,
    #[doc = "Specifies the number of elements allocated in  seiPayloadArray array."]
    pub seiPayloadArrayCnt: u32,
    #[doc = "Array of SEI payloads which will be inserted for this frame."]
    pub seiPayloadArray: *mut NV_ENC_SEI_PAYLOAD,
    #[doc = "This parameter in conjunction with sliceModeData specifies the way in which the picture is divided into slices"]
    #[doc = "sliceMode = 0 MB based slices, sliceMode = 1 Byte based slices, sliceMode = 2 MB row based slices, sliceMode = 3, numSlices in Picture"]
    #[doc = "When forceIntraRefreshWithFrameCnt is set it will have priority over sliceMode setting"]
    #[doc = "When sliceMode == 0 and sliceModeData == 0 whole picture will be coded with one slice"]
    pub sliceMode: u32,
    #[doc = "Specifies the parameter needed for sliceMode. For:"]
    #[doc = "sliceMode = 0, sliceModeData specifies # of MBs in each slice (except last slice)"]
    #[doc = "sliceMode = 1, sliceModeData specifies maximum # of bytes in each slice (except last slice)"]
    #[doc = "sliceMode = 2, sliceModeData specifies # of MB rows in each slice (except last slice)"]
    #[doc = "sliceMode = 3, sliceModeData specifies number of slices in the picture. Driver will divide picture into slices optimally"]
    pub sliceModeData: u32,
    #[doc = "Specifies the long term referenceframe index to use for marking this frame as LTR."]
    pub ltrMarkFrameIdx: u32,
    #[doc = "Specifies the the associated bitmap of LTR frame indices to use when encoding this frame."]
    pub ltrUseFrameBitmap: u32,
    #[doc = "Not supported. Reserved for future use and must be set to 0."]
    pub ltrUsageMode: u32,
    #[doc = "Specfies the number of slices to be forced to Intra in the current picture."]
    #[doc = "This option along with forceIntraSliceIdx[] array needs to be used with sliceMode = 3 only"]
    pub forceIntraSliceCount: u32,
    #[doc = "Slice indices to be forced to intra in the current picture. Each slice index should be <= num_slices_in_picture -1. Index starts from 0 for first slice."]
    #[doc = "The number of entries in this array should be equal to forceIntraSliceCount"]
    pub forceIntraSliceIdx: *mut u32,
    #[doc = "Specifies the H264 extension config parameters using this config."]
    pub h264ExtPicParams: NV_ENC_PIC_PARAMS_H264_EXT,
    #[doc = "Reserved and must be set to 0."]
    pub reserved: [u32; 210usize],
    #[doc = "Reserved and must be set to NULL."]
    pub reserved2: [*mut ::core::ffi::c_void; 61usize],
}
impl _NV_ENC_PIC_PARAMS_H264 {
    #[inline]
    pub fn constrainedFrame(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_constrainedFrame(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn sliceModeDataUpdate(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_sliceModeDataUpdate(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ltrMarkFrame(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ltrMarkFrame(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ltrUseFrames(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ltrUseFrames(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reservedBitFields(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 28u8) as u32) }
    }
    #[inline]
    pub fn set_reservedBitFields(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 28u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        constrainedFrame: u32,
        sliceModeDataUpdate: u32,
        ltrMarkFrame: u32,
        ltrUseFrames: u32,
        reservedBitFields: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let constrainedFrame: u32 = unsafe { ::core::mem::transmute(constrainedFrame) };
            constrainedFrame as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let sliceModeDataUpdate: u32 = unsafe { ::core::mem::transmute(sliceModeDataUpdate) };
            sliceModeDataUpdate as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let ltrMarkFrame: u32 = unsafe { ::core::mem::transmute(ltrMarkFrame) };
            ltrMarkFrame as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let ltrUseFrames: u32 = unsafe { ::core::mem::transmute(ltrUseFrames) };
            ltrUseFrames as u64
        });
        __bindgen_bitfield_unit.set(4usize, 28u8, {
            let reservedBitFields: u32 = unsafe { ::core::mem::transmute(reservedBitFields) };
            reservedBitFields as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_PIC_PARAMS_H264 = _NV_ENC_PIC_PARAMS_H264;
#[doc = " HEVC specific enc pic params. sent on a per frame basis."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_PIC_PARAMS_HEVC {
    #[doc = "Specifies the display POC syntax This is required to be set if client is handling the picture type decision."]
    pub displayPOCSyntax: u32,
    #[doc = "Set to 1 for a reference picture. This is ignored if NV_ENC_INITIALIZE_PARAMS::enablePTD is set to 1."]
    pub refPicFlag: u32,
    #[doc = "Specifies the temporal id of the picture"]
    pub temporalId: u32,
    #[doc = "Forces an intra refresh with duration equal to intraRefreshFrameCnt."]
    #[doc = "When outputRecoveryPointSEI is set this is value is used for recovery_frame_cnt in recovery point SEI message"]
    #[doc = "forceIntraRefreshWithFrameCnt cannot be used if B frames are used in the GOP structure specified"]
    pub forceIntraRefreshWithFrameCnt: u32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    #[doc = "Array which specifies the slice type used to force intra slice for a particular slice. Currently supported only for NV_ENC_CONFIG_H264::sliceMode == 3."]
    #[doc = "Client should allocate array of size sliceModeData where sliceModeData is specified in field of ::_NV_ENC_CONFIG_H264"]
    #[doc = "Array element with index n corresponds to nth slice. To force a particular slice to intra client should set corresponding array element to NV_ENC_SLICE_TYPE_I"]
    #[doc = "all other array elements should be set to NV_ENC_SLICE_TYPE_DEFAULT"]
    pub sliceTypeData: *mut u8,
    #[doc = "Client should set this to the number of elements allocated in sliceTypeData array. If sliceTypeData is NULL then this should be set to 0"]
    pub sliceTypeArrayCnt: u32,
    #[doc = "This parameter in conjunction with sliceModeData specifies the way in which the picture is divided into slices"]
    #[doc = "sliceMode = 0 CTU based slices, sliceMode = 1 Byte based slices, sliceMode = 2 CTU row based slices, sliceMode = 3, numSlices in Picture"]
    #[doc = "When forceIntraRefreshWithFrameCnt is set it will have priority over sliceMode setting"]
    #[doc = "When sliceMode == 0 and sliceModeData == 0 whole picture will be coded with one slice"]
    pub sliceMode: u32,
    #[doc = "Specifies the parameter needed for sliceMode. For:"]
    #[doc = "sliceMode = 0, sliceModeData specifies # of CTUs in each slice (except last slice)"]
    #[doc = "sliceMode = 1, sliceModeData specifies maximum # of bytes in each slice (except last slice)"]
    #[doc = "sliceMode = 2, sliceModeData specifies # of CTU rows in each slice (except last slice)"]
    #[doc = "sliceMode = 3, sliceModeData specifies number of slices in the picture. Driver will divide picture into slices optimally"]
    pub sliceModeData: u32,
    #[doc = "Specifies the long term reference frame index to use for marking this frame as LTR."]
    pub ltrMarkFrameIdx: u32,
    #[doc = "Specifies the associated bitmap of LTR frame indices to use when encoding this frame."]
    pub ltrUseFrameBitmap: u32,
    #[doc = "Not supported. Reserved for future use and must be set to 0."]
    pub ltrUsageMode: u32,
    #[doc = "Specifies the number of elements allocated in  seiPayloadArray array."]
    pub seiPayloadArrayCnt: u32,
    #[doc = "Reserved and must be set to 0."]
    pub reserved: u32,
    #[doc = "Array of SEI payloads which will be inserted for this frame."]
    pub seiPayloadArray: *mut NV_ENC_SEI_PAYLOAD,
    #[doc = "Reserved and must be set to 0."]
    pub reserved2: [u32; 244usize],
    #[doc = "Reserved and must be set to NULL."]
    pub reserved3: [*mut ::core::ffi::c_void; 61usize],
}
impl _NV_ENC_PIC_PARAMS_HEVC {
    #[inline]
    pub fn constrainedFrame(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_constrainedFrame(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn sliceModeDataUpdate(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_sliceModeDataUpdate(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ltrMarkFrame(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ltrMarkFrame(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ltrUseFrames(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ltrUseFrames(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reservedBitFields(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 28u8) as u32) }
    }
    #[inline]
    pub fn set_reservedBitFields(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 28u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        constrainedFrame: u32,
        sliceModeDataUpdate: u32,
        ltrMarkFrame: u32,
        ltrUseFrames: u32,
        reservedBitFields: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let constrainedFrame: u32 = unsafe { ::core::mem::transmute(constrainedFrame) };
            constrainedFrame as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let sliceModeDataUpdate: u32 = unsafe { ::core::mem::transmute(sliceModeDataUpdate) };
            sliceModeDataUpdate as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let ltrMarkFrame: u32 = unsafe { ::core::mem::transmute(ltrMarkFrame) };
            ltrMarkFrame as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let ltrUseFrames: u32 = unsafe { ::core::mem::transmute(ltrUseFrames) };
            ltrUseFrames as u64
        });
        __bindgen_bitfield_unit.set(4usize, 28u8, {
            let reservedBitFields: u32 = unsafe { ::core::mem::transmute(reservedBitFields) };
            reservedBitFields as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_PIC_PARAMS_HEVC = _NV_ENC_PIC_PARAMS_HEVC;
#[doc = " Codec specific per-picture encoding parameters."]
#[repr(C)]
#[derive(Copy, Clone)]
pub union _NV_ENC_CODEC_PIC_PARAMS {
    #[doc = "H264 encode picture params."]
    pub h264PicParams: NV_ENC_PIC_PARAMS_H264,
    #[doc = "HEVC encode picture params."]
    pub hevcPicParams: NV_ENC_PIC_PARAMS_HEVC,
    #[doc = "Reserved and must be set to 0."]
    pub reserved: [u32; 256usize],
    _bindgen_union_align: [u64; 192usize],
}
pub type NV_ENC_CODEC_PIC_PARAMS = _NV_ENC_CODEC_PIC_PARAMS;
#[doc = " Encoding parameters that need to be sent on a per frame basis."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_PIC_PARAMS {
    #[doc = "Struct version. Must be set to ::NV_ENC_PIC_PARAMS_VER."]
    pub version: u32,
    #[doc = "Specifies the input buffer width"]
    pub inputWidth: u32,
    #[doc = "Specifies the input buffer height"]
    pub inputHeight: u32,
    #[doc = "Specifies the input buffer pitch. If pitch value is not known, set this to inputWidth."]
    pub inputPitch: u32,
    #[doc = "Specifies bit-wise OR`ed encode pic flags. See ::NV_ENC_PIC_FLAGS enum."]
    pub encodePicFlags: u32,
    #[doc = "Specifies the frame index associated with the input frame (optional)."]
    pub frameIdx: u32,
    #[doc = "Specifies presentation timestamp associated with the input picture."]
    pub inputTimeStamp: u64,
    #[doc = "Specifies duration of the input picture"]
    pub inputDuration: u64,
    #[doc = "Specifies the input buffer pointer. Client must use a pointer obtained from ::NvEncCreateInputBuffer() or ::NvEncMapInputResource() APIs."]
    pub inputBuffer: NV_ENC_INPUT_PTR,
    #[doc = "Specifies the output buffer pointer."]
    #[doc = "If NV_ENC_INITIALIZE_PARAMS::enableOutputInVidmem is set to 0, specifies the pointer to output buffer. Client should use a pointer obtained from ::NvEncCreateBitstreamBuffer() API."]
    #[doc = "If NV_ENC_INITIALIZE_PARAMS::enableOutputInVidmem is set to 1, client should allocate buffer in video memory for NV_ENC_ENCODE_OUT_PARAMS struct and encoded bitstream data. Client"]
    #[doc = "should use a pointer obtained from ::NvEncMapInputResource() API, when mapping this output buffer and assign it to NV_ENC_PIC_PARAMS::outputBitstream."]
    #[doc = "First 256 bytes of this buffer should be interpreted as NV_ENC_ENCODE_OUT_PARAMS struct followed by encoded bitstream data. Recommended size for output buffer is sum of size of"]
    #[doc = "NV_ENC_ENCODE_OUT_PARAMS struct and twice the input frame size for lower resolution eg. CIF and 1.5 times the input frame size for higher resolutions. If encoded bitstream size is"]
    #[doc = "greater than the allocated buffer size for encoded bitstream, then the output buffer will have encoded bitstream data equal to buffer size. All CUDA operations on this buffer must use"]
    #[doc = "the default stream."]
    pub outputBitstream: NV_ENC_OUTPUT_PTR,
    #[doc = "Specifies an event to be signalled on completion of encoding of this Frame [only if operating in Asynchronous mode]. Each output buffer should be associated with a distinct event pointer."]
    pub completionEvent: *mut ::core::ffi::c_void,
    #[doc = "Specifies the input buffer format."]
    pub bufferFmt: NV_ENC_BUFFER_FORMAT,
    #[doc = "Specifies structure of the input picture."]
    pub pictureStruct: NV_ENC_PIC_STRUCT,
    #[doc = "Specifies input picture type. Client required to be set explicitly by the client if the client has not set NV_ENC_INITALIZE_PARAMS::enablePTD to 1 while calling NvInitializeEncoder."]
    pub pictureType: NV_ENC_PIC_TYPE,
    #[doc = "Specifies the codec specific per-picture encoding parameters."]
    pub codecPicParams: NV_ENC_CODEC_PIC_PARAMS,
    #[doc = "Specifies the number of hint candidates per block per direction for the current frame. `meHintCountsPerBlock[0]` is for L0 predictors and `meHintCountsPerBlock[1]` is for L1 predictors."]
    #[doc = "The candidate count in `NV_ENC_PIC_PARAMS::meHintCountsPerBlock[lx]` must never exceed `NV_ENC_INITIALIZE_PARAMS::maxMEHintCountsPerBlock[lx]` provided during encoder intialization."]
    pub meHintCountsPerBlock: [NVENC_EXTERNAL_ME_HINT_COUNTS_PER_BLOCKTYPE; 2usize],
    #[doc = "Specifies the pointer to ME external hints for the current frame. The size of ME hint buffer should be equal to number of macroblocks * the total number of candidates per macroblock."]
    #[doc = "The total number of candidates per MB per direction = `1*meHintCountsPerBlock[Lx].numCandsPerBlk16x16 + 2*meHintCountsPerBlock[Lx].numCandsPerBlk16x8 + 2*meHintCountsPerBlock[Lx].numCandsPerBlk8x8`"]
    #[doc = "`+ 4*meHintCountsPerBlock[Lx].numCandsPerBlk8x8`. For frames using bidirectional ME , the total number of candidates for single macroblock is sum of total number of candidates per MB for each direction (L0 and L1)"]
    pub meExternalHints: *mut NVENC_EXTERNAL_ME_HINT,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 6usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 2usize],
    #[doc = "Specifies the pointer to signed byte array containing value per MB in raster scan order for the current picture, which will be interpreted depending on NV_ENC_RC_PARAMS::qpMapMode."]
    #[doc = "If NV_ENC_RC_PARAMS::qpMapMode is NV_ENC_QP_MAP_DELTA, qpDeltaMap specifies QP modifier per MB. This QP modifier will be applied on top of the QP chosen by rate control."]
    #[doc = "If NV_ENC_RC_PARAMS::qpMapMode is NV_ENC_QP_MAP_EMPHASIS, qpDeltaMap specifies Emphasis Level Map per MB. This level value along with QP chosen by rate control is used to"]
    #[doc = "compute the QP modifier, which in turn is applied on top of QP chosen by rate control."]
    #[doc = "If NV_ENC_RC_PARAMS::qpMapMode is NV_ENC_QP_MAP_DISABLED, value in qpDeltaMap will be ignored."]
    pub qpDeltaMap: *mut i8,
    #[doc = "Specifies the size in bytes of qpDeltaMap surface allocated by client and pointed to by NV_ENC_PIC_PARAMS::qpDeltaMap. Surface (array) should be picWidthInMbs * picHeightInMbs"]
    pub qpDeltaMapSize: u32,
    #[doc = "Reserved bitfields and must be set to 0"]
    pub reservedBitFields: u32,
    #[doc = "Specifies temporal distance for reference picture (NVENC_EXTERNAL_ME_HINT::refidx = 0) used during external ME with NV_ENC_INITALIZE_PARAMS::enablePTD = 1 . `meHintRefPicDist[0]` is for L0 hints and `meHintRefPicDist[1]` is for L1 hints."]
    #[doc = "If not set, will internally infer distance of 1. Ignored for NV_ENC_INITALIZE_PARAMS::enablePTD = 0"]
    pub meHintRefPicDist: [u16; 2usize],
    #[doc = "Reserved and must be set to 0"]
    pub reserved3: [u32; 286usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved4: [*mut ::core::ffi::c_void; 60usize],
}
pub type NV_ENC_PIC_PARAMS = _NV_ENC_PIC_PARAMS;
#[doc = " MEOnly parameters that need to be sent on a per motion estimation basis."]
#[doc = " NV_ENC_MEONLY_PARAMS::meExternalHints is supported for H264 only."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_MEONLY_PARAMS {
    #[doc = "Struct version. Must be set to NV_ENC_MEONLY_PARAMS_VER."]
    pub version: u32,
    #[doc = "Specifies the input buffer width"]
    pub inputWidth: u32,
    #[doc = "Specifies the input buffer height"]
    pub inputHeight: u32,
    #[doc = "Specifies the input buffer pointer. Client must use a pointer obtained from NvEncCreateInputBuffer() or NvEncMapInputResource() APIs."]
    pub inputBuffer: NV_ENC_INPUT_PTR,
    #[doc = "Specifies the reference frame pointer"]
    pub referenceFrame: NV_ENC_INPUT_PTR,
    #[doc = "Specifies the output buffer pointer."]
    #[doc = "If NV_ENC_INITIALIZE_PARAMS::enableOutputInVidmem is set to 0, specifies the pointer to motion vector data buffer allocated by NvEncCreateMVBuffer."]
    #[doc = "Client must lock mvBuffer using ::NvEncLockBitstream() API to get the motion vector data."]
    #[doc = "If NV_ENC_INITIALIZE_PARAMS::enableOutputInVidmem is set to 1, client should allocate buffer in video memory for storing the motion vector data. The size of this buffer must"]
    #[doc = "be equal to total number of macroblocks multiplied by size of NV_ENC_H264_MV_DATA struct. Client should use a pointer obtained from ::NvEncMapInputResource() API, when mapping this"]
    #[doc = "output buffer and assign it to NV_ENC_MEONLY_PARAMS::mvBuffer. All CUDA operations on this buffer must use the default stream."]
    pub mvBuffer: NV_ENC_OUTPUT_PTR,
    #[doc = "Specifies the input buffer format."]
    pub bufferFmt: NV_ENC_BUFFER_FORMAT,
    #[doc = "Specifies an event to be signalled on completion of motion estimation"]
    #[doc = "of this Frame [only if operating in Asynchronous mode]."]
    #[doc = "Each output buffer should be associated with a distinct event pointer."]
    pub completionEvent: *mut ::core::ffi::c_void,
    #[doc = "Specifies left,right viewID if NV_ENC_CONFIG_H264_MEONLY::bStereoEnable is set."]
    #[doc = "viewID can be 0,1 if bStereoEnable is set, 0 otherwise."]
    pub viewID: u32,
    #[doc = "Specifies the number of hint candidates per block for the current frame. `meHintCountsPerBlock[0]` is for L0 predictors."]
    #[doc = "The candidate count in `NV_ENC_PIC_PARAMS::meHintCountsPerBlock[lx]` must never exceed `NV_ENC_INITIALIZE_PARAMS::maxMEHintCountsPerBlock[lx]` provided during encoder intialization."]
    pub meHintCountsPerBlock: [NVENC_EXTERNAL_ME_HINT_COUNTS_PER_BLOCKTYPE; 2usize],
    #[doc = "Specifies the pointer to ME external hints for the current frame. The size of ME hint buffer should be equal to number of macroblocks * the total number of candidates per macroblock."]
    #[doc = "The total number of candidates per MB per direction = `1*meHintCountsPerBlock[Lx].numCandsPerBlk16x16 + 2*meHintCountsPerBlock[Lx].numCandsPerBlk16x8 + 2*meHintCountsPerBlock[Lx].numCandsPerBlk8x8`"]
    #[doc = "`+ 4*meHintCountsPerBlock[Lx].numCandsPerBlk8x8`. For frames using bidirectional ME , the total number of candidates for single macroblock is sum of total number of candidates per MB for each direction (L0 and L1)"]
    pub meExternalHints: *mut NVENC_EXTERNAL_ME_HINT,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 243usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 59usize],
}
pub type NV_ENC_MEONLY_PARAMS = _NV_ENC_MEONLY_PARAMS;
#[doc = " Bitstream buffer lock parameters."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_LOCK_BITSTREAM {
    #[doc = "Struct version. Must be set to ::NV_ENC_LOCK_BITSTREAM_VER."]
    pub version: u32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    #[doc = "Pointer to the bitstream buffer being locked."]
    pub outputBitstream: *mut ::core::ffi::c_void,
    #[doc = "Array which receives the slice offsets. This is not supported if NV_ENC_CONFIG_H264::sliceMode is 1 on Kepler GPUs. Array size must be equal to size of frame in MBs."]
    pub sliceOffsets: *mut u32,
    #[doc = "Frame no. for which the bitstream is being retrieved."]
    pub frameIdx: u32,
    #[doc = "The NvEncodeAPI interface status for the locked picture."]
    pub hwEncodeStatus: u32,
    #[doc = "Number of slices in the encoded picture. Will be reported only if NV_ENC_INITIALIZE_PARAMS::reportSliceOffsets set to 1."]
    pub numSlices: u32,
    #[doc = "Actual number of bytes generated and copied to the memory pointed by bitstreamBufferPtr."]
    pub bitstreamSizeInBytes: u32,
    #[doc = "Presentation timestamp associated with the encoded output."]
    pub outputTimeStamp: u64,
    #[doc = "Presentation duration associates with the encoded output."]
    pub outputDuration: u64,
    #[doc = "Pointer to the generated output bitstream."]
    #[doc = "For MEOnly mode _NV_ENC_LOCK_BITSTREAM::bitstreamBufferPtr should be typecast to"]
    #[doc = "NV_ENC_H264_MV_DATA/NV_ENC_HEVC_MV_DATA pointer respectively for H264/HEVC"]
    pub bitstreamBufferPtr: *mut ::core::ffi::c_void,
    #[doc = "Picture type of the encoded picture."]
    pub pictureType: NV_ENC_PIC_TYPE,
    #[doc = "Structure of the generated output picture."]
    pub pictureStruct: NV_ENC_PIC_STRUCT,
    #[doc = "Average QP of the frame."]
    pub frameAvgQP: u32,
    #[doc = "Total SATD cost for whole frame."]
    pub frameSatd: u32,
    #[doc = "Frame index associated with this LTR frame."]
    pub ltrFrameIdx: u32,
    #[doc = "Bitmap of LTR frames indices which were used for encoding this frame. Value of 0 if no LTR frames were used."]
    pub ltrFrameBitmap: u32,
    #[doc = "Reserved and must be set to 0"]
    pub reserved: [u32; 13usize],
    #[doc = "For H264, Number of Intra MBs in the encoded frame. For HEVC, Number of Intra CTBs in the encoded frame. Supported only if _NV_ENC_LOCK_BITSTREAM::getRCStats set to 1."]
    pub intraMBCount: u32,
    #[doc = "For H264, Number of Inter MBs in the encoded frame, includes skip MBs. For HEVC, Number of Inter CTBs in the encoded frame. Supported only if _NV_ENC_LOCK_BITSTREAM::getRCStats set to 1."]
    pub interMBCount: u32,
    #[doc = "Average Motion Vector in X direction for the encoded frame. Supported only if _NV_ENC_LOCK_BITSTREAM::getRCStats set to 1."]
    pub averageMVX: i32,
    #[doc = "Average Motion Vector in y direction for the encoded frame. Supported only if _NV_ENC_LOCK_BITSTREAM::getRCStats set to 1."]
    pub averageMVY: i32,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 219usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
impl _NV_ENC_LOCK_BITSTREAM {
    #[inline]
    pub fn doNotWait(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_doNotWait(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ltrFrame(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ltrFrame(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn getRCStats(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_getRCStats(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reservedBitFields(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 29u8) as u32) }
    }
    #[inline]
    pub fn set_reservedBitFields(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 29u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        doNotWait: u32,
        ltrFrame: u32,
        getRCStats: u32,
        reservedBitFields: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let doNotWait: u32 = unsafe { ::core::mem::transmute(doNotWait) };
            doNotWait as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let ltrFrame: u32 = unsafe { ::core::mem::transmute(ltrFrame) };
            ltrFrame as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let getRCStats: u32 = unsafe { ::core::mem::transmute(getRCStats) };
            getRCStats as u64
        });
        __bindgen_bitfield_unit.set(3usize, 29u8, {
            let reservedBitFields: u32 = unsafe { ::core::mem::transmute(reservedBitFields) };
            reservedBitFields as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_LOCK_BITSTREAM = _NV_ENC_LOCK_BITSTREAM;
#[doc = " Uncompressed Input Buffer lock parameters."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_LOCK_INPUT_BUFFER {
    #[doc = " Struct version. Must be set to ::NV_ENC_LOCK_INPUT_BUFFER_VER."]
    pub version: u32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    #[doc = " Pointer to the input buffer to be locked, client should pass the pointer obtained from ::NvEncCreateInputBuffer() or ::NvEncMapInputResource API."]
    pub inputBuffer: NV_ENC_INPUT_PTR,
    #[doc = "Pointed to the locked input buffer data. Client can only access input buffer using the \\p bufferDataPtr."]
    pub bufferDataPtr: *mut ::core::ffi::c_void,
    #[doc = "Pitch of the locked input buffer."]
    pub pitch: u32,
    #[doc = " Reserved and must be set to 0"]
    pub reserved1: [u32; 251usize],
    #[doc = " Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
impl _NV_ENC_LOCK_INPUT_BUFFER {
    #[inline]
    pub fn doNotWait(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_doNotWait(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reservedBitFields(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 31u8) as u32) }
    }
    #[inline]
    pub fn set_reservedBitFields(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        doNotWait: u32,
        reservedBitFields: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let doNotWait: u32 = unsafe { ::core::mem::transmute(doNotWait) };
            doNotWait as u64
        });
        __bindgen_bitfield_unit.set(1usize, 31u8, {
            let reservedBitFields: u32 = unsafe { ::core::mem::transmute(reservedBitFields) };
            reservedBitFields as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type NV_ENC_LOCK_INPUT_BUFFER = _NV_ENC_LOCK_INPUT_BUFFER;
#[doc = " Map an input resource to a Nvidia Encoder Input Buffer"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_MAP_INPUT_RESOURCE {
    #[doc = " Struct version. Must be set to ::NV_ENC_MAP_INPUT_RESOURCE_VER."]
    pub version: u32,
    #[doc = " Deprecated. Do not use."]
    pub subResourceIndex: u32,
    #[doc = " Deprecated. Do not use."]
    pub inputResource: *mut ::core::ffi::c_void,
    #[doc = " The Registered resource handle obtained by calling NvEncRegisterInputResource."]
    pub registeredResource: NV_ENC_REGISTERED_PTR,
    #[doc = "Mapped pointer corresponding to the registeredResource. This pointer must be used in NV_ENC_PIC_PARAMS::inputBuffer parameter in ::NvEncEncodePicture() API."]
    pub mappedResource: NV_ENC_INPUT_PTR,
    #[doc = "Buffer format of the outputResource. This buffer format must be used in NV_ENC_PIC_PARAMS::bufferFmt if client using the above mapped resource pointer."]
    pub mappedBufferFmt: NV_ENC_BUFFER_FORMAT,
    #[doc = " Reserved and must be set to 0."]
    pub reserved1: [u32; 251usize],
    #[doc = " Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 63usize],
}
pub type NV_ENC_MAP_INPUT_RESOURCE = _NV_ENC_MAP_INPUT_RESOURCE;
#[doc = " NV_ENC_REGISTER_RESOURCE::resourceToRegister must be a pointer to a variable of this type,"]
#[doc = " when NV_ENC_REGISTER_RESOURCE::resourceType is NV_ENC_INPUT_RESOURCE_TYPE_OPENGL_TEX"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NV_ENC_INPUT_RESOURCE_OPENGL_TEX {
    #[doc = "The name of the texture to be used."]
    pub texture: u32,
    #[doc = "Accepted values are GL_TEXTURE_RECTANGLE and GL_TEXTURE_2D."]
    pub target: u32,
}
pub type NV_ENC_INPUT_RESOURCE_OPENGL_TEX = _NV_ENC_INPUT_RESOURCE_OPENGL_TEX;
#[doc = " Register a resource for future use with the Nvidia Video Encoder Interface."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_REGISTER_RESOURCE {
    #[doc = "Struct version. Must be set to ::NV_ENC_REGISTER_RESOURCE_VER."]
    pub version: u32,
    #[doc = "Specifies the type of resource to be registered."]
    #[doc = "Supported values are"]
    #[doc = "::NV_ENC_INPUT_RESOURCE_TYPE_DIRECTX,"]
    #[doc = "::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR,"]
    #[doc = "::NV_ENC_INPUT_RESOURCE_TYPE_OPENGL_TEX"]
    pub resourceType: NV_ENC_INPUT_RESOURCE_TYPE,
    #[doc = "Input buffer Width."]
    pub width: u32,
    #[doc = "Input buffer Height."]
    pub height: u32,
    #[doc = "Input buffer Pitch."]
    #[doc = "For ::NV_ENC_INPUT_RESOURCE_TYPE_DIRECTX resources, set this to 0."]
    #[doc = "For ::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR resources, set this to"]
    #[doc = "the pitch as obtained from cuMemAllocPitch(), or to the width in"]
    #[doc = "bytes (if this resource was created by using cuMemAlloc()). This"]
    #[doc = "value must be a multiple of 4."]
    #[doc = "For ::NV_ENC_INPUT_RESOURCE_TYPE_CUDAARRAY resources, set this to the"]
    #[doc = "width of the allocation in bytes (i.e."]
    #[doc = "CUDA_ARRAY3D_DESCRIPTOR::Width * CUDA_ARRAY3D_DESCRIPTOR::NumChannels)."]
    #[doc = "For ::NV_ENC_INPUT_RESOURCE_TYPE_OPENGL_TEX resources, set this to the"]
    #[doc = "texture width multiplied by the number of components in the texture"]
    #[doc = "format."]
    pub pitch: u32,
    #[doc = "Subresource Index of the DirectX resource to be registered. Should be set to 0 for other interfaces."]
    pub subResourceIndex: u32,
    #[doc = "Handle to the resource that is being registered."]
    pub resourceToRegister: *mut ::core::ffi::c_void,
    #[doc = "Registered resource handle. This should be used in future interactions with the Nvidia Video Encoder Interface."]
    pub registeredResource: NV_ENC_REGISTERED_PTR,
    #[doc = "Buffer format of resource to be registered."]
    pub bufferFormat: NV_ENC_BUFFER_FORMAT,
    #[doc = "Usage of resource to be registered."]
    pub bufferUsage: NV_ENC_BUFFER_USAGE,
    #[doc = "Reserved and must be set to 0."]
    pub reserved1: [u32; 247usize],
    #[doc = "Reserved and must be set to NULL."]
    pub reserved2: [*mut ::core::ffi::c_void; 62usize],
}
pub type NV_ENC_REGISTER_RESOURCE = _NV_ENC_REGISTER_RESOURCE;
#[doc = " Encode Stats structure."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_STAT {
    #[doc = " Struct version. Must be set to ::NV_ENC_STAT_VER."]
    pub version: u32,
    #[doc = " Reserved and must be set to 0"]
    pub reserved: u32,
    #[doc = "Specifies the pointer to output bitstream."]
    pub outputBitStream: NV_ENC_OUTPUT_PTR,
    #[doc = "Size of generated bitstream in bytes."]
    pub bitStreamSize: u32,
    #[doc = "Picture type of encoded picture. See ::NV_ENC_PIC_TYPE."]
    pub picType: u32,
    #[doc = "Offset of last valid bytes of completed bitstream"]
    pub lastValidByteOffset: u32,
    #[doc = "Offsets of each slice"]
    pub sliceOffsets: [u32; 16usize],
    #[doc = "Picture number"]
    pub picIdx: u32,
    #[doc = " Reserved and must be set to 0"]
    pub reserved1: [u32; 233usize],
    #[doc = " Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
pub type NV_ENC_STAT = _NV_ENC_STAT;
#[doc = " Sequence and picture paramaters payload."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_SEQUENCE_PARAM_PAYLOAD {
    #[doc = " Struct version. Must be set to ::NV_ENC_INITIALIZE_PARAMS_VER."]
    pub version: u32,
    #[doc = " Specifies the size of the spsppsBuffer provied by the client"]
    pub inBufferSize: u32,
    #[doc = " Specifies the SPS id to be used in sequence header. Default value is 0."]
    pub spsId: u32,
    #[doc = " Specifies the PPS id to be used in picture header. Default value is 0."]
    pub ppsId: u32,
    #[doc = " Specifies bitstream header pointer of size NV_ENC_SEQUENCE_PARAM_PAYLOAD::inBufferSize. It is the client's responsibility to manage this memory."]
    pub spsppsBuffer: *mut ::core::ffi::c_void,
    #[doc = "Size of the sequence and picture header in  bytes written by the NvEncodeAPI interface to the SPSPPSBuffer."]
    pub outSPSPPSPayloadSize: *mut u32,
    #[doc = " Reserved and must be set to 0"]
    pub reserved: [u32; 250usize],
    #[doc = " Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
pub type NV_ENC_SEQUENCE_PARAM_PAYLOAD = _NV_ENC_SEQUENCE_PARAM_PAYLOAD;
#[doc = " Event registration/unregistration parameters."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_EVENT_PARAMS {
    #[doc = "Struct version. Must be set to ::NV_ENC_EVENT_PARAMS_VER."]
    pub version: u32,
    #[doc = "Reserved and must be set to 0"]
    pub reserved: u32,
    #[doc = "Handle to event to be registered/unregistered with the NvEncodeAPI interface."]
    pub completionEvent: *mut ::core::ffi::c_void,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 253usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
pub type NV_ENC_EVENT_PARAMS = _NV_ENC_EVENT_PARAMS;
#[doc = " Encoder Session Creation parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENC_OPEN_ENCODE_SESSIONEX_PARAMS {
    #[doc = "Struct version. Must be set to ::NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER."]
    pub version: u32,
    #[doc = "Specified the device Type"]
    pub deviceType: NV_ENC_DEVICE_TYPE,
    #[doc = "Pointer to client device."]
    pub device: *mut ::core::ffi::c_void,
    #[doc = "Reserved and must be set to 0."]
    pub reserved: *mut ::core::ffi::c_void,
    #[doc = "API version. Should be set to NVENCAPI_VERSION."]
    pub apiVersion: u32,
    #[doc = "Reserved and must be set to 0"]
    pub reserved1: [u32; 253usize],
    #[doc = "Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 64usize],
}
pub type NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS = _NV_ENC_OPEN_ENCODE_SESSIONEX_PARAMS;
#[doc = " \\cond API PFN"]
pub type PNVENCOPENENCODESESSION = ::core::option::Option<
    unsafe extern "C" fn(
        device: *mut ::core::ffi::c_void,
        deviceType: u32,
        encoder: *mut *mut ::core::ffi::c_void,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODEGUIDCOUNT = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUIDCount: *mut u32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODEGUIDS = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        GUIDs: *mut GUID,
        guidArraySize: u32,
        GUIDCount: *mut u32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODEPROFILEGUIDCOUNT = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUID: GUID,
        encodeProfileGUIDCount: *mut u32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODEPROFILEGUIDS = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUID: GUID,
        profileGUIDs: *mut GUID,
        guidArraySize: u32,
        GUIDCount: *mut u32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETINPUTFORMATCOUNT = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUID: GUID,
        inputFmtCount: *mut u32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETINPUTFORMATS = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUID: GUID,
        inputFmts: *mut NV_ENC_BUFFER_FORMAT,
        inputFmtArraySize: u32,
        inputFmtCount: *mut u32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODECAPS = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUID: GUID,
        capsParam: *mut NV_ENC_CAPS_PARAM,
        capsVal: *mut i32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODEPRESETCOUNT = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUID: GUID,
        encodePresetGUIDCount: *mut u32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODEPRESETGUIDS = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUID: GUID,
        presetGUIDs: *mut GUID,
        guidArraySize: u32,
        encodePresetGUIDCount: *mut u32,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODEPRESETCONFIG = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeGUID: GUID,
        presetGUID: GUID,
        presetConfig: *mut NV_ENC_PRESET_CONFIG,
    ) -> NVENCSTATUS,
>;
pub type PNVENCINITIALIZEENCODER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        createEncodeParams: *mut NV_ENC_INITIALIZE_PARAMS,
    ) -> NVENCSTATUS,
>;
pub type PNVENCCREATEINPUTBUFFER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        createInputBufferParams: *mut NV_ENC_CREATE_INPUT_BUFFER,
    ) -> NVENCSTATUS,
>;
pub type PNVENCDESTROYINPUTBUFFER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        inputBuffer: NV_ENC_INPUT_PTR,
    ) -> NVENCSTATUS,
>;
pub type PNVENCCREATEBITSTREAMBUFFER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        createBitstreamBufferParams: *mut NV_ENC_CREATE_BITSTREAM_BUFFER,
    ) -> NVENCSTATUS,
>;
pub type PNVENCDESTROYBITSTREAMBUFFER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        bitstreamBuffer: NV_ENC_OUTPUT_PTR,
    ) -> NVENCSTATUS,
>;
pub type PNVENCENCODEPICTURE = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodePicParams: *mut NV_ENC_PIC_PARAMS,
    ) -> NVENCSTATUS,
>;
pub type PNVENCLOCKBITSTREAM = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        lockBitstreamBufferParams: *mut NV_ENC_LOCK_BITSTREAM,
    ) -> NVENCSTATUS,
>;
pub type PNVENCUNLOCKBITSTREAM = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        bitstreamBuffer: NV_ENC_OUTPUT_PTR,
    ) -> NVENCSTATUS,
>;
pub type PNVENCLOCKINPUTBUFFER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        lockInputBufferParams: *mut NV_ENC_LOCK_INPUT_BUFFER,
    ) -> NVENCSTATUS,
>;
pub type PNVENCUNLOCKINPUTBUFFER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        inputBuffer: NV_ENC_INPUT_PTR,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETENCODESTATS = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        encodeStats: *mut NV_ENC_STAT,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETSEQUENCEPARAMS = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        sequenceParamPayload: *mut NV_ENC_SEQUENCE_PARAM_PAYLOAD,
    ) -> NVENCSTATUS,
>;
pub type PNVENCREGISTERASYNCEVENT = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        eventParams: *mut NV_ENC_EVENT_PARAMS,
    ) -> NVENCSTATUS,
>;
pub type PNVENCUNREGISTERASYNCEVENT = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        eventParams: *mut NV_ENC_EVENT_PARAMS,
    ) -> NVENCSTATUS,
>;
pub type PNVENCMAPINPUTRESOURCE = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        mapInputResParams: *mut NV_ENC_MAP_INPUT_RESOURCE,
    ) -> NVENCSTATUS,
>;
pub type PNVENCUNMAPINPUTRESOURCE = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        mappedInputBuffer: NV_ENC_INPUT_PTR,
    ) -> NVENCSTATUS,
>;
pub type PNVENCDESTROYENCODER =
    ::core::option::Option<unsafe extern "C" fn(encoder: *mut ::core::ffi::c_void) -> NVENCSTATUS>;
pub type PNVENCINVALIDATEREFFRAMES = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        invalidRefFrameTimeStamp: u64,
    ) -> NVENCSTATUS,
>;
pub type PNVENCOPENENCODESESSIONEX = ::core::option::Option<
    unsafe extern "C" fn(
        openSessionExParams: *mut NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
        encoder: *mut *mut ::core::ffi::c_void,
    ) -> NVENCSTATUS,
>;
pub type PNVENCREGISTERRESOURCE = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        registerResParams: *mut NV_ENC_REGISTER_RESOURCE,
    ) -> NVENCSTATUS,
>;
pub type PNVENCUNREGISTERRESOURCE = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        registeredRes: NV_ENC_REGISTERED_PTR,
    ) -> NVENCSTATUS,
>;
pub type PNVENCRECONFIGUREENCODER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        reInitEncodeParams: *mut NV_ENC_RECONFIGURE_PARAMS,
    ) -> NVENCSTATUS,
>;
pub type PNVENCCREATEMVBUFFER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        createMVBufferParams: *mut NV_ENC_CREATE_MV_BUFFER,
    ) -> NVENCSTATUS,
>;
pub type PNVENCDESTROYMVBUFFER = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        mvBuffer: NV_ENC_OUTPUT_PTR,
    ) -> NVENCSTATUS,
>;
pub type PNVENCRUNMOTIONESTIMATIONONLY = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        meOnlyParams: *mut NV_ENC_MEONLY_PARAMS,
    ) -> NVENCSTATUS,
>;
pub type PNVENCGETLASTERROR =
    ::core::option::Option<unsafe extern "C" fn(encoder: *mut ::core::ffi::c_void) -> *const u8>;
pub type PNVENCSETIOCUDASTREAMS = ::core::option::Option<
    unsafe extern "C" fn(
        encoder: *mut ::core::ffi::c_void,
        inputStream: NV_ENC_CUSTREAM_PTR,
        outputStream: NV_ENC_CUSTREAM_PTR,
    ) -> NVENCSTATUS,
>;

pub type PNVENCODEAPICREATEINSTANCE = ::core::option::Option<
    unsafe extern "C" fn(functionList: *mut NV_ENCODE_API_FUNCTION_LIST) -> NVENCSTATUS,
>;

pub type PNVENCODEAPIGETMAXSUPPORTEDVERSION =
    ::core::option::Option<unsafe extern "C" fn(version: *mut u32) -> NVENCSTATUS>;

#[doc = " NV_ENCODE_API_FUNCTION_LIST"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NV_ENCODE_API_FUNCTION_LIST {
    #[doc = "Client should pass NV_ENCODE_API_FUNCTION_LIST_VER."]
    pub version: u32,
    #[doc = "Reserved and should be set to 0."]
    pub reserved: u32,
    #[doc = "Client should access ::NvEncOpenEncodeSession() API through this pointer."]
    pub nvEncOpenEncodeSession: PNVENCOPENENCODESESSION,
    #[doc = "Client should access ::NvEncGetEncodeGUIDCount() API through this pointer."]
    pub nvEncGetEncodeGUIDCount: PNVENCGETENCODEGUIDCOUNT,
    #[doc = "Client should access ::NvEncGetEncodeProfileGUIDCount() API through this pointer."]
    pub nvEncGetEncodeProfileGUIDCount: PNVENCGETENCODEPRESETCOUNT,
    #[doc = "Client should access ::NvEncGetEncodeProfileGUIDs() API through this pointer."]
    pub nvEncGetEncodeProfileGUIDs: PNVENCGETENCODEPRESETGUIDS,
    #[doc = "Client should access ::NvEncGetEncodeGUIDs() API through this pointer."]
    pub nvEncGetEncodeGUIDs: PNVENCGETENCODEGUIDS,
    #[doc = "Client should access ::NvEncGetInputFormatCount() API through this pointer."]
    pub nvEncGetInputFormatCount: PNVENCGETINPUTFORMATCOUNT,
    #[doc = "Client should access ::NvEncGetInputFormats() API through this pointer."]
    pub nvEncGetInputFormats: PNVENCGETINPUTFORMATS,
    #[doc = "Client should access ::NvEncGetEncodeCaps() API through this pointer."]
    pub nvEncGetEncodeCaps: PNVENCGETENCODECAPS,
    #[doc = "Client should access ::NvEncGetEncodePresetCount() API through this pointer."]
    pub nvEncGetEncodePresetCount: PNVENCGETENCODEPRESETCOUNT,
    #[doc = "Client should access ::NvEncGetEncodePresetGUIDs() API through this pointer."]
    pub nvEncGetEncodePresetGUIDs: PNVENCGETENCODEPRESETGUIDS,
    #[doc = "Client should access ::NvEncGetEncodePresetConfig() API through this pointer."]
    pub nvEncGetEncodePresetConfig: PNVENCGETENCODEPRESETCONFIG,
    #[doc = "Client should access ::NvEncInitializeEncoder() API through this pointer."]
    pub nvEncInitializeEncoder: PNVENCINITIALIZEENCODER,
    #[doc = "Client should access ::NvEncCreateInputBuffer() API through this pointer."]
    pub nvEncCreateInputBuffer: PNVENCCREATEINPUTBUFFER,
    #[doc = "Client should access ::NvEncDestroyInputBuffer() API through this pointer."]
    pub nvEncDestroyInputBuffer: PNVENCDESTROYINPUTBUFFER,
    #[doc = "Client should access ::NvEncCreateBitstreamBuffer() API through this pointer."]
    pub nvEncCreateBitstreamBuffer: PNVENCCREATEBITSTREAMBUFFER,
    #[doc = "Client should access ::NvEncDestroyBitstreamBuffer() API through this pointer."]
    pub nvEncDestroyBitstreamBuffer: PNVENCDESTROYBITSTREAMBUFFER,
    #[doc = "Client should access ::NvEncEncodePicture() API through this pointer."]
    pub nvEncEncodePicture: PNVENCENCODEPICTURE,
    #[doc = "Client should access ::NvEncLockBitstream() API through this pointer."]
    pub nvEncLockBitstream: PNVENCLOCKBITSTREAM,
    #[doc = "Client should access ::NvEncUnlockBitstream() API through this pointer."]
    pub nvEncUnlockBitstream: PNVENCUNLOCKBITSTREAM,
    #[doc = "Client should access ::NvEncLockInputBuffer() API through this pointer."]
    pub nvEncLockInputBuffer: PNVENCLOCKINPUTBUFFER,
    #[doc = "Client should access ::NvEncUnlockInputBuffer() API through this pointer."]
    pub nvEncUnlockInputBuffer: PNVENCUNLOCKINPUTBUFFER,
    #[doc = "Client should access ::NvEncGetEncodeStats() API through this pointer."]
    pub nvEncGetEncodeStats: PNVENCGETENCODESTATS,
    #[doc = "Client should access ::NvEncGetSequenceParams() API through this pointer."]
    pub nvEncGetSequenceParams: PNVENCGETSEQUENCEPARAMS,
    #[doc = "Client should access ::NvEncRegisterAsyncEvent() API through this pointer."]
    pub nvEncRegisterAsyncEvent: PNVENCREGISTERASYNCEVENT,
    #[doc = "Client should access ::NvEncUnregisterAsyncEvent() API through this pointer."]
    pub nvEncUnregisterAsyncEvent: PNVENCUNREGISTERASYNCEVENT,
    #[doc = "Client should access ::NvEncMapInputResource() API through this pointer."]
    pub nvEncMapInputResource: PNVENCMAPINPUTRESOURCE,
    #[doc = "Client should access ::NvEncUnmapInputResource() API through this pointer."]
    pub nvEncUnmapInputResource: PNVENCUNMAPINPUTRESOURCE,
    #[doc = "Client should access ::NvEncDestroyEncoder() API through this pointer."]
    pub nvEncDestroyEncoder: PNVENCDESTROYENCODER,
    #[doc = "Client should access ::NvEncInvalidateRefFrames() API through this pointer."]
    pub nvEncInvalidateRefFrames: PNVENCINVALIDATEREFFRAMES,
    #[doc = "Client should access ::NvEncOpenEncodeSession() API through this pointer."]
    pub nvEncOpenEncodeSessionEx: PNVENCOPENENCODESESSIONEX,
    #[doc = "Client should access ::NvEncRegisterResource() API through this pointer."]
    pub nvEncRegisterResource: PNVENCREGISTERRESOURCE,
    #[doc = "Client should access ::NvEncUnregisterResource() API through this pointer."]
    pub nvEncUnregisterResource: PNVENCUNREGISTERRESOURCE,
    #[doc = "Client should access ::NvEncReconfigureEncoder() API through this pointer."]
    pub nvEncReconfigureEncoder: PNVENCRECONFIGUREENCODER,
    pub reserved1: *mut ::core::ffi::c_void,
    #[doc = "Client should access ::NvEncCreateMVBuffer API through this pointer."]
    pub nvEncCreateMVBuffer: PNVENCCREATEMVBUFFER,
    #[doc = "Client should access ::NvEncDestroyMVBuffer API through this pointer."]
    pub nvEncDestroyMVBuffer: PNVENCDESTROYMVBUFFER,
    #[doc = "Client should access ::NvEncRunMotionEstimationOnly API through this pointer."]
    pub nvEncRunMotionEstimationOnly: PNVENCRUNMOTIONESTIMATIONONLY,
    #[doc = "Client should access ::nvEncGetLastErrorString API through this pointer."]
    pub nvEncGetLastErrorString: PNVENCGETLASTERROR,
    #[doc = "Client should access ::nvEncSetIOCudaStreams API through this pointer."]
    pub nvEncSetIOCudaStreams: PNVENCSETIOCUDASTREAMS,
    #[doc = " Reserved and must be set to NULL"]
    pub reserved2: [*mut ::core::ffi::c_void; 279usize],
}
pub type NV_ENCODE_API_FUNCTION_LIST = _NV_ENCODE_API_FUNCTION_LIST;
