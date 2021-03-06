// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Common Language Runtime Debugging Symbol Reader/Writer/Binder Interfaces
DEFINE_GUID!(CorSym_LanguageType_C, 0x63a08714, 0xfc37, 0x11d2,
    0x90, 0x4c, 0x0, 0xc0, 0x4f, 0xa3, 0x02, 0xa1);
DEFINE_GUID!(CorSym_LanguageType_CPlusPlus, 0x3a12d0b7, 0xc26c, 0x11d0,
    0xb4, 0x42, 0x0, 0xa0, 0x24, 0x4a, 0x1d, 0xd2);
DEFINE_GUID!(CorSym_LanguageType_CSharp, 0x3f5162f8, 0x07c6, 0x11d3,
    0x90, 0x53, 0x0, 0xc0, 0x4f, 0xa3, 0x02, 0xa1);
DEFINE_GUID!(CorSym_LanguageType_Basic, 0x3a12d0b8, 0xc26c, 0x11d0,
    0xb4, 0x42, 0x0, 0xa0, 0x24, 0x4a, 0x1d, 0xd2);
DEFINE_GUID!(CorSym_LanguageType_Java, 0x3a12d0b4, 0xc26c, 0x11d0,
    0xb4, 0x42, 0x0, 0xa0, 0x24, 0x4a, 0x1d, 0xd2);
DEFINE_GUID!(CorSym_LanguageType_Cobol, 0xaf046cd1, 0xd0e1, 0x11d2,
    0x97, 0x7c, 0x0, 0xa0, 0xc9, 0xb4, 0xd5, 0xc);
DEFINE_GUID!(CorSym_LanguageType_Pascal, 0xaf046cd2, 0xd0e1, 0x11d2,
    0x97, 0x7c, 0x0, 0xa0, 0xc9, 0xb4, 0xd5, 0xc);
DEFINE_GUID!(CorSym_LanguageType_ILAssembly, 0xaf046cd3, 0xd0e1, 0x11d2,
    0x97, 0x7c, 0x0, 0xa0, 0xc9, 0xb4, 0xd5, 0xc);
DEFINE_GUID!(CorSym_LanguageType_JScript, 0x3a12d0b6, 0xc26c, 0x11d0,
    0xb4, 0x42, 0x00, 0xa0, 0x24, 0x4a, 0x1d, 0xd2);
DEFINE_GUID!(CorSym_LanguageType_SMC, 0xd9b9f7b, 0x6611, 0x11d3,
    0xbd, 0x2a, 0x0, 0x0, 0xf8, 0x8, 0x49, 0xbd);
DEFINE_GUID!(CorSym_LanguageType_MCPlusPlus, 0x4b35fde8, 0x07c6, 0x11d3,
    0x90, 0x53, 0x0, 0xc0, 0x4f, 0xa3, 0x02, 0xa1);
DEFINE_GUID!(CorSym_LanguageVendor_Microsoft, 0x994b45c4, 0xe6e9, 0x11d2,
    0x90, 0x3f, 0x00, 0xc0, 0x4f, 0xa3, 0x02, 0xa1);
DEFINE_GUID!(CorSym_DocumentType_Text, 0x5a869d0b, 0x6611, 0x11d3,
    0xbd, 0x2a, 0x0, 0x0, 0xf8, 0x8, 0x49, 0xbd);
DEFINE_GUID!(CorSym_DocumentType_MC, 0xeb40cb65, 0x3c1f, 0x4352,
    0x9d, 0x7b, 0xba, 0xf, 0xc4, 0x7a, 0x9d, 0x77);
DEFINE_GUID!(CorSym_SourceHash_MD5,  0x406ea660, 0x64cf, 0x4c82,
    0xb6, 0xf0, 0x42, 0xd4, 0x81, 0x72, 0xa7, 0x99);
DEFINE_GUID!(CorSym_SourceHash_SHA1, 0xff1816ec, 0xaa5e, 0x4d10,
    0x87, 0xf7, 0x6f, 0x49, 0x63, 0x83, 0x34, 0x60);
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum CorSymAddrKind {
    ADDR_IL_OFFSET = 1,
    ADDR_NATIVE_RVA = 2,
    ADDR_NATIVE_REGISTER = 3,
    ADDR_NATIVE_REGREL = 4,
    ADDR_NATIVE_OFFSET = 5,
    ADDR_NATIVE_REGREG = 6,
    ADDR_NATIVE_REGSTK = 7,
    ADDR_NATIVE_STKREG = 8,
    ADDR_BITFIELD = 9,
    ADDR_NATIVE_ISECTOFFSET = 10,
}
pub use self::CorSymAddrKind::*;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum CorSymVarFlag {
    VAR_IS_COMP_GEN = 1,
    #[doc(hidden)] __,
}
pub use self::CorSymVarFlag::*;
RIDL!(
interface ISymUnmanagedBinder(ISymUnmanagedBinderVtbl): IUnknown(IUnknownVtbl) {
    fn GetReaderForFile(
        &mut self, importer: *mut ::IUnknown, fileName: *const ::WCHAR, searchPath: *const ::WCHAR,
        pRetVal: *mut *mut ISymUnmanagedReader
    ) -> ::HRESULT,
    fn GetReaderFromStream(
        &mut self, importer: *mut ::IUnknown, pstream: *mut ::IStream,
        pRetVal: *mut *mut ISymUnmanagedReader
    ) -> ::HRESULT
}
);
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum CorSymSearchPolicyAttributes {
    AllowRegistryAccess = 0x1,
    AllowSymbolServerAccess = 0x2,
    AllowOriginalPathAccess = 0x4,
    AllowReferencePathAccess = 0x8,
}
pub use self::CorSymSearchPolicyAttributes::*;
RIDL!(
interface ISymUnmanagedBinder2(ISymUnmanagedBinder2Vtbl):
    ISymUnmanagedBinder(ISymUnmanagedBinderVtbl) {
    fn GetReaderForFile2(
        &mut self, importer: *mut ::IUnknown, fileName: *const ::WCHAR, searchPath: *const ::WCHAR,
        searchPolicy: ::ULONG32, pRetVal: *mut *mut ISymUnmanagedReader
    ) -> ::HRESULT
}
);
#[derive(Clone, Copy)]
pub struct ISymUnmanagedReader;
