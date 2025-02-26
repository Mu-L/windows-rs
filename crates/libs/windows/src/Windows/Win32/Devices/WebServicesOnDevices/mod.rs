#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDAllocateLinkedMemory(pparent : *mut ::core::ffi::c_void, cbsize : usize) -> *mut ::core::ffi::c_void);
    WSDAllocateLinkedMemory(pparent, cbsize)
}
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void) {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDAttachLinkedMemory(pparent : *mut ::core::ffi::c_void, pchild : *mut ::core::ffi::c_void));
    WSDAttachLinkedMemory(pparent, pchild)
}
#[inline]
pub unsafe fn WSDCreateDeviceHost<P0, P1>(pszlocalid: P0, pcontext: P1) -> ::windows_core::Result<IWSDDeviceHost>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost(pszlocalid : ::windows_core::PCWSTR, pcontext : * mut::core::ffi::c_void, ppdevicehost : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDeviceHost(pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDeviceHost2<P0, P1>(pszlocalid: P0, pcontext: P1, pconfigparams: ::core::option::Option<&[WSD_CONFIG_PARAM]>) -> ::windows_core::Result<IWSDDeviceHost>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost2(pszlocalid : ::windows_core::PCWSTR, pcontext : * mut::core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdevicehost : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDeviceHost2(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(pconfigparams.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDeviceHostAdvanced<P0, P1>(pszlocalid: P0, pcontext: P1, pphostaddresses: ::core::option::Option<&[::core::option::Option<IWSDAddress>]>) -> ::windows_core::Result<IWSDDeviceHost>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDeviceHostAdvanced(pszlocalid : ::windows_core::PCWSTR, pcontext : * mut::core::ffi::c_void, pphostaddresses : *const * mut::core::ffi::c_void, dwhostaddresscount : u32, ppdevicehost : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDeviceHostAdvanced(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(pphostaddresses.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pphostaddresses.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDeviceProxy<P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2) -> ::windows_core::Result<IWSDDeviceProxy>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy(pszdeviceid : ::windows_core::PCWSTR, pszlocalid : ::windows_core::PCWSTR, pcontext : * mut::core::ffi::c_void, ppdeviceproxy : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDeviceProxy(pszdeviceid.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDeviceProxy2<P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2, pconfigparams: ::core::option::Option<&[WSD_CONFIG_PARAM]>) -> ::windows_core::Result<IWSDDeviceProxy>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy2(pszdeviceid : ::windows_core::PCWSTR, pszlocalid : ::windows_core::PCWSTR, pcontext : * mut::core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdeviceproxy : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDeviceProxy2(pszdeviceid.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(pconfigparams.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDeviceProxyAdvanced<P0, P1, P2, P3>(pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3) -> ::windows_core::Result<IWSDDeviceProxy>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IWSDAddress>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxyAdvanced(pszdeviceid : ::windows_core::PCWSTR, pdeviceaddress : * mut::core::ffi::c_void, pszlocalid : ::windows_core::PCWSTR, pcontext : * mut::core::ffi::c_void, ppdeviceproxy : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDeviceProxyAdvanced(pszdeviceid.into_param().abi(), pdeviceaddress.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider<P0>(pcontext: P0) -> ::windows_core::Result<IWSDiscoveryProvider>
where
    P0: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider(pcontext : * mut::core::ffi::c_void, ppprovider : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDiscoveryProvider(pcontext.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider2<P0>(pcontext: P0, pconfigparams: ::core::option::Option<&[WSD_CONFIG_PARAM]>) -> ::windows_core::Result<IWSDiscoveryProvider>
where
    P0: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider2(pcontext : * mut::core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppprovider : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDiscoveryProvider2(pcontext.into_param().abi(), ::core::mem::transmute(pconfigparams.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher<P0>(pcontext: P0) -> ::windows_core::Result<IWSDiscoveryPublisher>
where
    P0: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher(pcontext : * mut::core::ffi::c_void, pppublisher : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDiscoveryPublisher(pcontext.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher2<P0>(pcontext: P0, pconfigparams: ::core::option::Option<&[WSD_CONFIG_PARAM]>) -> ::windows_core::Result<IWSDiscoveryPublisher>
where
    P0: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher2(pcontext : * mut::core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, pppublisher : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateDiscoveryPublisher2(pcontext.into_param().abi(), ::core::mem::transmute(pconfigparams.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateHttpAddress() -> ::windows_core::Result<IWSDHttpAddress> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateHttpAddress(ppaddress : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateHttpAddress(&mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> ::windows_core::Result<IWSDHttpMessageParameters> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateHttpMessageParameters(pptxparams : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateHttpMessageParameters(&mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> ::windows_core::Result<IWSDOutboundAttachment> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateOutboundAttachment(ppattachment : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateOutboundAttachment(&mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> ::windows_core::Result<IWSDUdpAddress> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateUdpAddress(ppaddress : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateUdpAddress(&mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters() -> ::windows_core::Result<IWSDUdpMessageParameters> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDCreateUdpMessageParameters(pptxparams : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDCreateUdpMessageParameters(&mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDDetachLinkedMemory(pvoid : *mut ::core::ffi::c_void));
    WSDDetachLinkedMemory(pvoid)
}
#[inline]
pub unsafe fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDFreeLinkedMemory(pvoid : *mut ::core::ffi::c_void));
    WSDFreeLinkedMemory(pvoid)
}
#[inline]
pub unsafe fn WSDGenerateFault<P0, P1, P2, P3, P4>(pszcode: P0, pszsubcode: P1, pszreason: P2, pszdetail: P3, pcontext: P4) -> ::windows_core::Result<*mut WSD_SOAP_FAULT>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P4: ::windows_core::IntoParam<IWSDXMLContext>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDGenerateFault(pszcode : ::windows_core::PCWSTR, pszsubcode : ::windows_core::PCWSTR, pszreason : ::windows_core::PCWSTR, pszdetail : ::windows_core::PCWSTR, pcontext : * mut::core::ffi::c_void, ppfault : *mut *mut WSD_SOAP_FAULT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDGenerateFault(pszcode.into_param().abi(), pszsubcode.into_param().abi(), pszreason.into_param().abi(), pszdetail.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDGenerateFaultEx<P0>(pcode: *const WSDXML_NAME, psubcode: ::core::option::Option<*const WSDXML_NAME>, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: P0) -> ::windows_core::Result<*mut WSD_SOAP_FAULT>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDGenerateFaultEx(pcode : *const WSDXML_NAME, psubcode : *const WSDXML_NAME, preasons : *const WSD_LOCALIZED_STRING_LIST, pszdetail : ::windows_core::PCWSTR, ppfault : *mut *mut WSD_SOAP_FAULT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDGenerateFaultEx(pcode, ::core::mem::transmute(psubcode.unwrap_or(::std::ptr::null())), preasons, pszdetail.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDGetConfigurationOption(dwoption : u32, pvoid : *mut ::core::ffi::c_void, cboutbuffer : u32) -> ::windows_core::HRESULT);
    WSDGetConfigurationOption(dwoption, pvoid, cboutbuffer).ok()
}
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDSetConfigurationOption(dwoption : u32, pvoid : *const ::core::ffi::c_void, cbinbuffer : u32) -> ::windows_core::HRESULT);
    WSDSetConfigurationOption(dwoption, pvoid, cbinbuffer).ok()
}
#[inline]
pub unsafe fn WSDUriDecode(source: &[u16], destout: *mut ::windows_core::PWSTR, cchdestout: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDUriDecode(source : ::windows_core::PCWSTR, cchsource : u32, destout : *mut ::windows_core::PWSTR, cchdestout : *mut u32) -> ::windows_core::HRESULT);
    WSDUriDecode(::core::mem::transmute(source.as_ptr()), source.len().try_into().unwrap(), destout, ::core::mem::transmute(cchdestout.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WSDUriEncode(source: &[u16], destout: *mut ::windows_core::PWSTR, cchdestout: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDUriEncode(source : ::windows_core::PCWSTR, cchsource : u32, destout : *mut ::windows_core::PWSTR, cchdestout : *mut u32) -> ::windows_core::HRESULT);
    WSDUriEncode(::core::mem::transmute(source.as_ptr()), source.len().try_into().unwrap(), destout, ::core::mem::transmute(cchdestout.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDXMLAddChild(pparent : *mut WSDXML_ELEMENT, pchild : *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT);
    WSDXMLAddChild(pparent, pchild).ok()
}
#[inline]
pub unsafe fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDXMLAddSibling(pfirst : *mut WSDXML_ELEMENT, psecond : *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT);
    WSDXMLAddSibling(pfirst, psecond).ok()
}
#[inline]
pub unsafe fn WSDXMLBuildAnyForSingleElement<P0>(pelementname: *mut WSDXML_NAME, psztext: P0, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDXMLBuildAnyForSingleElement(pelementname : *mut WSDXML_NAME, psztext : ::windows_core::PCWSTR, ppany : *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT);
    WSDXMLBuildAnyForSingleElement(pelementname, psztext.into_param().abi(), ppany).ok()
}
#[inline]
pub unsafe fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDXMLCleanupElement(pany : *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT);
    WSDXMLCleanupElement(pany).ok()
}
#[inline]
pub unsafe fn WSDXMLCreateContext() -> ::windows_core::Result<IWSDXMLContext> {
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDXMLCreateContext(ppcontext : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDXMLCreateContext(&mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<P0, P1>(psznamespace: P0, pszname: P1) -> ::windows_core::Result<*mut WSDXML_NAME>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDXMLGetNameFromBuiltinNamespace(psznamespace : ::windows_core::PCWSTR, pszname : ::windows_core::PCWSTR, ppname : *mut *mut WSDXML_NAME) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WSDXMLGetNameFromBuiltinNamespace(psznamespace.into_param().abi(), pszname.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn WSDXMLGetValueFromAny<P0, P1>(psznamespace: P0, pszname: P1, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsdapi.dll" "system" fn WSDXMLGetValueFromAny(psznamespace : ::windows_core::PCWSTR, pszname : ::windows_core::PCWSTR, pany : *mut WSDXML_ELEMENT, ppszvalue : *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    WSDXMLGetValueFromAny(psznamespace.into_param().abi(), pszname.into_param().abi(), pany, ppszvalue).ok()
}
::windows_core::imp::com_interface!(IWSDAddress, IWSDAddress_Vtbl, 0xb9574c6c_12a6_4f74_93a1_3318ff605759);
::windows_core::imp::interface_hierarchy!(IWSDAddress, ::windows_core::IUnknown);
impl IWSDAddress {
    pub unsafe fn Serialize<P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), fsafe.into_param().abi()).ok()
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Deserialize)(::windows_core::Interface::as_raw(self), pszbuffer.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAddress_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbuffer: ::windows_core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbuffer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDAsyncCallback, IWSDAsyncCallback_Vtbl, 0xa63e109d_ce72_49e2_ba98_e845f5ee1666);
::windows_core::imp::interface_hierarchy!(IWSDAsyncCallback, ::windows_core::IUnknown);
impl IWSDAsyncCallback {
    pub unsafe fn AsyncOperationComplete<P0, P1>(&self, pasyncresult: P0, pasyncstate: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).AsyncOperationComplete)(::windows_core::Interface::as_raw(self), pasyncresult.into_param().abi(), pasyncstate.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDAsyncResult, IWSDAsyncResult_Vtbl, 0x11a9852a_8dd8_423e_b537_9356db4fbfb8);
::windows_core::imp::interface_hierarchy!(IWSDAsyncResult, ::windows_core::IUnknown);
impl IWSDAsyncResult {
    pub unsafe fn SetCallback<P0, P1>(&self, pcallback: P0, pasyncstate: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncCallback>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetCallback)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi(), pasyncstate.into_param().abi()).ok()
    }
    pub unsafe fn SetWaitHandle<P0>(&self, hwaithandle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).SetWaitHandle)(::windows_core::Interface::as_raw(self), hwaithandle.into_param().abi()).ok()
    }
    pub unsafe fn HasCompleted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HasCompleted)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAsyncState(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAsyncState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetEvent(&self, pevent: *mut WSD_EVENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEvent)(::windows_core::Interface::as_raw(self), pevent).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEndpointProxy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetWaitHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    pub HasCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetAsyncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows_core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDAttachment, IWSDAttachment_Vtbl, 0x5d55a616_9df8_4b09_b156_9ba351a48b76);
::windows_core::imp::interface_hierarchy!(IWSDAttachment, ::windows_core::IUnknown);
impl IWSDAttachment {}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
::windows_core::imp::com_interface!(IWSDDeviceHost, IWSDDeviceHost_Vtbl, 0x917fe891_3d13_4138_9809_934c8abeb12c);
::windows_core::imp::interface_hierarchy!(IWSDDeviceHost, ::windows_core::IUnknown);
impl IWSDDeviceHost {
    pub unsafe fn Init<P0, P1>(&self, pszlocalid: P0, pcontext: P1, pphostaddresses: ::core::option::Option<&[::core::option::Option<IWSDAddress>]>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWSDXMLContext>,
    {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(pphostaddresses.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pphostaddresses.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn Start<P0>(&self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDDeviceHostNotify>,
    {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), ullinstanceid, pscopelist, pnotificationsink.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterPortType)(::windows_core::Interface::as_raw(self), pporttype).ok()
    }
    pub unsafe fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: ::core::option::Option<*const WSD_HOST_METADATA>, pcustommetadata: ::core::option::Option<*const WSD_METADATA_SECTION_LIST>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMetadata)(::windows_core::Interface::as_raw(self), pthismodelmetadata, pthisdevicemetadata, ::core::mem::transmute(phostmetadata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pcustommetadata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn RegisterService<P0, P1>(&self, pszserviceid: P0, pservice: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).RegisterService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), pservice.into_param().abi()).ok()
    }
    pub unsafe fn RetireService<P0>(&self, pszserviceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RetireService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi()).ok()
    }
    pub unsafe fn AddDynamicService<P0, P1, P2>(&self, pszserviceid: P0, pszendpointaddress: P1, pporttype: ::core::option::Option<*const WSD_PORT_TYPE>, pportname: ::core::option::Option<*const WSDXML_NAME>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, pservice: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).AddDynamicService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), pszendpointaddress.into_param().abi(), ::core::mem::transmute(pporttype.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pportname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pservice.into_param().abi()).ok()
    }
    pub unsafe fn RemoveDynamicService<P0>(&self, pszserviceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveDynamicService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi()).ok()
    }
    pub unsafe fn SetServiceDiscoverable<P0, P1>(&self, pszserviceid: P0, fdiscoverable: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetServiceDiscoverable)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), fdiscoverable.into_param().abi()).ok()
    }
    pub unsafe fn SignalEvent<P0>(&self, pszserviceid: P0, pbody: ::core::option::Option<*const ::core::ffi::c_void>, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SignalEvent)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), ::core::mem::transmute(pbody.unwrap_or(::std::ptr::null())), poperation).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHost_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlocalid: ::windows_core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pphostaddresses: *const *mut ::core::ffi::c_void, dwhostaddresscount: u32) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterPortType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows_core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT,
    pub RegisterService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RetireService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub AddDynamicService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pszendpointaddress: ::windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveDynamicService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetServiceDiscoverable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SignalEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDDeviceHostNotify, IWSDDeviceHostNotify_Vtbl, 0xb5bee9f9_eeda_41fe_96f7_f45e14990fb0);
::windows_core::imp::interface_hierarchy!(IWSDDeviceHostNotify, ::windows_core::IUnknown);
impl IWSDDeviceHostNotify {
    pub unsafe fn GetService<P0>(&self, pszserviceid: P0) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDDeviceProxy, IWSDDeviceProxy_Vtbl, 0xeee0c031_c578_4c0e_9a3b_973c35f409db);
::windows_core::imp::interface_hierarchy!(IWSDDeviceProxy, ::windows_core::IUnknown);
impl IWSDDeviceProxy {
    pub unsafe fn Init<P0, P1, P2, P3, P4>(&self, pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3, psponsor: P4) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWSDAddress>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<IWSDXMLContext>,
        P4: ::windows_core::IntoParam<IWSDDeviceProxy>,
    {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), pszdeviceid.into_param().abi(), pdeviceaddress.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), psponsor.into_param().abi()).ok()
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginGetMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
    {
        (::windows_core::Interface::vtable(self).EndGetMetadata)(::windows_core::Interface::as_raw(self), presult.into_param().abi()).ok()
    }
    pub unsafe fn GetHostMetadata(&self) -> ::windows_core::Result<*mut WSD_HOST_METADATA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHostMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThisModelMetadata(&self) -> ::windows_core::Result<*mut WSD_THIS_MODEL_METADATA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetThisModelMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThisDeviceMetadata(&self) -> ::windows_core::Result<*mut WSD_THIS_DEVICE_METADATA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetThisDeviceMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAllMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAllMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetServiceProxyById<P0>(&self, pszserviceid: P0) -> ::windows_core::Result<IWSDServiceProxy>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceProxyById)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> ::windows_core::Result<IWSDServiceProxy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceProxyByType)(::windows_core::Interface::as_raw(self), ptype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEndpointProxy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceProxy_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdeviceid: ::windows_core::PCWSTR, pdeviceaddress: *mut ::core::ffi::c_void, pszlocalid: ::windows_core::PCWSTR, pcontext: *mut ::core::ffi::c_void, psponsor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetHostMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows_core::HRESULT,
    pub GetThisModelMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows_core::HRESULT,
    pub GetThisDeviceMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows_core::HRESULT,
    pub GetAllMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT,
    pub GetServiceProxyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetServiceProxyByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDEndpointProxy, IWSDEndpointProxy_Vtbl, 0x1860d430_b24c_4975_9f90_dbb39baa24ec);
::windows_core::imp::interface_hierarchy!(IWSDEndpointProxy, ::windows_core::IUnknown);
impl IWSDEndpointProxy {
    pub unsafe fn SendOneWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendOneWayRequest)(::windows_core::Interface::as_raw(self), pbody, poperation).ok()
    }
    pub unsafe fn SendTwoWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: ::core::option::Option<*const WSD_SYNCHRONOUS_RESPONSE_CONTEXT>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendTwoWayRequest)(::windows_core::Interface::as_raw(self), pbody, poperation, ::core::mem::transmute(presponsecontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SendTwoWayRequestAsync<P0, P1>(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: P0, pcallback: P1) -> ::windows_core::Result<IWSDAsyncResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IWSDAsyncCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SendTwoWayRequestAsync)(::windows_core::Interface::as_raw(self), pbody, poperation, pasyncstate.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn AbortAsyncOperation<P0>(&self, pasyncresult: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
    {
        (::windows_core::Interface::vtable(self).AbortAsyncOperation)(::windows_core::Interface::as_raw(self), pasyncresult.into_param().abi()).ok()
    }
    pub unsafe fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessFault)(::windows_core::Interface::as_raw(self), pfault).ok()
    }
    pub unsafe fn GetErrorInfo(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFaultInfo(&self) -> ::windows_core::Result<*mut WSD_SOAP_FAULT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFaultInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEndpointProxy_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SendOneWayRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT,
    pub SendTwoWayRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows_core::HRESULT,
    pub SendTwoWayRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, presult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AbortAsyncOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProcessFault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::HRESULT,
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetFaultInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDEventingStatus, IWSDEventingStatus_Vtbl, 0x49b17f52_637a_407a_ae99_fbe82a4d38c0);
::windows_core::imp::interface_hierarchy!(IWSDEventingStatus, ::windows_core::IUnknown);
impl IWSDEventingStatus {
    pub unsafe fn SubscriptionRenewed<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SubscriptionRenewed)(::windows_core::Interface::as_raw(self), pszsubscriptionaction.into_param().abi())
    }
    pub unsafe fn SubscriptionRenewalFailed<P0>(&self, pszsubscriptionaction: P0, hr: ::windows_core::HRESULT)
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SubscriptionRenewalFailed)(::windows_core::Interface::as_raw(self), pszsubscriptionaction.into_param().abi(), hr)
    }
    pub unsafe fn SubscriptionEnded<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SubscriptionEnded)(::windows_core::Interface::as_raw(self), pszsubscriptionaction.into_param().abi())
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SubscriptionRenewed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR),
    pub SubscriptionRenewalFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR, hr: ::windows_core::HRESULT),
    pub SubscriptionEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR),
}
::windows_core::imp::com_interface!(IWSDHttpAddress, IWSDHttpAddress_Vtbl, 0xd09ac7bd_2a3e_4b85_8605_2737ff3e4ea0);
::windows_core::imp::interface_hierarchy!(IWSDHttpAddress, ::windows_core::IUnknown, IWSDAddress, IWSDTransportAddress);
impl IWSDHttpAddress {
    pub unsafe fn Serialize<P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), fsafe.into_param().abi()).ok()
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Deserialize)(::windows_core::Interface::as_raw(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPort)(::windows_core::Interface::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransportAddressEx<P0>(&self, fsafe: P0) -> ::windows_core::Result<::windows_core::PCWSTR>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportAddressEx)(::windows_core::Interface::as_raw(self), fsafe.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransportAddress<P0>(&self, pszaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransportAddress)(::windows_core::Interface::as_raw(self), pszaddress.into_param().abi()).ok()
    }
    pub unsafe fn GetSecure(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSecure)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetSecure<P0>(&self, fsecure: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetSecure)(::windows_core::Interface::as_raw(self), fsecure.into_param().abi()).ok()
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPath<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    pub GetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDHttpAuthParameters, IWSDHttpAuthParameters_Vtbl, 0x0b476df0_8dac_480d_b05c_99781a5884aa);
::windows_core::imp::interface_hierarchy!(IWSDHttpAuthParameters, ::windows_core::IUnknown);
impl IWSDHttpAuthParameters {
    pub unsafe fn GetClientAccessToken(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetClientAccessToken)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAuthType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAuthParameters_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetClientAccessToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    pub GetAuthType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDHttpMessageParameters, IWSDHttpMessageParameters_Vtbl, 0x540bd122_5c83_4dec_b396_ea62a2697fdf);
::windows_core::imp::interface_hierarchy!(IWSDHttpMessageParameters, ::windows_core::IUnknown, IWSDMessageParameters);
impl IWSDHttpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLocalAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalAddress<P0>(&self, paddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAddress>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLocalAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRemoteAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddress<P0>(&self, paddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAddress>,
    {
        (::windows_core::Interface::vtable(self).base__.SetRemoteAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows_core::Result<IWSDMessageParameters> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLowerParameters)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInboundHttpHeaders<P0>(&self, pszheaders: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetInboundHttpHeaders)(::windows_core::Interface::as_raw(self), pszheaders.into_param().abi()).ok()
    }
    pub unsafe fn GetInboundHttpHeaders(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInboundHttpHeaders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutboundHttpHeaders<P0>(&self, pszheaders: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOutboundHttpHeaders)(::windows_core::Interface::as_raw(self), pszheaders.into_param().abi()).ok()
    }
    pub unsafe fn GetOutboundHttpHeaders(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutboundHttpHeaders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetID<P0>(&self, pszid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetID)(::windows_core::Interface::as_raw(self), pszid.into_param().abi()).ok()
    }
    pub unsafe fn GetID(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContext<P0>(&self, pcontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetContext)(::windows_core::Interface::as_raw(self), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn GetContext(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetInboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetInboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszid: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDInboundAttachment, IWSDInboundAttachment_Vtbl, 0x5bd6ca65_233c_4fb8_9f7a_2641619655c9);
::windows_core::imp::interface_hierarchy!(IWSDInboundAttachment, ::windows_core::IUnknown, IWSDAttachment);
impl IWSDInboundAttachment {
    pub unsafe fn Read(&self, pbuffer: &mut [u8], pdwnumberofbytesread: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), pdwnumberofbytesread).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDMessageParameters, IWSDMessageParameters_Vtbl, 0x1fafe8a2_e6fc_4b80_b6cf_b7d45c416d7c);
::windows_core::imp::interface_hierarchy!(IWSDMessageParameters, ::windows_core::IUnknown);
impl IWSDMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalAddress<P0>(&self, paddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAddress>,
    {
        (::windows_core::Interface::vtable(self).SetLocalAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRemoteAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddress<P0>(&self, paddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAddress>,
    {
        (::windows_core::Interface::vtable(self).SetRemoteAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows_core::Result<IWSDMessageParameters> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLowerParameters)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMessageParameters_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetLocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLowerParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptxparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDMetadataExchange, IWSDMetadataExchange_Vtbl, 0x06996d57_1d67_4928_9307_3d7833fdb846);
::windows_core::imp::interface_hierarchy!(IWSDMetadataExchange, ::windows_core::IUnknown);
impl IWSDMetadataExchange {
    pub unsafe fn GetMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMetadataExchange_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDOutboundAttachment, IWSDOutboundAttachment_Vtbl, 0xaa302f8d_5a22_4ba5_b392_aa8486f4c15d);
::windows_core::imp::interface_hierarchy!(IWSDOutboundAttachment, ::windows_core::IUnknown, IWSDAttachment);
impl IWSDOutboundAttachment {
    pub unsafe fn Write(&self, pbuffer: &[u8]) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDSSLClientCertificate, IWSDSSLClientCertificate_Vtbl, 0xde105e87_a0da_418e_98ad_27b9eed87bdc);
::windows_core::imp::interface_hierarchy!(IWSDSSLClientCertificate, ::windows_core::IUnknown);
impl IWSDSSLClientCertificate {
    #[doc = "Required features: `\"Win32_Security_Cryptography\"`"]
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn GetClientCertificate(&self) -> ::windows_core::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetClientCertificate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMappedAccessToken(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMappedAccessToken)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub GetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    GetClientCertificate: usize,
    pub GetMappedAccessToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDScopeMatchingRule, IWSDScopeMatchingRule_Vtbl, 0xfcafe424_fef5_481a_bd9f_33ce0574256f);
::windows_core::imp::interface_hierarchy!(IWSDScopeMatchingRule, ::windows_core::IUnknown);
impl IWSDScopeMatchingRule {
    pub unsafe fn GetScopeRule(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetScopeRule)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MatchScopes<P0, P1>(&self, pszscope1: P0, pszscope2: P1) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MatchScopes)(::windows_core::Interface::as_raw(self), pszscope1.into_param().abi(), pszscope2.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDScopeMatchingRule_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetScopeRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub MatchScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszscope1: ::windows_core::PCWSTR, pszscope2: ::windows_core::PCWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDServiceMessaging, IWSDServiceMessaging_Vtbl, 0x94974cf4_0cab_460d_a3f6_7a0ad623c0e6);
::windows_core::imp::interface_hierarchy!(IWSDServiceMessaging, ::windows_core::IUnknown);
impl IWSDServiceMessaging {
    pub unsafe fn SendResponse<P0>(&self, pbody: ::core::option::Option<*const ::core::ffi::c_void>, poperation: *const WSD_OPERATION, pmessageparameters: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDMessageParameters>,
    {
        (::windows_core::Interface::vtable(self).SendResponse)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbody.unwrap_or(::std::ptr::null())), poperation, pmessageparameters.into_param().abi()).ok()
    }
    pub unsafe fn FaultRequest<P0>(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: P0, pfault: ::core::option::Option<*const WSD_SOAP_FAULT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDMessageParameters>,
    {
        (::windows_core::Interface::vtable(self).FaultRequest)(::windows_core::Interface::as_raw(self), prequestheader, pmessageparameters.into_param().abi(), ::core::mem::transmute(pfault.unwrap_or(::std::ptr::null()))).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceMessaging_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SendResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FaultRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDServiceProxy, IWSDServiceProxy_Vtbl, 0xd4c7fb9c_03ab_4175_9d67_094fafebf487);
::windows_core::imp::interface_hierarchy!(IWSDServiceProxy, ::windows_core::IUnknown, IWSDMetadataExchange);
impl IWSDServiceProxy {
    pub unsafe fn GetMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginGetMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndGetMetadata)(::windows_core::Interface::as_raw(self), presult.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetServiceMetadata(&self) -> ::windows_core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubscribeToOperation<P0>(&self, poperation: *const WSD_OPERATION, punknown: P0, pany: *const WSDXML_ELEMENT, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SubscribeToOperation)(::windows_core::Interface::as_raw(self), poperation, punknown.into_param().abi(), pany, ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnsubscribeToOperation)(::windows_core::Interface::as_raw(self), poperation).ok()
    }
    pub unsafe fn SetEventingStatusCallback<P0>(&self, pstatus: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDEventingStatus>,
    {
        (::windows_core::Interface::vtable(self).SetEventingStatusCallback)(::windows_core::Interface::as_raw(self), pstatus.into_param().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEndpointProxy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxy_Vtbl {
    pub base__: IWSDMetadataExchange_Vtbl,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT,
    pub GetServiceMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows_core::HRESULT,
    pub SubscribeToOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub UnsubscribeToOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT,
    pub SetEventingStatusCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDServiceProxyEventing, IWSDServiceProxyEventing_Vtbl, 0xf9279d6d_1012_4a94_b8cc_fd35d2202bfe);
::windows_core::imp::interface_hierarchy!(IWSDServiceProxyEventing, ::windows_core::IUnknown, IWSDMetadataExchange, IWSDServiceProxy);
impl IWSDServiceProxyEventing {
    pub unsafe fn GetMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginGetMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndGetMetadata)(::windows_core::Interface::as_raw(self), presult.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetServiceMetadata(&self) -> ::windows_core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetServiceMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubscribeToOperation<P0>(&self, poperation: *const WSD_OPERATION, punknown: P0, pany: *const WSDXML_ELEMENT, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.SubscribeToOperation)(::windows_core::Interface::as_raw(self), poperation, punknown.into_param().abi(), pany, ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnsubscribeToOperation)(::windows_core::Interface::as_raw(self), poperation).ok()
    }
    pub unsafe fn SetEventingStatusCallback<P0>(&self, pstatus: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDEventingStatus>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEventingStatusCallback)(::windows_core::Interface::as_raw(self), pstatus.into_param().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEndpointProxy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubscribeToMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], punknown: P0, pexpires: ::core::option::Option<*const WSD_EVENTING_EXPIRES>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), punknown.into_param().abi(), ::core::mem::transmute(pexpires.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn BeginSubscribeToMultipleOperations<P0, P1, P2>(&self, poperations: &[WSD_OPERATION], punknown: P0, pexpires: ::core::option::Option<*const WSD_EVENTING_EXPIRES>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, pasyncstate: P1, pasynccallback: P2) -> ::windows_core::Result<IWSDAsyncResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<IWSDAsyncCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginSubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), punknown.into_param().abi(), ::core::mem::transmute(pexpires.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndSubscribeToMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
    {
        (::windows_core::Interface::vtable(self).EndSubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), presult.into_param().abi(), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnsubscribeToMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnsubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), pany).ok()
    }
    pub unsafe fn BeginUnsubscribeToMultipleOperations<P0, P1>(&self, poperations: &[WSD_OPERATION], pany: ::core::option::Option<*const WSDXML_ELEMENT>, pasyncstate: P0, pasynccallback: P1) -> ::windows_core::Result<IWSDAsyncResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IWSDAsyncCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginUnsubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndUnsubscribeToMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], presult: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
    {
        (::windows_core::Interface::vtable(self).EndUnsubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), presult.into_param().abi()).ok()
    }
    pub unsafe fn RenewMultipleOperations(&self, poperations: &[WSD_OPERATION], pexpires: ::core::option::Option<*const WSD_EVENTING_EXPIRES>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RenewMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), ::core::mem::transmute(pexpires.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn BeginRenewMultipleOperations<P0, P1>(&self, poperations: &[WSD_OPERATION], pexpires: ::core::option::Option<*const WSD_EVENTING_EXPIRES>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, pasyncstate: P0, pasynccallback: P1) -> ::windows_core::Result<IWSDAsyncResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IWSDAsyncCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginRenewMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), ::core::mem::transmute(pexpires.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndRenewMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
    {
        (::windows_core::Interface::vtable(self).EndRenewMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), presult.into_param().abi(), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStatusForMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: ::core::option::Option<*const WSDXML_ELEMENT>, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatusForMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn BeginGetStatusForMultipleOperations<P0, P1>(&self, poperations: &[WSD_OPERATION], pany: ::core::option::Option<*const WSDXML_ELEMENT>, pasyncstate: P0, pasynccallback: P1) -> ::windows_core::Result<IWSDAsyncResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IWSDAsyncCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginGetStatusForMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndGetStatusForMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAsyncResult>,
    {
        (::windows_core::Interface::vtable(self).EndGetStatusForMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), presult.into_param().abi(), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxyEventing_Vtbl {
    pub base__: IWSDServiceProxy_Vtbl,
    pub SubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub BeginSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub UnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub BeginUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub BeginRenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndRenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub GetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub BeginGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDSignatureProperty, IWSDSignatureProperty_Vtbl, 0x03ce20aa_71c4_45e2_b32e_3766c61c790f);
::windows_core::imp::interface_hierarchy!(IWSDSignatureProperty, ::windows_core::IUnknown);
impl IWSDSignatureProperty {
    pub unsafe fn IsMessageSigned(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsMessageSigned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsMessageSignatureTrusted(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsMessageSignatureTrusted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: ::core::option::Option<*mut u8>, pdwkeyinfosize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetKeyInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbkeyinfo.unwrap_or(::std::ptr::null_mut())), pdwkeyinfosize).ok()
    }
    pub unsafe fn GetSignature(&self, pbsignature: ::core::option::Option<*mut u8>, pdwsignaturesize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSignature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbsignature.unwrap_or(::std::ptr::null_mut())), pdwsignaturesize).ok()
    }
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: ::core::option::Option<*mut u8>, pdwhashsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSignedInfoHash)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbsignedinfohash.unwrap_or(::std::ptr::null_mut())), pdwhashsize).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsMessageSigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub IsMessageSignatureTrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetKeyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows_core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows_core::HRESULT,
    pub GetSignedInfoHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDTransportAddress, IWSDTransportAddress_Vtbl, 0x70d23498_4ee6_4340_a3df_d845d2235467);
::windows_core::imp::interface_hierarchy!(IWSDTransportAddress, ::windows_core::IUnknown, IWSDAddress);
impl IWSDTransportAddress {
    pub unsafe fn Serialize<P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), fsafe.into_param().abi()).ok()
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Deserialize)(::windows_core::Interface::as_raw(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPort)(::windows_core::Interface::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransportAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransportAddressEx<P0>(&self, fsafe: P0) -> ::windows_core::Result<::windows_core::PCWSTR>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransportAddressEx)(::windows_core::Interface::as_raw(self), fsafe.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransportAddress<P0>(&self, pszaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTransportAddress)(::windows_core::Interface::as_raw(self), pszaddress.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDTransportAddress_Vtbl {
    pub base__: IWSDAddress_Vtbl,
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows_core::HRESULT,
    pub GetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetTransportAddressEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaddress: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDUdpAddress, IWSDUdpAddress_Vtbl, 0x74d6124a_a441_4f78_a1eb_97a8d1996893);
::windows_core::imp::interface_hierarchy!(IWSDUdpAddress, ::windows_core::IUnknown, IWSDAddress, IWSDTransportAddress);
impl IWSDUdpAddress {
    pub unsafe fn Serialize<P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), fsafe.into_param().abi()).ok()
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Deserialize)(::windows_core::Interface::as_raw(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPort)(::windows_core::Interface::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransportAddressEx<P0>(&self, fsafe: P0) -> ::windows_core::Result<::windows_core::PCWSTR>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportAddressEx)(::windows_core::Interface::as_raw(self), fsafe.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransportAddress<P0>(&self, pszaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransportAddress)(::windows_core::Interface::as_raw(self), pszaddress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSockaddr)(::windows_core::Interface::as_raw(self), psockaddr).ok()
    }
    #[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn GetSockaddr(&self, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSockaddr)(::windows_core::Interface::as_raw(self), psockaddr).ok()
    }
    pub unsafe fn SetExclusive<P0>(&self, fexclusive: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetExclusive)(::windows_core::Interface::as_raw(self), fexclusive.into_param().abi()).ok()
    }
    pub unsafe fn GetExclusive(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetExclusive)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMessageType)(::windows_core::Interface::as_raw(self), messagetype).ok()
    }
    pub unsafe fn GetMessageType(&self) -> ::windows_core::Result<WSDUdpMessageType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessageType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTTL(&self, dwttl: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTTL)(::windows_core::Interface::as_raw(self), dwttl).ok()
    }
    pub unsafe fn GetTTL(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTTL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAlias(&self, palias: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlias)(::windows_core::Interface::as_raw(self), palias).ok()
    }
    pub unsafe fn GetAlias(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAlias)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub SetSockaddr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    SetSockaddr: usize,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub GetSockaddr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    GetSockaddr: usize,
    pub SetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows_core::HRESULT,
    pub GetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows_core::HRESULT,
    pub SetTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows_core::HRESULT,
    pub GetTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows_core::HRESULT,
    pub SetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palias: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palias: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDUdpMessageParameters, IWSDUdpMessageParameters_Vtbl, 0x9934149f_8f0c_447b_aa0b_73124b0ca7f0);
::windows_core::imp::interface_hierarchy!(IWSDUdpMessageParameters, ::windows_core::IUnknown, IWSDMessageParameters);
impl IWSDUdpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLocalAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalAddress<P0>(&self, paddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAddress>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLocalAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRemoteAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddress<P0>(&self, paddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDAddress>,
    {
        (::windows_core::Interface::vtable(self).base__.SetRemoteAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows_core::Result<IWSDMessageParameters> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLowerParameters)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRetransmitParams)(::windows_core::Interface::as_raw(self), pparams).ok()
    }
    pub unsafe fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRetransmitParams)(::windows_core::Interface::as_raw(self), pparams).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetRetransmitParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows_core::HRESULT,
    pub GetRetransmitParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDXMLContext, IWSDXMLContext_Vtbl, 0x75d8f3ee_3e5a_43b4_a15a_bcf6887460c0);
::windows_core::imp::interface_hierarchy!(IWSDXMLContext, ::windows_core::IUnknown);
impl IWSDXMLContext {
    pub unsafe fn AddNamespace<P0, P1>(&self, pszuri: P0, pszsuggestedprefix: P1, ppnamespace: ::core::option::Option<*mut *mut WSDXML_NAMESPACE>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddNamespace)(::windows_core::Interface::as_raw(self), pszuri.into_param().abi(), pszsuggestedprefix.into_param().abi(), ::core::mem::transmute(ppnamespace.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddNameToNamespace<P0, P1>(&self, pszuri: P0, pszname: P1, ppname: ::core::option::Option<*mut *mut WSDXML_NAME>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddNameToNamespace)(::windows_core::Interface::as_raw(self), pszuri.into_param().abi(), pszname.into_param().abi(), ::core::mem::transmute(ppname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetNamespaces(&self, pnamespaces: &[*const WSDXML_NAMESPACE], blayernumber: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamespaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pnamespaces.as_ptr()), pnamespaces.len().try_into().unwrap(), blayernumber).ok()
    }
    pub unsafe fn SetTypes(&self, ptypes: &[*const WSDXML_TYPE], blayernumber: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptypes.as_ptr()), ptypes.len().try_into().unwrap(), blayernumber).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuri: ::windows_core::PCWSTR, pszsuggestedprefix: ::windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows_core::HRESULT,
    pub AddNameToNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuri: ::windows_core::PCWSTR, pszname: ::windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_core::HRESULT,
    pub SetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows_core::HRESULT,
    pub SetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDiscoveredService, IWSDiscoveredService_Vtbl, 0x4bad8a3b_b374_4420_9632_aac945b374aa);
::windows_core::imp::interface_hierarchy!(IWSDiscoveredService, ::windows_core::IUnknown);
impl IWSDiscoveredService {
    pub unsafe fn GetEndpointReference(&self) -> ::windows_core::Result<*mut WSD_ENDPOINT_REFERENCE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEndpointReference)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypes(&self) -> ::windows_core::Result<*mut WSD_NAME_LIST> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScopes(&self) -> ::windows_core::Result<*mut WSD_URI_LIST> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetScopes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetXAddrs(&self) -> ::windows_core::Result<*mut WSD_URI_LIST> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXAddrs)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMetadataVersion(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMetadataVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetExtendedDiscoXML)(::windows_core::Interface::as_raw(self), ppheaderany, ppbodyany).ok()
    }
    pub unsafe fn GetProbeResolveTag(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProbeResolveTag)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRemoteTransportAddress(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRemoteTransportAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLocalTransportAddress(&self) -> ::windows_core::Result<::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalTransportAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLocalInterfaceGUID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalInterfaceGUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveredService_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetEndpointReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows_core::HRESULT,
    pub GetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows_core::HRESULT,
    pub GetScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub GetXAddrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub GetMetadataVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows_core::HRESULT,
    pub GetExtendedDiscoXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub GetProbeResolveTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztag: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetRemoteTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetLocalTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetLocalInterfaceGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDiscoveryProvider, IWSDiscoveryProvider_Vtbl, 0x8ffc8e55_f0eb_480f_88b7_b435dd281d45);
::windows_core::imp::interface_hierarchy!(IWSDiscoveryProvider, ::windows_core::IUnknown);
impl IWSDiscoveryProvider {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddressFamily)(::windows_core::Interface::as_raw(self), dwaddressfamily).ok()
    }
    pub unsafe fn Attach<P0>(&self, psink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDiscoveryProviderNotify>,
    {
        (::windows_core::Interface::vtable(self).Attach)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn Detach(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Detach)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SearchById<P0, P1>(&self, pszid: P0, psztag: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SearchById)(::windows_core::Interface::as_raw(self), pszid.into_param().abi(), psztag.into_param().abi()).ok()
    }
    pub unsafe fn SearchByAddress<P0, P1>(&self, pszaddress: P0, psztag: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SearchByAddress)(::windows_core::Interface::as_raw(self), pszaddress.into_param().abi(), psztag.into_param().abi()).ok()
    }
    pub unsafe fn SearchByType<P0, P1>(&self, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pszmatchby: P0, psztag: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SearchByType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())), pszmatchby.into_param().abi(), psztag.into_param().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows_core::Result<IWSDXMLContext> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXMLContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows_core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SearchById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SearchByAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaddress: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SearchByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDiscoveryProviderNotify, IWSDiscoveryProviderNotify_Vtbl, 0x73ee3ced_b6e6_4329_a546_3e8ad46563d2);
::windows_core::imp::interface_hierarchy!(IWSDiscoveryProviderNotify, ::windows_core::IUnknown);
impl IWSDiscoveryProviderNotify {
    pub unsafe fn Add<P0>(&self, pservice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDiscoveredService>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), pservice.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, pservice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDiscoveredService>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), pservice.into_param().abi()).ok()
    }
    pub unsafe fn SearchFailed<P0>(&self, hr: ::windows_core::HRESULT, psztag: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SearchFailed)(::windows_core::Interface::as_raw(self), hr, psztag.into_param().abi()).ok()
    }
    pub unsafe fn SearchComplete<P0>(&self, psztag: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SearchComplete)(::windows_core::Interface::as_raw(self), psztag.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProviderNotify_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SearchFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDiscoveryPublisher, IWSDiscoveryPublisher_Vtbl, 0xae01e1a8_3ff9_4148_8116_057cc616fe13);
::windows_core::imp::interface_hierarchy!(IWSDiscoveryPublisher, ::windows_core::IUnknown);
impl IWSDiscoveryPublisher {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddressFamily)(::windows_core::Interface::as_raw(self), dwaddressfamily).ok()
    }
    pub unsafe fn RegisterNotificationSink<P0>(&self, psink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDiscoveryPublisherNotify>,
    {
        (::windows_core::Interface::vtable(self).RegisterNotificationSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn UnRegisterNotificationSink<P0>(&self, psink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDiscoveryPublisherNotify>,
    {
        (::windows_core::Interface::vtable(self).UnRegisterNotificationSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn Publish<P0, P1>(&self, pszid: P0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Publish)(::windows_core::Interface::as_raw(self), pszid.into_param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into_param().abi(), ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn UnPublish<P0, P1>(&self, pszid: P0, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, pany: ::core::option::Option<*const WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).UnPublish)(::windows_core::Interface::as_raw(self), pszid.into_param().abi(), ullinstanceid, ullmessagenumber, pszsessionid.into_param().abi(), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn MatchProbe<P0, P1, P2>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDMessageParameters>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).MatchProbe)(::windows_core::Interface::as_raw(self), pprobemessage, pmessageparameters.into_param().abi(), pszid.into_param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into_param().abi(), ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn MatchResolve<P0, P1, P2>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDMessageParameters>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).MatchResolve)(::windows_core::Interface::as_raw(self), presolvemessage, pmessageparameters.into_param().abi(), pszid.into_param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into_param().abi(), ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn PublishEx<P0, P1>(&self, pszid: P0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>, pheaderany: ::core::option::Option<*const WSDXML_ELEMENT>, preferenceparameterany: ::core::option::Option<*const WSDXML_ELEMENT>, ppolicyany: ::core::option::Option<*const WSDXML_ELEMENT>, pendpointreferenceany: ::core::option::Option<*const WSDXML_ELEMENT>, pany: ::core::option::Option<*const WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).PublishEx)(
            ::windows_core::Interface::as_raw(self),
            pszid.into_param().abi(),
            ullmetadataversion,
            ullinstanceid,
            ullmessagenumber,
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pheaderany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(preferenceparameterany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ppolicyany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pendpointreferenceany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())),
        )
        .ok()
    }
    pub unsafe fn MatchProbeEx<P0, P1, P2>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>, pheaderany: ::core::option::Option<*const WSDXML_ELEMENT>, preferenceparameterany: ::core::option::Option<*const WSDXML_ELEMENT>, ppolicyany: ::core::option::Option<*const WSDXML_ELEMENT>, pendpointreferenceany: ::core::option::Option<*const WSDXML_ELEMENT>, pany: ::core::option::Option<*const WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDMessageParameters>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).MatchProbeEx)(
            ::windows_core::Interface::as_raw(self),
            pprobemessage,
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ullmetadataversion,
            ullinstanceid,
            ullmessagenumber,
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pheaderany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(preferenceparameterany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ppolicyany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pendpointreferenceany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())),
        )
        .ok()
    }
    pub unsafe fn MatchResolveEx<P0, P1, P2>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>, pheaderany: ::core::option::Option<*const WSDXML_ELEMENT>, preferenceparameterany: ::core::option::Option<*const WSDXML_ELEMENT>, ppolicyany: ::core::option::Option<*const WSDXML_ELEMENT>, pendpointreferenceany: ::core::option::Option<*const WSDXML_ELEMENT>, pany: ::core::option::Option<*const WSDXML_ELEMENT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDMessageParameters>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).MatchResolveEx)(
            ::windows_core::Interface::as_raw(self),
            presolvemessage,
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ullmetadataversion,
            ullinstanceid,
            ullmessagenumber,
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pheaderany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(preferenceparameterany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ppolicyany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pendpointreferenceany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())),
        )
        .ok()
    }
    pub unsafe fn RegisterScopeMatchingRule<P0>(&self, pscopematchingrule: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDScopeMatchingRule>,
    {
        (::windows_core::Interface::vtable(self).RegisterScopeMatchingRule)(::windows_core::Interface::as_raw(self), pscopematchingrule.into_param().abi()).ok()
    }
    pub unsafe fn UnRegisterScopeMatchingRule<P0>(&self, pscopematchingrule: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDScopeMatchingRule>,
    {
        (::windows_core::Interface::vtable(self).UnRegisterScopeMatchingRule)(::windows_core::Interface::as_raw(self), pscopematchingrule.into_param().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows_core::Result<IWSDXMLContext> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXMLContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisher_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows_core::HRESULT,
    pub RegisterNotificationSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnRegisterNotificationSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Publish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub UnPublish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub MatchProbe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub MatchResolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub PublishEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub MatchProbeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub MatchResolveEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub RegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnRegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSDiscoveryPublisherNotify, IWSDiscoveryPublisherNotify_Vtbl, 0xe67651b0_337a_4b3c_9758_733388568251);
::windows_core::imp::interface_hierarchy!(IWSDiscoveryPublisherNotify, ::windows_core::IUnknown);
impl IWSDiscoveryPublisherNotify {
    pub unsafe fn ProbeHandler<P0>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDMessageParameters>,
    {
        (::windows_core::Interface::vtable(self).ProbeHandler)(::windows_core::Interface::as_raw(self), psoap, pmessageparameters.into_param().abi()).ok()
    }
    pub unsafe fn ResolveHandler<P0>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWSDMessageParameters>,
    {
        (::windows_core::Interface::vtable(self).ResolveHandler)(::windows_core::Interface::as_raw(self), psoap, pmessageparameters.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisherNotify_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ProbeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResolveHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub const DirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(1i32);
pub const MulticastDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(0i32);
pub const ONE_WAY: WSDUdpMessageType = WSDUdpMessageType(0i32);
pub const OpAnyElement: WSDXML_OP = WSDXML_OP(6i32);
pub const OpAnyElements: WSDXML_OP = WSDXML_OP(7i32);
pub const OpAnyNumber: WSDXML_OP = WSDXML_OP(17i32);
pub const OpAnyText: WSDXML_OP = WSDXML_OP(8i32);
pub const OpAnything: WSDXML_OP = WSDXML_OP(16i32);
pub const OpAttribute_: WSDXML_OP = WSDXML_OP(9i32);
pub const OpBeginAll: WSDXML_OP = WSDXML_OP(14i32);
pub const OpBeginAnyElement: WSDXML_OP = WSDXML_OP(3i32);
pub const OpBeginChoice: WSDXML_OP = WSDXML_OP(10i32);
pub const OpBeginElement_: WSDXML_OP = WSDXML_OP(2i32);
pub const OpBeginSequence: WSDXML_OP = WSDXML_OP(12i32);
pub const OpElement_: WSDXML_OP = WSDXML_OP(5i32);
pub const OpEndAll: WSDXML_OP = WSDXML_OP(15i32);
pub const OpEndChoice: WSDXML_OP = WSDXML_OP(11i32);
pub const OpEndElement: WSDXML_OP = WSDXML_OP(4i32);
pub const OpEndOfTable: WSDXML_OP = WSDXML_OP(1i32);
pub const OpEndSequence: WSDXML_OP = WSDXML_OP(13i32);
pub const OpFormatBool_: WSDXML_OP = WSDXML_OP(20i32);
pub const OpFormatDateTime_: WSDXML_OP = WSDXML_OP(40i32);
pub const OpFormatDom_: WSDXML_OP = WSDXML_OP(30i32);
pub const OpFormatDouble_: WSDXML_OP = WSDXML_OP(42i32);
pub const OpFormatDuration_: WSDXML_OP = WSDXML_OP(39i32);
pub const OpFormatDynamicType_: WSDXML_OP = WSDXML_OP(37i32);
pub const OpFormatFloat_: WSDXML_OP = WSDXML_OP(41i32);
pub const OpFormatInt16_: WSDXML_OP = WSDXML_OP(22i32);
pub const OpFormatInt32_: WSDXML_OP = WSDXML_OP(23i32);
pub const OpFormatInt64_: WSDXML_OP = WSDXML_OP(24i32);
pub const OpFormatInt8_: WSDXML_OP = WSDXML_OP(21i32);
pub const OpFormatListInsertTail_: WSDXML_OP = WSDXML_OP(35i32);
pub const OpFormatLookupType_: WSDXML_OP = WSDXML_OP(38i32);
pub const OpFormatMax: WSDXML_OP = WSDXML_OP(46i32);
pub const OpFormatName_: WSDXML_OP = WSDXML_OP(34i32);
pub const OpFormatStruct_: WSDXML_OP = WSDXML_OP(31i32);
pub const OpFormatType_: WSDXML_OP = WSDXML_OP(36i32);
pub const OpFormatUInt16_: WSDXML_OP = WSDXML_OP(26i32);
pub const OpFormatUInt32_: WSDXML_OP = WSDXML_OP(27i32);
pub const OpFormatUInt64_: WSDXML_OP = WSDXML_OP(28i32);
pub const OpFormatUInt8_: WSDXML_OP = WSDXML_OP(25i32);
pub const OpFormatUnicodeString_: WSDXML_OP = WSDXML_OP(29i32);
pub const OpFormatUri_: WSDXML_OP = WSDXML_OP(32i32);
pub const OpFormatUuidUri_: WSDXML_OP = WSDXML_OP(33i32);
pub const OpFormatXMLDeclaration_: WSDXML_OP = WSDXML_OP(45i32);
pub const OpNone: WSDXML_OP = WSDXML_OP(0i32);
pub const OpOneOrMore: WSDXML_OP = WSDXML_OP(18i32);
pub const OpOptional: WSDXML_OP = WSDXML_OP(19i32);
pub const OpProcess_: WSDXML_OP = WSDXML_OP(43i32);
pub const OpQualifiedAttribute_: WSDXML_OP = WSDXML_OP(44i32);
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(2i32);
pub const TWO_WAY: WSDUdpMessageType = WSDUdpMessageType(1i32);
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
pub const WSDET_INCOMING_FAULT: WSDEventType = WSDEventType(2i32);
pub const WSDET_INCOMING_MESSAGE: WSDEventType = WSDEventType(1i32);
pub const WSDET_NONE: WSDEventType = WSDEventType(0i32);
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = WSDEventType(4i32);
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = WSDEventType(3i32);
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(10i32);
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(9i32);
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(1i32);
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(2i32);
pub const WSD_DEFAULT_EVENTING_ADDRESS: ::windows_core::PCWSTR = ::windows_core::w!("http://*:5357/");
pub const WSD_DEFAULT_HOSTING_ADDRESS: ::windows_core::PCWSTR = ::windows_core::w!("http://*:5357/");
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: ::windows_core::PCWSTR = ::windows_core::w!("https://*:5358/");
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(255i32);
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(2i32);
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(4i32);
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(0i32);
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(1i32);
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(7i32);
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(8i32);
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(12i32);
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(11i32);
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(3i32);
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(5i32);
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(6i32);
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(4i32);
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(13i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct DeviceDiscoveryMechanism(pub i32);
impl ::windows_core::TypeKind for DeviceDiscoveryMechanism {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceDiscoveryMechanism {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDiscoveryMechanism").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSDEventType(pub i32);
impl ::windows_core::TypeKind for WSDEventType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSDEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDEventType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSDUdpMessageType(pub i32);
impl ::windows_core::TypeKind for WSDUdpMessageType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSDUdpMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDUdpMessageType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSDXML_OP(pub i32);
impl ::windows_core::TypeKind for WSDXML_OP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSDXML_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDXML_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSD_CONFIG_PARAM_TYPE(pub i32);
impl ::windows_core::TypeKind for WSD_CONFIG_PARAM_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSD_CONFIG_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSD_PROTOCOL_TYPE(pub i32);
impl ::windows_core::TypeKind for WSD_PROTOCOL_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSD_PROTOCOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSD_PROTOCOL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_GetStatus {}
impl ::core::clone::Clone for REQUESTBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_GetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_GetStatus").field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for REQUESTBODY_GetStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for REQUESTBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
impl ::core::cmp::Eq for REQUESTBODY_GetStatus {}
impl ::core::default::Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Renew {}
impl ::core::clone::Clone for REQUESTBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_Renew {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Renew").field("Expires", &self.Expires).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for REQUESTBODY_Renew {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for REQUESTBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        self.Expires == other.Expires && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for REQUESTBODY_Renew {}
impl ::core::default::Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Subscribe {}
impl ::core::clone::Clone for REQUESTBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_Subscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Subscribe").field("EndTo", &self.EndTo).field("Delivery", &self.Delivery).field("Expires", &self.Expires).field("Filter", &self.Filter).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for REQUESTBODY_Subscribe {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for REQUESTBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.EndTo == other.EndTo && self.Delivery == other.Delivery && self.Expires == other.Expires && self.Filter == other.Filter && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for REQUESTBODY_Subscribe {}
impl ::core::default::Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Unsubscribe {}
impl ::core::clone::Clone for REQUESTBODY_Unsubscribe {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_Unsubscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Unsubscribe").field("any", &self.any).finish()
    }
}
impl ::windows_core::TypeKind for REQUESTBODY_Unsubscribe {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for REQUESTBODY_Unsubscribe {
    fn eq(&self, other: &Self) -> bool {
        self.any == other.any
    }
}
impl ::core::cmp::Eq for REQUESTBODY_Unsubscribe {}
impl ::core::default::Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
impl ::core::marker::Copy for RESPONSEBODY_GetMetadata {}
impl ::core::clone::Clone for RESPONSEBODY_GetMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_GetMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_GetMetadata").field("Metadata", &self.Metadata).finish()
    }
}
impl ::windows_core::TypeKind for RESPONSEBODY_GetMetadata {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_GetMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.Metadata == other.Metadata
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_GetMetadata {}
impl ::core::default::Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_GetStatus {}
impl ::core::clone::Clone for RESPONSEBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_GetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_GetStatus").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
impl ::windows_core::TypeKind for RESPONSEBODY_GetStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires && self.any == other.any
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_GetStatus {}
impl ::core::default::Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_Renew {}
impl ::core::clone::Clone for RESPONSEBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_Renew {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_Renew").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
impl ::windows_core::TypeKind for RESPONSEBODY_Renew {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires && self.any == other.any
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_Renew {}
impl ::core::default::Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_Subscribe {}
impl ::core::clone::Clone for RESPONSEBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_Subscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_Subscribe").field("SubscriptionManager", &self.SubscriptionManager).field("expires", &self.expires).field("any", &self.any).finish()
    }
}
impl ::windows_core::TypeKind for RESPONSEBODY_Subscribe {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.SubscriptionManager == other.SubscriptionManager && self.expires == other.expires && self.any == other.any
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_Subscribe {}
impl ::core::default::Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: ::windows_core::PCWSTR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_SubscriptionEnd {}
impl ::core::clone::Clone for RESPONSEBODY_SubscriptionEnd {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_SubscriptionEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_SubscriptionEnd").field("SubscriptionManager", &self.SubscriptionManager).field("Status", &self.Status).field("Reason", &self.Reason).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for RESPONSEBODY_SubscriptionEnd {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_SubscriptionEnd {
    fn eq(&self, other: &Self) -> bool {
        self.SubscriptionManager == other.SubscriptionManager && self.Status == other.Status && self.Reason == other.Reason && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_SubscriptionEnd {}
impl ::core::default::Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
impl ::core::marker::Copy for WSDUdpRetransmitParams {}
impl ::core::clone::Clone for WSDUdpRetransmitParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDUdpRetransmitParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDUdpRetransmitParams").field("ulSendDelay", &self.ulSendDelay).field("ulRepeat", &self.ulRepeat).field("ulRepeatMinDelay", &self.ulRepeatMinDelay).field("ulRepeatMaxDelay", &self.ulRepeatMaxDelay).field("ulRepeatUpperDelay", &self.ulRepeatUpperDelay).finish()
    }
}
impl ::windows_core::TypeKind for WSDUdpRetransmitParams {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDUdpRetransmitParams {
    fn eq(&self, other: &Self) -> bool {
        self.ulSendDelay == other.ulSendDelay && self.ulRepeat == other.ulRepeat && self.ulRepeatMinDelay == other.ulRepeatMinDelay && self.ulRepeatMaxDelay == other.ulRepeatMaxDelay && self.ulRepeatUpperDelay == other.ulRepeatUpperDelay
    }
}
impl ::core::cmp::Eq for WSDUdpRetransmitParams {}
impl ::core::default::Default for WSDUdpRetransmitParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_ATTRIBUTE {}
impl ::core::clone::Clone for WSDXML_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ATTRIBUTE").field("Element", &self.Element).field("Next", &self.Next).field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_ATTRIBUTE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.Element == other.Element && self.Next == other.Next && self.Name == other.Name && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for WSDXML_ATTRIBUTE {}
impl ::core::default::Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
impl ::core::marker::Copy for WSDXML_ELEMENT {}
impl ::core::clone::Clone for WSDXML_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ELEMENT").field("Node", &self.Node).field("Name", &self.Name).field("FirstAttribute", &self.FirstAttribute).field("FirstChild", &self.FirstChild).field("PrefixMappings", &self.PrefixMappings).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_ELEMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.Name == other.Name && self.FirstAttribute == other.FirstAttribute && self.FirstChild == other.FirstChild && self.PrefixMappings == other.PrefixMappings
    }
}
impl ::core::cmp::Eq for WSDXML_ELEMENT {}
impl ::core::default::Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSDXML_ELEMENT_LIST {}
impl ::core::clone::Clone for WSDXML_ELEMENT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ELEMENT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ELEMENT_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_ELEMENT_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_ELEMENT_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSDXML_ELEMENT_LIST {}
impl ::core::default::Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_NAME {}
impl ::core::clone::Clone for WSDXML_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NAME").field("Space", &self.Space).field("LocalName", &self.LocalName).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_NAME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Space == other.Space && self.LocalName == other.LocalName
    }
}
impl ::core::cmp::Eq for WSDXML_NAME {}
impl ::core::default::Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_NAMESPACE {
    pub Uri: ::windows_core::PCWSTR,
    pub PreferredPrefix: ::windows_core::PCWSTR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
impl ::core::marker::Copy for WSDXML_NAMESPACE {}
impl ::core::clone::Clone for WSDXML_NAMESPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NAMESPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NAMESPACE").field("Uri", &self.Uri).field("PreferredPrefix", &self.PreferredPrefix).field("Names", &self.Names).field("NamesCount", &self.NamesCount).field("Encoding", &self.Encoding).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_NAMESPACE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_NAMESPACE {
    fn eq(&self, other: &Self) -> bool {
        self.Uri == other.Uri && self.PreferredPrefix == other.PreferredPrefix && self.Names == other.Names && self.NamesCount == other.NamesCount && self.Encoding == other.Encoding
    }
}
impl ::core::cmp::Eq for WSDXML_NAMESPACE {}
impl ::core::default::Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
impl WSDXML_NODE {
    pub const ElementType: i32 = 0i32;
    pub const TextType: i32 = 1i32;
}
impl ::core::marker::Copy for WSDXML_NODE {}
impl ::core::clone::Clone for WSDXML_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NODE").field("Type", &self.Type).field("Parent", &self.Parent).field("Next", &self.Next).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_NODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Parent == other.Parent && self.Next == other.Next
    }
}
impl ::core::cmp::Eq for WSDXML_NODE {}
impl ::core::default::Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_PREFIX_MAPPING {}
impl ::core::clone::Clone for WSDXML_PREFIX_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_PREFIX_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_PREFIX_MAPPING").field("Refs", &self.Refs).field("Next", &self.Next).field("Space", &self.Space).field("Prefix", &self.Prefix).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_PREFIX_MAPPING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_PREFIX_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.Refs == other.Refs && self.Next == other.Next && self.Space == other.Space && self.Prefix == other.Prefix
    }
}
impl ::core::cmp::Eq for WSDXML_PREFIX_MAPPING {}
impl ::core::default::Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_TEXT {}
impl ::core::clone::Clone for WSDXML_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_TEXT").field("Node", &self.Node).field("Text", &self.Text).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_TEXT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.Text == other.Text
    }
}
impl ::core::cmp::Eq for WSDXML_TEXT {}
impl ::core::default::Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_TYPE {
    pub Uri: ::windows_core::PCWSTR,
    pub Table: *const u8,
}
impl ::core::marker::Copy for WSDXML_TYPE {}
impl ::core::clone::Clone for WSDXML_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_TYPE").field("Uri", &self.Uri).field("Table", &self.Table).finish()
    }
}
impl ::windows_core::TypeKind for WSDXML_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSDXML_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Uri == other.Uri && self.Table == other.Table
    }
}
impl ::core::cmp::Eq for WSDXML_TYPE {}
impl ::core::default::Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: ::windows_core::PCWSTR,
    pub MessageNumber: u64,
}
impl ::core::marker::Copy for WSD_APP_SEQUENCE {}
impl ::core::clone::Clone for WSD_APP_SEQUENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_APP_SEQUENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_APP_SEQUENCE").field("InstanceId", &self.InstanceId).field("SequenceId", &self.SequenceId).field("MessageNumber", &self.MessageNumber).finish()
    }
}
impl ::windows_core::TypeKind for WSD_APP_SEQUENCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_APP_SEQUENCE {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceId == other.InstanceId && self.SequenceId == other.SequenceId && self.MessageNumber == other.MessageNumber
    }
}
impl ::core::cmp::Eq for WSD_APP_SEQUENCE {}
impl ::core::default::Default for WSD_APP_SEQUENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_BYE {}
impl ::core::clone::Clone for WSD_BYE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_BYE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_BYE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_BYE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_BYE {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_BYE {}
impl ::core::default::Default for WSD_BYE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut ::core::option::Option<IWSDAddress>,
    pub dwAddressCount: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_ADDRESSES {}
impl ::core::clone::Clone for WSD_CONFIG_ADDRESSES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_ADDRESSES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_CONFIG_ADDRESSES").field("addresses", &self.addresses).field("dwAddressCount", &self.dwAddressCount).finish()
    }
}
impl ::windows_core::TypeKind for WSD_CONFIG_ADDRESSES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_CONFIG_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        self.addresses == other.addresses && self.dwAddressCount == other.dwAddressCount
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_ADDRESSES {}
impl ::core::default::Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut ::core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_PARAM {}
impl ::core::clone::Clone for WSD_CONFIG_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_CONFIG_PARAM").field("configParamType", &self.configParamType).field("pConfigData", &self.pConfigData).field("dwConfigDataSize", &self.dwConfigDataSize).finish()
    }
}
impl ::windows_core::TypeKind for WSD_CONFIG_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_CONFIG_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.configParamType == other.configParamType && self.pConfigData == other.pConfigData && self.dwConfigDataSize == other.dwConfigDataSize
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_PARAM {}
impl ::core::default::Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_DATETIME {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: super::super::Foundation::BOOL,
    pub TZIsPositive: super::super::Foundation::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
impl ::core::marker::Copy for WSD_DATETIME {}
impl ::core::clone::Clone for WSD_DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_DATETIME").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).field("TZIsLocal", &self.TZIsLocal).field("TZIsPositive", &self.TZIsPositive).field("TZHour", &self.TZHour).field("TZMinute", &self.TZMinute).finish()
    }
}
impl ::windows_core::TypeKind for WSD_DATETIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.isPositive == other.isPositive && self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.millisecond == other.millisecond && self.TZIsLocal == other.TZIsLocal && self.TZIsPositive == other.TZIsPositive && self.TZHour == other.TZHour && self.TZMinute == other.TZMinute
    }
}
impl ::core::cmp::Eq for WSD_DATETIME {}
impl ::core::default::Default for WSD_DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_DURATION {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
impl ::core::marker::Copy for WSD_DURATION {}
impl ::core::clone::Clone for WSD_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_DURATION").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).finish()
    }
}
impl ::windows_core::TypeKind for WSD_DURATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_DURATION {
    fn eq(&self, other: &Self) -> bool {
        self.isPositive == other.isPositive && self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.millisecond == other.millisecond
    }
}
impl ::core::cmp::Eq for WSD_DURATION {}
impl ::core::default::Default for WSD_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: ::windows_core::PCWSTR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_ENDPOINT_REFERENCE").field("Address", &self.Address).field("ReferenceProperties", &self.ReferenceProperties).field("ReferenceParameters", &self.ReferenceParameters).field("PortType", &self.PortType).field("ServiceName", &self.ServiceName).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_ENDPOINT_REFERENCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.ReferenceProperties == other.ReferenceProperties && self.ReferenceParameters == other.ReferenceParameters && self.PortType == other.PortType && self.ServiceName == other.ServiceName && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE {}
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_ENDPOINT_REFERENCE_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::windows_core::TypeKind for WSD_ENDPOINT_REFERENCE_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENT {
    pub Hr: ::windows_core::HRESULT,
    pub EventType: u32,
    pub DispatchTag: ::windows_core::PWSTR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: ::std::mem::ManuallyDrop<::core::option::Option<IWSDMessageParameters>>,
}
impl ::core::clone::Clone for WSD_EVENT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for WSD_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENT").field("Hr", &self.Hr).field("EventType", &self.EventType).field("DispatchTag", &self.DispatchTag).field("Soap", &self.Soap).field("Operation", &self.Operation).field("MessageParameters", &self.MessageParameters).finish()
    }
}
impl ::windows_core::TypeKind for WSD_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: ::windows_core::PCWSTR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_DELIVERY_MODE").field("Mode", &self.Mode).field("Push", &self.Push).field("Data", &self.Data).finish()
    }
}
impl ::windows_core::TypeKind for WSD_EVENTING_DELIVERY_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Push == other.Push && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_DELIVERY_MODE_PUSH").field("NotifyTo", &self.NotifyTo).finish()
    }
}
impl ::windows_core::TypeKind for WSD_EVENTING_DELIVERY_MODE_PUSH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn eq(&self, other: &Self) -> bool {
        self.NotifyTo == other.NotifyTo
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
impl ::core::marker::Copy for WSD_EVENTING_EXPIRES {}
impl ::core::clone::Clone for WSD_EVENTING_EXPIRES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_EXPIRES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_EXPIRES").field("Duration", &self.Duration).field("DateTime", &self.DateTime).finish()
    }
}
impl ::windows_core::TypeKind for WSD_EVENTING_EXPIRES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_EXPIRES {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration && self.DateTime == other.DateTime
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_EXPIRES {}
impl ::core::default::Default for WSD_EVENTING_EXPIRES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: ::windows_core::PCWSTR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_FILTER").field("Dialect", &self.Dialect).field("FilterAction", &self.FilterAction).field("Data", &self.Data).finish()
    }
}
impl ::windows_core::TypeKind for WSD_EVENTING_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.Dialect == other.Dialect && self.FilterAction == other.FilterAction && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_FILTER {}
impl ::core::default::Default for WSD_EVENTING_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER_ACTION {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_FILTER_ACTION").field("Actions", &self.Actions).finish()
    }
}
impl ::windows_core::TypeKind for WSD_EVENTING_FILTER_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER_ACTION {
    fn eq(&self, other: &Self) -> bool {
        self.Actions == other.Actions
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_FILTER_ACTION {}
impl ::core::default::Default for WSD_EVENTING_FILTER_ACTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut ::core::ffi::c_void,
    pub Unknown: ::std::mem::ManuallyDrop<::core::option::Option<::windows_core::IUnknown>>,
}
impl ::core::clone::Clone for WSD_HANDLER_CONTEXT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for WSD_HANDLER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HANDLER_CONTEXT").field("PVoid", &self.PVoid).field("Unknown", &self.Unknown).finish()
    }
}
impl ::windows_core::TypeKind for WSD_HANDLER_CONTEXT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSD_HEADER_RELATESTO {}
impl ::core::clone::Clone for WSD_HEADER_RELATESTO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HEADER_RELATESTO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HEADER_RELATESTO").field("RelationshipType", &self.RelationshipType).field("MessageID", &self.MessageID).finish()
    }
}
impl ::windows_core::TypeKind for WSD_HEADER_RELATESTO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_HEADER_RELATESTO {
    fn eq(&self, other: &Self) -> bool {
        self.RelationshipType == other.RelationshipType && self.MessageID == other.MessageID
    }
}
impl ::core::cmp::Eq for WSD_HEADER_RELATESTO {}
impl ::core::default::Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_HELLO {}
impl ::core::clone::Clone for WSD_HELLO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HELLO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HELLO").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_HELLO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_HELLO {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_HELLO {}
impl ::core::default::Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
impl ::core::marker::Copy for WSD_HOST_METADATA {}
impl ::core::clone::Clone for WSD_HOST_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HOST_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HOST_METADATA").field("Host", &self.Host).field("Hosted", &self.Hosted).finish()
    }
}
impl ::windows_core::TypeKind for WSD_HOST_METADATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_HOST_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Host == other.Host && self.Hosted == other.Hosted
    }
}
impl ::core::cmp::Eq for WSD_HOST_METADATA {}
impl ::core::default::Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_LOCALIZED_STRING {
    pub lang: ::windows_core::PCWSTR,
    pub String: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_LOCALIZED_STRING").field("lang", &self.lang).field("String", &self.String).finish()
    }
}
impl ::windows_core::TypeKind for WSD_LOCALIZED_STRING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang && self.String == other.String
    }
}
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING {}
impl ::core::default::Default for WSD_LOCALIZED_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING_LIST {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_LOCALIZED_STRING_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::windows_core::TypeKind for WSD_LOCALIZED_STRING_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING_LIST {}
impl ::core::default::Default for WSD_LOCALIZED_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_METADATA_SECTION {
    pub Dialect: ::windows_core::PCWSTR,
    pub Identifier: ::windows_core::PCWSTR,
    pub Data: *mut ::core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: ::windows_core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION {}
impl ::core::clone::Clone for WSD_METADATA_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_METADATA_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_METADATA_SECTION").field("Dialect", &self.Dialect).field("Identifier", &self.Identifier).field("Data", &self.Data).field("MetadataReference", &self.MetadataReference).field("Location", &self.Location).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_METADATA_SECTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION {
    fn eq(&self, other: &Self) -> bool {
        self.Dialect == other.Dialect && self.Identifier == other.Identifier && self.Data == other.Data && self.MetadataReference == other.MetadataReference && self.Location == other.Location && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_METADATA_SECTION {}
impl ::core::default::Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION_LIST {}
impl ::core::clone::Clone for WSD_METADATA_SECTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_METADATA_SECTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_METADATA_SECTION_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::windows_core::TypeKind for WSD_METADATA_SECTION_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_METADATA_SECTION_LIST {}
impl ::core::default::Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
impl ::core::marker::Copy for WSD_NAME_LIST {}
impl ::core::clone::Clone for WSD_NAME_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_NAME_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_NAME_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::windows_core::TypeKind for WSD_NAME_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_NAME_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_NAME_LIST {}
impl ::core::default::Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
impl ::core::marker::Copy for WSD_OPERATION {}
impl ::core::clone::Clone for WSD_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_OPERATION").field("RequestType", &self.RequestType).field("ResponseType", &self.ResponseType).finish()
    }
}
impl ::windows_core::TypeKind for WSD_OPERATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
impl ::core::marker::Copy for WSD_PORT_TYPE {}
impl ::core::clone::Clone for WSD_PORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PORT_TYPE").field("EncodedName", &self.EncodedName).field("OperationCount", &self.OperationCount).field("Operations", &self.Operations).field("ProtocolType", &self.ProtocolType).finish()
    }
}
impl ::windows_core::TypeKind for WSD_PORT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_PORT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.EncodedName == other.EncodedName && self.OperationCount == other.OperationCount && self.Operations == other.Operations && self.ProtocolType == other.ProtocolType
    }
}
impl ::core::cmp::Eq for WSD_PORT_TYPE {}
impl ::core::default::Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE {}
impl ::core::clone::Clone for WSD_PROBE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE").field("Types", &self.Types).field("Scopes", &self.Scopes).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_PROBE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_PROBE {
    fn eq(&self, other: &Self) -> bool {
        self.Types == other.Types && self.Scopes == other.Scopes && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_PROBE {}
impl ::core::default::Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH {}
impl ::core::clone::Clone for WSD_PROBE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_PROBE_MATCH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCH {}
impl ::core::default::Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCHES {}
impl ::core::clone::Clone for WSD_PROBE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCHES").field("ProbeMatch", &self.ProbeMatch).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_PROBE_MATCHES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        self.ProbeMatch == other.ProbeMatch && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCHES {}
impl ::core::default::Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH_LIST {}
impl ::core::clone::Clone for WSD_PROBE_MATCH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCH_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::windows_core::TypeKind for WSD_PROBE_MATCH_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCH_LIST {}
impl ::core::default::Default for WSD_PROBE_MATCH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PARAMETERS {}
impl ::core::clone::Clone for WSD_REFERENCE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_REFERENCE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_REFERENCE_PARAMETERS").field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_REFERENCE_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_REFERENCE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_REFERENCE_PARAMETERS {}
impl ::core::default::Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PROPERTIES {}
impl ::core::clone::Clone for WSD_REFERENCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_REFERENCE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_REFERENCE_PROPERTIES").field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_REFERENCE_PROPERTIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_REFERENCE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_REFERENCE_PROPERTIES {}
impl ::core::default::Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: ::windows_core::PCWSTR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RELATIONSHIP_METADATA {}
impl ::core::clone::Clone for WSD_RELATIONSHIP_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RELATIONSHIP_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RELATIONSHIP_METADATA").field("Type", &self.Type).field("Data", &self.Data).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_RELATIONSHIP_METADATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_RELATIONSHIP_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Data == other.Data && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_RELATIONSHIP_METADATA {}
impl ::core::default::Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE {}
impl ::core::clone::Clone for WSD_RESOLVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_RESOLVE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE {}
impl ::core::default::Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCH {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_RESOLVE_MATCH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE_MATCH {}
impl ::core::default::Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCHES {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE_MATCHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE_MATCHES").field("ResolveMatch", &self.ResolveMatch).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_RESOLVE_MATCHES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        self.ResolveMatch == other.ResolveMatch && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE_MATCHES {}
impl ::core::default::Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SCOPES {
    pub MatchBy: ::windows_core::PCWSTR,
    pub Scopes: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_SCOPES {}
impl ::core::clone::Clone for WSD_SCOPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SCOPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SCOPES").field("MatchBy", &self.MatchBy).field("Scopes", &self.Scopes).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SCOPES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SCOPES {
    fn eq(&self, other: &Self) -> bool {
        self.MatchBy == other.MatchBy && self.Scopes == other.Scopes
    }
}
impl ::core::cmp::Eq for WSD_SCOPES {}
impl ::core::default::Default for WSD_SCOPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Security_Cryptography\"`"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: ::windows_core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_CERT_VALIDATION").field("certMatchArray", &self.certMatchArray).field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount).field("hCertMatchStore", &self.hCertMatchStore).field("hCertIssuerStore", &self.hCertIssuerStore).field("dwCertCheckOptions", &self.dwCertCheckOptions).field("pszCNGHashAlgId", &self.pszCNGHashAlgId).field("pbCertHash", &self.pbCertHash).field("dwCertHashSize", &self.dwCertHashSize).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::windows_core::TypeKind for WSD_SECURITY_CERT_VALIDATION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        self.certMatchArray == other.certMatchArray && self.dwCertMatchArrayCount == other.dwCertMatchArrayCount && self.hCertMatchStore == other.hCertMatchStore && self.hCertIssuerStore == other.hCertIssuerStore && self.dwCertCheckOptions == other.dwCertCheckOptions && self.pszCNGHashAlgId == other.pszCNGHashAlgId && self.pbCertHash == other.pbCertHash && self.dwCertHashSize == other.dwCertHashSize
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Security_Cryptography\"`"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_CERT_VALIDATION_V1").field("certMatchArray", &self.certMatchArray).field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount).field("hCertMatchStore", &self.hCertMatchStore).field("hCertIssuerStore", &self.hCertIssuerStore).field("dwCertCheckOptions", &self.dwCertCheckOptions).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::windows_core::TypeKind for WSD_SECURITY_CERT_VALIDATION_V1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.certMatchArray == other.certMatchArray && self.dwCertMatchArrayCount == other.dwCertMatchArrayCount && self.hCertMatchStore == other.hCertMatchStore && self.hCertIssuerStore == other.hCertIssuerStore && self.dwCertCheckOptions == other.dwCertCheckOptions
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Security_Cryptography\"`"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_SIGNATURE_VALIDATION").field("signingCertArray", &self.signingCertArray).field("dwSigningCertArrayCount", &self.dwSigningCertArrayCount).field("hSigningCertStore", &self.hSigningCertStore).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::windows_core::TypeKind for WSD_SECURITY_SIGNATURE_VALIDATION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        self.signingCertArray == other.signingCertArray && self.dwSigningCertArrayCount == other.dwSigningCertArrayCount && self.hSigningCertStore == other.hSigningCertStore && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: ::windows_core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SERVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SERVICE_METADATA").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("ServiceId", &self.ServiceId).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SERVICE_METADATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.ServiceId == other.ServiceId && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_SERVICE_METADATA {}
impl ::core::default::Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA_LIST {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SERVICE_METADATA_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SERVICE_METADATA_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SERVICE_METADATA_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_SERVICE_METADATA_LIST {}
impl ::core::default::Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: ::windows_core::PCWSTR,
    pub Role: ::windows_core::PCWSTR,
    pub Detail: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT {}
impl ::core::clone::Clone for WSD_SOAP_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT").field("Code", &self.Code).field("Reason", &self.Reason).field("Node", &self.Node).field("Role", &self.Role).field("Detail", &self.Detail).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SOAP_FAULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT {
    fn eq(&self, other: &Self) -> bool {
        self.Code == other.Code && self.Reason == other.Reason && self.Node == other.Node && self.Role == other.Role && self.Detail == other.Detail
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT {}
impl ::core::default::Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_CODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_CODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SOAP_FAULT_CODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Subcode == other.Subcode
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_CODE {}
impl ::core::default::Default for WSD_SOAP_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_REASON {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_REASON").field("Text", &self.Text).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SOAP_FAULT_REASON {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.Text == other.Text
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_REASON {}
impl ::core::default::Default for WSD_SOAP_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_SUBCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_SUBCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_SUBCODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SOAP_FAULT_SUBCODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_SUBCODE {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Subcode == other.Subcode
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::default::Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_HEADER {
    pub To: ::windows_core::PCWSTR,
    pub Action: ::windows_core::PCWSTR,
    pub MessageID: ::windows_core::PCWSTR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_HEADER {}
impl ::core::clone::Clone for WSD_SOAP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_HEADER").field("To", &self.To).field("Action", &self.Action).field("MessageID", &self.MessageID).field("RelatesTo", &self.RelatesTo).field("ReplyTo", &self.ReplyTo).field("From", &self.From).field("FaultTo", &self.FaultTo).field("AppSequence", &self.AppSequence).field("AnyHeaders", &self.AnyHeaders).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SOAP_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SOAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.To == other.To && self.Action == other.Action && self.MessageID == other.MessageID && self.RelatesTo == other.RelatesTo && self.ReplyTo == other.ReplyTo && self.From == other.From && self.FaultTo == other.FaultTo && self.AppSequence == other.AppSequence && self.AnyHeaders == other.AnyHeaders
    }
}
impl ::core::cmp::Eq for WSD_SOAP_HEADER {}
impl ::core::default::Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut ::core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
impl ::core::marker::Copy for WSD_SOAP_MESSAGE {}
impl ::core::clone::Clone for WSD_SOAP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_MESSAGE").field("Header", &self.Header).field("Body", &self.Body).field("BodyType", &self.BodyType).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SOAP_MESSAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SOAP_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Body == other.Body && self.BodyType == other.BodyType
    }
}
impl ::core::cmp::Eq for WSD_SOAP_MESSAGE {}
impl ::core::default::Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: ::windows_core::HRESULT,
    pub eventHandle: super::super::Foundation::HANDLE,
    pub messageParameters: ::std::mem::ManuallyDrop<::core::option::Option<IWSDMessageParameters>>,
    pub results: *mut ::core::ffi::c_void,
}
impl ::core::clone::Clone for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SYNCHRONOUS_RESPONSE_CONTEXT").field("hr", &self.hr).field("eventHandle", &self.eventHandle).field("messageParameters", &self.messageParameters).field("results", &self.results).finish()
    }
}
impl ::windows_core::TypeKind for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.eventHandle == other.eventHandle && self.messageParameters == other.messageParameters && self.results == other.results
    }
}
impl ::core::cmp::Eq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
impl ::core::default::Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: ::windows_core::PCWSTR,
    pub SerialNumber: ::windows_core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_DEVICE_METADATA {}
impl ::core::clone::Clone for WSD_THIS_DEVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_THIS_DEVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_THIS_DEVICE_METADATA").field("FriendlyName", &self.FriendlyName).field("FirmwareVersion", &self.FirmwareVersion).field("SerialNumber", &self.SerialNumber).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_THIS_DEVICE_METADATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_THIS_DEVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.FriendlyName == other.FriendlyName && self.FirmwareVersion == other.FirmwareVersion && self.SerialNumber == other.SerialNumber && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_THIS_DEVICE_METADATA {}
impl ::core::default::Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: ::windows_core::PCWSTR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: ::windows_core::PCWSTR,
    pub ModelUrl: ::windows_core::PCWSTR,
    pub PresentationUrl: ::windows_core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_MODEL_METADATA {}
impl ::core::clone::Clone for WSD_THIS_MODEL_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_THIS_MODEL_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_THIS_MODEL_METADATA").field("Manufacturer", &self.Manufacturer).field("ManufacturerUrl", &self.ManufacturerUrl).field("ModelName", &self.ModelName).field("ModelNumber", &self.ModelNumber).field("ModelUrl", &self.ModelUrl).field("PresentationUrl", &self.PresentationUrl).field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_THIS_MODEL_METADATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_THIS_MODEL_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer && self.ManufacturerUrl == other.ManufacturerUrl && self.ModelName == other.ModelName && self.ModelNumber == other.ModelNumber && self.ModelUrl == other.ModelUrl && self.PresentationUrl == other.PresentationUrl && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_THIS_MODEL_METADATA {}
impl ::core::default::Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_UNKNOWN_LOOKUP {}
impl ::core::clone::Clone for WSD_UNKNOWN_LOOKUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_UNKNOWN_LOOKUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_UNKNOWN_LOOKUP").field("Any", &self.Any).finish()
    }
}
impl ::windows_core::TypeKind for WSD_UNKNOWN_LOOKUP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_UNKNOWN_LOOKUP {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_UNKNOWN_LOOKUP {}
impl ::core::default::Default for WSD_UNKNOWN_LOOKUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSD_URI_LIST {}
impl ::core::clone::Clone for WSD_URI_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_URI_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_URI_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::windows_core::TypeKind for WSD_URI_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSD_URI_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_URI_LIST {}
impl ::core::default::Default for WSD_URI_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type PWSD_SOAP_MESSAGE_HANDLER = ::core::option::Option<unsafe extern "system" fn(thisunknown: ::core::option::Option<::windows_core::IUnknown>, event: *mut WSD_EVENT) -> ::windows_core::HRESULT>;
pub type WSD_STUB_FUNCTION = ::core::option::Option<unsafe extern "system" fn(server: ::core::option::Option<::windows_core::IUnknown>, session: ::core::option::Option<IWSDServiceMessaging>, event: *mut WSD_EVENT) -> ::windows_core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
