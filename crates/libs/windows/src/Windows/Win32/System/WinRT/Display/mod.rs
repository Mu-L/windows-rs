::windows_core::imp::com_interface!(IDisplayDeviceInterop, IDisplayDeviceInterop_Vtbl, 0x64338358_366a_471b_bd56_dd8ef48e439b);
::windows_core::imp::interface_hierarchy!(IDisplayDeviceInterop, ::windows_core::IUnknown);
impl IDisplayDeviceInterop {
    #[doc = "Required features: `\"Win32_Security\"`"]
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn CreateSharedHandle<P0>(&self, pobject: P0, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::HANDLE>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSharedHandle)(::windows_core::Interface::as_raw(self), pobject.into_param().abi(), psecurityattributes, access, ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenSharedHandle<P0>(&self, nthandle: P0, riid: ::windows_core::GUID) -> ::windows_core::Result<*mut ::core::ffi::c_void>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OpenSharedHandle)(::windows_core::Interface::as_raw(self), nthandle.into_param().abi(), ::core::mem::transmute(riid), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDeviceInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security")]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    CreateSharedHandle: usize,
    pub OpenSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDisplayPathInterop, IDisplayPathInterop_Vtbl, 0xa6ba4205_e59e_4e71_b25b_4e436d21ee3d);
::windows_core::imp::interface_hierarchy!(IDisplayPathInterop, ::windows_core::IUnknown);
impl IDisplayPathInterop {
    pub unsafe fn CreateSourcePresentationHandle(&self) -> ::windows_core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSourcePresentationHandle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPathInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateSourcePresentationHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    pub GetSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
