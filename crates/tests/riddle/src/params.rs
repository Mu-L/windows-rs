// Bindings generated by `windows-bindgen` 0.52.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
::windows_core::imp::com_interface!(IParams, IParams_Vtbl, 0);
::windows_core::imp::interface_hierarchy!(
    IParams,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl IParams {
    pub fn Nothing(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Nothing)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Bool(&self, a: &mut bool, b: &mut bool) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bool)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn I8(&self, a: &mut i8, b: &mut i8) -> ::windows_core::Result<i8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).I8)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn U8(&self, a: &mut u8, b: &mut u8) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).U8)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn I16(&self, a: &mut i16, b: &mut i16) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).I16)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn U16(&self, a: &mut u16, b: &mut u16) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).U16)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn I32(&self, a: &mut i32, b: &mut i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).I32)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn U32(&self, a: &mut u32, b: &mut u32) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).U32)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn I64(&self, a: &mut i64, b: &mut i64) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).I64)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn U64(&self, a: &mut u64, b: &mut u64) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).U64)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn F32(&self, a: &mut f32, b: &mut f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).F32)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn F64(&self, a: &mut f64, b: &mut f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).F64)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ISize(&self, a: &mut isize, b: &mut isize) -> ::windows_core::Result<isize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ISize)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn USize(&self, a: &mut usize, b: &mut usize) -> ::windows_core::Result<usize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).USize)(
                ::windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for IParams {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IParams_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Nothing:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Bool: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut bool,
        b: *mut bool,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub I8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut i8,
        b: *mut i8,
        result__: *mut i8,
    ) -> ::windows_core::HRESULT,
    pub U8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut u8,
        b: *mut u8,
        result__: *mut u8,
    ) -> ::windows_core::HRESULT,
    pub I16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut i16,
        b: *mut i16,
        result__: *mut i16,
    ) -> ::windows_core::HRESULT,
    pub U16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut u16,
        b: *mut u16,
        result__: *mut u16,
    ) -> ::windows_core::HRESULT,
    pub I32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut i32,
        b: *mut i32,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub U32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut u32,
        b: *mut u32,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub I64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut i64,
        b: *mut i64,
        result__: *mut i64,
    ) -> ::windows_core::HRESULT,
    pub U64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut u64,
        b: *mut u64,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
    pub F32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut f32,
        b: *mut f32,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub F64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut f64,
        b: *mut f64,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ISize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut isize,
        b: *mut isize,
        result__: *mut isize,
    ) -> ::windows_core::HRESULT,
    pub USize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut usize,
        b: *mut usize,
        result__: *mut usize,
    ) -> ::windows_core::HRESULT,
}
