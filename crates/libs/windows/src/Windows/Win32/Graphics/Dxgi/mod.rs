#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
pub mod Common;
#[inline]
pub unsafe fn CreateDXGIFactory<T>() -> ::windows_core::Result<T>
where
    T: ::windows_core::Interface,
{
    ::windows_targets::link!("dxgi.dll" "system" fn CreateDXGIFactory(riid : *const ::windows_core::GUID, ppfactory : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    CreateDXGIFactory(&T::IID, &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn CreateDXGIFactory1<T>() -> ::windows_core::Result<T>
where
    T: ::windows_core::Interface,
{
    ::windows_targets::link!("dxgi.dll" "system" fn CreateDXGIFactory1(riid : *const ::windows_core::GUID, ppfactory : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    CreateDXGIFactory1(&T::IID, &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn CreateDXGIFactory2<T>(flags: u32) -> ::windows_core::Result<T>
where
    T: ::windows_core::Interface,
{
    ::windows_targets::link!("dxgi.dll" "system" fn CreateDXGIFactory2(flags : u32, riid : *const ::windows_core::GUID, ppfactory : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    CreateDXGIFactory2(flags, &T::IID, &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn DXGIDeclareAdapterRemovalSupport() -> ::windows_core::Result<()> {
    ::windows_targets::link!("dxgi.dll" "system" fn DXGIDeclareAdapterRemovalSupport() -> ::windows_core::HRESULT);
    DXGIDeclareAdapterRemovalSupport().ok()
}
#[inline]
pub unsafe fn DXGIDisableVBlankVirtualization() -> ::windows_core::Result<()> {
    ::windows_targets::link!("dxgi.dll" "system" fn DXGIDisableVBlankVirtualization() -> ::windows_core::HRESULT);
    DXGIDisableVBlankVirtualization().ok()
}
#[inline]
pub unsafe fn DXGIGetDebugInterface1<T>(flags: u32) -> ::windows_core::Result<T>
where
    T: ::windows_core::Interface,
{
    ::windows_targets::link!("dxgi.dll" "system" fn DXGIGetDebugInterface1(flags : u32, riid : *const ::windows_core::GUID, pdebug : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    DXGIGetDebugInterface1(flags, &T::IID, &mut result__).from_abi(result__)
}
::windows_core::imp::com_interface!(IDXGIAdapter, IDXGIAdapter_Vtbl, 0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
::windows_core::imp::interface_hierarchy!(IDXGIAdapter, ::windows_core::IUnknown, IDXGIObject);
impl IDXGIAdapter {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumOutputs)(::windows_core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows_core::GUID) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CheckInterfaceSupport)(::windows_core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIAdapter {}
unsafe impl ::core::marker::Sync for IDXGIAdapter {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_core::HRESULT,
    pub CheckInterfaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacename: *const ::windows_core::GUID, pumdversion: *mut i64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIAdapter1, IDXGIAdapter1_Vtbl, 0x29038f61_3839_4626_91fd_086879011a05);
::windows_core::imp::interface_hierarchy!(IDXGIAdapter1, ::windows_core::IUnknown, IDXGIObject, IDXGIAdapter);
impl IDXGIAdapter1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumOutputs)(::windows_core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows_core::GUID) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CheckInterfaceSupport)(::windows_core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIAdapter1 {}
unsafe impl ::core::marker::Sync for IDXGIAdapter1 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter1_Vtbl {
    pub base__: IDXGIAdapter_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIAdapter2, IDXGIAdapter2_Vtbl, 0x0aa1ae0a_fa0e_4b84_8644_e05ff8e5acb5);
::windows_core::imp::interface_hierarchy!(IDXGIAdapter2, ::windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1);
impl IDXGIAdapter2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumOutputs)(::windows_core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows_core::GUID) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CheckInterfaceSupport)(::windows_core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc2)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIAdapter2 {}
unsafe impl ::core::marker::Sync for IDXGIAdapter2 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter2_Vtbl {
    pub base__: IDXGIAdapter1_Vtbl,
    pub GetDesc2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIAdapter3, IDXGIAdapter3_Vtbl, 0x645967a4_1392_4310_a798_8053ce3e93fd);
::windows_core::imp::interface_hierarchy!(IDXGIAdapter3, ::windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter2);
impl IDXGIAdapter3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EnumOutputs)(::windows_core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows_core::GUID) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CheckInterfaceSupport)(::windows_core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDesc2)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterHardwareContentProtectionTeardownStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).UnregisterHardwareContentProtectionTeardownStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryVideoMemoryInfo)(::windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, pvideomemoryinfo).ok()
    }
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVideoMemoryReservation)(::windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, reservation).ok()
    }
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterVideoMemoryBudgetChangeNotificationEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).UnregisterVideoMemoryBudgetChangeNotification)(::windows_core::Interface::as_raw(self), dwcookie)
    }
}
unsafe impl ::core::marker::Send for IDXGIAdapter3 {}
unsafe impl ::core::marker::Sync for IDXGIAdapter3 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter3_Vtbl {
    pub base__: IDXGIAdapter2_Vtbl,
    pub RegisterHardwareContentProtectionTeardownStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub UnregisterHardwareContentProtectionTeardownStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    pub QueryVideoMemoryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows_core::HRESULT,
    pub SetVideoMemoryReservation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows_core::HRESULT,
    pub RegisterVideoMemoryBudgetChangeNotificationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub UnregisterVideoMemoryBudgetChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
}
::windows_core::imp::com_interface!(IDXGIAdapter4, IDXGIAdapter4_Vtbl, 0x3c8d99d1_4fbf_4181_a82c_af66bf7bd24e);
::windows_core::imp::interface_hierarchy!(IDXGIAdapter4, ::windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter2, IDXGIAdapter3);
impl IDXGIAdapter4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.EnumOutputs)(::windows_core::Interface::as_raw(self), output, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows_core::GUID) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CheckInterfaceSupport)(::windows_core::Interface::as_raw(self), interfacename, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDesc2)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RegisterHardwareContentProtectionTeardownStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.UnregisterHardwareContentProtectionTeardownStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.QueryVideoMemoryInfo)(::windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, pvideomemoryinfo).ok()
    }
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetVideoMemoryReservation)(::windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, reservation).ok()
    }
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RegisterVideoMemoryBudgetChangeNotificationEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.UnregisterVideoMemoryBudgetChangeNotification)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc3)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIAdapter4 {}
unsafe impl ::core::marker::Sync for IDXGIAdapter4 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter4_Vtbl {
    pub base__: IDXGIAdapter3_Vtbl,
    pub GetDesc3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIDebug, IDXGIDebug_Vtbl, 0x119e7452_de9e_40fe_8806_88f90c12b441);
::windows_core::imp::interface_hierarchy!(IDXGIDebug, ::windows_core::IUnknown);
impl IDXGIDebug {
    pub unsafe fn ReportLiveObjects(&self, apiid: ::windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportLiveObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(apiid), flags).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIDebug {}
unsafe impl ::core::marker::Sync for IDXGIDebug {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ReportLiveObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apiid: ::windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIDebug1, IDXGIDebug1_Vtbl, 0xc5a05f0c_16f2_4adf_9f4d_a8c4d58ac550);
::windows_core::imp::interface_hierarchy!(IDXGIDebug1, ::windows_core::IUnknown, IDXGIDebug);
impl IDXGIDebug1 {
    pub unsafe fn ReportLiveObjects(&self, apiid: ::windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReportLiveObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(apiid), flags).ok()
    }
    pub unsafe fn EnableLeakTrackingForThread(&self) {
        (::windows_core::Interface::vtable(self).EnableLeakTrackingForThread)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn DisableLeakTrackingForThread(&self) {
        (::windows_core::Interface::vtable(self).DisableLeakTrackingForThread)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).IsLeakTrackingEnabledForThread)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IDXGIDebug1 {}
unsafe impl ::core::marker::Sync for IDXGIDebug1 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug1_Vtbl {
    pub base__: IDXGIDebug_Vtbl,
    pub EnableLeakTrackingForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub DisableLeakTrackingForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub IsLeakTrackingEnabledForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
}
::windows_core::imp::com_interface!(IDXGIDecodeSwapChain, IDXGIDecodeSwapChain_Vtbl, 0x2633066b_4514_4c7a_8fd8_12ea98059d18);
::windows_core::imp::interface_hierarchy!(IDXGIDecodeSwapChain, ::windows_core::IUnknown);
impl IDXGIDecodeSwapChain {
    pub unsafe fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).PresentBuffer)(::windows_core::Interface::as_raw(self), buffertopresent, syncinterval, flags)
    }
    pub unsafe fn SetSourceRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSourceRect)(::windows_core::Interface::as_raw(self), prect).ok()
    }
    pub unsafe fn SetTargetRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetRect)(::windows_core::Interface::as_raw(self), prect).ok()
    }
    pub unsafe fn SetDestSize(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDestSize)(::windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceRect(&self) -> ::windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceRect)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTargetRect(&self) -> ::windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetRect)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDestSize)(::windows_core::Interface::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetColorSpace)(::windows_core::Interface::as_raw(self), colorspace).ok()
    }
    pub unsafe fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
        (::windows_core::Interface::vtable(self).GetColorSpace)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IDXGIDecodeSwapChain {}
unsafe impl ::core::marker::Sync for IDXGIDecodeSwapChain {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDecodeSwapChain_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub PresentBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT,
    pub SetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    pub SetTargetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    pub SetDestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT,
    pub GetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    pub GetTargetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    pub GetDestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::HRESULT,
    pub SetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows_core::HRESULT,
    pub GetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
}
::windows_core::imp::com_interface!(IDXGIDevice, IDXGIDevice_Vtbl, 0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
::windows_core::imp::interface_hierarchy!(IDXGIDevice, ::windows_core::IUnknown, IDXGIObject);
impl IDXGIDevice {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAdapter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateSurface)(::windows_core::Interface::as_raw(self), pdesc, ppsurface.len().try_into().unwrap(), usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryResourceResidency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGPUThreadPriority)(::windows_core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGPUThreadPriority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIDevice {}
unsafe impl ::core::marker::Sync for IDXGIDevice {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    pub QueryResourceResidency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows_core::HRESULT,
    pub GetGPUThreadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIDevice1, IDXGIDevice1_Vtbl, 0x77db970f_6276_48ba_ba28_070143b4392c);
::windows_core::imp::interface_hierarchy!(IDXGIDevice1, ::windows_core::IUnknown, IDXGIObject, IDXGIDevice);
impl IDXGIDevice1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAdapter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CreateSurface)(::windows_core::Interface::as_raw(self), pdesc, ppsurface.len().try_into().unwrap(), usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.QueryResourceResidency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetGPUThreadPriority)(::windows_core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGPUThreadPriority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIDevice1 {}
unsafe impl ::core::marker::Sync for IDXGIDevice1 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_Vtbl {
    pub base__: IDXGIDevice_Vtbl,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIDevice2, IDXGIDevice2_Vtbl, 0x05008617_fbfd_4051_a790_144884b4f6a9);
::windows_core::imp::interface_hierarchy!(IDXGIDevice2, ::windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1);
impl IDXGIDevice2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAdapter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CreateSurface)(::windows_core::Interface::as_raw(self), pdesc, ppsurface.len().try_into().unwrap(), usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.QueryResourceResidency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetGPUThreadPriority)(::windows_core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetGPUThreadPriority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OfferResources)(::windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), ::core::mem::transmute(ppresources.as_ptr()), priority).ok()
    }
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReclaimResources)(::windows_core::Interface::as_raw(self), numresources, ::core::mem::transmute(ppresources), ::core::mem::transmute(pdiscarded.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnqueueSetEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).EnqueueSetEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIDevice2 {}
unsafe impl ::core::marker::Sync for IDXGIDevice2 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice2_Vtbl {
    pub base__: IDXGIDevice1_Vtbl,
    pub OfferResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows_core::HRESULT,
    pub ReclaimResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub EnqueueSetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIDevice3, IDXGIDevice3_Vtbl, 0x6007896c_3244_4afd_bf18_a6d3beda5023);
::windows_core::imp::interface_hierarchy!(IDXGIDevice3, ::windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1, IDXGIDevice2);
impl IDXGIDevice3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetAdapter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateSurface)(::windows_core::Interface::as_raw(self), pdesc, ppsurface.len().try_into().unwrap(), usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.QueryResourceResidency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetGPUThreadPriority)(::windows_core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetGPUThreadPriority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OfferResources)(::windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), ::core::mem::transmute(ppresources.as_ptr()), priority).ok()
    }
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReclaimResources)(::windows_core::Interface::as_raw(self), numresources, ::core::mem::transmute(ppresources), ::core::mem::transmute(pdiscarded.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnqueueSetEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).base__.EnqueueSetEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn Trim(&self) {
        (::windows_core::Interface::vtable(self).Trim)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IDXGIDevice3 {}
unsafe impl ::core::marker::Sync for IDXGIDevice3 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice3_Vtbl {
    pub base__: IDXGIDevice2_Vtbl,
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(IDXGIDevice4, IDXGIDevice4_Vtbl, 0x95b4f95f_d8da_4ca4_9ee6_3b76d5968a10);
::windows_core::imp::interface_hierarchy!(IDXGIDevice4, ::windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1, IDXGIDevice2, IDXGIDevice3);
impl IDXGIDevice4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetAdapter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CreateSurface)(::windows_core::Interface::as_raw(self), pdesc, ppsurface.len().try_into().unwrap(), usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.QueryResourceResidency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetGPUThreadPriority)(::windows_core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetGPUThreadPriority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.OfferResources)(::windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), ::core::mem::transmute(ppresources.as_ptr()), priority).ok()
    }
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ReclaimResources)(::windows_core::Interface::as_raw(self), numresources, ::core::mem::transmute(ppresources), ::core::mem::transmute(pdiscarded.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnqueueSetEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.EnqueueSetEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn Trim(&self) {
        (::windows_core::Interface::vtable(self).base__.Trim)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn OfferResources1(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OfferResources1)(::windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), ::core::mem::transmute(ppresources.as_ptr()), priority, flags).ok()
    }
    pub unsafe fn ReclaimResources1(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReclaimResources1)(::windows_core::Interface::as_raw(self), numresources, ::core::mem::transmute(ppresources), presults).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIDevice4 {}
unsafe impl ::core::marker::Sync for IDXGIDevice4 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice4_Vtbl {
    pub base__: IDXGIDevice3_Vtbl,
    pub OfferResources1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows_core::HRESULT,
    pub ReclaimResources1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIDeviceSubObject, IDXGIDeviceSubObject_Vtbl, 0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
::windows_core::imp::interface_hierarchy!(IDXGIDeviceSubObject, ::windows_core::IUnknown, IDXGIObject);
impl IDXGIDeviceSubObject {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIDeviceSubObject {}
unsafe impl ::core::marker::Sync for IDXGIDeviceSubObject {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIDisplayControl, IDXGIDisplayControl_Vtbl, 0xea9dbf1a_c88e_4486_854a_98aa0138f30c);
::windows_core::imp::interface_hierarchy!(IDXGIDisplayControl, ::windows_core::IUnknown);
impl IDXGIDisplayControl {
    pub unsafe fn IsStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).IsStereoEnabled)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetStereoEnabled<P0>(&self, enabled: P0)
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetStereoEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi())
    }
}
unsafe impl ::core::marker::Send for IDXGIDisplayControl {}
unsafe impl ::core::marker::Sync for IDXGIDisplayControl {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDisplayControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub SetStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL),
}
::windows_core::imp::com_interface!(IDXGIFactory, IDXGIFactory_Vtbl, 0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
::windows_core::imp::interface_hierarchy!(IDXGIFactory, ::windows_core::IUnknown, IDXGIObject);
impl IDXGIFactory {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumAdapters)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).MakeWindowAssociation)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    pub unsafe fn GetWindowAssociation(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWindowAssociation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateSwapChain)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows_core::Result<IDXGIAdapter>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HMODULE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSoftwareAdapter)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIFactory {}
unsafe impl ::core::marker::Sync for IDXGIFactory {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumAdapters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MakeWindowAssociation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows_core::HRESULT,
    pub GetWindowAssociation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChain: usize,
    pub CreateSoftwareAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HMODULE, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIFactory1, IDXGIFactory1_Vtbl, 0x770aae78_f26f_4dba_a829_253c83d1b387);
::windows_core::imp::interface_hierarchy!(IDXGIFactory1, ::windows_core::IUnknown, IDXGIObject, IDXGIFactory);
impl IDXGIFactory1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumAdapters)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.MakeWindowAssociation)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    pub unsafe fn GetWindowAssociation(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWindowAssociation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.CreateSwapChain)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows_core::Result<IDXGIAdapter>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HMODULE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSoftwareAdapter)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumAdapters1)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).IsCurrent)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IDXGIFactory1 {}
unsafe impl ::core::marker::Sync for IDXGIFactory1 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_Vtbl {
    pub base__: IDXGIFactory_Vtbl,
    pub EnumAdapters1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
}
::windows_core::imp::com_interface!(IDXGIFactory2, IDXGIFactory2_Vtbl, 0x50c83a1c_e072_4c48_87b0_3630fa36a6d0);
::windows_core::imp::interface_hierarchy!(IDXGIFactory2, ::windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1);
impl IDXGIFactory2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumAdapters)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.MakeWindowAssociation)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    pub unsafe fn GetWindowAssociation(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetWindowAssociation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CreateSwapChain)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows_core::Result<IDXGIAdapter>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HMODULE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateSoftwareAdapter)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumAdapters1)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.IsCurrent)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).IsWindowedStereoEnabled)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSwapChainForHwnd)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSwapChainForCoreWindow)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows_core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSharedResourceAdapterLuid)(::windows_core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterStereoStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterStereoStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).UnregisterStereoStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterOcclusionStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterOcclusionStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).UnregisterOcclusionStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSwapChainForComposition)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIFactory2 {}
unsafe impl ::core::marker::Sync for IDXGIFactory2 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_Vtbl {
    pub base__: IDXGIFactory1_Vtbl,
    pub IsWindowedStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChainForHwnd: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChainForCoreWindow: usize,
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows_core::HRESULT,
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub UnregisterStereoStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChainForComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChainForComposition: usize,
}
::windows_core::imp::com_interface!(IDXGIFactory3, IDXGIFactory3_Vtbl, 0x25483823_cd46_4c7d_86ca_47aa95b837bd);
::windows_core::imp::interface_hierarchy!(IDXGIFactory3, ::windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2);
impl IDXGIFactory3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EnumAdapters)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.MakeWindowAssociation)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    pub unsafe fn GetWindowAssociation(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetWindowAssociation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateSwapChain)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows_core::Result<IDXGIAdapter>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HMODULE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateSoftwareAdapter)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumAdapters1)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.IsCurrent)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.IsWindowedStereoEnabled)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSwapChainForHwnd)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSwapChainForCoreWindow)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows_core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSharedResourceAdapterLuid)(::windows_core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RegisterStereoStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RegisterStereoStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.UnregisterStereoStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RegisterOcclusionStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RegisterOcclusionStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.UnregisterOcclusionStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSwapChainForComposition)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows_core::Interface::vtable(self).GetCreationFlags)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IDXGIFactory3 {}
unsafe impl ::core::marker::Sync for IDXGIFactory3 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory3_Vtbl {
    pub base__: IDXGIFactory2_Vtbl,
    pub GetCreationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
::windows_core::imp::com_interface!(IDXGIFactory4, IDXGIFactory4_Vtbl, 0x1bc6ea02_ef36_464f_bf0c_21ca39e5168a);
::windows_core::imp::interface_hierarchy!(IDXGIFactory4, ::windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3);
impl IDXGIFactory4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.EnumAdapters)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MakeWindowAssociation)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    pub unsafe fn GetWindowAssociation(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetWindowAssociation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CreateSwapChain)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows_core::Result<IDXGIAdapter>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HMODULE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CreateSoftwareAdapter)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EnumAdapters1)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsCurrent)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.IsWindowedStereoEnabled)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateSwapChainForHwnd)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateSwapChainForCoreWindow)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows_core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSharedResourceAdapterLuid)(::windows_core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RegisterStereoStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RegisterStereoStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.base__.UnregisterStereoStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RegisterOcclusionStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RegisterOcclusionStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.base__.UnregisterOcclusionStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateSwapChainForComposition)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows_core::Interface::vtable(self).base__.GetCreationFlags)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).EnumAdapterByLuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(adapterluid), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).EnumWarpAdapter)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIFactory4 {}
unsafe impl ::core::marker::Sync for IDXGIFactory4 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory4_Vtbl {
    pub base__: IDXGIFactory3_Vtbl,
    pub EnumAdapterByLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumWarpAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIFactory5, IDXGIFactory5_Vtbl, 0x7632e1f5_ee65_4dca_87fd_84cd75f8838d);
::windows_core::imp::interface_hierarchy!(IDXGIFactory5, ::windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4);
impl IDXGIFactory5 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.EnumAdapters)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    pub unsafe fn GetWindowAssociation(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetWindowAssociation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSwapChain)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows_core::Result<IDXGIAdapter>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HMODULE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.EnumAdapters1)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsCurrent)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsWindowedStereoEnabled)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateSwapChainForHwnd)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateSwapChainForCoreWindow)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows_core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetSharedResourceAdapterLuid)(::windows_core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RegisterStereoStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RegisterStereoStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.UnregisterStereoStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RegisterOcclusionStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RegisterOcclusionStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.UnregisterOcclusionStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateSwapChainForComposition)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows_core::Interface::vtable(self).base__.base__.GetCreationFlags)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.EnumAdapterByLuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(adapterluid), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.EnumWarpAdapter)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CheckFeatureSupport)(::windows_core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIFactory5 {}
unsafe impl ::core::marker::Sync for IDXGIFactory5 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory5_Vtbl {
    pub base__: IDXGIFactory4_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIFactory6, IDXGIFactory6_Vtbl, 0xc1b6694f_ff09_44a9_b03c_77900a0a1d17);
::windows_core::imp::interface_hierarchy!(IDXGIFactory6, ::windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5);
impl IDXGIFactory6 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.EnumAdapters)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    pub unsafe fn GetWindowAssociation(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetWindowAssociation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.CreateSwapChain)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows_core::Result<IDXGIAdapter>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HMODULE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.EnumAdapters1)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.IsCurrent)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsWindowedStereoEnabled)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CreateSwapChainForHwnd)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CreateSwapChainForCoreWindow)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows_core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetSharedResourceAdapterLuid)(::windows_core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RegisterStereoStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RegisterStereoStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.UnregisterStereoStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RegisterOcclusionStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RegisterOcclusionStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.UnregisterOcclusionStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CreateSwapChainForComposition)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetCreationFlags)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.EnumAdapterByLuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(adapterluid), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.EnumWarpAdapter)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CheckFeatureSupport)(::windows_core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn EnumAdapterByGpuPreference<T>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).EnumAdapterByGpuPreference)(::windows_core::Interface::as_raw(self), adapter, gpupreference, &T::IID, &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIFactory6 {}
unsafe impl ::core::marker::Sync for IDXGIFactory6 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory6_Vtbl {
    pub base__: IDXGIFactory5_Vtbl,
    pub EnumAdapterByGpuPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIFactory7, IDXGIFactory7_Vtbl, 0xa4966eed_76db_44da_84c1_ee9a7afb20a8);
::windows_core::imp::interface_hierarchy!(IDXGIFactory7, ::windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5, IDXGIFactory6);
impl IDXGIFactory7 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.EnumAdapters)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), flags).ok()
    }
    pub unsafe fn GetWindowAssociation(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetWindowAssociation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSwapChain)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows_core::Result<IDXGIAdapter>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HMODULE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows_core::Result<IDXGIAdapter1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.EnumAdapters1)(::windows_core::Interface::as_raw(self), adapter, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.IsCurrent)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.IsWindowedStereoEnabled)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForHwnd)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hwnd.into_param().abi(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForCoreWindow)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pwindow.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows_core::Result<super::super::Foundation::LUID>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetSharedResourceAdapterLuid)(::windows_core::Interface::as_raw(self), hresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.RegisterStereoStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.RegisterStereoStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.UnregisterStereoStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.RegisterOcclusionStatusWindow)(::windows_core::Interface::as_raw(self), windowhandle.into_param().abi(), wmsg, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.RegisterOcclusionStatusEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.UnregisterOcclusionStatus)(::windows_core::Interface::as_raw(self), dwcookie)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForComposition)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetCreationFlags)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EnumAdapterByLuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(adapterluid), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EnumWarpAdapter)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CheckFeatureSupport)(::windows_core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn EnumAdapterByGpuPreference<T>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.EnumAdapterByGpuPreference)(::windows_core::Interface::as_raw(self), adapter, gpupreference, &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterAdaptersChangedEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterAdaptersChangedEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterAdaptersChangedEvent)(::windows_core::Interface::as_raw(self), dwcookie).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIFactory7 {}
unsafe impl ::core::marker::Sync for IDXGIFactory7 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory7_Vtbl {
    pub base__: IDXGIFactory6_Vtbl,
    pub RegisterAdaptersChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub UnregisterAdaptersChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIFactoryMedia, IDXGIFactoryMedia_Vtbl, 0x41e7d1f2_a591_4f7b_a2e5_fa9c843e1c12);
::windows_core::imp::interface_hierarchy!(IDXGIFactoryMedia, ::windows_core::IUnknown);
impl IDXGIFactoryMedia {
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<P0, P1, P2>(&self, pdevice: P0, hsurface: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows_core::Result<IDXGISwapChain1>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
        P2: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSwapChainForCompositionSurfaceHandle)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hsurface.into_param().abi(), pdesc, prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<P0, P1, P2, P3>(&self, pdevice: P0, hsurface: P1, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: P2, prestricttooutput: P3) -> ::windows_core::Result<IDXGIDecodeSwapChain>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
        P2: ::windows_core::IntoParam<IDXGIResource>,
        P3: ::windows_core::IntoParam<IDXGIOutput>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDecodeSwapChainForCompositionSurfaceHandle)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), hsurface.into_param().abi(), pdesc, pyuvdecodebuffers.into_param().abi(), prestricttooutput.into_param().abi(), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIFactoryMedia {}
unsafe impl ::core::marker::Sync for IDXGIFactoryMedia {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactoryMedia_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChainForCompositionSurfaceHandle: usize,
    pub CreateDecodeSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut ::core::ffi::c_void, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIInfoQueue, IDXGIInfoQueue_Vtbl, 0xd67441c7_672a_476f_9e82_cd55b44949ce);
::windows_core::imp::interface_hierarchy!(IDXGIInfoQueue, ::windows_core::IUnknown);
impl IDXGIInfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, producer: ::windows_core::GUID, messagecountlimit: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMessageCountLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), messagecountlimit).ok()
    }
    pub unsafe fn ClearStoredMessages(&self, producer: ::windows_core::GUID) {
        (::windows_core::Interface::vtable(self).ClearStoredMessages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetMessage(&self, producer: ::windows_core::GUID, messageindex: u64, pmessage: ::core::option::Option<*mut DXGI_INFO_QUEUE_MESSAGE>, pmessagebytelength: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), messageindex, ::core::mem::transmute(pmessage.unwrap_or(::std::ptr::null_mut())), pmessagebytelength).ok()
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: ::windows_core::GUID) -> u64 {
        (::windows_core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetNumStoredMessages(&self, producer: ::windows_core::GUID) -> u64 {
        (::windows_core::Interface::vtable(self).GetNumStoredMessages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: ::windows_core::GUID) -> u64 {
        (::windows_core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetMessageCountLimit(&self, producer: ::windows_core::GUID) -> u64 {
        (::windows_core::Interface::vtable(self).GetMessageCountLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self, producer: ::windows_core::GUID) -> u64 {
        (::windows_core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self, producer: ::windows_core::GUID) -> u64 {
        (::windows_core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn AddStorageFilterEntries(&self, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStorageFilterEntries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), pfilter).ok()
    }
    pub unsafe fn GetStorageFilter(&self, producer: ::windows_core::GUID, pfilter: ::core::option::Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), ::core::mem::transmute(pfilter.unwrap_or(::std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearStorageFilter(&self, producer: ::windows_core::GUID) {
        (::windows_core::Interface::vtable(self).ClearStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn PushEmptyStorageFilter(&self, producer: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PushEmptyStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushDenyAllStorageFilter(&self, producer: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PushDenyAllStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushCopyOfStorageFilter(&self, producer: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PushCopyOfStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushStorageFilter(&self, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PushStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), pfilter).ok()
    }
    pub unsafe fn PopStorageFilter(&self, producer: ::windows_core::GUID) {
        (::windows_core::Interface::vtable(self).PopStorageFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetStorageFilterStackSize(&self, producer: ::windows_core::GUID) -> u32 {
        (::windows_core::Interface::vtable(self).GetStorageFilterStackSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRetrievalFilterEntries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), pfilter).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, producer: ::windows_core::GUID, pfilter: ::core::option::Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRetrievalFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), ::core::mem::transmute(pfilter.unwrap_or(::std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearRetrievalFilter(&self, producer: ::windows_core::GUID) {
        (::windows_core::Interface::vtable(self).ClearRetrievalFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self, producer: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PushEmptyRetrievalFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushDenyAllRetrievalFilter(&self, producer: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PushDenyAllRetrievalFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self, producer: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PushCopyOfRetrievalFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer)).ok()
    }
    pub unsafe fn PushRetrievalFilter(&self, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PushRetrievalFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), pfilter).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self, producer: ::windows_core::GUID) {
        (::windows_core::Interface::vtable(self).PopRetrievalFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self, producer: ::windows_core::GUID) -> u32 {
        (::windows_core::Interface::vtable(self).GetRetrievalFilterStackSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
    pub unsafe fn AddMessage<P0>(&self, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).AddMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), category, severity, id, pdescription.into_param().abi()).ok()
    }
    pub unsafe fn AddApplicationMessage<P0>(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).AddApplicationMessage)(::windows_core::Interface::as_raw(self), severity, pdescription.into_param().abi()).ok()
    }
    pub unsafe fn SetBreakOnCategory<P0>(&self, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBreakOnCategory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), category, benable.into_param().abi()).ok()
    }
    pub unsafe fn SetBreakOnSeverity<P0>(&self, producer: ::windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBreakOnSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), severity, benable.into_param().abi()).ok()
    }
    pub unsafe fn SetBreakOnID<P0>(&self, producer: ::windows_core::GUID, id: i32, benable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBreakOnID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), id, benable.into_param().abi()).ok()
    }
    pub unsafe fn GetBreakOnCategory(&self, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).GetBreakOnCategory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), category)
    }
    pub unsafe fn GetBreakOnSeverity(&self, producer: ::windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).GetBreakOnSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), severity)
    }
    pub unsafe fn GetBreakOnID(&self, producer: ::windows_core::GUID, id: i32) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).GetBreakOnID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), id)
    }
    pub unsafe fn SetMuteDebugOutput<P0>(&self, producer: ::windows_core::GUID, bmute: P0)
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetMuteDebugOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer), bmute.into_param().abi())
    }
    pub unsafe fn GetMuteDebugOutput(&self, producer: ::windows_core::GUID) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).GetMuteDebugOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(producer))
    }
}
unsafe impl ::core::marker::Send for IDXGIInfoQueue {}
unsafe impl ::core::marker::Sync for IDXGIInfoQueue {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIInfoQueue_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, messagecountlimit: u64) -> ::windows_core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID),
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::HRESULT,
    pub GetNumStoredMessagesAllowedByRetrievalFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PushDenyAllStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PushDenyAllRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u32,
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    pub SetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    pub GetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, id: i32) -> super::super::Foundation::BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, bmute: super::super::Foundation::BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> super::super::Foundation::BOOL,
}
::windows_core::imp::com_interface!(IDXGIKeyedMutex, IDXGIKeyedMutex_Vtbl, 0x9d8e1289_d7b3_465f_8126_250e349af85d);
::windows_core::imp::interface_hierarchy!(IDXGIKeyedMutex, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGIKeyedMutex {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireSync)(::windows_core::Interface::as_raw(self), key, dwmilliseconds).ok()
    }
    pub unsafe fn ReleaseSync(&self, key: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseSync)(::windows_core::Interface::as_raw(self), key).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIKeyedMutex {}
unsafe impl ::core::marker::Sync for IDXGIKeyedMutex {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIKeyedMutex_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub AcquireSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows_core::HRESULT,
    pub ReleaseSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: u64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIObject, IDXGIObject_Vtbl, 0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
::windows_core::imp::interface_hierarchy!(IDXGIObject, ::windows_core::IUnknown);
impl IDXGIObject {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIObject {}
unsafe impl ::core::marker::Sync for IDXGIObject {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows_core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIOutput, IDXGIOutput_Vtbl, 0xae02eedb_c735_4690_8d52_5a8dc20213aa);
::windows_core::imp::interface_hierarchy!(IDXGIOutput, ::windows_core::IUnknown, IDXGIObject);
impl IDXGIOutput {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayModeList)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).FindClosestMatchingMode)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WaitForVBlank)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TakeOwnership)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows_core::Interface::vtable(self).ReleaseOwnership)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGammaControlCapabilities)(::windows_core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).SetDisplaySurface)(::windows_core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).GetDisplaySurfaceData)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIOutput {}
unsafe impl ::core::marker::Sync for IDXGIOutput {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayModeList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayModeList: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub FindClosestMatchingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    FindClosestMatchingMode: usize,
    pub WaitForVBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub ReleaseOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetGammaControlCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetGammaControlCapabilities: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetGammaControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetGammaControl: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetGammaControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetGammaControl: usize,
    pub SetDisplaySurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscanoutsurface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDisplaySurfaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestination: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIOutput1, IDXGIOutput1_Vtbl, 0x00cddea8_939b_4b83_a340_a685226666cc);
::windows_core::imp::interface_hierarchy!(IDXGIOutput1, ::windows_core::IUnknown, IDXGIObject, IDXGIOutput);
impl IDXGIOutput1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDisplayModeList)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.FindClosestMatchingMode)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.WaitForVBlank)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TakeOwnership)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows_core::Interface::vtable(self).base__.ReleaseOwnership)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetGammaControlCapabilities)(::windows_core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDisplaySurface)(::windows_core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.GetDisplaySurfaceData)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayModeList1)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).FindClosestMatchingMode1)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGIResource>,
    {
        (::windows_core::Interface::vtable(self).GetDisplaySurfaceData1)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows_core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DuplicateOutput)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIOutput1 {}
unsafe impl ::core::marker::Sync for IDXGIOutput1 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput1_Vtbl {
    pub base__: IDXGIOutput_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayModeList1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayModeList1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub FindClosestMatchingMode1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    FindClosestMatchingMode1: usize,
    pub GetDisplaySurfaceData1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestination: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DuplicateOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIOutput2, IDXGIOutput2_Vtbl, 0x595e39d1_2724_4663_99b1_da969de28364);
::windows_core::imp::interface_hierarchy!(IDXGIOutput2, ::windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1);
impl IDXGIOutput2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDisplayModeList)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.FindClosestMatchingMode)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.WaitForVBlank)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.TakeOwnership)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.ReleaseOwnership)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetGammaControlCapabilities)(::windows_core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetDisplaySurface)(::windows_core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.GetDisplaySurfaceData)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDisplayModeList1)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.FindClosestMatchingMode1)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGIResource>,
    {
        (::windows_core::Interface::vtable(self).base__.GetDisplaySurfaceData1)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows_core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DuplicateOutput)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).SupportsOverlays)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IDXGIOutput2 {}
unsafe impl ::core::marker::Sync for IDXGIOutput2 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput2_Vtbl {
    pub base__: IDXGIOutput1_Vtbl,
    pub SupportsOverlays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
}
::windows_core::imp::com_interface!(IDXGIOutput3, IDXGIOutput3_Vtbl, 0x8a6bb301_7e7e_41f4_a8e0_5b32f7f99b18);
::windows_core::imp::interface_hierarchy!(IDXGIOutput3, ::windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2);
impl IDXGIOutput3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDisplayModeList)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.FindClosestMatchingMode)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.WaitForVBlank)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.TakeOwnership)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ReleaseOwnership)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetGammaControlCapabilities)(::windows_core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetDisplaySurface)(::windows_core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDisplaySurfaceData)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDisplayModeList1)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.FindClosestMatchingMode1)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGIResource>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.GetDisplaySurfaceData1)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows_core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DuplicateOutput)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.SupportsOverlays)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CheckOverlaySupport)(::windows_core::Interface::as_raw(self), enumformat, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIOutput3 {}
unsafe impl ::core::marker::Sync for IDXGIOutput3 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput3_Vtbl {
    pub base__: IDXGIOutput2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlaySupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlaySupport: usize,
}
::windows_core::imp::com_interface!(IDXGIOutput4, IDXGIOutput4_Vtbl, 0xdc7dca35_2196_414d_9f53_617884032a60);
::windows_core::imp::interface_hierarchy!(IDXGIOutput4, ::windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3);
impl IDXGIOutput4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDisplayModeList)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.FindClosestMatchingMode)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.WaitForVBlank)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.TakeOwnership)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.ReleaseOwnership)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetGammaControlCapabilities)(::windows_core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetDisplaySurface)(::windows_core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDisplaySurfaceData)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDisplayModeList1)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.FindClosestMatchingMode1)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGIResource>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDisplaySurfaceData1)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows_core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DuplicateOutput)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.SupportsOverlays)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CheckOverlaySupport)(::windows_core::Interface::as_raw(self), enumformat, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P0>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CheckOverlayColorSpaceSupport)(::windows_core::Interface::as_raw(self), format, colorspace, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIOutput4 {}
unsafe impl ::core::marker::Sync for IDXGIOutput4 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput4_Vtbl {
    pub base__: IDXGIOutput3_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlayColorSpaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlayColorSpaceSupport: usize,
}
::windows_core::imp::com_interface!(IDXGIOutput5, IDXGIOutput5_Vtbl, 0x80a07424_ab52_42eb_833c_0c42fd282d98);
::windows_core::imp::interface_hierarchy!(IDXGIOutput5, ::windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3, IDXGIOutput4);
impl IDXGIOutput5 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDisplayModeList)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.FindClosestMatchingMode)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.WaitForVBlank)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.TakeOwnership)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.ReleaseOwnership)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetGammaControlCapabilities)(::windows_core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetDisplaySurface)(::windows_core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDisplaySurfaceData)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDisplayModeList1)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.FindClosestMatchingMode1)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGIResource>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDisplaySurfaceData1)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows_core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DuplicateOutput)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SupportsOverlays)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CheckOverlaySupport)(::windows_core::Interface::as_raw(self), enumformat, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P0>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CheckOverlayColorSpaceSupport)(::windows_core::Interface::as_raw(self), format, colorspace, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DuplicateOutput1<P0>(&self, pdevice: P0, flags: u32, psupportedformats: &[Common::DXGI_FORMAT]) -> ::windows_core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DuplicateOutput1)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), flags, psupportedformats.len().try_into().unwrap(), ::core::mem::transmute(psupportedformats.as_ptr()), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIOutput5 {}
unsafe impl ::core::marker::Sync for IDXGIOutput5 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput5_Vtbl {
    pub base__: IDXGIOutput4_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub DuplicateOutput1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    DuplicateOutput1: usize,
}
::windows_core::imp::com_interface!(IDXGIOutput6, IDXGIOutput6_Vtbl, 0x068346e8_aaec_4b84_add7_137f513f77a1);
::windows_core::imp::interface_hierarchy!(IDXGIOutput6, ::windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3, IDXGIOutput4, IDXGIOutput5);
impl IDXGIOutput6 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetDisplayModeList)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.FindClosestMatchingMode)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.WaitForVBlank)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.TakeOwnership)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ReleaseOwnership)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetGammaControlCapabilities)(::windows_core::Interface::as_raw(self), pgammacaps).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetGammaControl)(::windows_core::Interface::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetDisplaySurface)(::windows_core::Interface::as_raw(self), pscanoutsurface.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetDisplaySurfaceData)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDisplayModeList1)(::windows_core::Interface::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.FindClosestMatchingMode1)(::windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into_param().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDXGIResource>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDisplaySurfaceData1)(::windows_core::Interface::as_raw(self), pdestination.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows_core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.DuplicateOutput)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SupportsOverlays)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CheckOverlaySupport)(::windows_core::Interface::as_raw(self), enumformat, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P0>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CheckOverlayColorSpaceSupport)(::windows_core::Interface::as_raw(self), format, colorspace, pconcerneddevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DuplicateOutput1<P0>(&self, pdevice: P0, flags: u32, psupportedformats: &[Common::DXGI_FORMAT]) -> ::windows_core::Result<IDXGIOutputDuplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DuplicateOutput1)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), flags, psupportedformats.len().try_into().unwrap(), ::core::mem::transmute(psupportedformats.as_ptr()), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckHardwareCompositionSupport(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CheckHardwareCompositionSupport)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIOutput6 {}
unsafe impl ::core::marker::Sync for IDXGIOutput6 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput6_Vtbl {
    pub base__: IDXGIOutput5_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc1: usize,
    pub CheckHardwareCompositionSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIOutputDuplication, IDXGIOutputDuplication_Vtbl, 0x191cfac3_a341_470d_b26e_a864f428319c);
::windows_core::imp::interface_hierarchy!(IDXGIOutputDuplication, ::windows_core::IUnknown, IDXGIObject);
impl IDXGIOutputDuplication {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC) {
        (::windows_core::Interface::vtable(self).GetDesc)(::windows_core::Interface::as_raw(self), pdesc)
    }
    pub unsafe fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::core::option::Option<IDXGIResource>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireNextFrame)(::windows_core::Interface::as_raw(self), timeoutinmilliseconds, pframeinfo, ::core::mem::transmute(ppdesktopresource)).ok()
    }
    pub unsafe fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFrameDirtyRects)(::windows_core::Interface::as_raw(self), dirtyrectsbuffersize, pdirtyrectsbuffer, pdirtyrectsbuffersizerequired).ok()
    }
    pub unsafe fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFrameMoveRects)(::windows_core::Interface::as_raw(self), moverectsbuffersize, pmoverectbuffer, pmoverectsbuffersizerequired).ok()
    }
    pub unsafe fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFramePointerShape)(::windows_core::Interface::as_raw(self), pointershapebuffersize, ppointershapebuffer, ppointershapebuffersizerequired, ppointershapeinfo).ok()
    }
    pub unsafe fn MapDesktopSurface(&self) -> ::windows_core::Result<DXGI_MAPPED_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MapDesktopSurface)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnMapDesktopSurface(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnMapDesktopSurface)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseFrame(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseFrame)(::windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGIOutputDuplication {}
unsafe impl ::core::marker::Sync for IDXGIOutputDuplication {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutputDuplication_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub AcquireNextFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFrameDirtyRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows_core::HRESULT,
    pub GetFrameMoveRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows_core::HRESULT,
    pub GetFramePointerShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows_core::HRESULT,
    pub MapDesktopSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows_core::HRESULT,
    pub UnMapDesktopSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIResource, IDXGIResource_Vtbl, 0x035f3ab4_482e_4e50_b41f_8a7f8bd8960b);
::windows_core::imp::interface_hierarchy!(IDXGIResource, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGIResource {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedHandle(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSharedHandle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUsage(&self) -> ::windows_core::Result<DXGI_USAGE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUsage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEvictionPriority)(::windows_core::Interface::as_raw(self), evictionpriority).ok()
    }
    pub unsafe fn GetEvictionPriority(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEvictionPriority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIResource {}
unsafe impl ::core::marker::Sync for IDXGIResource {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub GetSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    pub GetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusage: *mut DXGI_USAGE) -> ::windows_core::HRESULT,
    pub SetEvictionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: u32) -> ::windows_core::HRESULT,
    pub GetEvictionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGIResource1, IDXGIResource1_Vtbl, 0x30961379_4609_4a41_998e_54fe567ee0c1);
::windows_core::imp::interface_hierarchy!(IDXGIResource1, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGIResource);
impl IDXGIResource1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedHandle(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSharedHandle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUsage(&self) -> ::windows_core::Result<DXGI_USAGE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetUsage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEvictionPriority)(::windows_core::Interface::as_raw(self), evictionpriority).ok()
    }
    pub unsafe fn GetEvictionPriority(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEvictionPriority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSubresourceSurface(&self, index: u32) -> ::windows_core::Result<IDXGISurface2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSubresourceSurface)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Security\"`"]
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn CreateSharedHandle<P0>(&self, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwaccess: u32, lpname: P0) -> ::windows_core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSharedHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), dwaccess, lpname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGIResource1 {}
unsafe impl ::core::marker::Sync for IDXGIResource1 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource1_Vtbl {
    pub base__: IDXGIResource_Vtbl,
    pub CreateSubresourceSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: ::windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    CreateSharedHandle: usize,
}
::windows_core::imp::com_interface!(IDXGISurface, IDXGISurface_Vtbl, 0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
::windows_core::imp::interface_hierarchy!(IDXGISurface, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISurface {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Map)(::windows_core::Interface::as_raw(self), plockedrect, mapflags).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unmap)(::windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGISurface {}
unsafe impl ::core::marker::Sync for IDXGISurface {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGISurface1, IDXGISurface1_Vtbl, 0x4ae63092_6327_4c1b_80ae_bfe12ea32b86);
::windows_core::imp::interface_hierarchy!(IDXGISurface1, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface);
impl IDXGISurface1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Map)(::windows_core::Interface::as_raw(self), plockedrect, mapflags).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Unmap)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC<P0>(&self, discard: P0) -> ::windows_core::Result<super::Gdi::HDC>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDC)(::windows_core::Interface::as_raw(self), discard.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReleaseDC(&self, pdirtyrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdirtyrect.unwrap_or(::std::ptr::null()))).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGISurface1 {}
unsafe impl ::core::marker::Sync for IDXGISurface1 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface1_Vtbl {
    pub base__: IDXGISurface_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGISurface2, IDXGISurface2_Vtbl, 0xaba496dd_b617_4cb8_a866_bc44d7eb1fa2);
::windows_core::imp::interface_hierarchy!(IDXGISurface2, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface, IDXGISurface1);
impl IDXGISurface2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Map)(::windows_core::Interface::as_raw(self), plockedrect, mapflags).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Unmap)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC<P0>(&self, discard: P0) -> ::windows_core::Result<super::Gdi::HDC>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDC)(::windows_core::Interface::as_raw(self), discard.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReleaseDC(&self, pdirtyrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReleaseDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdirtyrect.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetResource<T>(&self, psubresourceindex: *mut u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetResource)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__, psubresourceindex).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGISurface2 {}
unsafe impl ::core::marker::Sync for IDXGISurface2 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface2_Vtbl {
    pub base__: IDXGISurface1_Vtbl,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGISwapChain, IDXGISwapChain_Vtbl, 0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
::windows_core::imp::interface_hierarchy!(IDXGISwapChain, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISwapChain {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Present)(::windows_core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetBuffer)(::windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        (::windows_core::Interface::vtable(self).SetFullscreenState)(::windows_core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFullscreenState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResizeBuffers)(::windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResizeTarget)(::windows_core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContainingOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastPresentCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGISwapChain {}
unsafe impl ::core::marker::Sync for IDXGISwapChain {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: u32, riid: *const ::windows_core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetFullscreenState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFullscreenState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeTarget: usize,
    pub GetContainingOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::HRESULT,
    pub GetLastPresentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGISwapChain1, IDXGISwapChain1_Vtbl, 0x790a45f7_0d42_4876_983a_0a55cfe6f4aa);
::windows_core::imp::interface_hierarchy!(IDXGISwapChain1, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain);
impl IDXGISwapChain1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.Present)(::windows_core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetBuffer)(::windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFullscreenState)(::windows_core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFullscreenState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ResizeBuffers)(::windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ResizeTarget)(::windows_core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetContainingOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLastPresentCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFullscreenDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHwnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetCoreWindow)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Present1)(::windows_core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).IsTemporaryMonoSupported)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRestrictToOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBackgroundColor)(::windows_core::Interface::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows_core::Result<DXGI_RGBA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBackgroundColor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRotation)(::windows_core::Interface::as_raw(self), rotation).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows_core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRotation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
unsafe impl ::core::marker::Send for IDXGISwapChain1 {}
unsafe impl ::core::marker::Sync for IDXGISwapChain1 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_Vtbl {
    pub base__: IDXGISwapChain_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFullscreenDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFullscreenDesc: usize,
    pub GetHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    pub GetCoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refiid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Present1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows_core::HRESULT,
    pub IsTemporaryMonoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetRestrictToOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows_core::HRESULT,
    pub GetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetRotation: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRotation: usize,
}
::windows_core::imp::com_interface!(IDXGISwapChain2, IDXGISwapChain2_Vtbl, 0xa8be2ac4_199f_4946_b331_79599fb98de7);
::windows_core::imp::interface_hierarchy!(IDXGISwapChain2, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1);
impl IDXGISwapChain2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.Present)(::windows_core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetBuffer)(::windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetFullscreenState)(::windows_core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetFullscreenState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ResizeBuffers)(::windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ResizeTarget)(::windows_core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetContainingOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetLastPresentCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFullscreenDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHwnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetCoreWindow)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.Present1)(::windows_core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.IsTemporaryMonoSupported)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRestrictToOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBackgroundColor)(::windows_core::Interface::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows_core::Result<DXGI_RGBA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBackgroundColor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRotation)(::windows_core::Interface::as_raw(self), rotation).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows_core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRotation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSourceSize)(::windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSourceSize)(::windows_core::Interface::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        (::windows_core::Interface::vtable(self).GetFrameLatencyWaitableObject)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrixTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMatrixTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGISwapChain2 {}
unsafe impl ::core::marker::Sync for IDXGISwapChain2 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_Vtbl {
    pub base__: IDXGISwapChain1_Vtbl,
    pub SetSourceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows_core::HRESULT,
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub SetMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows_core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGISwapChain3, IDXGISwapChain3_Vtbl, 0x94d99bdb_f1f8_4ab0_b236_7da0170edab1);
::windows_core::imp::interface_hierarchy!(IDXGISwapChain3, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2);
impl IDXGISwapChain3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Present)(::windows_core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetBuffer)(::windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetFullscreenState)(::windows_core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetFullscreenState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ResizeBuffers)(::windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ResizeTarget)(::windows_core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetContainingOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetLastPresentCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetFullscreenDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetHwnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.GetCoreWindow)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.Present1)(::windows_core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.IsTemporaryMonoSupported)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetRestrictToOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetBackgroundColor)(::windows_core::Interface::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows_core::Result<DXGI_RGBA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetBackgroundColor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRotation)(::windows_core::Interface::as_raw(self), rotation).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows_core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetRotation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSourceSize)(::windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSourceSize)(::windows_core::Interface::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        (::windows_core::Interface::vtable(self).base__.GetFrameLatencyWaitableObject)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMatrixTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMatrixTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows_core::Interface::vtable(self).GetCurrentBackBufferIndex)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CheckColorSpaceSupport)(::windows_core::Interface::as_raw(self), colorspace, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetColorSpace1)(::windows_core::Interface::as_raw(self), colorspace).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResizeBuffers1)(::windows_core::Interface::as_raw(self), buffercount, width, height, format, swapchainflags, pcreationnodemask, ::core::mem::transmute(pppresentqueue)).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGISwapChain3 {}
unsafe impl ::core::marker::Sync for IDXGISwapChain3 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain3_Vtbl {
    pub base__: IDXGISwapChain2_Vtbl,
    pub GetCurrentBackBufferIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckColorSpaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckColorSpaceSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers1: usize,
}
::windows_core::imp::com_interface!(IDXGISwapChain4, IDXGISwapChain4_Vtbl, 0x3d585d5a_bd4a_489e_b1f4_3dbcb6452ffb);
::windows_core::imp::interface_hierarchy!(IDXGISwapChain4, ::windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3);
impl IDXGISwapChain4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows_core::Interface::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows_core::GUID, punknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows_core::Interface::as_raw(self), name, punknown.into_param().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows_core::Interface::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetDevice)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Present)(::windows_core::Interface::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetBuffer)(::windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDXGIOutput>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetFullscreenState)(::windows_core::Interface::as_raw(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetFullscreenState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.ResizeBuffers)(::windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.ResizeTarget)(::windows_core::Interface::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetContainingOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetLastPresentCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDesc1)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetFullscreenDesc)(::windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetHwnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetCoreWindow)(::windows_core::Interface::as_raw(self), &T::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Present1)(::windows_core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsTemporaryMonoSupported)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows_core::Result<IDXGIOutput> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetRestrictToOutput)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetBackgroundColor)(::windows_core::Interface::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows_core::Result<DXGI_RGBA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetBackgroundColor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetRotation)(::windows_core::Interface::as_raw(self), rotation).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows_core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetRotation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetSourceSize)(::windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetSourceSize)(::windows_core::Interface::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMaximumFrameLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        (::windows_core::Interface::vtable(self).base__.base__.GetFrameLatencyWaitableObject)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetMatrixTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetMatrixTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows_core::Interface::vtable(self).base__.GetCurrentBackBufferIndex)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CheckColorSpaceSupport)(::windows_core::Interface::as_raw(self), colorspace, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetColorSpace1)(::windows_core::Interface::as_raw(self), colorspace).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ResizeBuffers1)(::windows_core::Interface::as_raw(self), buffercount, width, height, format, swapchainflags, pcreationnodemask, ::core::mem::transmute(pppresentqueue)).ok()
    }
    pub unsafe fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, pmetadata: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHDRMetaData)(::windows_core::Interface::as_raw(self), r#type, pmetadata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pmetadata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGISwapChain4 {}
unsafe impl ::core::marker::Sync for IDXGISwapChain4 {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain4_Vtbl {
    pub base__: IDXGISwapChain3_Vtbl,
    pub SetHDRMetaData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGISwapChainMedia, IDXGISwapChainMedia_Vtbl, 0xdd95b90b_f05f_4f6a_bd65_25bfb264bd84);
::windows_core::imp::interface_hierarchy!(IDXGISwapChainMedia, ::windows_core::IUnknown);
impl IDXGISwapChainMedia {
    pub unsafe fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFrameStatisticsMedia)(::windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn SetPresentDuration(&self, duration: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPresentDuration)(::windows_core::Interface::as_raw(self), duration).ok()
    }
    pub unsafe fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CheckPresentDurationSupport)(::windows_core::Interface::as_raw(self), desiredpresentduration, pclosestsmallerpresentduration, pclosestlargerpresentduration).ok()
    }
}
unsafe impl ::core::marker::Send for IDXGISwapChainMedia {}
unsafe impl ::core::marker::Sync for IDXGISwapChainMedia {}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChainMedia_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFrameStatisticsMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows_core::HRESULT,
    pub SetPresentDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows_core::HRESULT,
    pub CheckPresentDurationSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDXGraphicsAnalysis, IDXGraphicsAnalysis_Vtbl, 0x9f251514_9d4d_4902_9d60_18988ab7d4b5);
::windows_core::imp::interface_hierarchy!(IDXGraphicsAnalysis, ::windows_core::IUnknown);
impl IDXGraphicsAnalysis {
    pub unsafe fn BeginCapture(&self) {
        (::windows_core::Interface::vtable(self).BeginCapture)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn EndCapture(&self) {
        (::windows_core::Interface::vtable(self).EndCapture)(::windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGraphicsAnalysis_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub EndCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(4i32);
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(32i32);
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(0i32);
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(1i32);
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(2i32);
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(8i32);
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(16i32);
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(0i32);
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(1i32);
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(2i32);
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(1i32);
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(0i32);
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(4i32);
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(3i32);
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(2i32);
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1u32;
pub const DXGI_DEBUG_ALL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe48ae283_da80_490b_87e6_43e9a9cfda08);
pub const DXGI_DEBUG_APP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06cd6e01_4219_4ebd_8709_27ed23360c62);
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
pub const DXGI_DEBUG_DX: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35cdd7fc_13b2_421d_a5d7_7e4451287d64);
pub const DXGI_DEBUG_DXGI: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25cddaa4_b1c6_47e1_ac3e_98875b5a2e2a);
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(7i32);
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(2i32);
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(4i32);
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(1i32);
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8u32;
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1u32;
pub const DXGI_ENUM_MODES_SCALING: u32 = 2u32;
pub const DXGI_ENUM_MODES_STEREO: u32 = 4u32;
pub const DXGI_ERROR_ACCESS_DENIED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270485i32);
pub const DXGI_ERROR_ACCESS_LOST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270490i32);
pub const DXGI_ERROR_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270474i32);
pub const DXGI_ERROR_CACHE_CORRUPT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270477i32);
pub const DXGI_ERROR_CACHE_FULL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270476i32);
pub const DXGI_ERROR_CACHE_HASH_COLLISION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270475i32);
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270486i32);
pub const DXGI_ERROR_DEVICE_HUNG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270522i32);
pub const DXGI_ERROR_DEVICE_REMOVED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270523i32);
pub const DXGI_ERROR_DEVICE_RESET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270521i32);
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270496i32);
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270479i32);
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270517i32);
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270516i32);
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270480i32);
pub const DXGI_ERROR_INVALID_CALL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270527i32);
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270491i32);
pub const DXGI_ERROR_MORE_DATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270525i32);
pub const DXGI_ERROR_MPO_UNPINNED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270428i32);
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270484i32);
pub const DXGI_ERROR_NONEXCLUSIVE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270495i32);
pub const DXGI_ERROR_NON_COMPOSITED_UI: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270478i32);
pub const DXGI_ERROR_NOT_CURRENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270482i32);
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270494i32);
pub const DXGI_ERROR_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270526i32);
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270493i32);
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270492i32);
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270487i32);
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270483i32);
pub const DXGI_ERROR_SESSION_DISCONNECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270488i32);
pub const DXGI_ERROR_UNSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270524i32);
pub const DXGI_ERROR_WAIT_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270489i32);
pub const DXGI_ERROR_WAS_STILL_DRAWING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005270518i32);
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = DXGI_FEATURE(0i32);
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(0i32);
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(3i32);
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(2i32);
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(1i32);
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(2i32);
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(1i32);
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(0i32);
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(0i32);
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(4i32);
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(3i32);
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(1i32);
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(2i32);
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(4i32);
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(1i32);
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(2i32);
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(1i32);
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(2i32);
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(0i32);
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(3i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(4i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(9i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(2i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(1i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(8i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(10i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(5i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(7i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(6i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(0i32);
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(0i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(1i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(3i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(4i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(2i32);
pub const DXGI_MAP_DISCARD: u32 = 4u32;
pub const DXGI_MAP_READ: u32 = 1u32;
pub const DXGI_MAP_WRITE: u32 = 2u32;
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = DXGI_MEMORY_SEGMENT_GROUP(0i32);
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = DXGI_MEMORY_SEGMENT_GROUP(1i32);
pub const DXGI_MSG_DXGIGetDebugInterface1_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(231i32);
pub const DXGI_MSG_DXGIGetDebugInterface1_NULL_ppDebug: DXGI_Message_Id = DXGI_Message_Id(230i32);
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs2_InvalidEnumOutputs2Flag: DXGI_Message_Id = DXGI_Message_Id(257i32);
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(127i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetDestSize_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(245i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetSourceRect_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(243i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetTargetRect_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(244i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetColorSpace_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(239i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetDestSize_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(242i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetSourceRect_InvalidRect: DXGI_Message_Id = DXGI_Message_Id(240i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetTargetRect_InvalidRect: DXGI_Message_Id = DXGI_Message_Id(241i32);
pub const DXGI_MSG_IDXGIDevice_CreateSurface_InvalidParametersWithpSharedResource: DXGI_Message_Id = DXGI_Message_Id(63i32);
pub const DXGI_MSG_IDXGIDisplayControl_IsStereoEnabled_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(195i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCompositionSurface_InvalidHandle: DXGI_Message_Id = DXGI_Message_Id(173i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_ForegroundUnsupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(220i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidAlphaMode: DXGI_Message_Id = DXGI_Message_Id(222i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(221i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_UnsupportedOnWindows7: DXGI_Message_Id = DXGI_Message_Id(164i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsInvalid: DXGI_Message_Id = DXGI_Message_Id(172i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsNULL: DXGI_Message_Id = DXGI_Message_Id(165i32);
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusEvent_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(193i32);
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusWindow_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(192i32);
pub const DXGI_MSG_IDXGIFactory2_UnregisterStatus_CookieNotFound: DXGI_Message_Id = DXGI_Message_Id(129i32);
pub const DXGI_MSG_IDXGIFactory7_UnregisterAdaptersChangedEvent_CookieNotFound: DXGI_Message_Id = DXGI_Message_Id(293i32);
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidFeature: DXGI_Message_Id = DXGI_Message_Id(287i32);
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(288i32);
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ModuleIsNULL: DXGI_Message_Id = DXGI_Message_Id(68i32);
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ppAdapterInterfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(28i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidAlphaMode: DXGI_Message_Id = DXGI_Message_Id(196i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(189i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_OnlyFlipSequentialSupported: DXGI_Message_Id = DXGI_Message_Id(135i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(136i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnWindows7: DXGI_Message_Id = DXGI_Message_Id(137i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_WidthOrHeightIsZero: DXGI_Message_Id = DXGI_Message_Id(134i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForCoreWindow_InvalidSwapEffect: DXGI_Message_Id = DXGI_Message_Id(190i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForHwnd_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(278i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainOrRegisterOcclusionStatus_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = DXGI_Message_Id(208i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_10BitFormatNotSupported: DXGI_Message_Id = DXGI_Message_Id(272i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AllowTearingFlagIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(286i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(184i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsWindowlessOnly: DXGI_Message_Id = DXGI_Message_Id(183i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaUnrecognized: DXGI_Message_Id = DXGI_Message_Id(182i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_BufferCountOOBForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(100i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = DXGI_Message_Id(143i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyOnLegacy: DXGI_Message_Id = DXGI_Message_Id(186i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(144i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FSUnsupportedForModernApps: DXGI_Message_Id = DXGI_Message_Id(166i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FailedToGoFSButNonPreRotated: DXGI_Message_Id = DXGI_Message_Id(207i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSequentialNotSupportedOnD3D10: DXGI_Message_Id = DXGI_Message_Id(99i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSwapEffectRequired: DXGI_Message_Id = DXGI_Message_Id(273i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ForegroundIsCoreWindowOnly: DXGI_Message_Id = DXGI_Message_Id(219i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_HwProtectUnsupported: DXGI_Message_Id = DXGI_Message_Id(264i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidDevice: DXGI_Message_Id = DXGI_Message_Id(274i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(33i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFormatForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(101i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidHwProtect: DXGI_Message_Id = DXGI_Message_Id(263i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidQueue: DXGI_Message_Id = DXGI_Message_Id(276i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_LegacyBltModelSwapEffect: DXGI_Message_Id = DXGI_Message_Id(294i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultiSamplingNotSupportedForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(102i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultipleSwapchainRefToSurface_DeferredDtr: DXGI_Message_Id = DXGI_Message_Id(297i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = DXGI_Message_Id(86i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedFlagAndWindowed: DXGI_Message_Id = DXGI_Message_Id(34i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NullDeviceInterface: DXGI_Message_Id = DXGI_Message_Id(35i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictOutputNotSupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(119i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictToOutputAdapterMismatch: DXGI_Message_Id = DXGI_Message_Id(185i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(141i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneRequiresWindows8OrNewer: DXGI_Message_Id = DXGI_Message_Id(175i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingUnrecognized: DXGI_Message_Id = DXGI_Message_Id(142i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ShaderInputUnsupported_YUV: DXGI_Message_Id = DXGI_Message_Id(254i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(124i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnknownSwapEffect: DXGI_Message_Id = DXGI_Message_Id(32i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnsupportedBufferUsageFlags: DXGI_Message_Id = DXGI_Message_Id(114i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(210i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreNotFullscreen: DXGI_Message_Id = DXGI_Message_Id(211i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(31i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDeviceHasMismatchedDXGIFactory: DXGI_Message_Id = DXGI_Message_Id(97i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pRestrictToOutputFromOtherIDXGIFactory: DXGI_Message_Id = DXGI_Message_Id(118i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ppSwapChainIsNULL: DXGI_Message_Id = DXGI_Message_Id(30i32);
pub const DXGI_MSG_IDXGIFactory_Creation_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(76i32);
pub const DXGI_MSG_IDXGIFactory_EnumAdapters_ppAdapterInterfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(29i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidLUID: DXGI_Message_Id = DXGI_Message_Id(198i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidResource: DXGI_Message_Id = DXGI_Message_Id(197i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(199i32);
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(126i32);
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_phWndIsNULL: DXGI_Message_Id = DXGI_Message_Id(36i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(37i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_ModernApp: DXGI_Message_Id = DXGI_Message_Id(167i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_NoOpBehavior: DXGI_Message_Id = DXGI_Message_Id(298i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(125i32);
pub const DXGI_MSG_IDXGIFactory_Release_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(83i32);
pub const DXGI_MSG_IDXGIObject_GetPrivateData_puiDataSizeIsNULL: DXGI_Message_Id = DXGI_Message_Id(64i32);
pub const DXGI_MSG_IDXGIOutput1_DuplicateOutput_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(194i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_2DOnly: DXGI_Message_Id = DXGI_Message_Id(200i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_MappedOrOfferedResource: DXGI_Message_Id = DXGI_Message_Id(205i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NeedCPUAccessWrite: DXGI_Message_Id = DXGI_Message_Id(202i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NoShared: DXGI_Message_Id = DXGI_Message_Id(203i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_OnlyMipLevels1: DXGI_Message_Id = DXGI_Message_Id(204i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_StagingOnly: DXGI_Message_Id = DXGI_Message_Id(201i32);
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(256i32);
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(255i32);
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(260i32);
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(259i32);
pub const DXGI_MSG_IDXGIOutput6_CheckHardwareCompositionSupport_NullPointer: DXGI_Message_Id = DXGI_Message_Id(289i32);
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput1_PerMonitorDpiRequired: DXGI_Message_Id = DXGI_Message_Id(292i32);
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput_PerMonitorDpiShimApplied: DXGI_Message_Id = DXGI_Message_Id(291i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(69i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeFormatAndDeviceCombination: DXGI_Message_Id = DXGI_Message_Id(75i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScaling: DXGI_Message_Id = DXGI_Message_Id(74i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScanlineOrdering: DXGI_Message_Id = DXGI_Message_Id(73i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasInvalidWidthOrHeight: DXGI_Message_Id = DXGI_Message_Id(46i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasRefreshRateDenominatorZero: DXGI_Message_Id = DXGI_Message_Id(71i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(62i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(95i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_UnknownFormatIsInvalidForConfiguration: DXGI_Message_Id = DXGI_Message_Id(72i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_pModeToMatchOrpClosestMatchIsNULL: DXGI_Message_Id = DXGI_Message_Id(70i32);
pub const DXGI_MSG_IDXGIOutput_GetCammaControlCapabilities_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(47i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(61i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(96i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_pNumModesIsNULL: DXGI_Message_Id = DXGI_Message_Id(45i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_ArraySizeMismatch: DXGI_Message_Id = DXGI_Message_Id(180i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_InvalidTargetSurfaceFormat: DXGI_Message_Id = DXGI_Message_Id(67i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_MapOfDestinationFailed: DXGI_Message_Id = DXGI_Message_Id(51i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(49i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_pDestinationIsNULL: DXGI_Message_Id = DXGI_Message_Id(50i32);
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(52i32);
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = DXGI_Message_Id(53i32);
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoGammaControls: DXGI_Message_Id = DXGI_Message_Id(56i32);
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(55i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_IDXGIResourceNotSupportedBypPrimary: DXGI_Message_Id = DXGI_Message_Id(57i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_ModernApp: DXGI_Message_Id = DXGI_Message_Id(170i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(59i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_pPrimaryIsInvalid: DXGI_Message_Id = DXGI_Message_Id(58i32);
pub const DXGI_MSG_IDXGIOutput_SetGammaControl_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(54i32);
pub const DXGI_MSG_IDXGIOutput_SetOrGetGammaControl_pArrayIsNULL: DXGI_Message_Id = DXGI_Message_Id(81i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_FailedToAcquireFullscreenMutex: DXGI_Message_Id = DXGI_Message_Id(27i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_ModernApp: DXGI_Message_Id = DXGI_Message_Id(171i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(60i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(94i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_Unsupported: DXGI_Message_Id = DXGI_Message_Id(275i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_pDeviceIsNULL: DXGI_Message_Id = DXGI_Message_Id(48i32);
pub const DXGI_MSG_IDXGIResource1_CreateSharedHandle_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(191i32);
pub const DXGI_MSG_IDXGIResource1_CreateSubresourceSurface_InvalidIndex: DXGI_Message_Id = DXGI_Message_Id(188i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_GDICompatibleFlagNotSet: DXGI_Message_Id = DXGI_Message_Id(89i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_ModernApp: DXGI_Message_Id = DXGI_Message_Id(174i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_SurfaceNotTexture2D: DXGI_Message_Id = DXGI_Message_Id(88i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(90i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_pHdcIsNULL: DXGI_Message_Id = DXGI_Message_Id(87i32);
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_GetDCNotCalled: DXGI_Message_Id = DXGI_Message_Id(92i32);
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_InvalidRectangleDimensions: DXGI_Message_Id = DXGI_Message_Id(93i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardAndReadFlagSet: DXGI_Message_Id = DXGI_Message_Id(40i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardButNotWriteFlagSet: DXGI_Message_Id = DXGI_Message_Id(41i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardFlagSetButCPUAccessIsNotDynamic: DXGI_Message_Id = DXGI_Message_Id(44i32);
pub const DXGI_MSG_IDXGISurface_Map_FlagsSetToZero: DXGI_Message_Id = DXGI_Message_Id(39i32);
pub const DXGI_MSG_IDXGISurface_Map_InvalidSurface: DXGI_Message_Id = DXGI_Message_Id(38i32);
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess: DXGI_Message_Id = DXGI_Message_Id(42i32);
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess2: DXGI_Message_Id = DXGI_Message_Id(91i32);
pub const DXGI_MSG_IDXGISurface_Map_ReadFlagSetButCPUAccessIsDynamic: DXGI_Message_Id = DXGI_Message_Id(43i32);
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(158i32);
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(153i32);
pub const DXGI_MSG_IDXGISwapChain1_SetBackgroundColor_OutOfRange: DXGI_Message_Id = DXGI_Message_Id(148i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(156i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_InvalidRotation: DXGI_Message_Id = DXGI_Message_Id(157i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(152i32);
pub const DXGI_MSG_IDXGISwapChain3_CheckColorSpaceSupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(261i32);
pub const DXGI_MSG_IDXGISwapChain3_ResizeBuffers1_InvalidQueue: DXGI_Message_Id = DXGI_Message_Id(277i32);
pub const DXGI_MSG_IDXGISwapChain3_SetColorSpace1_InvalidColorSpace: DXGI_Message_Id = DXGI_Message_Id(262i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(280i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(279i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidType: DXGI_Message_Id = DXGI_Message_Id(281i32);
pub const DXGI_MSG_IDXGISwapChain4_SetHDRMetaData_MetadataUnchanged: DXGI_Message_Id = DXGI_Message_Id(295i32);
pub const DXGI_MSG_IDXGISwapChain_CreateSwapChain_InvalidHwProtectGdiFlag: DXGI_Message_Id = DXGI_Message_Id(270i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferHeightInferred: DXGI_Message_Id = DXGI_Message_Id(2i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferWidthInferred: DXGI_Message_Id = DXGI_Message_Id(1i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_InvalidOutputWindow: DXGI_Message_Id = DXGI_Message_Id(0i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_NoScanoutFlagChanged: DXGI_Message_Id = DXGI_Message_Id(3i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_FSUnsupportedForFlipDiscard: DXGI_Message_Id = DXGI_Message_Id(258i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_StereoDisabled: DXGI_Message_Id = DXGI_Message_Id(128i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidOutputWindow: DXGI_Message_Id = DXGI_Message_Id(65i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidWindowStyle: DXGI_Message_Id = DXGI_Message_Id(78i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_MaxBufferCountExceeded: DXGI_Message_Id = DXGI_Message_Id(4i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_NoOutputWindow: DXGI_Message_Id = DXGI_Message_Id(6i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_TooFewBuffers: DXGI_Message_Id = DXGI_Message_Id(5i32);
pub const DXGI_MSG_IDXGISwapChain_Destruction_OtherMethodsCalled: DXGI_Message_Id = DXGI_Message_Id(7i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_NoAllocatedBuffers: DXGI_Message_Id = DXGI_Message_Id(10i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferMustBeZero: DXGI_Message_Id = DXGI_Message_Id(11i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferOOB: DXGI_Message_Id = DXGI_Message_Id(12i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_ppSurfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(9i32);
pub const DXGI_MSG_IDXGISwapChain_GetCompositionSurface_WrongType: DXGI_Message_Id = DXGI_Message_Id(160i32);
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_SwapchainAdapterDoesNotControlOutput: DXGI_Message_Id = DXGI_Message_Id(80i32);
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_ppOutputIsNULL: DXGI_Message_Id = DXGI_Message_Id(13i32);
pub const DXGI_MSG_IDXGISwapChain_GetCoreWindow_WrongType: DXGI_Message_Id = DXGI_Message_Id(161i32);
pub const DXGI_MSG_IDXGISwapChain_GetDesc_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(8i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameLatencyWaitableObject_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(214i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_UnsupportedStatistics: DXGI_Message_Id = DXGI_Message_Id(79i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = DXGI_Message_Id(24i32);
pub const DXGI_MSG_IDXGISwapChain_GetFullscreenDesc_NonHwnd: DXGI_Message_Id = DXGI_Message_Id(162i32);
pub const DXGI_MSG_IDXGISwapChain_GetHwnd_WrongType: DXGI_Message_Id = DXGI_Message_Id(159i32);
pub const DXGI_MSG_IDXGISwapChain_GetLastPresentCount_pLastPresentCountIsNULL: DXGI_Message_Id = DXGI_Message_Id(25i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = DXGI_Message_Id(228i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(229i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(250i32);
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(215i32);
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_pMaxLatencyIsNULL: DXGI_Message_Id = DXGI_Message_Id(216i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(238i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_NullPointers: DXGI_Message_Id = DXGI_Message_Id(237i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(248i32);
pub const DXGI_MSG_IDXGISwapChain_PresentBuffer_YUV: DXGI_Message_Id = DXGI_Message_Id(246i32);
pub const DXGI_MSG_IDXGISwapChain_Present_11On12_Released_Resource: DXGI_Message_Id = DXGI_Message_Id(296i32);
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresCreationFlag: DXGI_Message_Id = DXGI_Message_Id(284i32);
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresPresentIntervalZero: DXGI_Message_Id = DXGI_Message_Id(283i32);
pub const DXGI_MSG_IDXGISwapChain_Present_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = DXGI_Message_Id(209i32);
pub const DXGI_MSG_IDXGISwapChain_Present_Decode: DXGI_Message_Id = DXGI_Message_Id(232i32);
pub const DXGI_MSG_IDXGISwapChain_Present_DirtyRectOutOfBackbufferBounds: DXGI_Message_Id = DXGI_Message_Id(113i32);
pub const DXGI_MSG_IDXGISwapChain_Present_DoNotSequenceFlagSetButPreviousBufferIsUndefined: DXGI_Message_Id = DXGI_Message_Id(115i32);
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyDirtyRect: DXGI_Message_Id = DXGI_Message_Id(112i32);
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyScrollRect: DXGI_Message_Id = DXGI_Message_Id(109i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FlipModelChainMustResizeOrCreateOnFSTransition: DXGI_Message_Id = DXGI_Message_Id(117i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenAllowTearingIsInvalid: DXGI_Message_Id = DXGI_Message_Id(282i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenPartialPresentIsInvalid: DXGI_Message_Id = DXGI_Message_Id(106i32);
pub const DXGI_MSG_IDXGISwapChain_Present_GetDXGIAdapterFailed: DXGI_Message_Id = DXGI_Message_Id(17i32);
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidNonPreRotatedFlag: DXGI_Message_Id = DXGI_Message_Id(15i32);
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidPresentTestOrDoNotSequenceFlag: DXGI_Message_Id = DXGI_Message_Id(107i32);
pub const DXGI_MSG_IDXGISwapChain_Present_NoAllocatedBuffers: DXGI_Message_Id = DXGI_Message_Id(16i32);
pub const DXGI_MSG_IDXGISwapChain_Present_NonOptimalFSConfiguration: DXGI_Message_Id = DXGI_Message_Id(98i32);
pub const DXGI_MSG_IDXGISwapChain_Present_OtherFlagsCausingInvalidPresentTestFlag: DXGI_Message_Id = DXGI_Message_Id(123i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationBeforeStandardPresentation: DXGI_Message_Id = DXGI_Message_Id(105i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithMSAABuffers: DXGI_Message_Id = DXGI_Message_Id(155i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithSwapEffectDiscard: DXGI_Message_Id = DXGI_Message_Id(181i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentation_YUV: DXGI_Message_Id = DXGI_Message_Id(251i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity: DXGI_Message_Id = DXGI_Message_Id(133i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFSOrOverlay: DXGI_Message_Id = DXGI_Message_Id(130i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFlipSequential: DXGI_Message_Id = DXGI_Message_Id(131i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentWithRDPDriver: DXGI_Message_Id = DXGI_Message_Id(132i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedWindowlessPresentationRequiresDisplayOnly: DXGI_Message_Id = DXGI_Message_Id(146i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestartIsFullscreenOnly: DXGI_Message_Id = DXGI_Message_Id(145i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictOutputFlagWithStaleSwapChain: DXGI_Message_Id = DXGI_Message_Id(122i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagSetButInvalidpRestrictToOutput: DXGI_Message_Id = DXGI_Message_Id(120i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagdWithFullscreen: DXGI_Message_Id = DXGI_Message_Id(121i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollInfoWithNoDirtyRectsSpecified: DXGI_Message_Id = DXGI_Message_Id(108i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBounds: DXGI_Message_Id = DXGI_Message_Id(110i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBoundsWithOffset: DXGI_Message_Id = DXGI_Message_Id(111i32);
pub const DXGI_MSG_IDXGISwapChain_Present_SyncIntervalOOB: DXGI_Message_Id = DXGI_Message_Id(14i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoAndPreferRight: DXGI_Message_Id = DXGI_Message_Id(176i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithDoNotSequence: DXGI_Message_Id = DXGI_Message_Id(177i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithoutStereo: DXGI_Message_Id = DXGI_Message_Id(178i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoUnsupported: DXGI_Message_Id = DXGI_Message_Id(179i32);
pub const DXGI_MSG_IDXGISwapChain_Present_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(84i32);
pub const DXGI_MSG_IDXGISwapChain_Present_UnsupportedFlags: DXGI_Message_Id = DXGI_Message_Id(116i32);
pub const DXGI_MSG_IDXGISwapChain_Release_SwapChainIsFullscreen: DXGI_Message_Id = DXGI_Message_Id(66i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_D3D12Only: DXGI_Message_Id = DXGI_Message_Id(267i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_FlipModel: DXGI_Message_Id = DXGI_Message_Id(268i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_NodeMaskAndQueueRequired: DXGI_Message_Id = DXGI_Message_Id(269i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Alignment_YUV: DXGI_Message_Id = DXGI_Message_Id(253i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOB: DXGI_Message_Id = DXGI_Message_Id(18i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOBForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(103i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveAllowTearingFlag: DXGI_Message_Id = DXGI_Message_Id(285i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveFlag_YUV: DXGI_Message_Id = DXGI_Message_Id(252i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveForegroundFlag: DXGI_Message_Id = DXGI_Message_Id(223i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveWaitableFlag: DXGI_Message_Id = DXGI_Message_Id(213i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Decode: DXGI_Message_Id = DXGI_Message_Id(233i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = DXGI_Message_Id(149i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyOnLegacy: DXGI_Message_Id = DXGI_Message_Id(187i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(150i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_HwProtectUnsupported: DXGI_Message_Id = DXGI_Message_Id(266i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidFormatForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(104i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtect: DXGI_Message_Id = DXGI_Message_Id(265i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtectGdiFlag: DXGI_Message_Id = DXGI_Message_Id(271i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidNonPreRotatedFlag: DXGI_Message_Id = DXGI_Message_Id(21i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidSwapChainFlag: DXGI_Message_Id = DXGI_Message_Id(20i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = DXGI_Message_Id(85i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_UnreleasedReferences: DXGI_Message_Id = DXGI_Message_Id(19i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_WidthOrHeightIsZero: DXGI_Message_Id = DXGI_Message_Id(140i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_InvalidWithCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(139i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_ModernApp: DXGI_Message_Id = DXGI_Message_Id(168i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_RefreshRateDivideByZero: DXGI_Message_Id = DXGI_Message_Id(22i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_pNewTargetParametersIsNULL: DXGI_Message_Id = DXGI_Message_Id(169i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_CoreWindow: DXGI_Message_Id = DXGI_Message_Id(163i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(147i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSTransitionWithCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(138i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSUnsupportedForModernApps: DXGI_Message_Id = DXGI_Message_Id(206i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FullscreenInvalidForChildWindows: DXGI_Message_Id = DXGI_Message_Id(82i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_InvalidTarget: DXGI_Message_Id = DXGI_Message_Id(23i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_OutputNotOwnedBySwapChainDevice: DXGI_Message_Id = DXGI_Message_Id(77i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_PerMonitorDpiShimApplied: DXGI_Message_Id = DXGI_Message_Id(290i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_RemoteNotSupported: DXGI_Message_Id = DXGI_Message_Id(26i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_Waitable: DXGI_Message_Id = DXGI_Message_Id(212i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeFinite: DXGI_Message_Id = DXGI_Message_Id(226i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeTranslateAndOrScale: DXGI_Message_Id = DXGI_Message_Id(227i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = DXGI_Message_Id(224i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(225i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(249i32);
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_MaxLatencyIsOutOfBounds: DXGI_Message_Id = DXGI_Message_Id(218i32);
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(217i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(235i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_FlipModel: DXGI_Message_Id = DXGI_Message_Id(234i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_WidthHeight: DXGI_Message_Id = DXGI_Message_Id(236i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(247i32);
pub const DXGI_MSG_IDXGISwapchain_Present_FullscreenRotation: DXGI_Message_Id = DXGI_Message_Id(154i32);
pub const DXGI_MSG_IDXGISwapchain_Present_ScrollUnsupported: DXGI_Message_Id = DXGI_Message_Id(151i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_DISCARD_BufferCount: DXGI_Message_Id = DXGI_Message_Id(1001i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_Modern_CoreWindow_Only: DXGI_Message_Id = DXGI_Message_Id(1028i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_SEQUENTIAL_BufferCount: DXGI_Message_Id = DXGI_Message_Id(1027i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FailedRegisterWithCompositor: DXGI_Message_Id = DXGI_Message_Id(1025i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_MSAA_NotSupported: DXGI_Message_Id = DXGI_Message_Id(1021i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow: DXGI_Message_Id = DXGI_Message_Id(1000i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow_AtRendering: DXGI_Message_Id = DXGI_Message_Id(1026i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_ScalingAspectRatioStretch_Supported_ModernApp: DXGI_Message_Id = DXGI_Message_Id(1022i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_GetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(1031i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_GetFrameStatistics_NotAvailable_ModernApp: DXGI_Message_Id = DXGI_Message_Id(1023i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present1_RequiresOverlays: DXGI_Message_Id = DXGI_Message_Id(1029i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidBlend: DXGI_Message_Id = DXGI_Message_Id(1009i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidDestinationRect: DXGI_Message_Id = DXGI_Message_Id(1016i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidFlag: DXGI_Message_Id = DXGI_Message_Id(1019i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForOverlay: DXGI_Message_Id = DXGI_Message_Id(1013i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForPrimary: DXGI_Message_Id = DXGI_Message_Id(1012i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidInterval: DXGI_Message_Id = DXGI_Message_Id(1020i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerFlag: DXGI_Message_Id = DXGI_Message_Id(1007i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerIndex: DXGI_Message_Id = DXGI_Message_Id(1005i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidMultiPlaneOverlayResource: DXGI_Message_Id = DXGI_Message_Id(1011i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidResource: DXGI_Message_Id = DXGI_Message_Id(1010i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidRotation: DXGI_Message_Id = DXGI_Message_Id(1008i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSourceRect: DXGI_Message_Id = DXGI_Message_Id(1015i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSubResourceIndex: DXGI_Message_Id = DXGI_Message_Id(1014i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleLayerIndex: DXGI_Message_Id = DXGI_Message_Id(1006i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleResource: DXGI_Message_Id = DXGI_Message_Id(1017i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_NotSharedResource: DXGI_Message_Id = DXGI_Message_Id(1018i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_ReplaceInterval0With1: DXGI_Message_Id = DXGI_Message_Id(1024i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeBuffers_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1003i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeTarget_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1004i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_SetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(1030i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_SetFullscreenState_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1002i32);
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(2i32);
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(1i32);
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(4i32);
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2u32;
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4u32;
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1u32;
pub const DXGI_MWA_VALID: u32 = 7u32;
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = DXGI_OFFER_RESOURCE_FLAGS(1i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(3i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(1i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(2i32);
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = DXGI_OUTDUPL_FLAG(1i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(2i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(4i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(1i32);
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(1i32);
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG = DXGI_OVERLAY_SUPPORT_FLAG(1i32);
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG = DXGI_OVERLAY_SUPPORT_FLAG(2i32);
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512u32;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2u32;
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8u32;
pub const DXGI_PRESENT_RESTART: u32 = 4u32;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64u32;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16u32;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32u32;
pub const DXGI_PRESENT_TEST: u32 = 1u32;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256u32;
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(1i32);
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(2i32);
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(0i32);
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = DXGI_RESIDENCY(3i32);
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = DXGI_RESIDENCY(1i32);
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = DXGI_RESIDENCY(2i32);
pub const DXGI_RESOURCE_PRIORITY_HIGH: u32 = 2684354560u32;
pub const DXGI_RESOURCE_PRIORITY_LOW: u32 = 1342177280u32;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200u32;
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640u32;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920u32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = DXGI_SCALING(2i32);
pub const DXGI_SCALING_NONE: DXGI_SCALING = DXGI_SCALING(1i32);
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = DXGI_SCALING(0i32);
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648u32;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1u32;
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(2i32);
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(1i32);
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2i32);
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2048i32);
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(32i32);
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(128i32);
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(64i32);
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(256i32);
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4i32);
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1024i32);
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(8i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4096i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(16i32);
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(512i32);
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(0i32);
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(4i32);
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(3i32);
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(1i32);
pub const DXGI_USAGE_BACK_BUFFER: DXGI_USAGE = DXGI_USAGE(64u32);
pub const DXGI_USAGE_DISCARD_ON_PRESENT: DXGI_USAGE = DXGI_USAGE(512u32);
pub const DXGI_USAGE_READ_ONLY: DXGI_USAGE = DXGI_USAGE(256u32);
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: DXGI_USAGE = DXGI_USAGE(32u32);
pub const DXGI_USAGE_SHADER_INPUT: DXGI_USAGE = DXGI_USAGE(16u32);
pub const DXGI_USAGE_SHARED: DXGI_USAGE = DXGI_USAGE(128u32);
pub const DXGI_USAGE_UNORDERED_ACCESS: DXGI_USAGE = DXGI_USAGE(1024u32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_ADAPTER_FLAG(pub i32);
impl ::windows_core::TypeKind for DXGI_ADAPTER_FLAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_ADAPTER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ADAPTER_FLAG").field(&self.0).finish()
    }
}
impl DXGI_ADAPTER_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ADAPTER_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ADAPTER_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_ADAPTER_FLAG3(pub i32);
impl ::windows_core::TypeKind for DXGI_ADAPTER_FLAG3 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_ADAPTER_FLAG3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ADAPTER_FLAG3").field(&self.0).finish()
    }
}
impl DXGI_ADAPTER_FLAG3 {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ADAPTER_FLAG3 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ADAPTER_FLAG3 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_COMPUTE_PREEMPTION_GRANULARITY(pub i32);
impl ::windows_core::TypeKind for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_COMPUTE_PREEMPTION_GRANULARITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_DEBUG_RLO_FLAGS(pub i32);
impl ::windows_core::TypeKind for DXGI_DEBUG_RLO_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_DEBUG_RLO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_DEBUG_RLO_FLAGS").field(&self.0).finish()
    }
}
impl DXGI_DEBUG_RLO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_FEATURE(pub i32);
impl ::windows_core::TypeKind for DXGI_FEATURE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FEATURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_FRAME_PRESENTATION_MODE(pub i32);
impl ::windows_core::TypeKind for DXGI_FRAME_PRESENTATION_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_FRAME_PRESENTATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FRAME_PRESENTATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_GPU_PREFERENCE(pub i32);
impl ::windows_core::TypeKind for DXGI_GPU_PREFERENCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_GPU_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_GPU_PREFERENCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_GRAPHICS_PREEMPTION_GRANULARITY(pub i32);
impl ::windows_core::TypeKind for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_GRAPHICS_PREEMPTION_GRANULARITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(pub i32);
impl ::windows_core::TypeKind for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_HDR_METADATA_TYPE(pub i32);
impl ::windows_core::TypeKind for DXGI_HDR_METADATA_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_HDR_METADATA_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_INFO_QUEUE_MESSAGE_CATEGORY(pub i32);
impl ::windows_core::TypeKind for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_INFO_QUEUE_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_INFO_QUEUE_MESSAGE_SEVERITY(pub i32);
impl ::windows_core::TypeKind for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_INFO_QUEUE_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_MEMORY_SEGMENT_GROUP(pub i32);
impl ::windows_core::TypeKind for DXGI_MEMORY_SEGMENT_GROUP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_MEMORY_SEGMENT_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MEMORY_SEGMENT_GROUP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(pub i32);
impl ::windows_core::TypeKind for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_Message_Id(pub i32);
impl ::windows_core::TypeKind for DXGI_Message_Id {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_Message_Id {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_Message_Id").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_OFFER_RESOURCE_FLAGS(pub i32);
impl ::windows_core::TypeKind for DXGI_OFFER_RESOURCE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OFFER_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OFFER_RESOURCE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_OFFER_RESOURCE_PRIORITY(pub i32);
impl ::windows_core::TypeKind for DXGI_OFFER_RESOURCE_PRIORITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OFFER_RESOURCE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OFFER_RESOURCE_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_OUTDUPL_FLAG(pub i32);
impl ::windows_core::TypeKind for DXGI_OUTDUPL_FLAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OUTDUPL_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_TYPE(pub i32);
impl ::windows_core::TypeKind for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OUTDUPL_POINTER_SHAPE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(pub i32);
impl ::windows_core::TypeKind for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_OVERLAY_SUPPORT_FLAG(pub i32);
impl ::windows_core::TypeKind for DXGI_OVERLAY_SUPPORT_FLAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_OVERLAY_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OVERLAY_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_RECLAIM_RESOURCE_RESULTS(pub i32);
impl ::windows_core::TypeKind for DXGI_RECLAIM_RESOURCE_RESULTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RECLAIM_RESOURCE_RESULTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_RESIDENCY(pub i32);
impl ::windows_core::TypeKind for DXGI_RESIDENCY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_RESIDENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RESIDENCY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_SCALING(pub i32);
impl ::windows_core::TypeKind for DXGI_SCALING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_SCALING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SCALING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(pub i32);
impl ::windows_core::TypeKind for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_SWAP_CHAIN_FLAG(pub i32);
impl ::windows_core::TypeKind for DXGI_SWAP_CHAIN_FLAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_CHAIN_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_SWAP_EFFECT(pub i32);
impl ::windows_core::TypeKind for DXGI_SWAP_EFFECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_SWAP_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_EFFECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DXGI_USAGE(pub u32);
impl ::windows_core::TypeKind for DXGI_USAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DXGI_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_USAGE").field(&self.0).finish()
    }
}
impl DXGI_USAGE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DXGI_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
}
impl ::core::marker::Copy for DXGI_ADAPTER_DESC {}
impl ::core::clone::Clone for DXGI_ADAPTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC").field("Description", &self.Description).field("VendorId", &self.VendorId).field("DeviceId", &self.DeviceId).field("SubSysId", &self.SubSysId).field("Revision", &self.Revision).field("DedicatedVideoMemory", &self.DedicatedVideoMemory).field("DedicatedSystemMemory", &self.DedicatedSystemMemory).field("SharedSystemMemory", &self.SharedSystemMemory).field("AdapterLuid", &self.AdapterLuid).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_ADAPTER_DESC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid
    }
}
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC {}
impl ::core::default::Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
}
impl ::core::marker::Copy for DXGI_ADAPTER_DESC1 {}
impl ::core::clone::Clone for DXGI_ADAPTER_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC1").field("Description", &self.Description).field("VendorId", &self.VendorId).field("DeviceId", &self.DeviceId).field("SubSysId", &self.SubSysId).field("Revision", &self.Revision).field("DedicatedVideoMemory", &self.DedicatedVideoMemory).field("DedicatedSystemMemory", &self.DedicatedSystemMemory).field("SharedSystemMemory", &self.SharedSystemMemory).field("AdapterLuid", &self.AdapterLuid).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_ADAPTER_DESC1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC1 {}
impl ::core::default::Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
impl ::core::marker::Copy for DXGI_ADAPTER_DESC2 {}
impl ::core::clone::Clone for DXGI_ADAPTER_DESC2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC2")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .field("Flags", &self.Flags)
            .field("GraphicsPreemptionGranularity", &self.GraphicsPreemptionGranularity)
            .field("ComputePreemptionGranularity", &self.ComputePreemptionGranularity)
            .finish()
    }
}
impl ::windows_core::TypeKind for DXGI_ADAPTER_DESC2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC2 {}
impl ::core::default::Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
impl ::core::marker::Copy for DXGI_ADAPTER_DESC3 {}
impl ::core::clone::Clone for DXGI_ADAPTER_DESC3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC3")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .field("Flags", &self.Flags)
            .field("GraphicsPreemptionGranularity", &self.GraphicsPreemptionGranularity)
            .field("ComputePreemptionGranularity", &self.ComputePreemptionGranularity)
            .finish()
    }
}
impl ::windows_core::TypeKind for DXGI_ADAPTER_DESC3 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC3 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC3 {}
impl ::core::default::Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
impl ::core::marker::Copy for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::clone::Clone for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_DECODE_SWAP_CHAIN_DESC").field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_DECODE_SWAP_CHAIN_DESC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::default::Default for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [f32; 16],
    pub WhitePoints: [f32; 32],
}
impl ::core::marker::Copy for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::clone::Clone for DXGI_DISPLAY_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_DISPLAY_COLOR_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_DISPLAY_COLOR_SPACE").field("PrimaryCoordinates", &self.PrimaryCoordinates).field("WhitePoints", &self.WhitePoints).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_DISPLAY_COLOR_SPACE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_DISPLAY_COLOR_SPACE {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryCoordinates == other.PrimaryCoordinates && self.WhitePoints == other.WhitePoints
    }
}
impl ::core::cmp::Eq for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::default::Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_FRAME_STATISTICS").field("PresentCount", &self.PresentCount).field("PresentRefreshCount", &self.PresentRefreshCount).field("SyncRefreshCount", &self.SyncRefreshCount).field("SyncQPCTime", &self.SyncQPCTime).field("SyncGPUTime", &self.SyncGPUTime).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_FRAME_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount && self.PresentRefreshCount == other.PresentRefreshCount && self.SyncRefreshCount == other.SyncRefreshCount && self.SyncQPCTime == other.SyncQPCTime && self.SyncGPUTime == other.SyncGPUTime
    }
}
impl ::core::cmp::Eq for DXGI_FRAME_STATISTICS {}
impl ::core::default::Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS_MEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_FRAME_STATISTICS_MEDIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_FRAME_STATISTICS_MEDIA").field("PresentCount", &self.PresentCount).field("PresentRefreshCount", &self.PresentRefreshCount).field("SyncRefreshCount", &self.SyncRefreshCount).field("SyncQPCTime", &self.SyncQPCTime).field("SyncGPUTime", &self.SyncGPUTime).field("CompositionMode", &self.CompositionMode).field("ApprovedPresentDuration", &self.ApprovedPresentDuration).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_FRAME_STATISTICS_MEDIA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_FRAME_STATISTICS_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount && self.PresentRefreshCount == other.PresentRefreshCount && self.SyncRefreshCount == other.SyncRefreshCount && self.SyncQPCTime == other.SyncQPCTime && self.SyncGPUTime == other.SyncGPUTime && self.CompositionMode == other.CompositionMode && self.ApprovedPresentDuration == other.ApprovedPresentDuration
    }
}
impl ::core::cmp::Eq for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::default::Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10 {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_HDR10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_HDR_METADATA_HDR10").field("RedPrimary", &self.RedPrimary).field("GreenPrimary", &self.GreenPrimary).field("BluePrimary", &self.BluePrimary).field("WhitePoint", &self.WhitePoint).field("MaxMasteringLuminance", &self.MaxMasteringLuminance).field("MinMasteringLuminance", &self.MinMasteringLuminance).field("MaxContentLightLevel", &self.MaxContentLightLevel).field("MaxFrameAverageLightLevel", &self.MaxFrameAverageLightLevel).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_HDR_METADATA_HDR10 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_HDR_METADATA_HDR10 {
    fn eq(&self, other: &Self) -> bool {
        self.RedPrimary == other.RedPrimary && self.GreenPrimary == other.GreenPrimary && self.BluePrimary == other.BluePrimary && self.WhitePoint == other.WhitePoint && self.MaxMasteringLuminance == other.MaxMasteringLuminance && self.MinMasteringLuminance == other.MinMasteringLuminance && self.MaxContentLightLevel == other.MaxContentLightLevel && self.MaxFrameAverageLightLevel == other.MaxFrameAverageLightLevel
    }
}
impl ::core::cmp::Eq for DXGI_HDR_METADATA_HDR10 {}
impl ::core::default::Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10PLUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_HDR10PLUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_HDR_METADATA_HDR10PLUS").field("Data", &self.Data).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_HDR_METADATA_HDR10PLUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_HDR_METADATA_HDR10PLUS {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::default::Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_FILTER").field("AllowList", &self.AllowList).field("DenyList", &self.DenyList).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_INFO_QUEUE_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AllowList == other.AllowList && self.DenyList == other.DenyList
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_FILTER {}
impl ::core::default::Default for DXGI_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut i32,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_FILTER_DESC").field("NumCategories", &self.NumCategories).field("pCategoryList", &self.pCategoryList).field("NumSeverities", &self.NumSeverities).field("pSeverityList", &self.pSeverityList).field("NumIDs", &self.NumIDs).field("pIDList", &self.pIDList).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_INFO_QUEUE_FILTER_DESC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumCategories == other.NumCategories && self.pCategoryList == other.pCategoryList && self.NumSeverities == other.NumSeverities && self.pSeverityList == other.pSeverityList && self.NumIDs == other.NumIDs && self.pIDList == other.pIDList
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::default::Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: ::windows_core::GUID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: i32,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_MESSAGE").field("Producer", &self.Producer).field("Category", &self.Category).field("Severity", &self.Severity).field("ID", &self.ID).field("pDescription", &self.pDescription).field("DescriptionByteLength", &self.DescriptionByteLength).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_INFO_QUEUE_MESSAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Producer == other.Producer && self.Category == other.Category && self.Severity == other.Severity && self.ID == other.ID && self.pDescription == other.pDescription && self.DescriptionByteLength == other.DescriptionByteLength
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl ::core::marker::Copy for DXGI_MAPPED_RECT {}
impl ::core::clone::Clone for DXGI_MAPPED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_MAPPED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MAPPED_RECT").field("Pitch", &self.Pitch).field("pBits", &self.pBits).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_MAPPED_RECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_MAPPED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Pitch == other.Pitch && self.pBits == other.pBits
    }
}
impl ::core::cmp::Eq for DXGI_MAPPED_RECT {}
impl ::core::default::Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl ::core::marker::Copy for DXGI_MATRIX_3X2_F {}
impl ::core::clone::Clone for DXGI_MATRIX_3X2_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_MATRIX_3X2_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MATRIX_3X2_F").field("_11", &self._11).field("_12", &self._12).field("_21", &self._21).field("_22", &self._22).field("_31", &self._31).field("_32", &self._32).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_MATRIX_3X2_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_MATRIX_3X2_F {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._21 == other._21 && self._22 == other._22 && self._31 == other._31 && self._32 == other._32
    }
}
impl ::core::cmp::Eq for DXGI_MATRIX_3X2_F {}
impl ::core::default::Default for DXGI_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub Format: Common::DXGI_FORMAT,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Stereo: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_MODE_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_MODE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_MODE_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MODE_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).field("Stereo", &self.Stereo).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::TypeKind for DXGI_MODE_DESC1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_MODE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.RefreshRate == other.RefreshRate && self.Format == other.Format && self.ScanlineOrdering == other.ScanlineOrdering && self.Scaling == other.Scaling && self.Stereo == other.Stereo
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_MODE_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_MODE_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: Common::DXGI_MODE_DESC,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_OUTDUPL_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_OUTDUPL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_DESC").field("ModeDesc", &self.ModeDesc).field("Rotation", &self.Rotation).field("DesktopImageInSystemMemory", &self.DesktopImageInSystemMemory).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::TypeKind for DXGI_OUTDUPL_DESC {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ModeDesc == other.ModeDesc && self.Rotation == other.Rotation && self.DesktopImageInSystemMemory == other.DesktopImageInSystemMemory
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_OUTDUPL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: super::super::Foundation::BOOL,
    pub ProtectedContentMaskedOut: super::super::Foundation::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
impl ::core::marker::Copy for DXGI_OUTDUPL_FRAME_INFO {}
impl ::core::clone::Clone for DXGI_OUTDUPL_FRAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_FRAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_FRAME_INFO")
            .field("LastPresentTime", &self.LastPresentTime)
            .field("LastMouseUpdateTime", &self.LastMouseUpdateTime)
            .field("AccumulatedFrames", &self.AccumulatedFrames)
            .field("RectsCoalesced", &self.RectsCoalesced)
            .field("ProtectedContentMaskedOut", &self.ProtectedContentMaskedOut)
            .field("PointerPosition", &self.PointerPosition)
            .field("TotalMetadataBufferSize", &self.TotalMetadataBufferSize)
            .field("PointerShapeBufferSize", &self.PointerShapeBufferSize)
            .finish()
    }
}
impl ::windows_core::TypeKind for DXGI_OUTDUPL_FRAME_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastPresentTime == other.LastPresentTime && self.LastMouseUpdateTime == other.LastMouseUpdateTime && self.AccumulatedFrames == other.AccumulatedFrames && self.RectsCoalesced == other.RectsCoalesced && self.ProtectedContentMaskedOut == other.ProtectedContentMaskedOut && self.PointerPosition == other.PointerPosition && self.TotalMetadataBufferSize == other.TotalMetadataBufferSize && self.PointerShapeBufferSize == other.PointerShapeBufferSize
    }
}
impl ::core::cmp::Eq for DXGI_OUTDUPL_FRAME_INFO {}
impl ::core::default::Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::super::Foundation::POINT,
    pub DestinationRect: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for DXGI_OUTDUPL_MOVE_RECT {}
impl ::core::clone::Clone for DXGI_OUTDUPL_MOVE_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_MOVE_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_MOVE_RECT").field("SourcePoint", &self.SourcePoint).field("DestinationRect", &self.DestinationRect).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_OUTDUPL_MOVE_RECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_MOVE_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.SourcePoint == other.SourcePoint && self.DestinationRect == other.DestinationRect
    }
}
impl ::core::cmp::Eq for DXGI_OUTDUPL_MOVE_RECT {}
impl ::core::default::Default for DXGI_OUTDUPL_MOVE_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::super::Foundation::POINT,
    pub Visible: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_POSITION {}
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_POINTER_POSITION").field("Position", &self.Position).field("Visible", &self.Visible).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_OUTDUPL_POINTER_POSITION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Visible == other.Visible
    }
}
impl ::core::cmp::Eq for DXGI_OUTDUPL_POINTER_POSITION {}
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::super::Foundation::POINT,
}
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_POINTER_SHAPE_INFO").field("Type", &self.Type).field("Width", &self.Width).field("Height", &self.Height).field("Pitch", &self.Pitch).field("HotSpot", &self.HotSpot).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Width == other.Width && self.Height == other.Height && self.Pitch == other.Pitch && self.HotSpot == other.HotSpot
    }
}
impl ::core::cmp::Eq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DXGI_OUTPUT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTPUT_DESC").field("DeviceName", &self.DeviceName).field("DesktopCoordinates", &self.DesktopCoordinates).field("AttachedToDesktop", &self.AttachedToDesktop).field("Rotation", &self.Rotation).field("Monitor", &self.Monitor).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for DXGI_OUTPUT_DESC {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DXGI_OUTPUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DesktopCoordinates == other.DesktopCoordinates && self.AttachedToDesktop == other.AttachedToDesktop && self.Rotation == other.Rotation && self.Monitor == other.Monitor
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: Common::DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DXGI_OUTPUT_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTPUT_DESC1")
            .field("DeviceName", &self.DeviceName)
            .field("DesktopCoordinates", &self.DesktopCoordinates)
            .field("AttachedToDesktop", &self.AttachedToDesktop)
            .field("Rotation", &self.Rotation)
            .field("Monitor", &self.Monitor)
            .field("BitsPerColor", &self.BitsPerColor)
            .field("ColorSpace", &self.ColorSpace)
            .field("RedPrimary", &self.RedPrimary)
            .field("GreenPrimary", &self.GreenPrimary)
            .field("BluePrimary", &self.BluePrimary)
            .field("WhitePoint", &self.WhitePoint)
            .field("MinLuminance", &self.MinLuminance)
            .field("MaxLuminance", &self.MaxLuminance)
            .field("MaxFullFrameLuminance", &self.MaxFullFrameLuminance)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for DXGI_OUTPUT_DESC1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DXGI_OUTPUT_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DesktopCoordinates == other.DesktopCoordinates && self.AttachedToDesktop == other.AttachedToDesktop && self.Rotation == other.Rotation && self.Monitor == other.Monitor && self.BitsPerColor == other.BitsPerColor && self.ColorSpace == other.ColorSpace && self.RedPrimary == other.RedPrimary && self.GreenPrimary == other.GreenPrimary && self.BluePrimary == other.BluePrimary && self.WhitePoint == other.WhitePoint && self.MinLuminance == other.MinLuminance && self.MaxLuminance == other.MaxLuminance && self.MaxFullFrameLuminance == other.MaxFullFrameLuminance
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::super::Foundation::RECT,
    pub pScrollRect: *mut super::super::Foundation::RECT,
    pub pScrollOffset: *mut super::super::Foundation::POINT,
}
impl ::core::marker::Copy for DXGI_PRESENT_PARAMETERS {}
impl ::core::clone::Clone for DXGI_PRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_PRESENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_PRESENT_PARAMETERS").field("DirtyRectsCount", &self.DirtyRectsCount).field("pDirtyRects", &self.pDirtyRects).field("pScrollRect", &self.pScrollRect).field("pScrollOffset", &self.pScrollOffset).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_PRESENT_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_PRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.DirtyRectsCount == other.DirtyRectsCount && self.pDirtyRects == other.pDirtyRects && self.pScrollRect == other.pScrollRect && self.pScrollOffset == other.pScrollOffset
    }
}
impl ::core::cmp::Eq for DXGI_PRESENT_PARAMETERS {}
impl ::core::default::Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
impl ::core::marker::Copy for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::clone::Clone for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_QUERY_VIDEO_MEMORY_INFO").field("Budget", &self.Budget).field("CurrentUsage", &self.CurrentUsage).field("AvailableForReservation", &self.AvailableForReservation).field("CurrentReservation", &self.CurrentReservation).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_QUERY_VIDEO_MEMORY_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Budget == other.Budget && self.CurrentUsage == other.CurrentUsage && self.AvailableForReservation == other.AvailableForReservation && self.CurrentReservation == other.CurrentReservation
    }
}
impl ::core::cmp::Eq for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::default::Default for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for DXGI_RGBA {}
impl ::core::clone::Clone for DXGI_RGBA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_RGBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_RGBA").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_RGBA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for DXGI_RGBA {}
impl ::core::default::Default for DXGI_RGBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for DXGI_SHARED_RESOURCE {}
impl ::core::clone::Clone for DXGI_SHARED_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_SHARED_RESOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SHARED_RESOURCE").field("Handle", &self.Handle).finish()
    }
}
impl ::windows_core::TypeKind for DXGI_SHARED_RESOURCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DXGI_SHARED_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle
    }
}
impl ::core::cmp::Eq for DXGI_SHARED_RESOURCE {}
impl ::core::default::Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_SURFACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_SURFACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SURFACE_DESC").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::TypeKind for DXGI_SURFACE_DESC {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_SURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.SampleDesc == other.SampleDesc
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_SURFACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: Common::DXGI_MODE_DESC,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: super::super::Foundation::HWND,
    pub Windowed: super::super::Foundation::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_DESC").field("BufferDesc", &self.BufferDesc).field("SampleDesc", &self.SampleDesc).field("BufferUsage", &self.BufferUsage).field("BufferCount", &self.BufferCount).field("OutputWindow", &self.OutputWindow).field("Windowed", &self.Windowed).field("SwapEffect", &self.SwapEffect).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::TypeKind for DXGI_SWAP_CHAIN_DESC {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BufferDesc == other.BufferDesc && self.SampleDesc == other.SampleDesc && self.BufferUsage == other.BufferUsage && self.BufferCount == other.BufferCount && self.OutputWindow == other.OutputWindow && self.Windowed == other.Windowed && self.SwapEffect == other.SwapEffect && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub Stereo: super::super::Foundation::BOOL,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: Common::DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("Stereo", &self.Stereo).field("SampleDesc", &self.SampleDesc).field("BufferUsage", &self.BufferUsage).field("BufferCount", &self.BufferCount).field("Scaling", &self.Scaling).field("SwapEffect", &self.SwapEffect).field("AlphaMode", &self.AlphaMode).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::TypeKind for DXGI_SWAP_CHAIN_DESC1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.Stereo == other.Stereo && self.SampleDesc == other.SampleDesc && self.BufferUsage == other.BufferUsage && self.BufferCount == other.BufferCount && self.Scaling == other.Scaling && self.SwapEffect == other.SwapEffect && self.AlphaMode == other.AlphaMode && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_SWAP_CHAIN_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Windowed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_FULLSCREEN_DESC").field("RefreshRate", &self.RefreshRate).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).field("Windowed", &self.Windowed).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::TypeKind for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.RefreshRate == other.RefreshRate && self.ScanlineOrdering == other.ScanlineOrdering && self.Scaling == other.Scaling && self.Windowed == other.Windowed
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
