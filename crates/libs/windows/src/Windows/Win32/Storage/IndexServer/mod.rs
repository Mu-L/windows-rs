#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn BindIFilterFromStorage<P0, P1>(pstg: P0, punkouter: P1, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::System::Com::StructuredStorage::IStorage>,
    P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("query.dll" "system" fn BindIFilterFromStorage(pstg : * mut::core::ffi::c_void, punkouter : * mut::core::ffi::c_void, ppiunk : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    BindIFilterFromStorage(pstg.into_param().abi(), punkouter.into_param().abi(), ppiunk).ok()
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn BindIFilterFromStream<P0, P1>(pstm: P0, punkouter: P1, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("query.dll" "system" fn BindIFilterFromStream(pstm : * mut::core::ffi::c_void, punkouter : * mut::core::ffi::c_void, ppiunk : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    BindIFilterFromStream(pstm.into_param().abi(), punkouter.into_param().abi(), ppiunk).ok()
}
#[inline]
pub unsafe fn LoadIFilter<P0, P1>(pwcspath: P0, punkouter: P1, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("query.dll" "system" fn LoadIFilter(pwcspath : ::windows_core::PCWSTR, punkouter : * mut::core::ffi::c_void, ppiunk : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    LoadIFilter(pwcspath.into_param().abi(), punkouter.into_param().abi(), ppiunk).ok()
}
#[inline]
pub unsafe fn LoadIFilterEx<P0>(pwcspath: P0, dwflags: u32, riid: *const ::windows_core::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("query.dll" "system" fn LoadIFilterEx(pwcspath : ::windows_core::PCWSTR, dwflags : u32, riid : *const ::windows_core::GUID, ppiunk : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    LoadIFilterEx(pwcspath.into_param().abi(), dwflags, riid, ppiunk).ok()
}
::windows_core::imp::com_interface!(IFilter, IFilter_Vtbl, 0x89bcb740_6119_101a_bcb7_00dd010655af);
::windows_core::imp::interface_hierarchy!(IFilter, ::windows_core::IUnknown);
impl IFilter {
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Init(&self, grfflags: u32, aattributes: &[FULLPROPSPEC], pflags: *mut u32) -> i32 {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), grfflags, aattributes.len().try_into().unwrap(), ::core::mem::transmute(aattributes.as_ptr()), pflags)
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> i32 {
        (::windows_core::Interface::vtable(self).GetChunk)(::windows_core::Interface::as_raw(self), pstat)
    }
    pub unsafe fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: ::windows_core::PWSTR) -> i32 {
        (::windows_core::Interface::vtable(self).GetText)(::windows_core::Interface::as_raw(self), pcwcbuffer, ::core::mem::transmute(awcbuffer))
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), pppropvalue)
    }
    pub unsafe fn BindRegion(&self, origpos: FILTERREGION, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32 {
        (::windows_core::Interface::vtable(self).BindRegion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(origpos), riid, ppunk)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Init: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetChunk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstat: *mut STAT_CHUNK) -> i32,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetChunk: usize,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcwcbuffer: *mut u32, awcbuffer: ::windows_core::PWSTR) -> i32,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetValue: usize,
    pub BindRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origpos: FILTERREGION, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32,
}
::windows_core::imp::com_interface!(IPhraseSink, IPhraseSink_Vtbl, 0xcc906ff0_c058_101a_b554_08002b33b0e6);
::windows_core::imp::interface_hierarchy!(IPhraseSink, ::windows_core::IUnknown);
impl IPhraseSink {
    pub unsafe fn PutSmallPhrase<P0, P1>(&self, pwcnoun: P0, cwcnoun: u32, pwcmodifier: P1, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).PutSmallPhrase)(::windows_core::Interface::as_raw(self), pwcnoun.into_param().abi(), cwcnoun, pwcmodifier.into_param().abi(), cwcmodifier, ulattachmenttype).ok()
    }
    pub unsafe fn PutPhrase<P0>(&self, pwcphrase: P0, cwcphrase: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).PutPhrase)(::windows_core::Interface::as_raw(self), pwcphrase.into_param().abi(), cwcphrase).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhraseSink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub PutSmallPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcnoun: ::windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: ::windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows_core::HRESULT,
    pub PutPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcphrase: ::windows_core::PCWSTR, cwcphrase: u32) -> ::windows_core::HRESULT,
}
pub const CHUNK_EOC: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(4i32);
pub const CHUNK_EOP: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(3i32);
pub const CHUNK_EOS: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(2i32);
pub const CHUNK_EOW: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(1i32);
pub const CHUNK_FILTER_OWNED_VALUE: CHUNKSTATE = CHUNKSTATE(4i32);
pub const CHUNK_NO_BREAK: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(0i32);
pub const CHUNK_TEXT: CHUNKSTATE = CHUNKSTATE(1i32);
pub const CHUNK_VALUE: CHUNKSTATE = CHUNKSTATE(2i32);
pub const CIADMIN: ::windows_core::PCWSTR = ::windows_core::w!("::_nodocstore_::");
pub const CICAT_ALL_OPENED: u32 = 32u32;
pub const CICAT_GET_STATE: u32 = 16u32;
pub const CICAT_NO_QUERY: u32 = 8u32;
pub const CICAT_READONLY: u32 = 2u32;
pub const CICAT_STOPPED: u32 = 1u32;
pub const CICAT_WRITABLE: u32 = 4u32;
pub const CINULLCATALOG: ::windows_core::PCWSTR = ::windows_core::w!("::_noindex_::");
pub const CI_PROVIDER_ALL: u32 = 4294967295u32;
pub const CI_PROVIDER_INDEXING_SERVICE: u32 = 2u32;
pub const CI_PROVIDER_MSSEARCH: u32 = 1u32;
pub const CI_STATE_ANNEALING_MERGE: u32 = 8u32;
pub const CI_STATE_BATTERY_POLICY: u32 = 262144u32;
pub const CI_STATE_BATTERY_POWER: u32 = 2048u32;
pub const CI_STATE_CONTENT_SCAN_REQUIRED: u32 = 4u32;
pub const CI_STATE_DELETION_MERGE: u32 = 32768u32;
pub const CI_STATE_HIGH_CPU: u32 = 131072u32;
pub const CI_STATE_HIGH_IO: u32 = 256u32;
pub const CI_STATE_INDEX_MIGRATION_MERGE: u32 = 64u32;
pub const CI_STATE_LOW_DISK: u32 = 65536u32;
pub const CI_STATE_LOW_MEMORY: u32 = 128u32;
pub const CI_STATE_MASTER_MERGE: u32 = 2u32;
pub const CI_STATE_MASTER_MERGE_PAUSED: u32 = 512u32;
pub const CI_STATE_READING_USNS: u32 = 16384u32;
pub const CI_STATE_READ_ONLY: u32 = 1024u32;
pub const CI_STATE_RECOVERING: u32 = 32u32;
pub const CI_STATE_SCANNING: u32 = 16u32;
pub const CI_STATE_SHADOW_MERGE: u32 = 1u32;
pub const CI_STATE_STARTING: u32 = 8192u32;
pub const CI_STATE_USER_ACTIVE: u32 = 4096u32;
pub const CI_VERSION_WDS30: u32 = 258u32;
pub const CI_VERSION_WDS40: u32 = 265u32;
pub const CI_VERSION_WIN70: u32 = 1792u32;
pub const CLSID_INDEX_SERVER_DSO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9ae8980_7e52_11d0_8964_00c04fd611d7);
pub const DBKIND_GUID: DBKINDENUM = DBKINDENUM(6i32);
pub const DBKIND_GUID_NAME: DBKINDENUM = DBKINDENUM(0i32);
pub const DBKIND_GUID_PROPID: DBKINDENUM = DBKINDENUM(1i32);
pub const DBKIND_NAME: DBKINDENUM = DBKINDENUM(2i32);
pub const DBKIND_PGUID_NAME: DBKINDENUM = DBKINDENUM(3i32);
pub const DBKIND_PGUID_PROPID: DBKINDENUM = DBKINDENUM(4i32);
pub const DBKIND_PROPID: DBKINDENUM = DBKINDENUM(5i32);
pub const DBPROPSET_CIFRMWRKCORE_EXT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafafaca5_b5d1_11d0_8c62_00c04fc2db8d);
pub const DBPROPSET_FSCIFRMWRK_EXT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9bd1526_6a80_11d0_8c9d_0020af1d740e);
pub const DBPROPSET_MSIDXS_ROWSETEXT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa6ee6b0_e828_11d0_b23e_00aa0047fc01);
pub const DBPROPSET_QUERYEXT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7ac77ed_f8d7_11ce_a798_0020f8008025);
pub const DBPROPSET_SESS_QUERYEXT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63623309_2d8b_4d17_b152_6e2956c26a70);
pub const DBPROP_APPLICATION_NAME: u32 = 11u32;
pub const DBPROP_CATALOGLISTID: u32 = 9u32;
pub const DBPROP_CI_CATALOG_NAME: u32 = 2u32;
pub const DBPROP_CI_DEPTHS: u32 = 4u32;
pub const DBPROP_CI_EXCLUDE_SCOPES: u32 = 5u32;
pub const DBPROP_CI_INCLUDE_SCOPES: u32 = 3u32;
pub const DBPROP_CI_PROVIDER: u32 = 8u32;
pub const DBPROP_CI_QUERY_TYPE: u32 = 7u32;
pub const DBPROP_CI_SCOPE_FLAGS: u32 = 4u32;
pub const DBPROP_CI_SECURITY_ID: u32 = 6u32;
pub const DBPROP_CLIENT_CLSID: u32 = 3u32;
pub const DBPROP_DEFAULT_EQUALS_BEHAVIOR: u32 = 2u32;
pub const DBPROP_DEFERCATALOGVERIFICATION: u32 = 8u32;
pub const DBPROP_DEFERNONINDEXEDTRIMMING: u32 = 3u32;
pub const DBPROP_DONOTCOMPUTEEXPENSIVEPROPS: u32 = 15u32;
pub const DBPROP_ENABLEROWSETEVENTS: u32 = 16u32;
pub const DBPROP_FIRSTROWS: u32 = 7u32;
pub const DBPROP_FREETEXTANYTERM: u32 = 12u32;
pub const DBPROP_FREETEXTUSESTEMMING: u32 = 13u32;
pub const DBPROP_GENERATEPARSETREE: u32 = 10u32;
pub const DBPROP_GENERICOPTIONS_STRING: u32 = 6u32;
pub const DBPROP_IGNORENOISEONLYCLAUSES: u32 = 5u32;
pub const DBPROP_IGNORESBRI: u32 = 14u32;
pub const DBPROP_MACHINE: u32 = 2u32;
pub const DBPROP_USECONTENTINDEX: u32 = 2u32;
pub const DBPROP_USEEXTENDEDDBTYPES: u32 = 4u32;
pub const DBSETFUNC_ALL: u32 = 1u32;
pub const DBSETFUNC_DISTINCT: u32 = 2u32;
pub const DBSETFUNC_NONE: u32 = 0u32;
pub const FILTER_E_ACCESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215613i32);
pub const FILTER_E_EMBEDDING_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215609i32);
pub const FILTER_E_END_OF_CHUNKS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215616i32);
pub const FILTER_E_LINK_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215608i32);
pub const FILTER_E_NO_MORE_TEXT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215615i32);
pub const FILTER_E_NO_MORE_VALUES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215614i32);
pub const FILTER_E_NO_TEXT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215611i32);
pub const FILTER_E_NO_VALUES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215610i32);
pub const FILTER_E_PASSWORD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215605i32);
pub const FILTER_E_UNKNOWNFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147215604i32);
pub const FILTER_S_LAST_TEXT: ::windows_core::HRESULT = ::windows_core::HRESULT(268041i32);
pub const FILTER_S_LAST_VALUES: ::windows_core::HRESULT = ::windows_core::HRESULT(268042i32);
pub const FILTER_W_MONIKER_CLIPPED: ::windows_core::HRESULT = ::windows_core::HRESULT(268036i32);
pub const GENERATE_METHOD_EXACT: u32 = 0u32;
pub const GENERATE_METHOD_INFLECT: u32 = 2u32;
pub const GENERATE_METHOD_PREFIX: u32 = 1u32;
pub const IFILTER_FLAGS_OLE_PROPERTIES: IFILTER_FLAGS = IFILTER_FLAGS(1i32);
pub const IFILTER_INIT_APPLY_CRAWL_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(256i32);
pub const IFILTER_INIT_APPLY_INDEX_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(16i32);
pub const IFILTER_INIT_APPLY_OTHER_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(32i32);
pub const IFILTER_INIT_CANON_HYPHENS: IFILTER_INIT = IFILTER_INIT(4i32);
pub const IFILTER_INIT_CANON_PARAGRAPHS: IFILTER_INIT = IFILTER_INIT(1i32);
pub const IFILTER_INIT_CANON_SPACES: IFILTER_INIT = IFILTER_INIT(8i32);
pub const IFILTER_INIT_DISABLE_EMBEDDED: IFILTER_INIT = IFILTER_INIT(2048i32);
pub const IFILTER_INIT_EMIT_FORMATTING: IFILTER_INIT = IFILTER_INIT(4096i32);
pub const IFILTER_INIT_FILTER_AGGRESSIVE_BREAK: IFILTER_INIT = IFILTER_INIT(1024i32);
pub const IFILTER_INIT_FILTER_OWNED_VALUE_OK: IFILTER_INIT = IFILTER_INIT(512i32);
pub const IFILTER_INIT_HARD_LINE_BREAKS: IFILTER_INIT = IFILTER_INIT(2i32);
pub const IFILTER_INIT_INDEXING_ONLY: IFILTER_INIT = IFILTER_INIT(64i32);
pub const IFILTER_INIT_SEARCH_LINKS: IFILTER_INIT = IFILTER_INIT(128i32);
pub const LIFF_FORCE_TEXT_FILTER_FALLBACK: u32 = 3u32;
pub const LIFF_IMPLEMENT_TEXT_FILTER_FALLBACK_POLICY: u32 = 2u32;
pub const LIFF_LOAD_DEFINED_FILTER: u32 = 1u32;
pub const MSIDXSPROP_COMMAND_LOCALE_STRING: u32 = 3u32;
pub const MSIDXSPROP_MAX_RANK: u32 = 6u32;
pub const MSIDXSPROP_PARSE_TREE: u32 = 5u32;
pub const MSIDXSPROP_QUERY_RESTRICTION: u32 = 4u32;
pub const MSIDXSPROP_RESULTS_FOUND: u32 = 7u32;
pub const MSIDXSPROP_ROWSETQUERYSTATUS: u32 = 2u32;
pub const MSIDXSPROP_SAME_SORTORDER_USED: u32 = 14u32;
pub const MSIDXSPROP_SERVER_NLSVERSION: u32 = 12u32;
pub const MSIDXSPROP_SERVER_NLSVER_DEFINED: u32 = 13u32;
pub const MSIDXSPROP_SERVER_VERSION: u32 = 9u32;
pub const MSIDXSPROP_SERVER_WINVER_MAJOR: u32 = 10u32;
pub const MSIDXSPROP_SERVER_WINVER_MINOR: u32 = 11u32;
pub const MSIDXSPROP_WHEREID: u32 = 8u32;
pub const NOT_AN_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(524288i32);
pub const PID_FILENAME: u32 = 100u32;
pub const PROPID_QUERY_ALL: u32 = 6u32;
pub const PROPID_QUERY_HITCOUNT: u32 = 4u32;
pub const PROPID_QUERY_LASTSEENTIME: u32 = 10u32;
pub const PROPID_QUERY_RANK: u32 = 3u32;
pub const PROPID_QUERY_RANKVECTOR: u32 = 2u32;
pub const PROPID_QUERY_UNFILTERED: u32 = 7u32;
pub const PROPID_QUERY_VIRTUALPATH: u32 = 9u32;
pub const PROPID_QUERY_WORKID: u32 = 5u32;
pub const PROPID_STG_CONTENTS: u32 = 19u32;
pub const PROXIMITY_UNIT_CHAPTER: u32 = 3u32;
pub const PROXIMITY_UNIT_PARAGRAPH: u32 = 2u32;
pub const PROXIMITY_UNIT_SENTENCE: u32 = 1u32;
pub const PROXIMITY_UNIT_WORD: u32 = 0u32;
pub const PSGUID_FILENAME: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41cf5ae0_f75a_4806_bd87_59c7d9248eb9);
pub const QUERY_DEEP: u32 = 1u32;
pub const QUERY_PHYSICAL_PATH: u32 = 0u32;
pub const QUERY_SHALLOW: u32 = 0u32;
pub const QUERY_VIRTUAL_PATH: u32 = 2u32;
pub const SCOPE_FLAG_DEEP: u32 = 2u32;
pub const SCOPE_FLAG_INCLUDE: u32 = 1u32;
pub const SCOPE_FLAG_MASK: u32 = 255u32;
pub const SCOPE_TYPE_MASK: u32 = 4294967040u32;
pub const SCOPE_TYPE_VPATH: u32 = 512u32;
pub const SCOPE_TYPE_WINPATH: u32 = 256u32;
pub const STAT_BUSY: u32 = 0u32;
pub const STAT_COALESCE_COMP_ALL_NOISE: u32 = 8192u32;
pub const STAT_CONTENT_OUT_OF_DATE: u32 = 32u32;
pub const STAT_CONTENT_QUERY_INCOMPLETE: u32 = 128u32;
pub const STAT_DONE: u32 = 2u32;
pub const STAT_ERROR: u32 = 1u32;
pub const STAT_MISSING_PROP_IN_RELDOC: u32 = 2048u32;
pub const STAT_MISSING_RELDOC: u32 = 1024u32;
pub const STAT_NOISE_WORDS: u32 = 16u32;
pub const STAT_PARTIAL_SCOPE: u32 = 8u32;
pub const STAT_REFRESH: u32 = 3u32;
pub const STAT_REFRESH_INCOMPLETE: u32 = 64u32;
pub const STAT_RELDOC_ACCESS_DENIED: u32 = 4096u32;
pub const STAT_SHARING_VIOLATION: u32 = 512u32;
pub const STAT_TIME_LIMIT_EXCEEDED: u32 = 256u32;
pub const VECTOR_RANK_DICE: u32 = 3u32;
pub const VECTOR_RANK_INNER: u32 = 2u32;
pub const VECTOR_RANK_JACCARD: u32 = 4u32;
pub const VECTOR_RANK_MAX: u32 = 1u32;
pub const VECTOR_RANK_MIN: u32 = 0u32;
pub const WORDREP_BREAK_EOC: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(3i32);
pub const WORDREP_BREAK_EOP: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(2i32);
pub const WORDREP_BREAK_EOS: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(1i32);
pub const WORDREP_BREAK_EOW: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(0i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct CHUNKSTATE(pub i32);
impl ::windows_core::TypeKind for CHUNKSTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CHUNKSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHUNKSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct CHUNK_BREAKTYPE(pub i32);
impl ::windows_core::TypeKind for CHUNK_BREAKTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CHUNK_BREAKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHUNK_BREAKTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DBKINDENUM(pub i32);
impl ::windows_core::TypeKind for DBKINDENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DBKINDENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBKINDENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct IFILTER_FLAGS(pub i32);
impl ::windows_core::TypeKind for IFILTER_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IFILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFILTER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct IFILTER_INIT(pub i32);
impl ::windows_core::TypeKind for IFILTER_INIT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IFILTER_INIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFILTER_INIT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WORDREP_BREAK_TYPE(pub i32);
impl ::windows_core::TypeKind for WORDREP_BREAK_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WORDREP_BREAK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORDREP_BREAK_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct CI_STATE {
    pub cbStruct: u32,
    pub cWordList: u32,
    pub cPersistentIndex: u32,
    pub cQueries: u32,
    pub cDocuments: u32,
    pub cFreshTest: u32,
    pub dwMergeProgress: u32,
    pub eState: u32,
    pub cFilteredDocuments: u32,
    pub cTotalDocuments: u32,
    pub cPendingScans: u32,
    pub dwIndexSize: u32,
    pub cUniqueKeys: u32,
    pub cSecQDocuments: u32,
    pub dwPropCacheSize: u32,
}
impl ::core::marker::Copy for CI_STATE {}
impl ::core::clone::Clone for CI_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CI_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CI_STATE")
            .field("cbStruct", &self.cbStruct)
            .field("cWordList", &self.cWordList)
            .field("cPersistentIndex", &self.cPersistentIndex)
            .field("cQueries", &self.cQueries)
            .field("cDocuments", &self.cDocuments)
            .field("cFreshTest", &self.cFreshTest)
            .field("dwMergeProgress", &self.dwMergeProgress)
            .field("eState", &self.eState)
            .field("cFilteredDocuments", &self.cFilteredDocuments)
            .field("cTotalDocuments", &self.cTotalDocuments)
            .field("cPendingScans", &self.cPendingScans)
            .field("dwIndexSize", &self.dwIndexSize)
            .field("cUniqueKeys", &self.cUniqueKeys)
            .field("cSecQDocuments", &self.cSecQDocuments)
            .field("dwPropCacheSize", &self.dwPropCacheSize)
            .finish()
    }
}
impl ::windows_core::TypeKind for CI_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CI_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cWordList == other.cWordList && self.cPersistentIndex == other.cPersistentIndex && self.cQueries == other.cQueries && self.cDocuments == other.cDocuments && self.cFreshTest == other.cFreshTest && self.dwMergeProgress == other.dwMergeProgress && self.eState == other.eState && self.cFilteredDocuments == other.cFilteredDocuments && self.cTotalDocuments == other.cTotalDocuments && self.cPendingScans == other.cPendingScans && self.dwIndexSize == other.dwIndexSize && self.cUniqueKeys == other.cUniqueKeys && self.cSecQDocuments == other.cSecQDocuments && self.dwPropCacheSize == other.dwPropCacheSize
    }
}
impl ::core::cmp::Eq for CI_STATE {}
impl ::core::default::Default for CI_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: u32,
    pub uName: DBID_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBID {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for DBID {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub union DBID_0 {
    pub guid: ::windows_core::GUID,
    pub pguid: *mut ::windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBID_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for DBID_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub union DBID_1 {
    pub pwszName: ::windows_core::PWSTR,
    pub ulPropid: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBID_1 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBID_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for DBID_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBID_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: u32,
    pub uName: DBID_1,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBID {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for DBID {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
pub union DBID_0 {
    pub guid: ::windows_core::GUID,
    pub pguid: *mut ::windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBID_0 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for DBID_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
pub union DBID_1 {
    pub pwszName: ::windows_core::PWSTR,
    pub ulPropid: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBID_1 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBID_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for DBID_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBID_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FILTERREGION {
    pub idChunk: u32,
    pub cwcStart: u32,
    pub cwcExtent: u32,
}
impl ::core::marker::Copy for FILTERREGION {}
impl ::core::clone::Clone for FILTERREGION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTERREGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTERREGION").field("idChunk", &self.idChunk).field("cwcStart", &self.cwcStart).field("cwcExtent", &self.cwcExtent).finish()
    }
}
impl ::windows_core::TypeKind for FILTERREGION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FILTERREGION {
    fn eq(&self, other: &Self) -> bool {
        self.idChunk == other.idChunk && self.cwcStart == other.cwcStart && self.cwcExtent == other.cwcExtent
    }
}
impl ::core::cmp::Eq for FILTERREGION {}
impl ::core::default::Default for FILTERREGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub struct FULLPROPSPEC {
    pub guidPropSet: ::windows_core::GUID,
    pub psProperty: super::super::System::Com::StructuredStorage::PROPSPEC,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::marker::Copy for FULLPROPSPEC {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for FULLPROPSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::TypeKind for FULLPROPSPEC {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::default::Default for FULLPROPSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub struct STAT_CHUNK {
    pub idChunk: u32,
    pub breakType: CHUNK_BREAKTYPE,
    pub flags: CHUNKSTATE,
    pub locale: u32,
    pub attribute: FULLPROPSPEC,
    pub idChunkSource: u32,
    pub cwcStartSource: u32,
    pub cwcLenSource: u32,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::marker::Copy for STAT_CHUNK {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for STAT_CHUNK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::TypeKind for STAT_CHUNK {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::default::Default for STAT_CHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
