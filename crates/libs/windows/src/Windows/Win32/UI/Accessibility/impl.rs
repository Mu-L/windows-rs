pub trait IAccIdentity_Impl: Sized {
    fn GetIdentityString(&self, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IAccIdentity {}
impl IAccIdentity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccIdentity_Impl, const OFFSET: isize>() -> IAccIdentity_Vtbl {
        unsafe extern "system" fn GetIdentityString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIdentityString(::core::mem::transmute_copy(&dwidchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdentityString: GetIdentityString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccIdentity as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAccPropServer_Impl: Sized {
    fn GetPropValue(&self, pidstring: *const u8, dwidstringlen: u32, idprop: &::windows_core::GUID, pvarvalue: *mut super::super::System::Variant::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAccPropServer {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAccPropServer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServer_Impl, const OFFSET: isize>() -> IAccPropServer_Vtbl {
        unsafe extern "system" fn GetPropValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows_core::GUID, pvarvalue: *mut super::super::System::Variant::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropValue(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute(&idprop), ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pfhasprop)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPropValue: GetPropValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccPropServer as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IAccPropServices_Impl: Sized {
    fn SetPropValue(&self, pidstring: *const u8, dwidstringlen: u32, idprop: &::windows_core::GUID, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetPropServer(&self, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: ::core::option::Option<&IAccPropServer>, annoscope: AnnoScope) -> ::windows_core::Result<()>;
    fn ClearProps(&self, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::Result<()>;
    fn SetHwndProp(&self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: &::windows_core::GUID, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetHwndPropStr(&self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: &::windows_core::GUID, str: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHwndPropServer(&self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: ::core::option::Option<&IAccPropServer>, annoscope: AnnoScope) -> ::windows_core::Result<()>;
    fn ClearHwndProps(&self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::Result<()>;
    fn ComposeHwndIdentityString(&self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::Result<()>;
    fn DecomposeHwndIdentityString(&self, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows_core::Result<()>;
    fn SetHmenuProp(&self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: &::windows_core::GUID, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetHmenuPropStr(&self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: &::windows_core::GUID, str: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHmenuPropServer(&self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: ::core::option::Option<&IAccPropServer>, annoscope: AnnoScope) -> ::windows_core::Result<()>;
    fn ClearHmenuProps(&self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::Result<()>;
    fn ComposeHmenuIdentityString(&self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::Result<()>;
    fn DecomposeHmenuIdentityString(&self, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IAccPropServices {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl IAccPropServices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>() -> IAccPropServices_Vtbl {
        unsafe extern "system" fn SetPropValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows_core::GUID, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropValue(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute(&idprop), ::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn SetPropServer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropServer(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::windows_core::from_raw_borrowed(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearProps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearProps(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn SetHwndProp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows_core::GUID, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHwndProp(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&idprop), ::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn SetHwndPropStr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows_core::GUID, str: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHwndPropStr(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&idprop), ::core::mem::transmute(&str)).into()
        }
        unsafe extern "system" fn SetHwndPropServer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHwndPropServer(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::windows_core::from_raw_borrowed(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearHwndProps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearHwndProps(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn ComposeHwndIdentityString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ComposeHwndIdentityString(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        unsafe extern "system" fn DecomposeHwndIdentityString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DecomposeHwndIdentityString(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pidobject), ::core::mem::transmute_copy(&pidchild)).into()
        }
        unsafe extern "system" fn SetHmenuProp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows_core::GUID, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHmenuProp(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&idprop), ::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn SetHmenuPropStr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows_core::GUID, str: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHmenuPropStr(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&idprop), ::core::mem::transmute(&str)).into()
        }
        unsafe extern "system" fn SetHmenuPropServer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHmenuPropServer(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::windows_core::from_raw_borrowed(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearHmenuProps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearHmenuProps(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn ComposeHmenuIdentityString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ComposeHmenuIdentityString(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        unsafe extern "system" fn DecomposeHmenuIdentityString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DecomposeHmenuIdentityString(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&phmenu), ::core::mem::transmute_copy(&pidchild)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPropValue: SetPropValue::<Identity, Impl, OFFSET>,
            SetPropServer: SetPropServer::<Identity, Impl, OFFSET>,
            ClearProps: ClearProps::<Identity, Impl, OFFSET>,
            SetHwndProp: SetHwndProp::<Identity, Impl, OFFSET>,
            SetHwndPropStr: SetHwndPropStr::<Identity, Impl, OFFSET>,
            SetHwndPropServer: SetHwndPropServer::<Identity, Impl, OFFSET>,
            ClearHwndProps: ClearHwndProps::<Identity, Impl, OFFSET>,
            ComposeHwndIdentityString: ComposeHwndIdentityString::<Identity, Impl, OFFSET>,
            DecomposeHwndIdentityString: DecomposeHwndIdentityString::<Identity, Impl, OFFSET>,
            SetHmenuProp: SetHmenuProp::<Identity, Impl, OFFSET>,
            SetHmenuPropStr: SetHmenuPropStr::<Identity, Impl, OFFSET>,
            SetHmenuPropServer: SetHmenuPropServer::<Identity, Impl, OFFSET>,
            ClearHmenuProps: ClearHmenuProps::<Identity, Impl, OFFSET>,
            ComposeHmenuIdentityString: ComposeHmenuIdentityString::<Identity, Impl, OFFSET>,
            DecomposeHmenuIdentityString: DecomposeHmenuIdentityString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccPropServices as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAccessible_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn accParent(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn accChildCount(&self) -> ::windows_core::Result<i32>;
    fn get_accChild(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn get_accName(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_accValue(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_accDescription(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_accRole(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_accState(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_accHelp(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_accHelpTopic(&self, pszhelpfile: *mut ::windows_core::BSTR, varchild: &super::super::System::Variant::VARIANT, pidtopic: *mut i32) -> ::windows_core::Result<()>;
    fn get_accKeyboardShortcut(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn accFocus(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn accSelection(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_accDefaultAction(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn accSelect(&self, flagsselect: i32, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn accLocation(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn accNavigate(&self, navdir: i32, varstart: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn accHitTest(&self, xleft: i32, ytop: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn accDoDefaultAction(&self, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn put_accName(&self, varchild: &super::super::System::Variant::VARIANT, szname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn put_accValue(&self, varchild: &super::super::System::Variant::VARIANT, szvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAccessible {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAccessible_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>() -> IAccessible_Vtbl {
        unsafe extern "system" fn accParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdispparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.accParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accChildCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcountchildren: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.accChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcountchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, ppdispchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accChild(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accName(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accValue(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accDescription(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pvarrole: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accRole(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pvarstate: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accState(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accHelp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszhelp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accHelp(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszhelp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accHelpTopic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, varchild: super::super::System::Variant::VARIANT, pidtopic: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_accHelpTopic(::core::mem::transmute_copy(&pszhelpfile), ::core::mem::transmute(&varchild), ::core::mem::transmute_copy(&pidtopic)).into()
        }
        unsafe extern "system" fn get_accKeyboardShortcut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszkeyboardshortcut: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accKeyboardShortcut(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszkeyboardshortcut, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchild: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.accFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchildren: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.accSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_accDefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszdefaultaction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_accDefaultAction(::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdefaultaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accSelect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32, varchild: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.accSelect(::core::mem::transmute_copy(&flagsselect), ::core::mem::transmute(&varchild)).into()
        }
        unsafe extern "system" fn accLocation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.accLocation(::core::mem::transmute_copy(&pxleft), ::core::mem::transmute_copy(&pytop), ::core::mem::transmute_copy(&pcxwidth), ::core::mem::transmute_copy(&pcyheight), ::core::mem::transmute(&varchild)).into()
        }
        unsafe extern "system" fn accNavigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navdir: i32, varstart: super::super::System::Variant::VARIANT, pvarendupat: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.accNavigate(::core::mem::transmute_copy(&navdir), ::core::mem::transmute(&varstart)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarendupat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHitTest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xleft: i32, ytop: i32, pvarchild: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.accHitTest(::core::mem::transmute_copy(&xleft), ::core::mem::transmute_copy(&ytop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDoDefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.accDoDefaultAction(::core::mem::transmute(&varchild)).into()
        }
        unsafe extern "system" fn put_accName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, szname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_accName(::core::mem::transmute(&varchild), ::core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn put_accValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, szvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_accValue(::core::mem::transmute(&varchild), ::core::mem::transmute(&szvalue)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            accParent: accParent::<Identity, Impl, OFFSET>,
            accChildCount: accChildCount::<Identity, Impl, OFFSET>,
            get_accChild: get_accChild::<Identity, Impl, OFFSET>,
            get_accName: get_accName::<Identity, Impl, OFFSET>,
            get_accValue: get_accValue::<Identity, Impl, OFFSET>,
            get_accDescription: get_accDescription::<Identity, Impl, OFFSET>,
            get_accRole: get_accRole::<Identity, Impl, OFFSET>,
            get_accState: get_accState::<Identity, Impl, OFFSET>,
            get_accHelp: get_accHelp::<Identity, Impl, OFFSET>,
            get_accHelpTopic: get_accHelpTopic::<Identity, Impl, OFFSET>,
            get_accKeyboardShortcut: get_accKeyboardShortcut::<Identity, Impl, OFFSET>,
            accFocus: accFocus::<Identity, Impl, OFFSET>,
            accSelection: accSelection::<Identity, Impl, OFFSET>,
            get_accDefaultAction: get_accDefaultAction::<Identity, Impl, OFFSET>,
            accSelect: accSelect::<Identity, Impl, OFFSET>,
            accLocation: accLocation::<Identity, Impl, OFFSET>,
            accNavigate: accNavigate::<Identity, Impl, OFFSET>,
            accHitTest: accHitTest::<Identity, Impl, OFFSET>,
            accDoDefaultAction: accDoDefaultAction::<Identity, Impl, OFFSET>,
            put_accName: put_accName::<Identity, Impl, OFFSET>,
            put_accValue: put_accValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccessible as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleEx_Impl: Sized {
    fn GetObjectForChild(&self, idchild: i32) -> ::windows_core::Result<IAccessibleEx>;
    fn GetIAccessiblePair(&self, ppacc: *mut ::core::option::Option<IAccessible>, pidchild: *mut i32) -> ::windows_core::Result<()>;
    fn GetRuntimeId(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ConvertReturnedElement(&self, pin: ::core::option::Option<&IRawElementProviderSimple>) -> ::windows_core::Result<IAccessibleEx>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAccessibleEx {}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: isize>() -> IAccessibleEx_Vtbl {
        unsafe extern "system" fn GetObjectForChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idchild: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObjectForChild(::core::mem::transmute_copy(&idchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessiblePair<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacc: *mut *mut ::core::ffi::c_void, pidchild: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIAccessiblePair(::core::mem::transmute_copy(&ppacc), ::core::mem::transmute_copy(&pidchild)).into()
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertReturnedElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, ppretvalout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConvertReturnedElement(::windows_core::from_raw_borrowed(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppretvalout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectForChild: GetObjectForChild::<Identity, Impl, OFFSET>,
            GetIAccessiblePair: GetIAccessiblePair::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            ConvertReturnedElement: ConvertReturnedElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccessibleEx as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleHandler_Impl: Sized {
    fn AccessibleObjectFromID(&self, hwnd: i32, lobjectid: i32) -> ::windows_core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAccessibleHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleHandler_Impl, const OFFSET: isize>() -> IAccessibleHandler_Vtbl {
        unsafe extern "system" fn AccessibleObjectFromID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, lobjectid: i32, piaccessible: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessibleObjectFromID(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&lobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piaccessible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AccessibleObjectFromID: AccessibleObjectFromID::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccessibleHandler as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleHostingElementProviders_Impl: Sized {
    fn GetEmbeddedFragmentRoots(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetObjectIdForProvider(&self, pprovider: ::core::option::Option<&IRawElementProviderSimple>) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAccessibleHostingElementProviders {}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleHostingElementProviders_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>() -> IAccessibleHostingElementProviders_Vtbl {
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEmbeddedFragmentRoots() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectIdForProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, pidobject: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObjectIdForProvider(::windows_core::from_raw_borrowed(&pprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pidobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Identity, Impl, OFFSET>,
            GetObjectIdForProvider: GetObjectIdForProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccessibleHostingElementProviders as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleWindowlessSite_Impl: Sized {
    fn AcquireObjectIdRange(&self, rangesize: i32, prangeowner: ::core::option::Option<&IAccessibleHandler>) -> ::windows_core::Result<i32>;
    fn ReleaseObjectIdRange(&self, rangebase: i32, prangeowner: ::core::option::Option<&IAccessibleHandler>) -> ::windows_core::Result<()>;
    fn QueryObjectIdRanges(&self, prangesowner: ::core::option::Option<&IAccessibleHandler>) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetParentAccessible(&self) -> ::windows_core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAccessibleWindowlessSite {}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleWindowlessSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>() -> IAccessibleWindowlessSite_Vtbl {
        unsafe extern "system" fn AcquireObjectIdRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangesize: i32, prangeowner: *mut ::core::ffi::c_void, prangebase: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AcquireObjectIdRange(::core::mem::transmute_copy(&rangesize), ::windows_core::from_raw_borrowed(&prangeowner)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prangebase, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseObjectIdRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangebase: i32, prangeowner: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseObjectIdRange(::core::mem::transmute_copy(&rangebase), ::windows_core::from_raw_borrowed(&prangeowner)).into()
        }
        unsafe extern "system" fn QueryObjectIdRanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangesowner: *mut ::core::ffi::c_void, psaranges: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryObjectIdRanges(::windows_core::from_raw_borrowed(&prangesowner)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psaranges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentAccessible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireObjectIdRange: AcquireObjectIdRange::<Identity, Impl, OFFSET>,
            ReleaseObjectIdRange: ReleaseObjectIdRange::<Identity, Impl, OFFSET>,
            QueryObjectIdRanges: QueryObjectIdRanges::<Identity, Impl, OFFSET>,
            GetParentAccessible: GetParentAccessible::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccessibleWindowlessSite as ::windows_core::Interface>::IID
    }
}
pub trait IAnnotationProvider_Impl: Sized {
    fn AnnotationTypeId(&self) -> ::windows_core::Result<UIA_ANNOTATIONTYPE>;
    fn AnnotationTypeName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Author(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DateTime(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Target(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::RuntimeName for IAnnotationProvider {}
impl IAnnotationProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: isize>() -> IAnnotationProvider_Vtbl {
        unsafe extern "system" fn AnnotationTypeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnnotationTypeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Author() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Target<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Target() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AnnotationTypeId: AnnotationTypeId::<Identity, Impl, OFFSET>,
            AnnotationTypeName: AnnotationTypeName::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            DateTime: DateTime::<Identity, Impl, OFFSET>,
            Target: Target::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAnnotationProvider as ::windows_core::Interface>::IID
    }
}
pub trait ICustomNavigationProvider_Impl: Sized {
    fn Navigate(&self, direction: NavigateDirection) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::RuntimeName for ICustomNavigationProvider {}
impl ICustomNavigationProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomNavigationProvider_Impl, const OFFSET: isize>() -> ICustomNavigationProvider_Vtbl {
        unsafe extern "system" fn Navigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomNavigationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Navigate: Navigate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICustomNavigationProvider as ::windows_core::Interface>::IID
    }
}
pub trait IDockProvider_Impl: Sized {
    fn SetDockPosition(&self, dockposition: DockPosition) -> ::windows_core::Result<()>;
    fn DockPosition(&self) -> ::windows_core::Result<DockPosition>;
}
impl ::windows_core::RuntimeName for IDockProvider {}
impl IDockProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDockProvider_Impl, const OFFSET: isize>() -> IDockProvider_Vtbl {
        unsafe extern "system" fn SetDockPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDockProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockposition: DockPosition) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDockPosition(::core::mem::transmute_copy(&dockposition)).into()
        }
        unsafe extern "system" fn DockPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDockProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut DockPosition) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDockPosition: SetDockPosition::<Identity, Impl, OFFSET>,
            DockPosition: DockPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDockProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDragProvider_Impl: Sized {
    fn IsGrabbed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DropEffect(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DropEffects(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetGrabbedItems(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDragProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IDragProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: isize>() -> IDragProvider_Vtbl {
        unsafe extern "system" fn IsGrabbed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrabbedItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsGrabbed: IsGrabbed::<Identity, Impl, OFFSET>,
            DropEffect: DropEffect::<Identity, Impl, OFFSET>,
            DropEffects: DropEffects::<Identity, Impl, OFFSET>,
            GetGrabbedItems: GetGrabbedItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDragProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDropTargetProvider_Impl: Sized {
    fn DropTargetEffect(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DropTargetEffects(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDropTargetProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IDropTargetProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropTargetProvider_Impl, const OFFSET: isize>() -> IDropTargetProvider_Vtbl {
        unsafe extern "system" fn DropTargetEffect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropTargetEffects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DropTargetEffect: DropTargetEffect::<Identity, Impl, OFFSET>,
            DropTargetEffects: DropTargetEffects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDropTargetProvider as ::windows_core::Interface>::IID
    }
}
pub trait IExpandCollapseProvider_Impl: Sized {
    fn Expand(&self) -> ::windows_core::Result<()>;
    fn Collapse(&self) -> ::windows_core::Result<()>;
    fn ExpandCollapseState(&self) -> ::windows_core::Result<ExpandCollapseState>;
}
impl ::windows_core::RuntimeName for IExpandCollapseProvider {}
impl IExpandCollapseProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>() -> IExpandCollapseProvider_Vtbl {
        unsafe extern "system" fn Expand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Expand().into()
        }
        unsafe extern "system" fn Collapse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Collapse().into()
        }
        unsafe extern "system" fn ExpandCollapseState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ExpandCollapseState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Expand: Expand::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            ExpandCollapseState: ExpandCollapseState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IExpandCollapseProvider as ::windows_core::Interface>::IID
    }
}
pub trait IGridItemProvider_Impl: Sized {
    fn Row(&self) -> ::windows_core::Result<i32>;
    fn Column(&self) -> ::windows_core::Result<i32>;
    fn RowSpan(&self) -> ::windows_core::Result<i32>;
    fn ColumnSpan(&self) -> ::windows_core::Result<i32>;
    fn ContainingGrid(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::RuntimeName for IGridItemProvider {}
impl IGridItemProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: isize>() -> IGridItemProvider_Vtbl {
        unsafe extern "system" fn Row<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Row() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Column<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Column() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowSpan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnSpan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainingGrid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Row: Row::<Identity, Impl, OFFSET>,
            Column: Column::<Identity, Impl, OFFSET>,
            RowSpan: RowSpan::<Identity, Impl, OFFSET>,
            ColumnSpan: ColumnSpan::<Identity, Impl, OFFSET>,
            ContainingGrid: ContainingGrid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGridItemProvider as ::windows_core::Interface>::IID
    }
}
pub trait IGridProvider_Impl: Sized {
    fn GetItem(&self, row: i32, column: i32) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn RowCount(&self) -> ::windows_core::Result<i32>;
    fn ColumnCount(&self) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IGridProvider {}
impl IGridProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridProvider_Impl, const OFFSET: isize>() -> IGridProvider_Vtbl {
        unsafe extern "system" fn GetItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItem(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RowCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            RowCount: RowCount::<Identity, Impl, OFFSET>,
            ColumnCount: ColumnCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGridProvider as ::windows_core::Interface>::IID
    }
}
pub trait IInvokeProvider_Impl: Sized {
    fn Invoke(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IInvokeProvider {}
impl IInvokeProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInvokeProvider_Impl, const OFFSET: isize>() -> IInvokeProvider_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInvokeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInvokeProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IItemContainerProvider_Impl: Sized {
    fn FindItemByProperty(&self, pstartafter: ::core::option::Option<&IRawElementProviderSimple>, propertyid: UIA_PROPERTY_ID, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IItemContainerProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IItemContainerProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IItemContainerProvider_Impl, const OFFSET: isize>() -> IItemContainerProvider_Vtbl {
        unsafe extern "system" fn FindItemByProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IItemContainerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Variant::VARIANT, pfound: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindItemByProperty(::windows_core::from_raw_borrowed(&pstartafter), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindItemByProperty: FindItemByProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IItemContainerProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ILegacyIAccessibleProvider_Impl: Sized {
    fn Select(&self, flagsselect: i32) -> ::windows_core::Result<()>;
    fn DoDefaultAction(&self) -> ::windows_core::Result<()>;
    fn SetValue(&self, szvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetIAccessible(&self) -> ::windows_core::Result<IAccessible>;
    fn ChildId(&self) -> ::windows_core::Result<i32>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Value(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Role(&self) -> ::windows_core::Result<u32>;
    fn State(&self) -> ::windows_core::Result<u32>;
    fn Help(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn KeyboardShortcut(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSelection(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DefaultAction(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ILegacyIAccessibleProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ILegacyIAccessibleProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>() -> ILegacyIAccessibleProvider_Vtbl {
        unsafe extern "system" fn Select<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Select(::core::mem::transmute_copy(&flagsselect)).into()
        }
        unsafe extern "system" fn DoDefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoDefaultAction().into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&szvalue)).into()
        }
        unsafe extern "system" fn GetIAccessible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccessible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ChildId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Role<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Role() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Help<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Help() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszhelp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardShortcut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszkeyboardshortcut, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselectedchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdefaultaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, Impl, OFFSET>,
            DoDefaultAction: DoDefaultAction::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetIAccessible: GetIAccessible::<Identity, Impl, OFFSET>,
            ChildId: ChildId::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Help: Help::<Identity, Impl, OFFSET>,
            KeyboardShortcut: KeyboardShortcut::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            DefaultAction: DefaultAction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILegacyIAccessibleProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMultipleViewProvider_Impl: Sized {
    fn GetViewName(&self, viewid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCurrentView(&self, viewid: i32) -> ::windows_core::Result<()>;
    fn CurrentView(&self) -> ::windows_core::Result<i32>;
    fn GetSupportedViews(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IMultipleViewProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IMultipleViewProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>() -> IMultipleViewProvider_Vtbl {
        unsafe extern "system" fn GetViewName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewName(::core::mem::transmute_copy(&viewid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentView(::core::mem::transmute_copy(&viewid)).into()
        }
        unsafe extern "system" fn CurrentView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedViews<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetViewName: GetViewName::<Identity, Impl, OFFSET>,
            SetCurrentView: SetCurrentView::<Identity, Impl, OFFSET>,
            CurrentView: CurrentView::<Identity, Impl, OFFSET>,
            GetSupportedViews: GetSupportedViews::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMultipleViewProvider as ::windows_core::Interface>::IID
    }
}
pub trait IObjectModelProvider_Impl: Sized {
    fn GetUnderlyingObjectModel(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for IObjectModelProvider {}
impl IObjectModelProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectModelProvider_Impl, const OFFSET: isize>() -> IObjectModelProvider_Vtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectModelProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUnderlyingObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectModelProvider as ::windows_core::Interface>::IID
    }
}
pub trait IProxyProviderWinEventHandler_Impl: Sized {
    fn RespondToWinEvent(&self, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: ::core::option::Option<&IProxyProviderWinEventSink>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IProxyProviderWinEventHandler {}
impl IProxyProviderWinEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>() -> IProxyProviderWinEventHandler_Vtbl {
        unsafe extern "system" fn RespondToWinEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RespondToWinEvent(::core::mem::transmute_copy(&idwinevent), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::windows_core::from_raw_borrowed(&psink)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RespondToWinEvent: RespondToWinEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProxyProviderWinEventHandler as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProxyProviderWinEventSink_Impl: Sized {
    fn AddAutomationPropertyChangedEvent(&self, pprovider: ::core::option::Option<&IRawElementProviderSimple>, id: UIA_PROPERTY_ID, newvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddAutomationEvent(&self, pprovider: ::core::option::Option<&IRawElementProviderSimple>, id: UIA_EVENT_ID) -> ::windows_core::Result<()>;
    fn AddStructureChangedEvent(&self, pprovider: ::core::option::Option<&IRawElementProviderSimple>, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IProxyProviderWinEventSink {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProxyProviderWinEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>() -> IProxyProviderWinEventSink_Vtbl {
        unsafe extern "system" fn AddAutomationPropertyChangedEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, id: UIA_PROPERTY_ID, newvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAutomationPropertyChangedEvent(::windows_core::from_raw_borrowed(&pprovider), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&newvalue)).into()
        }
        unsafe extern "system" fn AddAutomationEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, id: UIA_EVENT_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAutomationEvent(::windows_core::from_raw_borrowed(&pprovider), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn AddStructureChangedEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStructureChangedEvent(::windows_core::from_raw_borrowed(&pprovider), ::core::mem::transmute_copy(&structurechangetype), ::core::mem::transmute_copy(&runtimeid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAutomationPropertyChangedEvent: AddAutomationPropertyChangedEvent::<Identity, Impl, OFFSET>,
            AddAutomationEvent: AddAutomationEvent::<Identity, Impl, OFFSET>,
            AddStructureChangedEvent: AddStructureChangedEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProxyProviderWinEventSink as ::windows_core::Interface>::IID
    }
}
pub trait IRangeValueProvider_Impl: Sized {
    fn SetValue(&self, val: f64) -> ::windows_core::Result<()>;
    fn Value(&self) -> ::windows_core::Result<f64>;
    fn IsReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Maximum(&self) -> ::windows_core::Result<f64>;
    fn Minimum(&self) -> ::windows_core::Result<f64>;
    fn LargeChange(&self) -> ::windows_core::Result<f64>;
    fn SmallChange(&self) -> ::windows_core::Result<f64>;
}
impl ::windows_core::RuntimeName for IRangeValueProvider {}
impl IRangeValueProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: isize>() -> IRangeValueProvider_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Maximum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Minimum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            Maximum: Maximum::<Identity, Impl, OFFSET>,
            Minimum: Minimum::<Identity, Impl, OFFSET>,
            LargeChange: LargeChange::<Identity, Impl, OFFSET>,
            SmallChange: SmallChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRangeValueProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderAdviseEvents_Impl: Sized {
    fn AdviseEventAdded(&self, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn AdviseEventRemoved(&self, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IRawElementProviderAdviseEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderAdviseEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>() -> IRawElementProviderAdviseEvents_Vtbl {
        unsafe extern "system" fn AdviseEventAdded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseEventAdded(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyids)).into()
        }
        unsafe extern "system" fn AdviseEventRemoved<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseEventRemoved(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyids)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseEventAdded: AdviseEventAdded::<Identity, Impl, OFFSET>,
            AdviseEventRemoved: AdviseEventRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderAdviseEvents as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderFragment_Impl: Sized {
    fn Navigate(&self, direction: NavigateDirection) -> ::windows_core::Result<IRawElementProviderFragment>;
    fn GetRuntimeId(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn BoundingRectangle(&self) -> ::windows_core::Result<UiaRect>;
    fn GetEmbeddedFragmentRoots(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFocus(&self) -> ::windows_core::Result<()>;
    fn FragmentRoot(&self) -> ::windows_core::Result<IRawElementProviderFragmentRoot>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IRawElementProviderFragment {}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderFragment_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>() -> IRawElementProviderFragment_Vtbl {
        unsafe extern "system" fn Navigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRectangle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut UiaRect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEmbeddedFragmentRoots() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFocus().into()
        }
        unsafe extern "system" fn FragmentRoot<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FragmentRoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Navigate: Navigate::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            BoundingRectangle: BoundingRectangle::<Identity, Impl, OFFSET>,
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            FragmentRoot: FragmentRoot::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderFragment as ::windows_core::Interface>::IID
    }
}
pub trait IRawElementProviderFragmentRoot_Impl: Sized {
    fn ElementProviderFromPoint(&self, x: f64, y: f64) -> ::windows_core::Result<IRawElementProviderFragment>;
    fn GetFocus(&self) -> ::windows_core::Result<IRawElementProviderFragment>;
}
impl ::windows_core::RuntimeName for IRawElementProviderFragmentRoot {}
impl IRawElementProviderFragmentRoot_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>() -> IRawElementProviderFragmentRoot_Vtbl {
        unsafe extern "system" fn ElementProviderFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElementProviderFromPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ElementProviderFromPoint: ElementProviderFromPoint::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderFragmentRoot as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderHostingAccessibles_Impl: Sized {
    fn GetEmbeddedAccessibles(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IRawElementProviderHostingAccessibles {}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderHostingAccessibles_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>() -> IRawElementProviderHostingAccessibles_Vtbl {
        unsafe extern "system" fn GetEmbeddedAccessibles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEmbeddedAccessibles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetEmbeddedAccessibles: GetEmbeddedAccessibles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderHostingAccessibles as ::windows_core::Interface>::IID
    }
}
pub trait IRawElementProviderHwndOverride_Impl: Sized {
    fn GetOverrideProviderForHwnd(&self, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::RuntimeName for IRawElementProviderHwndOverride {}
impl IRawElementProviderHwndOverride_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>() -> IRawElementProviderHwndOverride_Vtbl {
        unsafe extern "system" fn GetOverrideProviderForHwnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOverrideProviderForHwnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOverrideProviderForHwnd: GetOverrideProviderForHwnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderHwndOverride as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawElementProviderSimple_Impl: Sized {
    fn ProviderOptions(&self) -> ::windows_core::Result<ProviderOptions>;
    fn GetPatternProvider(&self, patternid: UIA_PATTERN_ID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn HostRawElementProvider(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IRawElementProviderSimple {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawElementProviderSimple_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>() -> IRawElementProviderSimple_Vtbl {
        unsafe extern "system" fn ProviderOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ProviderOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProviderOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPatternProvider(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, pretval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostRawElementProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HostRawElementProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProviderOptions: ProviderOptions::<Identity, Impl, OFFSET>,
            GetPatternProvider: GetPatternProvider::<Identity, Impl, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, Impl, OFFSET>,
            HostRawElementProvider: HostRawElementProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderSimple as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawElementProviderSimple2_Impl: Sized + IRawElementProviderSimple_Impl {
    fn ShowContextMenu(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IRawElementProviderSimple2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawElementProviderSimple2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple2_Impl, const OFFSET: isize>() -> IRawElementProviderSimple2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowContextMenu().into()
        }
        Self { base__: IRawElementProviderSimple_Vtbl::new::<Identity, Impl, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderSimple2 as ::windows_core::Interface>::IID || iid == &<IRawElementProviderSimple as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawElementProviderSimple3_Impl: Sized + IRawElementProviderSimple2_Impl {
    fn GetMetadataValue(&self, targetid: i32, metadataid: UIA_METADATA_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IRawElementProviderSimple3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawElementProviderSimple3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple3_Impl, const OFFSET: isize>() -> IRawElementProviderSimple3_Vtbl {
        unsafe extern "system" fn GetMetadataValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderSimple3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: UIA_METADATA_ID, returnval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataValue(::core::mem::transmute_copy(&targetid), ::core::mem::transmute_copy(&metadataid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IRawElementProviderSimple2_Vtbl::new::<Identity, Impl, OFFSET>(), GetMetadataValue: GetMetadataValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderSimple3 as ::windows_core::Interface>::IID || iid == &<IRawElementProviderSimple as ::windows_core::Interface>::IID || iid == &<IRawElementProviderSimple2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderWindowlessSite_Impl: Sized {
    fn GetAdjacentFragment(&self, direction: NavigateDirection) -> ::windows_core::Result<IRawElementProviderFragment>;
    fn GetRuntimeIdPrefix(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IRawElementProviderWindowlessSite {}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderWindowlessSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>() -> IRawElementProviderWindowlessSite_Vtbl {
        unsafe extern "system" fn GetAdjacentFragment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAdjacentFragment(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeIdPrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRuntimeIdPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAdjacentFragment: GetAdjacentFragment::<Identity, Impl, OFFSET>,
            GetRuntimeIdPrefix: GetRuntimeIdPrefix::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawElementProviderWindowlessSite as ::windows_core::Interface>::IID
    }
}
pub trait IRichEditUiaInformation_Impl: Sized {
    fn GetBoundaryRectangle(&self, puiarect: *mut UiaRect) -> ::windows_core::Result<()>;
    fn IsVisible(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IRichEditUiaInformation {}
impl IRichEditUiaInformation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditUiaInformation_Impl, const OFFSET: isize>() -> IRichEditUiaInformation_Vtbl {
        unsafe extern "system" fn GetBoundaryRectangle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiarect: *mut UiaRect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBoundaryRectangle(::core::mem::transmute_copy(&puiarect)).into()
        }
        unsafe extern "system" fn IsVisible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsVisible().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBoundaryRectangle: GetBoundaryRectangle::<Identity, Impl, OFFSET>,
            IsVisible: IsVisible::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRichEditUiaInformation as ::windows_core::Interface>::IID
    }
}
pub trait IRicheditWindowlessAccessibility_Impl: Sized {
    fn CreateProvider(&self, psite: ::core::option::Option<&IRawElementProviderWindowlessSite>) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::RuntimeName for IRicheditWindowlessAccessibility {}
impl IRicheditWindowlessAccessibility_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>() -> IRicheditWindowlessAccessibility_Vtbl {
        unsafe extern "system" fn CreateProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateProvider(::windows_core::from_raw_borrowed(&psite)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateProvider: CreateProvider::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRicheditWindowlessAccessibility as ::windows_core::Interface>::IID
    }
}
pub trait IScrollItemProvider_Impl: Sized {
    fn ScrollIntoView(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IScrollItemProvider {}
impl IScrollItemProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollItemProvider_Impl, const OFFSET: isize>() -> IScrollItemProvider_Vtbl {
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScrollIntoView().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IScrollItemProvider as ::windows_core::Interface>::IID
    }
}
pub trait IScrollProvider_Impl: Sized {
    fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows_core::Result<()>;
    fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> ::windows_core::Result<()>;
    fn HorizontalScrollPercent(&self) -> ::windows_core::Result<f64>;
    fn VerticalScrollPercent(&self) -> ::windows_core::Result<f64>;
    fn HorizontalViewSize(&self) -> ::windows_core::Result<f64>;
    fn VerticalViewSize(&self) -> ::windows_core::Result<f64>;
    fn HorizontallyScrollable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn VerticallyScrollable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IScrollProvider {}
impl IScrollProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>() -> IScrollProvider_Vtbl {
        unsafe extern "system" fn Scroll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Scroll(::core::mem::transmute_copy(&horizontalamount), ::core::mem::transmute_copy(&verticalamount)).into()
        }
        unsafe extern "system" fn SetScrollPercent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScrollPercent(::core::mem::transmute_copy(&horizontalpercent), ::core::mem::transmute_copy(&verticalpercent)).into()
        }
        unsafe extern "system" fn HorizontalScrollPercent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalScrollPercent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalViewSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalViewSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontallyScrollable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticallyScrollable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            SetScrollPercent: SetScrollPercent::<Identity, Impl, OFFSET>,
            HorizontalScrollPercent: HorizontalScrollPercent::<Identity, Impl, OFFSET>,
            VerticalScrollPercent: VerticalScrollPercent::<Identity, Impl, OFFSET>,
            HorizontalViewSize: HorizontalViewSize::<Identity, Impl, OFFSET>,
            VerticalViewSize: VerticalViewSize::<Identity, Impl, OFFSET>,
            HorizontallyScrollable: HorizontallyScrollable::<Identity, Impl, OFFSET>,
            VerticallyScrollable: VerticallyScrollable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IScrollProvider as ::windows_core::Interface>::IID
    }
}
pub trait ISelectionItemProvider_Impl: Sized {
    fn Select(&self) -> ::windows_core::Result<()>;
    fn AddToSelection(&self) -> ::windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows_core::Result<()>;
    fn IsSelected(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SelectionContainer(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::RuntimeName for ISelectionItemProvider {}
impl ISelectionItemProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>() -> ISelectionItemProvider_Vtbl {
        unsafe extern "system" fn Select<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFromSelection().into()
        }
        unsafe extern "system" fn IsSelected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            SelectionContainer: SelectionContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISelectionItemProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISelectionProvider_Impl: Sized {
    fn GetSelection(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CanSelectMultiple(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsSelectionRequired(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ISelectionProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ISelectionProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider_Impl, const OFFSET: isize>() -> ISelectionProvider_Vtbl {
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSelectMultiple<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelectionRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            CanSelectMultiple: CanSelectMultiple::<Identity, Impl, OFFSET>,
            IsSelectionRequired: IsSelectionRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISelectionProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISelectionProvider2_Impl: Sized + ISelectionProvider_Impl {
    fn FirstSelectedItem(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn LastSelectedItem(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn CurrentSelectedItem(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn ItemCount(&self) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ISelectionProvider2 {}
#[cfg(feature = "Win32_System_Com")]
impl ISelectionProvider2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: isize>() -> ISelectionProvider2_Vtbl {
        unsafe extern "system" fn FirstSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISelectionProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            FirstSelectedItem: FirstSelectedItem::<Identity, Impl, OFFSET>,
            LastSelectedItem: LastSelectedItem::<Identity, Impl, OFFSET>,
            CurrentSelectedItem: CurrentSelectedItem::<Identity, Impl, OFFSET>,
            ItemCount: ItemCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISelectionProvider2 as ::windows_core::Interface>::IID || iid == &<ISelectionProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISpreadsheetItemProvider_Impl: Sized {
    fn Formula(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAnnotationObjects(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetAnnotationTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ISpreadsheetItemProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ISpreadsheetItemProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>() -> ISpreadsheetItemProvider_Vtbl {
        unsafe extern "system" fn Formula<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Formula() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Formula: Formula::<Identity, Impl, OFFSET>,
            GetAnnotationObjects: GetAnnotationObjects::<Identity, Impl, OFFSET>,
            GetAnnotationTypes: GetAnnotationTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpreadsheetItemProvider as ::windows_core::Interface>::IID
    }
}
pub trait ISpreadsheetProvider_Impl: Sized {
    fn GetItemByName(&self, name: &::windows_core::PCWSTR) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::RuntimeName for ISpreadsheetProvider {}
impl ISpreadsheetProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpreadsheetProvider_Impl, const OFFSET: isize>() -> ISpreadsheetProvider_Vtbl {
        unsafe extern "system" fn GetItemByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpreadsheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetItemByName: GetItemByName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpreadsheetProvider as ::windows_core::Interface>::IID
    }
}
pub trait IStylesProvider_Impl: Sized {
    fn StyleId(&self) -> ::windows_core::Result<UIA_STYLE_ID>;
    fn StyleName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FillColor(&self) -> ::windows_core::Result<i32>;
    fn FillPatternStyle(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Shape(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FillPatternColor(&self) -> ::windows_core::Result<i32>;
    fn ExtendedProperties(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IStylesProvider {}
impl IStylesProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: isize>() -> IStylesProvider_Vtbl {
        unsafe extern "system" fn StyleId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StyleId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StyleName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FillColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Shape() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StyleId: StyleId::<Identity, Impl, OFFSET>,
            StyleName: StyleName::<Identity, Impl, OFFSET>,
            FillColor: FillColor::<Identity, Impl, OFFSET>,
            FillPatternStyle: FillPatternStyle::<Identity, Impl, OFFSET>,
            Shape: Shape::<Identity, Impl, OFFSET>,
            FillPatternColor: FillPatternColor::<Identity, Impl, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStylesProvider as ::windows_core::Interface>::IID
    }
}
pub trait ISynchronizedInputProvider_Impl: Sized {
    fn StartListening(&self, inputtype: SynchronizedInputType) -> ::windows_core::Result<()>;
    fn Cancel(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISynchronizedInputProvider {}
impl ISynchronizedInputProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>() -> ISynchronizedInputProvider_Vtbl {
        unsafe extern "system" fn StartListening<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartListening(::core::mem::transmute_copy(&inputtype)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartListening: StartListening::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISynchronizedInputProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITableItemProvider_Impl: Sized {
    fn GetRowHeaderItems(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetColumnHeaderItems(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITableItemProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ITableItemProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITableItemProvider_Impl, const OFFSET: isize>() -> ITableItemProvider_Vtbl {
        unsafe extern "system" fn GetRowHeaderItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaderItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowHeaderItems: GetRowHeaderItems::<Identity, Impl, OFFSET>,
            GetColumnHeaderItems: GetColumnHeaderItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITableItemProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITableProvider_Impl: Sized {
    fn GetRowHeaders(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetColumnHeaders(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn RowOrColumnMajor(&self) -> ::windows_core::Result<RowOrColumnMajor>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITableProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ITableProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITableProvider_Impl, const OFFSET: isize>() -> ITableProvider_Vtbl {
        unsafe extern "system" fn GetRowHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowOrColumnMajor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowHeaders: GetRowHeaders::<Identity, Impl, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, Impl, OFFSET>,
            RowOrColumnMajor: RowOrColumnMajor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITableProvider as ::windows_core::Interface>::IID
    }
}
pub trait ITextChildProvider_Impl: Sized {
    fn TextContainer(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn TextRange(&self) -> ::windows_core::Result<ITextRangeProvider>;
}
impl ::windows_core::RuntimeName for ITextChildProvider {}
impl ITextChildProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextChildProvider_Impl, const OFFSET: isize>() -> ITextChildProvider_Vtbl {
        unsafe extern "system" fn TextContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TextContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TextRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TextContainer: TextContainer::<Identity, Impl, OFFSET>,
            TextRange: TextRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextChildProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITextEditProvider_Impl: Sized + ITextProvider_Impl {
    fn GetActiveComposition(&self) -> ::windows_core::Result<ITextRangeProvider>;
    fn GetConversionTarget(&self) -> ::windows_core::Result<ITextRangeProvider>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITextEditProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ITextEditProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextEditProvider_Impl, const OFFSET: isize>() -> ITextEditProvider_Vtbl {
        unsafe extern "system" fn GetActiveComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveComposition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConversionTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITextProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Identity, Impl, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextEditProvider as ::windows_core::Interface>::IID || iid == &<ITextProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITextProvider_Impl: Sized {
    fn GetSelection(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleRanges(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn RangeFromChild(&self, childelement: ::core::option::Option<&IRawElementProviderSimple>) -> ::windows_core::Result<ITextRangeProvider>;
    fn RangeFromPoint(&self, point: &UiaPoint) -> ::windows_core::Result<ITextRangeProvider>;
    fn DocumentRange(&self) -> ::windows_core::Result<ITextRangeProvider>;
    fn SupportedTextSelection(&self) -> ::windows_core::Result<SupportedTextSelection>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITextProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ITextProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: isize>() -> ITextProvider_Vtbl {
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisibleRanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childelement: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RangeFromChild(::windows_core::from_raw_borrowed(&childelement)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: UiaPoint, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RangeFromPoint(::core::mem::transmute(&point)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DocumentRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut SupportedTextSelection) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedTextSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, Impl, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, Impl, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
            DocumentRange: DocumentRange::<Identity, Impl, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITextProvider2_Impl: Sized + ITextProvider_Impl {
    fn RangeFromAnnotation(&self, annotationelement: ::core::option::Option<&IRawElementProviderSimple>) -> ::windows_core::Result<ITextRangeProvider>;
    fn GetCaretRange(&self, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::core::option::Option<ITextRangeProvider>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITextProvider2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITextProvider2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider2_Impl, const OFFSET: isize>() -> ITextProvider2_Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationelement: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RangeFromAnnotation(::windows_core::from_raw_borrowed(&annotationelement)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaretRange(::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&pretval)).into()
        }
        Self {
            base__: ITextProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Identity, Impl, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextProvider2 as ::windows_core::Interface>::IID || iid == &<ITextProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextRangeProvider_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<ITextRangeProvider>;
    fn Compare(&self, range: ::core::option::Option<&ITextRangeProvider>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareEndpoints(&self, endpoint: TextPatternRangeEndpoint, targetrange: ::core::option::Option<&ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::Result<i32>;
    fn ExpandToEnclosingUnit(&self, unit: TextUnit) -> ::windows_core::Result<()>;
    fn FindAttribute(&self, attributeid: UIA_TEXTATTRIBUTE_ID, val: &super::super::System::Variant::VARIANT, backward: super::super::Foundation::BOOL) -> ::windows_core::Result<ITextRangeProvider>;
    fn FindText(&self, text: &::windows_core::BSTR, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL) -> ::windows_core::Result<ITextRangeProvider>;
    fn GetAttributeValue(&self, attributeid: UIA_TEXTATTRIBUTE_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetBoundingRectangles(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEnclosingElement(&self) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn GetText(&self, maxlength: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Move(&self, unit: TextUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEndpointByRange(&self, endpoint: TextPatternRangeEndpoint, targetrange: ::core::option::Option<&ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::Result<()>;
    fn Select(&self) -> ::windows_core::Result<()>;
    fn AddToSelection(&self) -> ::windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows_core::Result<()>;
    fn ScrollIntoView(&self, aligntotop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetChildren(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ITextRangeProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextRangeProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>() -> ITextRangeProvider_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Compare(::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareEndpoints(::core::mem::transmute_copy(&endpoint), ::windows_core::from_raw_borrowed(&targetrange), ::core::mem::transmute_copy(&targetendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExpandToEnclosingUnit(::core::mem::transmute_copy(&unit)).into()
        }
        unsafe extern "system" fn FindAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Variant::VARIANT, backward: super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindAttribute(::core::mem::transmute_copy(&attributeid), ::core::mem::transmute(&val), ::core::mem::transmute_copy(&backward)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindText(::core::mem::transmute(&text), ::core::mem::transmute_copy(&backward), ::core::mem::transmute_copy(&ignorecase)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: UIA_TEXTATTRIBUTE_ID, pretval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeValue(::core::mem::transmute_copy(&attributeid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoundingRectangles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnclosingElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetText(::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Move(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveEndpointByUnit(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveEndpointByRange(::core::mem::transmute_copy(&endpoint), ::windows_core::from_raw_borrowed(&targetrange), ::core::mem::transmute_copy(&targetendpoint)).into()
        }
        unsafe extern "system" fn Select<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFromSelection().into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScrollIntoView(::core::mem::transmute_copy(&aligntotop)).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            CompareEndpoints: CompareEndpoints::<Identity, Impl, OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Identity, Impl, OFFSET>,
            FindAttribute: FindAttribute::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, Impl, OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Identity, Impl, OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Identity, Impl, OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextRangeProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextRangeProvider2_Impl: Sized + ITextRangeProvider_Impl {
    fn ShowContextMenu(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ITextRangeProvider2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextRangeProvider2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider2_Impl, const OFFSET: isize>() -> ITextRangeProvider2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRangeProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowContextMenu().into()
        }
        Self { base__: ITextRangeProvider_Vtbl::new::<Identity, Impl, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextRangeProvider2 as ::windows_core::Interface>::IID || iid == &<ITextRangeProvider as ::windows_core::Interface>::IID
    }
}
pub trait IToggleProvider_Impl: Sized {
    fn Toggle(&self) -> ::windows_core::Result<()>;
    fn ToggleState(&self) -> ::windows_core::Result<ToggleState>;
}
impl ::windows_core::RuntimeName for IToggleProvider {}
impl IToggleProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToggleProvider_Impl, const OFFSET: isize>() -> IToggleProvider_Vtbl {
        unsafe extern "system" fn Toggle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToggleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Toggle().into()
        }
        unsafe extern "system" fn ToggleState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToggleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ToggleState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Toggle: Toggle::<Identity, Impl, OFFSET>,
            ToggleState: ToggleState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IToggleProvider as ::windows_core::Interface>::IID
    }
}
pub trait ITransformProvider_Impl: Sized {
    fn Move(&self, x: f64, y: f64) -> ::windows_core::Result<()>;
    fn Resize(&self, width: f64, height: f64) -> ::windows_core::Result<()>;
    fn Rotate(&self, degrees: f64) -> ::windows_core::Result<()>;
    fn CanMove(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanResize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanRotate(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for ITransformProvider {}
impl ITransformProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: isize>() -> ITransformProvider_Vtbl {
        unsafe extern "system" fn Move<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rotate(::core::mem::transmute_copy(&degrees)).into()
        }
        unsafe extern "system" fn CanMove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanMove() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanResize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanResize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRotate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Move: Move::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            CanMove: CanMove::<Identity, Impl, OFFSET>,
            CanResize: CanResize::<Identity, Impl, OFFSET>,
            CanRotate: CanRotate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITransformProvider as ::windows_core::Interface>::IID
    }
}
pub trait ITransformProvider2_Impl: Sized + ITransformProvider_Impl {
    fn Zoom(&self, zoom: f64) -> ::windows_core::Result<()>;
    fn CanZoom(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ZoomLevel(&self) -> ::windows_core::Result<f64>;
    fn ZoomMinimum(&self) -> ::windows_core::Result<f64>;
    fn ZoomMaximum(&self) -> ::windows_core::Result<f64>;
    fn ZoomByUnit(&self, zoomunit: ZoomUnit) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITransformProvider2 {}
impl ITransformProvider2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: isize>() -> ITransformProvider2_Vtbl {
        unsafe extern "system" fn Zoom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Zoom(::core::mem::transmute_copy(&zoom)).into()
        }
        unsafe extern "system" fn CanZoom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomMinimum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomMaximum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomByUnit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ZoomByUnit(::core::mem::transmute_copy(&zoomunit)).into()
        }
        Self {
            base__: ITransformProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            Zoom: Zoom::<Identity, Impl, OFFSET>,
            CanZoom: CanZoom::<Identity, Impl, OFFSET>,
            ZoomLevel: ZoomLevel::<Identity, Impl, OFFSET>,
            ZoomMinimum: ZoomMinimum::<Identity, Impl, OFFSET>,
            ZoomMaximum: ZoomMaximum::<Identity, Impl, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITransformProvider2 as ::windows_core::Interface>::IID || iid == &<ITransformProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation_Impl: Sized {
    fn CompareElements(&self, el1: ::core::option::Option<&IUIAutomationElement>, el2: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareRuntimeIds(&self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetRootElement(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandle(&self, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromPoint(&self, pt: &super::super::Foundation::POINT) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElement(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetRootElementBuildCache(&self, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandleBuildCache(&self, hwnd: super::super::Foundation::HWND, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromPointBuildCache(&self, pt: &super::super::Foundation::POINT, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElementBuildCache(&self, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn CreateTreeWalker(&self, pcondition: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationTreeWalker>;
    fn ControlViewWalker(&self) -> ::windows_core::Result<IUIAutomationTreeWalker>;
    fn ContentViewWalker(&self) -> ::windows_core::Result<IUIAutomationTreeWalker>;
    fn RawViewWalker(&self) -> ::windows_core::Result<IUIAutomationTreeWalker>;
    fn RawViewCondition(&self) -> ::windows_core::Result<IUIAutomationCondition>;
    fn ControlViewCondition(&self) -> ::windows_core::Result<IUIAutomationCondition>;
    fn ContentViewCondition(&self) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateCacheRequest(&self) -> ::windows_core::Result<IUIAutomationCacheRequest>;
    fn CreateTrueCondition(&self) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateFalseCondition(&self) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyCondition(&self, propertyid: UIA_PROPERTY_ID, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyConditionEx(&self, propertyid: UIA_PROPERTY_ID, value: &super::super::System::Variant::VARIANT, flags: PropertyConditionFlags) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateAndCondition(&self, condition1: ::core::option::Option<&IUIAutomationCondition>, condition2: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromNativeArray(&self, conditions: *const ::core::option::Option<IUIAutomationCondition>, conditioncount: i32) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateOrCondition(&self, condition1: ::core::option::Option<&IUIAutomationCondition>, condition2: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromNativeArray(&self, conditions: *const ::core::option::Option<IUIAutomationCondition>, conditioncount: i32) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateNotCondition(&self, condition: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationCondition>;
    fn AddAutomationEventHandler(&self, eventid: UIA_EVENT_ID, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveAutomationEventHandler(&self, eventid: UIA_EVENT_ID, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationEventHandler>) -> ::windows_core::Result<()>;
    fn AddPropertyChangedEventHandlerNativeArray(&self, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationPropertyChangedEventHandler>, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn RemovePropertyChangedEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationPropertyChangedEventHandler>) -> ::windows_core::Result<()>;
    fn AddStructureChangedEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationStructureChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveStructureChangedEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationStructureChangedEventHandler>) -> ::windows_core::Result<()>;
    fn AddFocusChangedEventHandler(&self, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationFocusChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveFocusChangedEventHandler(&self, handler: ::core::option::Option<&IUIAutomationFocusChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveAllEventHandlers(&self) -> ::windows_core::Result<()>;
    fn IntNativeArrayToSafeArray(&self, array: *const i32, arraycount: i32) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn IntSafeArrayToNativeArray(&self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows_core::Result<()>;
    fn RectToVariant(&self, rc: &super::super::Foundation::RECT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn VariantToRect(&self, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn SafeArrayToRectNativeArray(&self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows_core::Result<()>;
    fn CreateProxyFactoryEntry(&self, factory: ::core::option::Option<&IUIAutomationProxyFactory>) -> ::windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn ProxyFactoryMapping(&self) -> ::windows_core::Result<IUIAutomationProxyFactoryMapping>;
    fn GetPropertyProgrammaticName(&self, property: UIA_PROPERTY_ID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPatternProgrammaticName(&self, pattern: UIA_PATTERN_ID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PollForPotentialSupportedPatterns(&self, pelement: ::core::option::Option<&IUIAutomationElement>, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn PollForPotentialSupportedProperties(&self, pelement: ::core::option::Option<&IUIAutomationElement>, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn CheckNotSupported(&self, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ReservedNotSupportedValue(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ReservedMixedAttributeValue(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ElementFromIAccessible(&self, accessible: ::core::option::Option<&IAccessible>, childid: i32) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromIAccessibleBuildCache(&self, accessible: ::core::option::Option<&IAccessible>, childid: i32, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomation {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>() -> IUIAutomation_Vtbl {
        unsafe extern "system" fn CompareElements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, el1: *mut ::core::ffi::c_void, el2: *mut ::core::ffi::c_void, aresame: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareElements(::windows_core::from_raw_borrowed(&el1), ::windows_core::from_raw_borrowed(&el2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(aresame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareRuntimeIds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY, aresame: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareRuntimeIds(::core::mem::transmute_copy(&runtimeid1), ::core::mem::transmute_copy(&runtimeid2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(aresame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRootElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(root, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElementFromHandle(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElementFromPoint(::core::mem::transmute(&pt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRootElementBuildCache(::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(root, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromHandleBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElementFromHandleBuildCache(::core::mem::transmute_copy(&hwnd), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromPointBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElementFromPointBuildCache(::core::mem::transmute(&pt), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFocusedElementBuildCache(::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTreeWalker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTreeWalker(::windows_core::from_raw_borrowed(&pcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(walker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlViewWalker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ControlViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(walker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentViewWalker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(walker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawViewWalker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RawViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(walker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawViewCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RawViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlViewCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ControlViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentViewCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCacheRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCacheRequest() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cacherequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrueCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTrueCondition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFalseCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFalseCondition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Variant::VARIANT, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePropertyCondition(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyConditionEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Variant::VARIANT, flags: PropertyConditionFlags, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePropertyConditionEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&value), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: *mut ::core::ffi::c_void, condition2: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAndCondition(::windows_core::from_raw_borrowed(&condition1), ::windows_core::from_raw_borrowed(&condition2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndConditionFromArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAndConditionFromArray(::core::mem::transmute_copy(&conditions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndConditionFromNativeArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const *mut ::core::ffi::c_void, conditioncount: i32, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAndConditionFromNativeArray(::core::mem::transmute_copy(&conditions), ::core::mem::transmute_copy(&conditioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: *mut ::core::ffi::c_void, condition2: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateOrCondition(::windows_core::from_raw_borrowed(&condition1), ::windows_core::from_raw_borrowed(&condition2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrConditionFromArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateOrConditionFromArray(::core::mem::transmute_copy(&conditions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrConditionFromNativeArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const *mut ::core::ffi::c_void, conditioncount: i32, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateOrConditionFromNativeArray(::core::mem::transmute_copy(&conditions), ::core::mem::transmute_copy(&conditioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNotCondition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNotCondition(::windows_core::from_raw_borrowed(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyChangedEventHandlerNativeArray(::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyChangedEventHandler(::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&propertyarray)).into()
        }
        unsafe extern "system" fn RemovePropertyChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePropertyChangedEventHandler(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStructureChangedEventHandler(::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn RemoveStructureChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStructureChangedEventHandler(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn AddFocusChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFocusChangedEventHandler(::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn RemoveFocusChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFocusChangedEventHandler(::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn RemoveAllEventHandlers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllEventHandlers().into()
        }
        unsafe extern "system" fn IntNativeArrayToSafeArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IntNativeArrayToSafeArray(::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(safearray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntSafeArrayToNativeArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IntSafeArrayToNativeArray(::core::mem::transmute_copy(&intarray), ::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)).into()
        }
        unsafe extern "system" fn RectToVariant<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, var: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RectToVariant(::core::mem::transmute(&rc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(var, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VariantToRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: super::super::System::Variant::VARIANT, rc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VariantToRect(::core::mem::transmute(&var)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SafeArrayToRectNativeArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SafeArrayToRectNativeArray(::core::mem::transmute_copy(&rects), ::core::mem::transmute_copy(&rectarray), ::core::mem::transmute_copy(&rectarraycount)).into()
        }
        unsafe extern "system" fn CreateProxyFactoryEntry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, factoryentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateProxyFactoryEntry(::windows_core::from_raw_borrowed(&factory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factoryentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyFactoryMapping<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorymapping: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyFactoryMapping() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factorymapping, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: UIA_PROPERTY_ID, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyProgrammaticName(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternProgrammaticName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: UIA_PATTERN_ID, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPatternProgrammaticName(::core::mem::transmute_copy(&pattern)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedPatterns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PollForPotentialSupportedPatterns(::windows_core::from_raw_borrowed(&pelement), ::core::mem::transmute_copy(&patternids), ::core::mem::transmute_copy(&patternnames)).into()
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PollForPotentialSupportedProperties(::windows_core::from_raw_borrowed(&pelement), ::core::mem::transmute_copy(&propertyids), ::core::mem::transmute_copy(&propertynames)).into()
        }
        unsafe extern "system" fn CheckNotSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::Variant::VARIANT, isnotsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckNotSupported(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnotsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReservedNotSupportedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notsupportedvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReservedNotSupportedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notsupportedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReservedMixedAttributeValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mixedattributevalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReservedMixedAttributeValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mixedattributevalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromIAccessible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: *mut ::core::ffi::c_void, childid: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElementFromIAccessible(::windows_core::from_raw_borrowed(&accessible), ::core::mem::transmute_copy(&childid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromIAccessibleBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: *mut ::core::ffi::c_void, childid: i32, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElementFromIAccessibleBuildCache(::windows_core::from_raw_borrowed(&accessible), ::core::mem::transmute_copy(&childid), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompareElements: CompareElements::<Identity, Impl, OFFSET>,
            CompareRuntimeIds: CompareRuntimeIds::<Identity, Impl, OFFSET>,
            GetRootElement: GetRootElement::<Identity, Impl, OFFSET>,
            ElementFromHandle: ElementFromHandle::<Identity, Impl, OFFSET>,
            ElementFromPoint: ElementFromPoint::<Identity, Impl, OFFSET>,
            GetFocusedElement: GetFocusedElement::<Identity, Impl, OFFSET>,
            GetRootElementBuildCache: GetRootElementBuildCache::<Identity, Impl, OFFSET>,
            ElementFromHandleBuildCache: ElementFromHandleBuildCache::<Identity, Impl, OFFSET>,
            ElementFromPointBuildCache: ElementFromPointBuildCache::<Identity, Impl, OFFSET>,
            GetFocusedElementBuildCache: GetFocusedElementBuildCache::<Identity, Impl, OFFSET>,
            CreateTreeWalker: CreateTreeWalker::<Identity, Impl, OFFSET>,
            ControlViewWalker: ControlViewWalker::<Identity, Impl, OFFSET>,
            ContentViewWalker: ContentViewWalker::<Identity, Impl, OFFSET>,
            RawViewWalker: RawViewWalker::<Identity, Impl, OFFSET>,
            RawViewCondition: RawViewCondition::<Identity, Impl, OFFSET>,
            ControlViewCondition: ControlViewCondition::<Identity, Impl, OFFSET>,
            ContentViewCondition: ContentViewCondition::<Identity, Impl, OFFSET>,
            CreateCacheRequest: CreateCacheRequest::<Identity, Impl, OFFSET>,
            CreateTrueCondition: CreateTrueCondition::<Identity, Impl, OFFSET>,
            CreateFalseCondition: CreateFalseCondition::<Identity, Impl, OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Identity, Impl, OFFSET>,
            CreatePropertyConditionEx: CreatePropertyConditionEx::<Identity, Impl, OFFSET>,
            CreateAndCondition: CreateAndCondition::<Identity, Impl, OFFSET>,
            CreateAndConditionFromArray: CreateAndConditionFromArray::<Identity, Impl, OFFSET>,
            CreateAndConditionFromNativeArray: CreateAndConditionFromNativeArray::<Identity, Impl, OFFSET>,
            CreateOrCondition: CreateOrCondition::<Identity, Impl, OFFSET>,
            CreateOrConditionFromArray: CreateOrConditionFromArray::<Identity, Impl, OFFSET>,
            CreateOrConditionFromNativeArray: CreateOrConditionFromNativeArray::<Identity, Impl, OFFSET>,
            CreateNotCondition: CreateNotCondition::<Identity, Impl, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, Impl, OFFSET>,
            RemoveAutomationEventHandler: RemoveAutomationEventHandler::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandlerNativeArray: AddPropertyChangedEventHandlerNativeArray::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            RemovePropertyChangedEventHandler: RemovePropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveStructureChangedEventHandler: RemoveStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            AddFocusChangedEventHandler: AddFocusChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveFocusChangedEventHandler: RemoveFocusChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveAllEventHandlers: RemoveAllEventHandlers::<Identity, Impl, OFFSET>,
            IntNativeArrayToSafeArray: IntNativeArrayToSafeArray::<Identity, Impl, OFFSET>,
            IntSafeArrayToNativeArray: IntSafeArrayToNativeArray::<Identity, Impl, OFFSET>,
            RectToVariant: RectToVariant::<Identity, Impl, OFFSET>,
            VariantToRect: VariantToRect::<Identity, Impl, OFFSET>,
            SafeArrayToRectNativeArray: SafeArrayToRectNativeArray::<Identity, Impl, OFFSET>,
            CreateProxyFactoryEntry: CreateProxyFactoryEntry::<Identity, Impl, OFFSET>,
            ProxyFactoryMapping: ProxyFactoryMapping::<Identity, Impl, OFFSET>,
            GetPropertyProgrammaticName: GetPropertyProgrammaticName::<Identity, Impl, OFFSET>,
            GetPatternProgrammaticName: GetPatternProgrammaticName::<Identity, Impl, OFFSET>,
            PollForPotentialSupportedPatterns: PollForPotentialSupportedPatterns::<Identity, Impl, OFFSET>,
            PollForPotentialSupportedProperties: PollForPotentialSupportedProperties::<Identity, Impl, OFFSET>,
            CheckNotSupported: CheckNotSupported::<Identity, Impl, OFFSET>,
            ReservedNotSupportedValue: ReservedNotSupportedValue::<Identity, Impl, OFFSET>,
            ReservedMixedAttributeValue: ReservedMixedAttributeValue::<Identity, Impl, OFFSET>,
            ElementFromIAccessible: ElementFromIAccessible::<Identity, Impl, OFFSET>,
            ElementFromIAccessibleBuildCache: ElementFromIAccessibleBuildCache::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomation as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation2_Impl: Sized + IUIAutomation_Impl {
    fn AutoSetFocus(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAutoSetFocus(&self, autosetfocus: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ConnectionTimeout(&self) -> ::windows_core::Result<u32>;
    fn SetConnectionTimeout(&self, timeout: u32) -> ::windows_core::Result<()>;
    fn TransactionTimeout(&self) -> ::windows_core::Result<u32>;
    fn SetTransactionTimeout(&self, timeout: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomation2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomation2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: isize>() -> IUIAutomation2_Vtbl {
        unsafe extern "system" fn AutoSetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoSetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(autosetfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoSetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoSetFocus(::core::mem::transmute_copy(&autosetfocus)).into()
        }
        unsafe extern "system" fn ConnectionTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectionTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectionTimeout(::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn TransactionTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransactionTimeout(::core::mem::transmute_copy(&timeout)).into()
        }
        Self {
            base__: IUIAutomation_Vtbl::new::<Identity, Impl, OFFSET>(),
            AutoSetFocus: AutoSetFocus::<Identity, Impl, OFFSET>,
            SetAutoSetFocus: SetAutoSetFocus::<Identity, Impl, OFFSET>,
            ConnectionTimeout: ConnectionTimeout::<Identity, Impl, OFFSET>,
            SetConnectionTimeout: SetConnectionTimeout::<Identity, Impl, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, Impl, OFFSET>,
            SetTransactionTimeout: SetTransactionTimeout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomation2 as ::windows_core::Interface>::IID || iid == &<IUIAutomation as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation3_Impl: Sized + IUIAutomation2_Impl {
    fn AddTextEditTextChangedEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationTextEditTextChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveTextEditTextChangedEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationTextEditTextChangedEventHandler>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomation3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomation3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation3_Impl, const OFFSET: isize>() -> IUIAutomation3_Vtbl {
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTextEditTextChangedEventHandler(::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&texteditchangetype), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn RemoveTextEditTextChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveTextEditTextChangedEventHandler(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: IUIAutomation2_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveTextEditTextChangedEventHandler: RemoveTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomation3 as ::windows_core::Interface>::IID || iid == &<IUIAutomation as ::windows_core::Interface>::IID || iid == &<IUIAutomation2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation4_Impl: Sized + IUIAutomation3_Impl {
    fn AddChangesEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationChangesEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveChangesEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationChangesEventHandler>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomation4 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomation4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation4_Impl, const OFFSET: isize>() -> IUIAutomation4_Vtbl {
        unsafe extern "system" fn AddChangesEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddChangesEventHandler(::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&changetypes), ::core::mem::transmute_copy(&changescount), ::windows_core::from_raw_borrowed(&pcacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn RemoveChangesEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveChangesEventHandler(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: IUIAutomation3_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddChangesEventHandler: AddChangesEventHandler::<Identity, Impl, OFFSET>,
            RemoveChangesEventHandler: RemoveChangesEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomation4 as ::windows_core::Interface>::IID || iid == &<IUIAutomation as ::windows_core::Interface>::IID || iid == &<IUIAutomation2 as ::windows_core::Interface>::IID || iid == &<IUIAutomation3 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation5_Impl: Sized + IUIAutomation4_Impl {
    fn AddNotificationEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationNotificationEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveNotificationEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationNotificationEventHandler>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomation5 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomation5_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation5_Impl, const OFFSET: isize>() -> IUIAutomation5_Vtbl {
        unsafe extern "system" fn AddNotificationEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNotificationEventHandler(::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn RemoveNotificationEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveNotificationEventHandler(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: IUIAutomation4_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddNotificationEventHandler: AddNotificationEventHandler::<Identity, Impl, OFFSET>,
            RemoveNotificationEventHandler: RemoveNotificationEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomation5 as ::windows_core::Interface>::IID || iid == &<IUIAutomation as ::windows_core::Interface>::IID || iid == &<IUIAutomation2 as ::windows_core::Interface>::IID || iid == &<IUIAutomation3 as ::windows_core::Interface>::IID || iid == &<IUIAutomation4 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation6_Impl: Sized + IUIAutomation5_Impl {
    fn CreateEventHandlerGroup(&self) -> ::windows_core::Result<IUIAutomationEventHandlerGroup>;
    fn AddEventHandlerGroup(&self, element: ::core::option::Option<&IUIAutomationElement>, handlergroup: ::core::option::Option<&IUIAutomationEventHandlerGroup>) -> ::windows_core::Result<()>;
    fn RemoveEventHandlerGroup(&self, element: ::core::option::Option<&IUIAutomationElement>, handlergroup: ::core::option::Option<&IUIAutomationEventHandlerGroup>) -> ::windows_core::Result<()>;
    fn ConnectionRecoveryBehavior(&self) -> ::windows_core::Result<ConnectionRecoveryBehaviorOptions>;
    fn SetConnectionRecoveryBehavior(&self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows_core::Result<()>;
    fn CoalesceEvents(&self) -> ::windows_core::Result<CoalesceEventsOptions>;
    fn SetCoalesceEvents(&self, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows_core::Result<()>;
    fn AddActiveTextPositionChangedEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveActiveTextPositionChangedEventHandler(&self, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomation6 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomation6_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>() -> IUIAutomation6_Vtbl {
        unsafe extern "system" fn CreateEventHandlerGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handlergroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEventHandlerGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(handlergroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEventHandlerGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handlergroup: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddEventHandlerGroup(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handlergroup)).into()
        }
        unsafe extern "system" fn RemoveEventHandlerGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handlergroup: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveEventHandlerGroup(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handlergroup)).into()
        }
        unsafe extern "system" fn ConnectionRecoveryBehavior<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectionRecoveryBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionrecoverybehavioroptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionRecoveryBehavior<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectionRecoveryBehavior(::core::mem::transmute_copy(&connectionrecoverybehavioroptions)).into()
        }
        unsafe extern "system" fn CoalesceEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CoalesceEvents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(coalesceeventsoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoalesceEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCoalesceEvents(::core::mem::transmute_copy(&coalesceeventsoptions)).into()
        }
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddActiveTextPositionChangedEventHandler(::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn RemoveActiveTextPositionChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveActiveTextPositionChangedEventHandler(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: IUIAutomation5_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateEventHandlerGroup: CreateEventHandlerGroup::<Identity, Impl, OFFSET>,
            AddEventHandlerGroup: AddEventHandlerGroup::<Identity, Impl, OFFSET>,
            RemoveEventHandlerGroup: RemoveEventHandlerGroup::<Identity, Impl, OFFSET>,
            ConnectionRecoveryBehavior: ConnectionRecoveryBehavior::<Identity, Impl, OFFSET>,
            SetConnectionRecoveryBehavior: SetConnectionRecoveryBehavior::<Identity, Impl, OFFSET>,
            CoalesceEvents: CoalesceEvents::<Identity, Impl, OFFSET>,
            SetCoalesceEvents: SetCoalesceEvents::<Identity, Impl, OFFSET>,
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveActiveTextPositionChangedEventHandler: RemoveActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomation6 as ::windows_core::Interface>::IID || iid == &<IUIAutomation as ::windows_core::Interface>::IID || iid == &<IUIAutomation2 as ::windows_core::Interface>::IID || iid == &<IUIAutomation3 as ::windows_core::Interface>::IID || iid == &<IUIAutomation4 as ::windows_core::Interface>::IID || iid == &<IUIAutomation5 as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationActiveTextPositionChangedEventHandler_Impl: Sized {
    fn HandleActiveTextPositionChangedEvent(&self, sender: ::core::option::Option<&IUIAutomationElement>, range: ::core::option::Option<&IUIAutomationTextRange>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationActiveTextPositionChangedEventHandler {}
impl IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleActiveTextPositionChangedEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleActiveTextPositionChangedEvent(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&range)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleActiveTextPositionChangedEvent: HandleActiveTextPositionChangedEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationActiveTextPositionChangedEventHandler as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationAndCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn ChildCount(&self) -> ::windows_core::Result<i32>;
    fn GetChildrenAsNativeArray(&self, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows_core::Result<()>;
    fn GetChildren(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationAndCondition {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationAndCondition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>() -> IUIAutomationAndCondition_Vtbl {
        unsafe extern "system" fn ChildCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChildrenAsNativeArray(::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            ChildCount: ChildCount::<Identity, Impl, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationAndCondition as ::windows_core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationAnnotationPattern_Impl: Sized {
    fn CurrentAnnotationTypeId(&self) -> ::windows_core::Result<UIA_ANNOTATIONTYPE>;
    fn CurrentAnnotationTypeName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentAuthor(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentDateTime(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentTarget(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedAnnotationTypeId(&self) -> ::windows_core::Result<UIA_ANNOTATIONTYPE>;
    fn CachedAnnotationTypeName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedAuthor(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedDateTime(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedTarget(&self) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::RuntimeName for IUIAutomationAnnotationPattern {}
impl IUIAutomationAnnotationPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>() -> IUIAutomationAnnotationPattern_Vtbl {
        unsafe extern "system" fn CurrentAnnotationTypeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAuthor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAuthor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDateTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAuthor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAuthor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDateTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CurrentAnnotationTypeId: CurrentAnnotationTypeId::<Identity, Impl, OFFSET>,
            CurrentAnnotationTypeName: CurrentAnnotationTypeName::<Identity, Impl, OFFSET>,
            CurrentAuthor: CurrentAuthor::<Identity, Impl, OFFSET>,
            CurrentDateTime: CurrentDateTime::<Identity, Impl, OFFSET>,
            CurrentTarget: CurrentTarget::<Identity, Impl, OFFSET>,
            CachedAnnotationTypeId: CachedAnnotationTypeId::<Identity, Impl, OFFSET>,
            CachedAnnotationTypeName: CachedAnnotationTypeName::<Identity, Impl, OFFSET>,
            CachedAuthor: CachedAuthor::<Identity, Impl, OFFSET>,
            CachedDateTime: CachedDateTime::<Identity, Impl, OFFSET>,
            CachedTarget: CachedTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationAnnotationPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationBoolCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn BooleanValue(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IUIAutomationBoolCondition {}
impl IUIAutomationBoolCondition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationBoolCondition_Impl, const OFFSET: isize>() -> IUIAutomationBoolCondition_Vtbl {
        unsafe extern "system" fn BooleanValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationBoolCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boolval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BooleanValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(boolval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(), BooleanValue: BooleanValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationBoolCondition as ::windows_core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationCacheRequest_Impl: Sized {
    fn AddProperty(&self, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<()>;
    fn AddPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IUIAutomationCacheRequest>;
    fn TreeScope(&self) -> ::windows_core::Result<TreeScope>;
    fn SetTreeScope(&self, scope: TreeScope) -> ::windows_core::Result<()>;
    fn TreeFilter(&self) -> ::windows_core::Result<IUIAutomationCondition>;
    fn SetTreeFilter(&self, filter: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<()>;
    fn AutomationElementMode(&self) -> ::windows_core::Result<AutomationElementMode>;
    fn SetAutomationElementMode(&self, mode: AutomationElementMode) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationCacheRequest {}
impl IUIAutomationCacheRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>() -> IUIAutomationCacheRequest_Vtbl {
        unsafe extern "system" fn AddProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddProperty(::core::mem::transmute_copy(&propertyid)).into()
        }
        unsafe extern "system" fn AddPattern<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPattern(::core::mem::transmute_copy(&patternid)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clonedrequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TreeScope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut TreeScope) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TreeScope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreeScope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTreeScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn TreeFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TreeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreeFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTreeFilter(::windows_core::from_raw_borrowed(&filter)).into()
        }
        unsafe extern "system" fn AutomationElementMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut AutomationElementMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutomationElementMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomationElementMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: AutomationElementMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutomationElementMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProperty: AddProperty::<Identity, Impl, OFFSET>,
            AddPattern: AddPattern::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            TreeScope: TreeScope::<Identity, Impl, OFFSET>,
            SetTreeScope: SetTreeScope::<Identity, Impl, OFFSET>,
            TreeFilter: TreeFilter::<Identity, Impl, OFFSET>,
            SetTreeFilter: SetTreeFilter::<Identity, Impl, OFFSET>,
            AutomationElementMode: AutomationElementMode::<Identity, Impl, OFFSET>,
            SetAutomationElementMode: SetAutomationElementMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationCacheRequest as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationChangesEventHandler_Impl: Sized {
    fn HandleChangesEvent(&self, sender: ::core::option::Option<&IUIAutomationElement>, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationChangesEventHandler {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationChangesEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationChangesEventHandler_Vtbl {
        unsafe extern "system" fn HandleChangesEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleChangesEvent(::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&uiachanges), ::core::mem::transmute_copy(&changescount)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleChangesEvent: HandleChangesEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationChangesEventHandler as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationCondition_Impl: Sized {}
impl ::windows_core::RuntimeName for IUIAutomationCondition {}
impl IUIAutomationCondition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCondition_Impl, const OFFSET: isize>() -> IUIAutomationCondition_Vtbl {
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationCondition as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationCustomNavigationPattern_Impl: Sized {
    fn Navigate(&self, direction: NavigateDirection) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::RuntimeName for IUIAutomationCustomNavigationPattern {}
impl IUIAutomationCustomNavigationPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>() -> IUIAutomationCustomNavigationPattern_Vtbl {
        unsafe extern "system" fn Navigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Navigate: Navigate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationCustomNavigationPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationDockPattern_Impl: Sized {
    fn SetDockPosition(&self, dockpos: DockPosition) -> ::windows_core::Result<()>;
    fn CurrentDockPosition(&self) -> ::windows_core::Result<DockPosition>;
    fn CachedDockPosition(&self) -> ::windows_core::Result<DockPosition>;
}
impl ::windows_core::RuntimeName for IUIAutomationDockPattern {}
impl IUIAutomationDockPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>() -> IUIAutomationDockPattern_Vtbl {
        unsafe extern "system" fn SetDockPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockpos: DockPosition) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDockPosition(::core::mem::transmute_copy(&dockpos)).into()
        }
        unsafe extern "system" fn CurrentDockPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDockPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDockPosition: SetDockPosition::<Identity, Impl, OFFSET>,
            CurrentDockPosition: CurrentDockPosition::<Identity, Impl, OFFSET>,
            CachedDockPosition: CachedDockPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationDockPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationDragPattern_Impl: Sized {
    fn CurrentIsGrabbed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsGrabbed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentDropEffect(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedDropEffect(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentDropEffects(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedDropEffects(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentGrabbedItems(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedGrabbedItems(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationDragPattern {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationDragPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>() -> IUIAutomationDragPattern_Vtbl {
        unsafe extern "system" fn CurrentIsGrabbed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsGrabbed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropEffect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropEffect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropEffects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropEffects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentGrabbedItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedGrabbedItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CurrentIsGrabbed: CurrentIsGrabbed::<Identity, Impl, OFFSET>,
            CachedIsGrabbed: CachedIsGrabbed::<Identity, Impl, OFFSET>,
            CurrentDropEffect: CurrentDropEffect::<Identity, Impl, OFFSET>,
            CachedDropEffect: CachedDropEffect::<Identity, Impl, OFFSET>,
            CurrentDropEffects: CurrentDropEffects::<Identity, Impl, OFFSET>,
            CachedDropEffects: CachedDropEffects::<Identity, Impl, OFFSET>,
            GetCurrentGrabbedItems: GetCurrentGrabbedItems::<Identity, Impl, OFFSET>,
            GetCachedGrabbedItems: GetCachedGrabbedItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationDragPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationDropTargetPattern_Impl: Sized {
    fn CurrentDropTargetEffect(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedDropTargetEffect(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentDropTargetEffects(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedDropTargetEffects(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationDropTargetPattern {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationDropTargetPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>() -> IUIAutomationDropTargetPattern_Vtbl {
        unsafe extern "system" fn CurrentDropTargetEffect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropTargetEffect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropTargetEffects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropTargetEffects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CurrentDropTargetEffect: CurrentDropTargetEffect::<Identity, Impl, OFFSET>,
            CachedDropTargetEffect: CachedDropTargetEffect::<Identity, Impl, OFFSET>,
            CurrentDropTargetEffects: CurrentDropTargetEffects::<Identity, Impl, OFFSET>,
            CachedDropTargetEffects: CachedDropTargetEffects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationDropTargetPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement_Impl: Sized {
    fn SetFocus(&self) -> ::windows_core::Result<()>;
    fn GetRuntimeId(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn FindFirst(&self, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationElement>;
    fn FindAll(&self, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstBuildCache(&self, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn FindAllBuildCache(&self, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn BuildUpdatedCache(&self, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCurrentPropertyValueEx(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCachedPropertyValueEx(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCurrentPatternAs(&self, patternid: UIA_PATTERN_ID, riid: *const ::windows_core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCachedPatternAs(&self, patternid: UIA_PATTERN_ID, riid: *const ::windows_core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetCachedParent(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetCachedChildren(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentProcessId(&self) -> ::windows_core::Result<i32>;
    fn CurrentControlType(&self) -> ::windows_core::Result<UIA_CONTROLTYPE_ID>;
    fn CurrentLocalizedControlType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentAcceleratorKey(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentAccessKey(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentHasKeyboardFocus(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsKeyboardFocusable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsEnabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentAutomationId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentClassName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentHelpText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentCulture(&self) -> ::windows_core::Result<i32>;
    fn CurrentIsControlElement(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsContentElement(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsPassword(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentNativeWindowHandle(&self) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn CurrentItemType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentIsOffscreen(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentOrientation(&self) -> ::windows_core::Result<OrientationType>;
    fn CurrentFrameworkId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentIsRequiredForForm(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentItemStatus(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentBoundingRectangle(&self) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn CurrentLabeledBy(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentAriaRole(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentAriaProperties(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentIsDataValidForForm(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentControllerFor(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentDescribedBy(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentFlowsTo(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentProviderDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedProcessId(&self) -> ::windows_core::Result<i32>;
    fn CachedControlType(&self) -> ::windows_core::Result<UIA_CONTROLTYPE_ID>;
    fn CachedLocalizedControlType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedAcceleratorKey(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedAccessKey(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedHasKeyboardFocus(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsKeyboardFocusable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsEnabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedAutomationId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedClassName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedHelpText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedCulture(&self) -> ::windows_core::Result<i32>;
    fn CachedIsControlElement(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsContentElement(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsPassword(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedNativeWindowHandle(&self) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn CachedItemType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedIsOffscreen(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedOrientation(&self) -> ::windows_core::Result<OrientationType>;
    fn CachedFrameworkId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedIsRequiredForForm(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedItemStatus(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedBoundingRectangle(&self) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn CachedLabeledBy(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedAriaRole(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedAriaProperties(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedIsDataValidForForm(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedControllerFor(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedDescribedBy(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedFlowsTo(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedProviderDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>() -> IUIAutomationElement_Vtbl {
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFocus().into()
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(runtimeid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirst<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirst(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindAll(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstBuildCache(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindAllBuildCache(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildUpdatedCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, updatedelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BuildUpdatedCache(::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(updatedelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValueEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentPropertyValueEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ignoredefaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPropertyValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPropertyValueEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedPropertyValueEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ignoredefaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPatternAs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, riid: *const ::windows_core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentPatternAs(::core::mem::transmute_copy(&patternid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&patternobject)).into()
        }
        unsafe extern "system" fn GetCachedPatternAs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, riid: *const ::windows_core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCachedPatternAs(::core::mem::transmute_copy(&patternid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&patternobject)).into()
        }
        unsafe extern "system" fn GetCurrentPattern<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentPattern(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(patternobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPattern<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedPattern(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(patternobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProcessId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentControlType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_CONTROLTYPE_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentControlType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocalizedControlType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentLocalizedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAcceleratorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAcceleratorKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAccessKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHasKeyboardFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentHasKeyboardFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsKeyboardFocusable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsKeyboardFocusable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAutomationId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAutomationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentClassName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentClassName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHelpText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCulture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCulture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsControlElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsControlElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsContentElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsContentElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsPassword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsPassword() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentNativeWindowHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentNativeWindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentItemType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsOffscreen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsOffscreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentOrientation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFrameworkId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFrameworkId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsRequiredForForm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsRequiredForForm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentItemStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentBoundingRectangle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLabeledBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentLabeledBy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAriaRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAriaRole() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAriaProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAriaProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsDataValidForForm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsDataValidForForm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentControllerFor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentControllerFor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDescribedBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDescribedBy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFlowsTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFlowsTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProviderDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentProviderDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedProcessId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedControlType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_CONTROLTYPE_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLocalizedControlType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedLocalizedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAcceleratorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAcceleratorKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAccessKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHasKeyboardFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedHasKeyboardFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsKeyboardFocusable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsKeyboardFocusable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAutomationId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAutomationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedClassName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedClassName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHelpText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCulture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCulture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsControlElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsControlElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsContentElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsContentElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsPassword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsPassword() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedNativeWindowHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedNativeWindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedItemType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsOffscreen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsOffscreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedOrientation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFrameworkId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFrameworkId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsRequiredForForm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsRequiredForForm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedItemStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedBoundingRectangle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLabeledBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedLabeledBy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAriaRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAriaRole() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAriaProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAriaProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsDataValidForForm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsDataValidForForm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedControllerFor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedControllerFor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDescribedBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDescribedBy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFlowsTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFlowsTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedProviderDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedProviderDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClickablePoint(::core::mem::transmute_copy(&clickable), ::core::mem::transmute_copy(&gotclickable)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindAll: FindAll::<Identity, Impl, OFFSET>,
            FindFirstBuildCache: FindFirstBuildCache::<Identity, Impl, OFFSET>,
            FindAllBuildCache: FindAllBuildCache::<Identity, Impl, OFFSET>,
            BuildUpdatedCache: BuildUpdatedCache::<Identity, Impl, OFFSET>,
            GetCurrentPropertyValue: GetCurrentPropertyValue::<Identity, Impl, OFFSET>,
            GetCurrentPropertyValueEx: GetCurrentPropertyValueEx::<Identity, Impl, OFFSET>,
            GetCachedPropertyValue: GetCachedPropertyValue::<Identity, Impl, OFFSET>,
            GetCachedPropertyValueEx: GetCachedPropertyValueEx::<Identity, Impl, OFFSET>,
            GetCurrentPatternAs: GetCurrentPatternAs::<Identity, Impl, OFFSET>,
            GetCachedPatternAs: GetCachedPatternAs::<Identity, Impl, OFFSET>,
            GetCurrentPattern: GetCurrentPattern::<Identity, Impl, OFFSET>,
            GetCachedPattern: GetCachedPattern::<Identity, Impl, OFFSET>,
            GetCachedParent: GetCachedParent::<Identity, Impl, OFFSET>,
            GetCachedChildren: GetCachedChildren::<Identity, Impl, OFFSET>,
            CurrentProcessId: CurrentProcessId::<Identity, Impl, OFFSET>,
            CurrentControlType: CurrentControlType::<Identity, Impl, OFFSET>,
            CurrentLocalizedControlType: CurrentLocalizedControlType::<Identity, Impl, OFFSET>,
            CurrentName: CurrentName::<Identity, Impl, OFFSET>,
            CurrentAcceleratorKey: CurrentAcceleratorKey::<Identity, Impl, OFFSET>,
            CurrentAccessKey: CurrentAccessKey::<Identity, Impl, OFFSET>,
            CurrentHasKeyboardFocus: CurrentHasKeyboardFocus::<Identity, Impl, OFFSET>,
            CurrentIsKeyboardFocusable: CurrentIsKeyboardFocusable::<Identity, Impl, OFFSET>,
            CurrentIsEnabled: CurrentIsEnabled::<Identity, Impl, OFFSET>,
            CurrentAutomationId: CurrentAutomationId::<Identity, Impl, OFFSET>,
            CurrentClassName: CurrentClassName::<Identity, Impl, OFFSET>,
            CurrentHelpText: CurrentHelpText::<Identity, Impl, OFFSET>,
            CurrentCulture: CurrentCulture::<Identity, Impl, OFFSET>,
            CurrentIsControlElement: CurrentIsControlElement::<Identity, Impl, OFFSET>,
            CurrentIsContentElement: CurrentIsContentElement::<Identity, Impl, OFFSET>,
            CurrentIsPassword: CurrentIsPassword::<Identity, Impl, OFFSET>,
            CurrentNativeWindowHandle: CurrentNativeWindowHandle::<Identity, Impl, OFFSET>,
            CurrentItemType: CurrentItemType::<Identity, Impl, OFFSET>,
            CurrentIsOffscreen: CurrentIsOffscreen::<Identity, Impl, OFFSET>,
            CurrentOrientation: CurrentOrientation::<Identity, Impl, OFFSET>,
            CurrentFrameworkId: CurrentFrameworkId::<Identity, Impl, OFFSET>,
            CurrentIsRequiredForForm: CurrentIsRequiredForForm::<Identity, Impl, OFFSET>,
            CurrentItemStatus: CurrentItemStatus::<Identity, Impl, OFFSET>,
            CurrentBoundingRectangle: CurrentBoundingRectangle::<Identity, Impl, OFFSET>,
            CurrentLabeledBy: CurrentLabeledBy::<Identity, Impl, OFFSET>,
            CurrentAriaRole: CurrentAriaRole::<Identity, Impl, OFFSET>,
            CurrentAriaProperties: CurrentAriaProperties::<Identity, Impl, OFFSET>,
            CurrentIsDataValidForForm: CurrentIsDataValidForForm::<Identity, Impl, OFFSET>,
            CurrentControllerFor: CurrentControllerFor::<Identity, Impl, OFFSET>,
            CurrentDescribedBy: CurrentDescribedBy::<Identity, Impl, OFFSET>,
            CurrentFlowsTo: CurrentFlowsTo::<Identity, Impl, OFFSET>,
            CurrentProviderDescription: CurrentProviderDescription::<Identity, Impl, OFFSET>,
            CachedProcessId: CachedProcessId::<Identity, Impl, OFFSET>,
            CachedControlType: CachedControlType::<Identity, Impl, OFFSET>,
            CachedLocalizedControlType: CachedLocalizedControlType::<Identity, Impl, OFFSET>,
            CachedName: CachedName::<Identity, Impl, OFFSET>,
            CachedAcceleratorKey: CachedAcceleratorKey::<Identity, Impl, OFFSET>,
            CachedAccessKey: CachedAccessKey::<Identity, Impl, OFFSET>,
            CachedHasKeyboardFocus: CachedHasKeyboardFocus::<Identity, Impl, OFFSET>,
            CachedIsKeyboardFocusable: CachedIsKeyboardFocusable::<Identity, Impl, OFFSET>,
            CachedIsEnabled: CachedIsEnabled::<Identity, Impl, OFFSET>,
            CachedAutomationId: CachedAutomationId::<Identity, Impl, OFFSET>,
            CachedClassName: CachedClassName::<Identity, Impl, OFFSET>,
            CachedHelpText: CachedHelpText::<Identity, Impl, OFFSET>,
            CachedCulture: CachedCulture::<Identity, Impl, OFFSET>,
            CachedIsControlElement: CachedIsControlElement::<Identity, Impl, OFFSET>,
            CachedIsContentElement: CachedIsContentElement::<Identity, Impl, OFFSET>,
            CachedIsPassword: CachedIsPassword::<Identity, Impl, OFFSET>,
            CachedNativeWindowHandle: CachedNativeWindowHandle::<Identity, Impl, OFFSET>,
            CachedItemType: CachedItemType::<Identity, Impl, OFFSET>,
            CachedIsOffscreen: CachedIsOffscreen::<Identity, Impl, OFFSET>,
            CachedOrientation: CachedOrientation::<Identity, Impl, OFFSET>,
            CachedFrameworkId: CachedFrameworkId::<Identity, Impl, OFFSET>,
            CachedIsRequiredForForm: CachedIsRequiredForForm::<Identity, Impl, OFFSET>,
            CachedItemStatus: CachedItemStatus::<Identity, Impl, OFFSET>,
            CachedBoundingRectangle: CachedBoundingRectangle::<Identity, Impl, OFFSET>,
            CachedLabeledBy: CachedLabeledBy::<Identity, Impl, OFFSET>,
            CachedAriaRole: CachedAriaRole::<Identity, Impl, OFFSET>,
            CachedAriaProperties: CachedAriaProperties::<Identity, Impl, OFFSET>,
            CachedIsDataValidForForm: CachedIsDataValidForForm::<Identity, Impl, OFFSET>,
            CachedControllerFor: CachedControllerFor::<Identity, Impl, OFFSET>,
            CachedDescribedBy: CachedDescribedBy::<Identity, Impl, OFFSET>,
            CachedFlowsTo: CachedFlowsTo::<Identity, Impl, OFFSET>,
            CachedProviderDescription: CachedProviderDescription::<Identity, Impl, OFFSET>,
            GetClickablePoint: GetClickablePoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement2_Impl: Sized + IUIAutomationElement_Impl {
    fn CurrentOptimizeForVisualContent(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedOptimizeForVisualContent(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentLiveSetting(&self) -> ::windows_core::Result<LiveSetting>;
    fn CachedLiveSetting(&self) -> ::windows_core::Result<LiveSetting>;
    fn CurrentFlowsFrom(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedFlowsFrom(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>() -> IUIAutomationElement2_Vtbl {
        unsafe extern "system" fn CurrentOptimizeForVisualContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentOptimizeForVisualContent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedOptimizeForVisualContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedOptimizeForVisualContent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLiveSetting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentLiveSetting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLiveSetting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedLiveSetting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFlowsFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFlowsFrom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFlowsFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFlowsFrom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentOptimizeForVisualContent: CurrentOptimizeForVisualContent::<Identity, Impl, OFFSET>,
            CachedOptimizeForVisualContent: CachedOptimizeForVisualContent::<Identity, Impl, OFFSET>,
            CurrentLiveSetting: CurrentLiveSetting::<Identity, Impl, OFFSET>,
            CachedLiveSetting: CachedLiveSetting::<Identity, Impl, OFFSET>,
            CurrentFlowsFrom: CurrentFlowsFrom::<Identity, Impl, OFFSET>,
            CachedFlowsFrom: CachedFlowsFrom::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement3_Impl: Sized + IUIAutomationElement2_Impl {
    fn ShowContextMenu(&self) -> ::windows_core::Result<()>;
    fn CurrentIsPeripheral(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsPeripheral(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement3_Impl, const OFFSET: isize>() -> IUIAutomationElement3_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowContextMenu().into()
        }
        unsafe extern "system" fn CurrentIsPeripheral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsPeripheral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsPeripheral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsPeripheral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationElement2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET>,
            CurrentIsPeripheral: CurrentIsPeripheral::<Identity, Impl, OFFSET>,
            CachedIsPeripheral: CachedIsPeripheral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement3 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement4_Impl: Sized + IUIAutomationElement3_Impl {
    fn CurrentPositionInSet(&self) -> ::windows_core::Result<i32>;
    fn CurrentSizeOfSet(&self) -> ::windows_core::Result<i32>;
    fn CurrentLevel(&self) -> ::windows_core::Result<i32>;
    fn CurrentAnnotationTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentAnnotationObjects(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedPositionInSet(&self) -> ::windows_core::Result<i32>;
    fn CachedSizeOfSet(&self) -> ::windows_core::Result<i32>;
    fn CachedLevel(&self) -> ::windows_core::Result<i32>;
    fn CachedAnnotationTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedAnnotationObjects(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement4 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>() -> IUIAutomationElement4_Vtbl {
        unsafe extern "system" fn CurrentPositionInSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPositionInSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSizeOfSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentSizeOfSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedPositionInSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedPositionInSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSizeOfSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedSizeOfSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationElement3_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentPositionInSet: CurrentPositionInSet::<Identity, Impl, OFFSET>,
            CurrentSizeOfSet: CurrentSizeOfSet::<Identity, Impl, OFFSET>,
            CurrentLevel: CurrentLevel::<Identity, Impl, OFFSET>,
            CurrentAnnotationTypes: CurrentAnnotationTypes::<Identity, Impl, OFFSET>,
            CurrentAnnotationObjects: CurrentAnnotationObjects::<Identity, Impl, OFFSET>,
            CachedPositionInSet: CachedPositionInSet::<Identity, Impl, OFFSET>,
            CachedSizeOfSet: CachedSizeOfSet::<Identity, Impl, OFFSET>,
            CachedLevel: CachedLevel::<Identity, Impl, OFFSET>,
            CachedAnnotationTypes: CachedAnnotationTypes::<Identity, Impl, OFFSET>,
            CachedAnnotationObjects: CachedAnnotationObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement4 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement5_Impl: Sized + IUIAutomationElement4_Impl {
    fn CurrentLandmarkType(&self) -> ::windows_core::Result<UIA_LANDMARKTYPE_ID>;
    fn CurrentLocalizedLandmarkType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedLandmarkType(&self) -> ::windows_core::Result<UIA_LANDMARKTYPE_ID>;
    fn CachedLocalizedLandmarkType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement5 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement5_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>() -> IUIAutomationElement5_Vtbl {
        unsafe extern "system" fn CurrentLandmarkType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_LANDMARKTYPE_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocalizedLandmarkType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentLocalizedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLandmarkType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_LANDMARKTYPE_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLocalizedLandmarkType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedLocalizedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationElement4_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentLandmarkType: CurrentLandmarkType::<Identity, Impl, OFFSET>,
            CurrentLocalizedLandmarkType: CurrentLocalizedLandmarkType::<Identity, Impl, OFFSET>,
            CachedLandmarkType: CachedLandmarkType::<Identity, Impl, OFFSET>,
            CachedLocalizedLandmarkType: CachedLocalizedLandmarkType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement5 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement6_Impl: Sized + IUIAutomationElement5_Impl {
    fn CurrentFullDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedFullDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement6 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement6_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement6_Impl, const OFFSET: isize>() -> IUIAutomationElement6_Vtbl {
        unsafe extern "system" fn CurrentFullDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFullDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFullDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFullDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationElement5_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentFullDescription: CurrentFullDescription::<Identity, Impl, OFFSET>,
            CachedFullDescription: CachedFullDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement6 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement7_Impl: Sized + IUIAutomationElement6_Impl {
    fn FindFirstWithOptions(&self, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn FindAllWithOptions(&self, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstWithOptionsBuildCache(&self, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn FindAllWithOptionsBuildCache(&self, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: UIA_METADATA_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement7 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement7_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>() -> IUIAutomationElement7_Vtbl {
        unsafe extern "system" fn FindFirstWithOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstWithOptions(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::core::mem::transmute_copy(&traversaloptions), ::windows_core::from_raw_borrowed(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindAllWithOptions(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::core::mem::transmute_copy(&traversaloptions), ::windows_core::from_raw_borrowed(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstWithOptionsBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstWithOptionsBuildCache(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::windows_core::from_raw_borrowed(&cacherequest), ::core::mem::transmute_copy(&traversaloptions), ::windows_core::from_raw_borrowed(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithOptionsBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindAllWithOptionsBuildCache(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::windows_core::from_raw_borrowed(&cacherequest), ::core::mem::transmute_copy(&traversaloptions), ::windows_core::from_raw_borrowed(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentMetadataValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: UIA_METADATA_ID, returnval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentMetadataValue(::core::mem::transmute_copy(&targetid), ::core::mem::transmute_copy(&metadataid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationElement6_Vtbl::new::<Identity, Impl, OFFSET>(),
            FindFirstWithOptions: FindFirstWithOptions::<Identity, Impl, OFFSET>,
            FindAllWithOptions: FindAllWithOptions::<Identity, Impl, OFFSET>,
            FindFirstWithOptionsBuildCache: FindFirstWithOptionsBuildCache::<Identity, Impl, OFFSET>,
            FindAllWithOptionsBuildCache: FindAllWithOptionsBuildCache::<Identity, Impl, OFFSET>,
            GetCurrentMetadataValue: GetCurrentMetadataValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement7 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement6 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement8_Impl: Sized + IUIAutomationElement7_Impl {
    fn CurrentHeadingLevel(&self) -> ::windows_core::Result<UIA_HEADINGLEVEL_ID>;
    fn CachedHeadingLevel(&self) -> ::windows_core::Result<UIA_HEADINGLEVEL_ID>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement8 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement8_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement8_Impl, const OFFSET: isize>() -> IUIAutomationElement8_Vtbl {
        unsafe extern "system" fn CurrentHeadingLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_HEADINGLEVEL_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentHeadingLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHeadingLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_HEADINGLEVEL_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedHeadingLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationElement7_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentHeadingLevel: CurrentHeadingLevel::<Identity, Impl, OFFSET>,
            CachedHeadingLevel: CachedHeadingLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement8 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement6 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement7 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement9_Impl: Sized + IUIAutomationElement8_Impl {
    fn CurrentIsDialog(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsDialog(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationElement9 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationElement9_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement9_Impl, const OFFSET: isize>() -> IUIAutomationElement9_Vtbl {
        unsafe extern "system" fn CurrentIsDialog<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsDialog<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationElement8_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentIsDialog: CurrentIsDialog::<Identity, Impl, OFFSET>,
            CachedIsDialog: CachedIsDialog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement9 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement6 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement7 as ::windows_core::Interface>::IID || iid == &<IUIAutomationElement8 as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationElementArray_Impl: Sized {
    fn Length(&self) -> ::windows_core::Result<i32>;
    fn GetElement(&self, index: i32) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::RuntimeName for IUIAutomationElementArray {}
impl IUIAutomationElementArray_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElementArray_Impl, const OFFSET: isize>() -> IUIAutomationElementArray_Vtbl {
        unsafe extern "system" fn Length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetElement(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Length: Length::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationElementArray as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationEventHandler_Impl: Sized {
    fn HandleAutomationEvent(&self, sender: ::core::option::Option<&IUIAutomationElement>, eventid: UIA_EVENT_ID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationEventHandler {}
impl IUIAutomationEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationEventHandler_Vtbl {
        unsafe extern "system" fn HandleAutomationEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleAutomationEvent(::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&eventid)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleAutomationEvent: HandleAutomationEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationEventHandler as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationEventHandlerGroup_Impl: Sized {
    fn AddActiveTextPositionChangedEventHandler(&self, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows_core::Result<()>;
    fn AddAutomationEventHandler(&self, eventid: UIA_EVENT_ID, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationEventHandler>) -> ::windows_core::Result<()>;
    fn AddChangesEventHandler(&self, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationChangesEventHandler>) -> ::windows_core::Result<()>;
    fn AddNotificationEventHandler(&self, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationNotificationEventHandler>) -> ::windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(&self, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationPropertyChangedEventHandler>, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows_core::Result<()>;
    fn AddStructureChangedEventHandler(&self, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationStructureChangedEventHandler>) -> ::windows_core::Result<()>;
    fn AddTextEditTextChangedEventHandler(&self, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationTextEditTextChangedEventHandler>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationEventHandlerGroup {}
impl IUIAutomationEventHandlerGroup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>() -> IUIAutomationEventHandlerGroup_Vtbl {
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddActiveTextPositionChangedEventHandler(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn AddChangesEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddChangesEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&changetypes), ::core::mem::transmute_copy(&changescount), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn AddNotificationEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNotificationEventHandler(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyChangedEventHandler(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStructureChangedEventHandler(::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTextEditTextChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&texteditchangetype), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, Impl, OFFSET>,
            AddChangesEventHandler: AddChangesEventHandler::<Identity, Impl, OFFSET>,
            AddNotificationEventHandler: AddNotificationEventHandler::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationEventHandlerGroup as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationExpandCollapsePattern_Impl: Sized {
    fn Expand(&self) -> ::windows_core::Result<()>;
    fn Collapse(&self) -> ::windows_core::Result<()>;
    fn CurrentExpandCollapseState(&self) -> ::windows_core::Result<ExpandCollapseState>;
    fn CachedExpandCollapseState(&self) -> ::windows_core::Result<ExpandCollapseState>;
}
impl ::windows_core::RuntimeName for IUIAutomationExpandCollapsePattern {}
impl IUIAutomationExpandCollapsePattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>() -> IUIAutomationExpandCollapsePattern_Vtbl {
        unsafe extern "system" fn Expand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Expand().into()
        }
        unsafe extern "system" fn Collapse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Collapse().into()
        }
        unsafe extern "system" fn CurrentExpandCollapseState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedExpandCollapseState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Expand: Expand::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            CurrentExpandCollapseState: CurrentExpandCollapseState::<Identity, Impl, OFFSET>,
            CachedExpandCollapseState: CachedExpandCollapseState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationExpandCollapsePattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationFocusChangedEventHandler_Impl: Sized {
    fn HandleFocusChangedEvent(&self, sender: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationFocusChangedEventHandler {}
impl IUIAutomationFocusChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationFocusChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleFocusChangedEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleFocusChangedEvent(::windows_core::from_raw_borrowed(&sender)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleFocusChangedEvent: HandleFocusChangedEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationFocusChangedEventHandler as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationGridItemPattern_Impl: Sized {
    fn CurrentContainingGrid(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentRow(&self) -> ::windows_core::Result<i32>;
    fn CurrentColumn(&self) -> ::windows_core::Result<i32>;
    fn CurrentRowSpan(&self) -> ::windows_core::Result<i32>;
    fn CurrentColumnSpan(&self) -> ::windows_core::Result<i32>;
    fn CachedContainingGrid(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedRow(&self) -> ::windows_core::Result<i32>;
    fn CachedColumn(&self) -> ::windows_core::Result<i32>;
    fn CachedRowSpan(&self) -> ::windows_core::Result<i32>;
    fn CachedColumnSpan(&self) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IUIAutomationGridItemPattern {}
impl IUIAutomationGridItemPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationGridItemPattern_Vtbl {
        unsafe extern "system" fn CurrentContainingGrid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentRow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentColumn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowSpan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentRowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumnSpan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedContainingGrid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedRow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedColumn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowSpan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedRowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumnSpan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CurrentContainingGrid: CurrentContainingGrid::<Identity, Impl, OFFSET>,
            CurrentRow: CurrentRow::<Identity, Impl, OFFSET>,
            CurrentColumn: CurrentColumn::<Identity, Impl, OFFSET>,
            CurrentRowSpan: CurrentRowSpan::<Identity, Impl, OFFSET>,
            CurrentColumnSpan: CurrentColumnSpan::<Identity, Impl, OFFSET>,
            CachedContainingGrid: CachedContainingGrid::<Identity, Impl, OFFSET>,
            CachedRow: CachedRow::<Identity, Impl, OFFSET>,
            CachedColumn: CachedColumn::<Identity, Impl, OFFSET>,
            CachedRowSpan: CachedRowSpan::<Identity, Impl, OFFSET>,
            CachedColumnSpan: CachedColumnSpan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationGridItemPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationGridPattern_Impl: Sized {
    fn GetItem(&self, row: i32, column: i32) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentRowCount(&self) -> ::windows_core::Result<i32>;
    fn CurrentColumnCount(&self) -> ::windows_core::Result<i32>;
    fn CachedRowCount(&self) -> ::windows_core::Result<i32>;
    fn CachedColumnCount(&self) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IUIAutomationGridPattern {}
impl IUIAutomationGridPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>() -> IUIAutomationGridPattern_Vtbl {
        unsafe extern "system" fn GetItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItem(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentRowCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumnCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedRowCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumnCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            CurrentRowCount: CurrentRowCount::<Identity, Impl, OFFSET>,
            CurrentColumnCount: CurrentColumnCount::<Identity, Impl, OFFSET>,
            CachedRowCount: CachedRowCount::<Identity, Impl, OFFSET>,
            CachedColumnCount: CachedColumnCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationGridPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationInvokePattern_Impl: Sized {
    fn Invoke(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationInvokePattern {}
impl IUIAutomationInvokePattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationInvokePattern_Impl, const OFFSET: isize>() -> IUIAutomationInvokePattern_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationInvokePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationInvokePattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationItemContainerPattern_Impl: Sized {
    fn FindItemByProperty(&self, pstartafter: ::core::option::Option<&IUIAutomationElement>, propertyid: UIA_PROPERTY_ID, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationItemContainerPattern {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationItemContainerPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>() -> IUIAutomationItemContainerPattern_Vtbl {
        unsafe extern "system" fn FindItemByProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Variant::VARIANT, pfound: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindItemByProperty(::windows_core::from_raw_borrowed(&pstartafter), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindItemByProperty: FindItemByProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationItemContainerPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationLegacyIAccessiblePattern_Impl: Sized {
    fn Select(&self, flagsselect: i32) -> ::windows_core::Result<()>;
    fn DoDefaultAction(&self) -> ::windows_core::Result<()>;
    fn SetValue(&self, szvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CurrentChildId(&self) -> ::windows_core::Result<i32>;
    fn CurrentName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentRole(&self) -> ::windows_core::Result<u32>;
    fn CurrentState(&self) -> ::windows_core::Result<u32>;
    fn CurrentHelp(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentKeyboardShortcut(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCurrentSelection(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentDefaultAction(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedChildId(&self) -> ::windows_core::Result<i32>;
    fn CachedName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedRole(&self) -> ::windows_core::Result<u32>;
    fn CachedState(&self) -> ::windows_core::Result<u32>;
    fn CachedHelp(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedKeyboardShortcut(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCachedSelection(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedDefaultAction(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetIAccessible(&self) -> ::windows_core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationLegacyIAccessiblePattern {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationLegacyIAccessiblePattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>() -> IUIAutomationLegacyIAccessiblePattern_Vtbl {
        unsafe extern "system" fn Select<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Select(::core::mem::transmute_copy(&flagsselect)).into()
        }
        unsafe extern "system" fn DoDefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoDefaultAction().into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&szvalue)).into()
        }
        unsafe extern "system" fn CurrentChildId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentChildId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentRole() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHelp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentHelp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszhelp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentKeyboardShortcut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentKeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszkeyboardshortcut, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselectedchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdefaultaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedChildId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedChildId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedRole() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHelp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedHelp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszhelp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedKeyboardShortcut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedKeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszkeyboardshortcut, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselectedchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedDefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdefaultaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccessible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, Impl, OFFSET>,
            DoDefaultAction: DoDefaultAction::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentChildId: CurrentChildId::<Identity, Impl, OFFSET>,
            CurrentName: CurrentName::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentDescription: CurrentDescription::<Identity, Impl, OFFSET>,
            CurrentRole: CurrentRole::<Identity, Impl, OFFSET>,
            CurrentState: CurrentState::<Identity, Impl, OFFSET>,
            CurrentHelp: CurrentHelp::<Identity, Impl, OFFSET>,
            CurrentKeyboardShortcut: CurrentKeyboardShortcut::<Identity, Impl, OFFSET>,
            GetCurrentSelection: GetCurrentSelection::<Identity, Impl, OFFSET>,
            CurrentDefaultAction: CurrentDefaultAction::<Identity, Impl, OFFSET>,
            CachedChildId: CachedChildId::<Identity, Impl, OFFSET>,
            CachedName: CachedName::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedDescription: CachedDescription::<Identity, Impl, OFFSET>,
            CachedRole: CachedRole::<Identity, Impl, OFFSET>,
            CachedState: CachedState::<Identity, Impl, OFFSET>,
            CachedHelp: CachedHelp::<Identity, Impl, OFFSET>,
            CachedKeyboardShortcut: CachedKeyboardShortcut::<Identity, Impl, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, Impl, OFFSET>,
            CachedDefaultAction: CachedDefaultAction::<Identity, Impl, OFFSET>,
            GetIAccessible: GetIAccessible::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationLegacyIAccessiblePattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationMultipleViewPattern_Impl: Sized {
    fn GetViewName(&self, view: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCurrentView(&self, view: i32) -> ::windows_core::Result<()>;
    fn CurrentCurrentView(&self) -> ::windows_core::Result<i32>;
    fn GetCurrentSupportedViews(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedCurrentView(&self) -> ::windows_core::Result<i32>;
    fn GetCachedSupportedViews(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationMultipleViewPattern {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationMultipleViewPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>() -> IUIAutomationMultipleViewPattern_Vtbl {
        unsafe extern "system" fn GetViewName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewName(::core::mem::transmute_copy(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentView(::core::mem::transmute_copy(&view)).into()
        }
        unsafe extern "system" fn CurrentCurrentView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSupportedViews<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCurrentView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSupportedViews<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetViewName: GetViewName::<Identity, Impl, OFFSET>,
            SetCurrentView: SetCurrentView::<Identity, Impl, OFFSET>,
            CurrentCurrentView: CurrentCurrentView::<Identity, Impl, OFFSET>,
            GetCurrentSupportedViews: GetCurrentSupportedViews::<Identity, Impl, OFFSET>,
            CachedCurrentView: CachedCurrentView::<Identity, Impl, OFFSET>,
            GetCachedSupportedViews: GetCachedSupportedViews::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationMultipleViewPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationNotCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn GetChild(&self) -> ::windows_core::Result<IUIAutomationCondition>;
}
impl ::windows_core::RuntimeName for IUIAutomationNotCondition {}
impl IUIAutomationNotCondition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationNotCondition_Impl, const OFFSET: isize>() -> IUIAutomationNotCondition_Vtbl {
        unsafe extern "system" fn GetChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationNotCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(), GetChild: GetChild::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationNotCondition as ::windows_core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationNotificationEventHandler_Impl: Sized {
    fn HandleNotificationEvent(&self, sender: ::core::option::Option<&IUIAutomationElement>, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: &::windows_core::BSTR, activityid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationNotificationEventHandler {}
impl IUIAutomationNotificationEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationNotificationEventHandler_Vtbl {
        unsafe extern "system" fn HandleNotificationEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, activityid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleNotificationEvent(::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&notificationkind), ::core::mem::transmute_copy(&notificationprocessing), ::core::mem::transmute(&displaystring), ::core::mem::transmute(&activityid)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleNotificationEvent: HandleNotificationEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationNotificationEventHandler as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationObjectModelPattern_Impl: Sized {
    fn GetUnderlyingObjectModel(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for IUIAutomationObjectModelPattern {}
impl IUIAutomationObjectModelPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationObjectModelPattern_Impl, const OFFSET: isize>() -> IUIAutomationObjectModelPattern_Vtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationObjectModelPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUnderlyingObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationObjectModelPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationOrCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn ChildCount(&self) -> ::windows_core::Result<i32>;
    fn GetChildrenAsNativeArray(&self, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows_core::Result<()>;
    fn GetChildren(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationOrCondition {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationOrCondition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>() -> IUIAutomationOrCondition_Vtbl {
        unsafe extern "system" fn ChildCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChildrenAsNativeArray(::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            ChildCount: ChildCount::<Identity, Impl, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationOrCondition as ::windows_core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationPatternHandler_Impl: Sized {
    fn CreateClientWrapper(&self, ppatterninstance: ::core::option::Option<&IUIAutomationPatternInstance>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Dispatch(&self, ptarget: ::core::option::Option<&::windows_core::IUnknown>, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationPatternHandler {}
impl IUIAutomationPatternHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: isize>() -> IUIAutomationPatternHandler_Vtbl {
        unsafe extern "system" fn CreateClientWrapper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppatterninstance: *mut ::core::ffi::c_void, pclientwrapper: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateClientWrapper(::windows_core::from_raw_borrowed(&ppatterninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclientwrapper, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Dispatch(::windows_core::from_raw_borrowed(&ptarget), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&cparams)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateClientWrapper: CreateClientWrapper::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationPatternHandler as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationPatternInstance_Impl: Sized {
    fn GetProperty(&self, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CallMethod(&self, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationPatternInstance {}
impl IUIAutomationPatternInstance_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: isize>() -> IUIAutomationPatternInstance_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&cached), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pptr)).into()
        }
        unsafe extern "system" fn CallMethod<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CallMethod(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&cparams)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            CallMethod: CallMethod::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationPatternInstance as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationPropertyChangedEventHandler_Impl: Sized {
    fn HandlePropertyChangedEvent(&self, sender: ::core::option::Option<&IUIAutomationElement>, propertyid: UIA_PROPERTY_ID, newvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationPropertyChangedEventHandler {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationPropertyChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandlePropertyChangedEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, newvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandlePropertyChangedEvent(::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&newvalue)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandlePropertyChangedEvent: HandlePropertyChangedEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationPropertyChangedEventHandler as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationPropertyCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn PropertyId(&self) -> ::windows_core::Result<UIA_PROPERTY_ID>;
    fn PropertyValue(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PropertyConditionFlags(&self) -> ::windows_core::Result<PropertyConditionFlags>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationPropertyCondition {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationPropertyCondition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>() -> IUIAutomationPropertyCondition_Vtbl {
        unsafe extern "system" fn PropertyId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut UIA_PROPERTY_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyConditionFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut PropertyConditionFlags) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyConditionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            PropertyId: PropertyId::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            PropertyConditionFlags: PropertyConditionFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationPropertyCondition as ::windows_core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationProxyFactory_Impl: Sized {
    fn CreateProvider(&self, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn ProxyFactoryId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IUIAutomationProxyFactory {}
impl IUIAutomationProxyFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: isize>() -> IUIAutomationProxyFactory_Vtbl {
        unsafe extern "system" fn CreateProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, provider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateProvider(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(provider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyFactoryId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoryid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyFactoryId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factoryid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateProvider: CreateProvider::<Identity, Impl, OFFSET>,
            ProxyFactoryId: ProxyFactoryId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactory as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationProxyFactoryEntry_Impl: Sized {
    fn ProxyFactory(&self) -> ::windows_core::Result<IUIAutomationProxyFactory>;
    fn ClassName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ImageName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AllowSubstringMatch(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanCheckBaseClass(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn NeedsAdviseEvents(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetClassName(&self, classname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetImageName(&self, imagename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetAllowSubstringMatch(&self, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetCanCheckBaseClass(&self, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetNeedsAdviseEvents(&self, adviseevents: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetWinEventsForAutomationEvent(&self, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn GetWinEventsForAutomationEvent(&self, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationProxyFactoryEntry {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationProxyFactoryEntry_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>() -> IUIAutomationProxyFactoryEntry_Vtbl {
        unsafe extern "system" fn ProxyFactory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyFactory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(classname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImageName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSubstringMatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowSubstringMatch() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allowsubstringmatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCheckBaseClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanCheckBaseClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cancheckbaseclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeedsAdviseEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NeedsAdviseEvents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(adviseevents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClassName(::core::mem::transmute(&classname)).into()
        }
        unsafe extern "system" fn SetImageName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImageName(::core::mem::transmute(&imagename)).into()
        }
        unsafe extern "system" fn SetAllowSubstringMatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowSubstringMatch(::core::mem::transmute_copy(&allowsubstringmatch)).into()
        }
        unsafe extern "system" fn SetCanCheckBaseClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCanCheckBaseClass(::core::mem::transmute_copy(&cancheckbaseclass)).into()
        }
        unsafe extern "system" fn SetNeedsAdviseEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNeedsAdviseEvents(::core::mem::transmute_copy(&adviseevents)).into()
        }
        unsafe extern "system" fn SetWinEventsForAutomationEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWinEventsForAutomationEvent(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&winevents)).into()
        }
        unsafe extern "system" fn GetWinEventsForAutomationEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWinEventsForAutomationEvent(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(winevents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProxyFactory: ProxyFactory::<Identity, Impl, OFFSET>,
            ClassName: ClassName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            AllowSubstringMatch: AllowSubstringMatch::<Identity, Impl, OFFSET>,
            CanCheckBaseClass: CanCheckBaseClass::<Identity, Impl, OFFSET>,
            NeedsAdviseEvents: NeedsAdviseEvents::<Identity, Impl, OFFSET>,
            SetClassName: SetClassName::<Identity, Impl, OFFSET>,
            SetImageName: SetImageName::<Identity, Impl, OFFSET>,
            SetAllowSubstringMatch: SetAllowSubstringMatch::<Identity, Impl, OFFSET>,
            SetCanCheckBaseClass: SetCanCheckBaseClass::<Identity, Impl, OFFSET>,
            SetNeedsAdviseEvents: SetNeedsAdviseEvents::<Identity, Impl, OFFSET>,
            SetWinEventsForAutomationEvent: SetWinEventsForAutomationEvent::<Identity, Impl, OFFSET>,
            GetWinEventsForAutomationEvent: GetWinEventsForAutomationEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactoryEntry as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationProxyFactoryMapping_Impl: Sized {
    fn Count(&self) -> ::windows_core::Result<u32>;
    fn GetTable(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEntry(&self, index: u32) -> ::windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn SetTable(&self, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn InsertEntries(&self, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn InsertEntry(&self, before: u32, factory: ::core::option::Option<&IUIAutomationProxyFactoryEntry>) -> ::windows_core::Result<()>;
    fn RemoveEntry(&self, index: u32) -> ::windows_core::Result<()>;
    fn ClearTable(&self) -> ::windows_core::Result<()>;
    fn RestoreDefaultTable(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationProxyFactoryMapping {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationProxyFactoryMapping_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>() -> IUIAutomationProxyFactoryMapping_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, table: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(table, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, entry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEntry(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTable(::core::mem::transmute_copy(&factorylist)).into()
        }
        unsafe extern "system" fn InsertEntries<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertEntries(::core::mem::transmute_copy(&before), ::core::mem::transmute_copy(&factorylist)).into()
        }
        unsafe extern "system" fn InsertEntry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factory: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertEntry(::core::mem::transmute_copy(&before), ::windows_core::from_raw_borrowed(&factory)).into()
        }
        unsafe extern "system" fn RemoveEntry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveEntry(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn ClearTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearTable().into()
        }
        unsafe extern "system" fn RestoreDefaultTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreDefaultTable().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetTable: GetTable::<Identity, Impl, OFFSET>,
            GetEntry: GetEntry::<Identity, Impl, OFFSET>,
            SetTable: SetTable::<Identity, Impl, OFFSET>,
            InsertEntries: InsertEntries::<Identity, Impl, OFFSET>,
            InsertEntry: InsertEntry::<Identity, Impl, OFFSET>,
            RemoveEntry: RemoveEntry::<Identity, Impl, OFFSET>,
            ClearTable: ClearTable::<Identity, Impl, OFFSET>,
            RestoreDefaultTable: RestoreDefaultTable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactoryMapping as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationRangeValuePattern_Impl: Sized {
    fn SetValue(&self, val: f64) -> ::windows_core::Result<()>;
    fn CurrentValue(&self) -> ::windows_core::Result<f64>;
    fn CurrentIsReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentMaximum(&self) -> ::windows_core::Result<f64>;
    fn CurrentMinimum(&self) -> ::windows_core::Result<f64>;
    fn CurrentLargeChange(&self) -> ::windows_core::Result<f64>;
    fn CurrentSmallChange(&self) -> ::windows_core::Result<f64>;
    fn CachedValue(&self) -> ::windows_core::Result<f64>;
    fn CachedIsReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedMaximum(&self) -> ::windows_core::Result<f64>;
    fn CachedMinimum(&self) -> ::windows_core::Result<f64>;
    fn CachedLargeChange(&self) -> ::windows_core::Result<f64>;
    fn CachedSmallChange(&self) -> ::windows_core::Result<f64>;
}
impl ::windows_core::RuntimeName for IUIAutomationRangeValuePattern {}
impl IUIAutomationRangeValuePattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>() -> IUIAutomationRangeValuePattern_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMaximum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMinimum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLargeChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentLargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSmallChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentSmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedMaximum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedMinimum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLargeChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedLargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSmallChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedSmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Identity, Impl, OFFSET>,
            CurrentMaximum: CurrentMaximum::<Identity, Impl, OFFSET>,
            CurrentMinimum: CurrentMinimum::<Identity, Impl, OFFSET>,
            CurrentLargeChange: CurrentLargeChange::<Identity, Impl, OFFSET>,
            CurrentSmallChange: CurrentSmallChange::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Identity, Impl, OFFSET>,
            CachedMaximum: CachedMaximum::<Identity, Impl, OFFSET>,
            CachedMinimum: CachedMinimum::<Identity, Impl, OFFSET>,
            CachedLargeChange: CachedLargeChange::<Identity, Impl, OFFSET>,
            CachedSmallChange: CachedSmallChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationRangeValuePattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationRegistrar_Impl: Sized {
    fn RegisterProperty(&self, property: *const UIAutomationPropertyInfo) -> ::windows_core::Result<i32>;
    fn RegisterEvent(&self, event: *const UIAutomationEventInfo) -> ::windows_core::Result<i32>;
    fn RegisterPattern(&self, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationRegistrar {}
impl IUIAutomationRegistrar_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>() -> IUIAutomationRegistrar_Vtbl {
        unsafe extern "system" fn RegisterProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterProperty(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const UIAutomationEventInfo, eventid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterEvent(::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPattern<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterPattern(::core::mem::transmute_copy(&pattern), ::core::mem::transmute_copy(&ppatternid), ::core::mem::transmute_copy(&ppatternavailablepropertyid), ::core::mem::transmute_copy(&propertyidcount), ::core::mem::transmute_copy(&ppropertyids), ::core::mem::transmute_copy(&eventidcount), ::core::mem::transmute_copy(&peventids)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterProperty: RegisterProperty::<Identity, Impl, OFFSET>,
            RegisterEvent: RegisterEvent::<Identity, Impl, OFFSET>,
            RegisterPattern: RegisterPattern::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationRegistrar as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationScrollItemPattern_Impl: Sized {
    fn ScrollIntoView(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationScrollItemPattern {}
impl IUIAutomationScrollItemPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationScrollItemPattern_Vtbl {
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScrollIntoView().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationScrollItemPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationScrollPattern_Impl: Sized {
    fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows_core::Result<()>;
    fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> ::windows_core::Result<()>;
    fn CurrentHorizontalScrollPercent(&self) -> ::windows_core::Result<f64>;
    fn CurrentVerticalScrollPercent(&self) -> ::windows_core::Result<f64>;
    fn CurrentHorizontalViewSize(&self) -> ::windows_core::Result<f64>;
    fn CurrentVerticalViewSize(&self) -> ::windows_core::Result<f64>;
    fn CurrentHorizontallyScrollable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentVerticallyScrollable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedHorizontalScrollPercent(&self) -> ::windows_core::Result<f64>;
    fn CachedVerticalScrollPercent(&self) -> ::windows_core::Result<f64>;
    fn CachedHorizontalViewSize(&self) -> ::windows_core::Result<f64>;
    fn CachedVerticalViewSize(&self) -> ::windows_core::Result<f64>;
    fn CachedHorizontallyScrollable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedVerticallyScrollable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IUIAutomationScrollPattern {}
impl IUIAutomationScrollPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>() -> IUIAutomationScrollPattern_Vtbl {
        unsafe extern "system" fn Scroll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Scroll(::core::mem::transmute_copy(&horizontalamount), ::core::mem::transmute_copy(&verticalamount)).into()
        }
        unsafe extern "system" fn SetScrollPercent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScrollPercent(::core::mem::transmute_copy(&horizontalpercent), ::core::mem::transmute_copy(&verticalpercent)).into()
        }
        unsafe extern "system" fn CurrentHorizontalScrollPercent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentHorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticalScrollPercent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentVerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontalViewSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentHorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticalViewSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentVerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontallyScrollable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentHorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticallyScrollable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentVerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontalScrollPercent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedHorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticalScrollPercent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedVerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontalViewSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedHorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticalViewSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedVerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontallyScrollable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedHorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticallyScrollable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedVerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            SetScrollPercent: SetScrollPercent::<Identity, Impl, OFFSET>,
            CurrentHorizontalScrollPercent: CurrentHorizontalScrollPercent::<Identity, Impl, OFFSET>,
            CurrentVerticalScrollPercent: CurrentVerticalScrollPercent::<Identity, Impl, OFFSET>,
            CurrentHorizontalViewSize: CurrentHorizontalViewSize::<Identity, Impl, OFFSET>,
            CurrentVerticalViewSize: CurrentVerticalViewSize::<Identity, Impl, OFFSET>,
            CurrentHorizontallyScrollable: CurrentHorizontallyScrollable::<Identity, Impl, OFFSET>,
            CurrentVerticallyScrollable: CurrentVerticallyScrollable::<Identity, Impl, OFFSET>,
            CachedHorizontalScrollPercent: CachedHorizontalScrollPercent::<Identity, Impl, OFFSET>,
            CachedVerticalScrollPercent: CachedVerticalScrollPercent::<Identity, Impl, OFFSET>,
            CachedHorizontalViewSize: CachedHorizontalViewSize::<Identity, Impl, OFFSET>,
            CachedVerticalViewSize: CachedVerticalViewSize::<Identity, Impl, OFFSET>,
            CachedHorizontallyScrollable: CachedHorizontallyScrollable::<Identity, Impl, OFFSET>,
            CachedVerticallyScrollable: CachedVerticallyScrollable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationScrollPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationSelectionItemPattern_Impl: Sized {
    fn Select(&self) -> ::windows_core::Result<()>;
    fn AddToSelection(&self) -> ::windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows_core::Result<()>;
    fn CurrentIsSelected(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentSelectionContainer(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedIsSelected(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedSelectionContainer(&self) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::RuntimeName for IUIAutomationSelectionItemPattern {}
impl IUIAutomationSelectionItemPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationSelectionItemPattern_Vtbl {
        unsafe extern "system" fn Select<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFromSelection().into()
        }
        unsafe extern "system" fn CurrentIsSelected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSelectionContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentSelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsSelected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSelectionContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedSelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            CurrentIsSelected: CurrentIsSelected::<Identity, Impl, OFFSET>,
            CurrentSelectionContainer: CurrentSelectionContainer::<Identity, Impl, OFFSET>,
            CachedIsSelected: CachedIsSelected::<Identity, Impl, OFFSET>,
            CachedSelectionContainer: CachedSelectionContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationSelectionItemPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationSelectionPattern_Impl: Sized {
    fn GetCurrentSelection(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentCanSelectMultiple(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsSelectionRequired(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCachedSelection(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedCanSelectMultiple(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsSelectionRequired(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IUIAutomationSelectionPattern {}
impl IUIAutomationSelectionPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>() -> IUIAutomationSelectionPattern_Vtbl {
        unsafe extern "system" fn GetCurrentSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanSelectMultiple<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsSelectionRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanSelectMultiple<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsSelectionRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentSelection: GetCurrentSelection::<Identity, Impl, OFFSET>,
            CurrentCanSelectMultiple: CurrentCanSelectMultiple::<Identity, Impl, OFFSET>,
            CurrentIsSelectionRequired: CurrentIsSelectionRequired::<Identity, Impl, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, Impl, OFFSET>,
            CachedCanSelectMultiple: CachedCanSelectMultiple::<Identity, Impl, OFFSET>,
            CachedIsSelectionRequired: CachedIsSelectionRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationSelectionPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationSelectionPattern2_Impl: Sized + IUIAutomationSelectionPattern_Impl {
    fn CurrentFirstSelectedItem(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentLastSelectedItem(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentCurrentSelectedItem(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentItemCount(&self) -> ::windows_core::Result<i32>;
    fn CachedFirstSelectedItem(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedLastSelectedItem(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedCurrentSelectedItem(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedItemCount(&self) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IUIAutomationSelectionPattern2 {}
impl IUIAutomationSelectionPattern2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>() -> IUIAutomationSelectionPattern2_Vtbl {
        unsafe extern "system" fn CurrentFirstSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLastSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentLastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCurrentSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFirstSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLastSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedLastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCurrentSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationSelectionPattern_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentFirstSelectedItem: CurrentFirstSelectedItem::<Identity, Impl, OFFSET>,
            CurrentLastSelectedItem: CurrentLastSelectedItem::<Identity, Impl, OFFSET>,
            CurrentCurrentSelectedItem: CurrentCurrentSelectedItem::<Identity, Impl, OFFSET>,
            CurrentItemCount: CurrentItemCount::<Identity, Impl, OFFSET>,
            CachedFirstSelectedItem: CachedFirstSelectedItem::<Identity, Impl, OFFSET>,
            CachedLastSelectedItem: CachedLastSelectedItem::<Identity, Impl, OFFSET>,
            CachedCurrentSelectedItem: CachedCurrentSelectedItem::<Identity, Impl, OFFSET>,
            CachedItemCount: CachedItemCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationSelectionPattern2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationSelectionPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationSpreadsheetItemPattern_Impl: Sized {
    fn CurrentFormula(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCurrentAnnotationObjects(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentAnnotationTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedFormula(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCachedAnnotationObjects(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedAnnotationTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationSpreadsheetItemPattern {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationSpreadsheetItemPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationSpreadsheetItemPattern_Vtbl {
        unsafe extern "system" fn CurrentFormula<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFormula() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFormula<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFormula() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedAnnotationObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedAnnotationTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CurrentFormula: CurrentFormula::<Identity, Impl, OFFSET>,
            GetCurrentAnnotationObjects: GetCurrentAnnotationObjects::<Identity, Impl, OFFSET>,
            GetCurrentAnnotationTypes: GetCurrentAnnotationTypes::<Identity, Impl, OFFSET>,
            CachedFormula: CachedFormula::<Identity, Impl, OFFSET>,
            GetCachedAnnotationObjects: GetCachedAnnotationObjects::<Identity, Impl, OFFSET>,
            GetCachedAnnotationTypes: GetCachedAnnotationTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetItemPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationSpreadsheetPattern_Impl: Sized {
    fn GetItemByName(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::RuntimeName for IUIAutomationSpreadsheetPattern {}
impl IUIAutomationSpreadsheetPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: isize>() -> IUIAutomationSpreadsheetPattern_Vtbl {
        unsafe extern "system" fn GetItemByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetItemByName: GetItemByName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationStructureChangedEventHandler_Impl: Sized {
    fn HandleStructureChangedEvent(&self, sender: ::core::option::Option<&IUIAutomationElement>, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationStructureChangedEventHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationStructureChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationStructureChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleStructureChangedEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleStructureChangedEvent(::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&runtimeid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleStructureChangedEvent: HandleStructureChangedEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationStructureChangedEventHandler as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationStylesPattern_Impl: Sized {
    fn CurrentStyleId(&self) -> ::windows_core::Result<UIA_STYLE_ID>;
    fn CurrentStyleName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentFillColor(&self) -> ::windows_core::Result<i32>;
    fn CurrentFillPatternStyle(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentShape(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentFillPatternColor(&self) -> ::windows_core::Result<i32>;
    fn CurrentExtendedProperties(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCurrentExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows_core::Result<()>;
    fn CachedStyleId(&self) -> ::windows_core::Result<UIA_STYLE_ID>;
    fn CachedStyleName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedFillColor(&self) -> ::windows_core::Result<i32>;
    fn CachedFillPatternStyle(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedShape(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedFillPatternColor(&self) -> ::windows_core::Result<i32>;
    fn CachedExtendedProperties(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCachedExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationStylesPattern {}
impl IUIAutomationStylesPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>() -> IUIAutomationStylesPattern_Vtbl {
        unsafe extern "system" fn CurrentStyleId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentStyleId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStyleName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentStyleName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFillColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillPatternStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentShape<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentShape() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillPatternColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentExtendedProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentExtendedPropertiesAsArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentExtendedPropertiesAsArray(::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn CachedStyleId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedStyleId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedStyleName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedStyleName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFillColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillPatternStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedShape<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedShape() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillPatternColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedFillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedExtendedProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedExtendedPropertiesAsArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCachedExtendedPropertiesAsArray(::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CurrentStyleId: CurrentStyleId::<Identity, Impl, OFFSET>,
            CurrentStyleName: CurrentStyleName::<Identity, Impl, OFFSET>,
            CurrentFillColor: CurrentFillColor::<Identity, Impl, OFFSET>,
            CurrentFillPatternStyle: CurrentFillPatternStyle::<Identity, Impl, OFFSET>,
            CurrentShape: CurrentShape::<Identity, Impl, OFFSET>,
            CurrentFillPatternColor: CurrentFillPatternColor::<Identity, Impl, OFFSET>,
            CurrentExtendedProperties: CurrentExtendedProperties::<Identity, Impl, OFFSET>,
            GetCurrentExtendedPropertiesAsArray: GetCurrentExtendedPropertiesAsArray::<Identity, Impl, OFFSET>,
            CachedStyleId: CachedStyleId::<Identity, Impl, OFFSET>,
            CachedStyleName: CachedStyleName::<Identity, Impl, OFFSET>,
            CachedFillColor: CachedFillColor::<Identity, Impl, OFFSET>,
            CachedFillPatternStyle: CachedFillPatternStyle::<Identity, Impl, OFFSET>,
            CachedShape: CachedShape::<Identity, Impl, OFFSET>,
            CachedFillPatternColor: CachedFillPatternColor::<Identity, Impl, OFFSET>,
            CachedExtendedProperties: CachedExtendedProperties::<Identity, Impl, OFFSET>,
            GetCachedExtendedPropertiesAsArray: GetCachedExtendedPropertiesAsArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationStylesPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationSynchronizedInputPattern_Impl: Sized {
    fn StartListening(&self, inputtype: SynchronizedInputType) -> ::windows_core::Result<()>;
    fn Cancel(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationSynchronizedInputPattern {}
impl IUIAutomationSynchronizedInputPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>() -> IUIAutomationSynchronizedInputPattern_Vtbl {
        unsafe extern "system" fn StartListening<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartListening(::core::mem::transmute_copy(&inputtype)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartListening: StartListening::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationSynchronizedInputPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTableItemPattern_Impl: Sized {
    fn GetCurrentRowHeaderItems(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaderItems(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedRowHeaderItems(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaderItems(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
}
impl ::windows_core::RuntimeName for IUIAutomationTableItemPattern {}
impl IUIAutomationTableItemPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationTableItemPattern_Vtbl {
        unsafe extern "system" fn GetCurrentRowHeaderItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaderItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedRowHeaderItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaderItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentRowHeaderItems: GetCurrentRowHeaderItems::<Identity, Impl, OFFSET>,
            GetCurrentColumnHeaderItems: GetCurrentColumnHeaderItems::<Identity, Impl, OFFSET>,
            GetCachedRowHeaderItems: GetCachedRowHeaderItems::<Identity, Impl, OFFSET>,
            GetCachedColumnHeaderItems: GetCachedColumnHeaderItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTableItemPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTablePattern_Impl: Sized {
    fn GetCurrentRowHeaders(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaders(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentRowOrColumnMajor(&self) -> ::windows_core::Result<RowOrColumnMajor>;
    fn GetCachedRowHeaders(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaders(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedRowOrColumnMajor(&self) -> ::windows_core::Result<RowOrColumnMajor>;
}
impl ::windows_core::RuntimeName for IUIAutomationTablePattern {}
impl IUIAutomationTablePattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>() -> IUIAutomationTablePattern_Vtbl {
        unsafe extern "system" fn GetCurrentRowHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowOrColumnMajor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentRowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedRowHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCachedColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowOrColumnMajor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedRowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentRowHeaders: GetCurrentRowHeaders::<Identity, Impl, OFFSET>,
            GetCurrentColumnHeaders: GetCurrentColumnHeaders::<Identity, Impl, OFFSET>,
            CurrentRowOrColumnMajor: CurrentRowOrColumnMajor::<Identity, Impl, OFFSET>,
            GetCachedRowHeaders: GetCachedRowHeaders::<Identity, Impl, OFFSET>,
            GetCachedColumnHeaders: GetCachedColumnHeaders::<Identity, Impl, OFFSET>,
            CachedRowOrColumnMajor: CachedRowOrColumnMajor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTablePattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTextChildPattern_Impl: Sized {
    fn TextContainer(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn TextRange(&self) -> ::windows_core::Result<IUIAutomationTextRange>;
}
impl ::windows_core::RuntimeName for IUIAutomationTextChildPattern {}
impl IUIAutomationTextChildPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>() -> IUIAutomationTextChildPattern_Vtbl {
        unsafe extern "system" fn TextContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TextContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(container, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TextRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TextContainer: TextContainer::<Identity, Impl, OFFSET>,
            TextRange: TextRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextChildPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTextEditPattern_Impl: Sized + IUIAutomationTextPattern_Impl {
    fn GetActiveComposition(&self) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn GetConversionTarget(&self) -> ::windows_core::Result<IUIAutomationTextRange>;
}
impl ::windows_core::RuntimeName for IUIAutomationTextEditPattern {}
impl IUIAutomationTextEditPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>() -> IUIAutomationTextEditPattern_Vtbl {
        unsafe extern "system" fn GetActiveComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveComposition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConversionTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationTextPattern_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Identity, Impl, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextEditPattern as ::windows_core::Interface>::IID || iid == &<IUIAutomationTextPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationTextEditTextChangedEventHandler_Impl: Sized {
    fn HandleTextEditTextChangedEvent(&self, sender: ::core::option::Option<&IUIAutomationElement>, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUIAutomationTextEditTextChangedEventHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationTextEditTextChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationTextEditTextChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleTextEditTextChangedEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleTextEditTextChangedEvent(::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&texteditchangetype), ::core::mem::transmute_copy(&eventstrings)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleTextEditTextChangedEvent: HandleTextEditTextChangedEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextEditTextChangedEventHandler as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTextPattern_Impl: Sized {
    fn RangeFromPoint(&self, pt: &super::super::Foundation::POINT) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn RangeFromChild(&self, child: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn GetSelection(&self) -> ::windows_core::Result<IUIAutomationTextRangeArray>;
    fn GetVisibleRanges(&self) -> ::windows_core::Result<IUIAutomationTextRangeArray>;
    fn DocumentRange(&self) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn SupportedTextSelection(&self) -> ::windows_core::Result<SupportedTextSelection>;
}
impl ::windows_core::RuntimeName for IUIAutomationTextPattern {}
impl IUIAutomationTextPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>() -> IUIAutomationTextPattern_Vtbl {
        unsafe extern "system" fn RangeFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RangeFromPoint(::core::mem::transmute(&pt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RangeFromChild(::windows_core::from_raw_borrowed(&child)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ranges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisibleRanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ranges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DocumentRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedtextselection: *mut SupportedTextSelection) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedTextSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedtextselection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, Impl, OFFSET>,
            DocumentRange: DocumentRange::<Identity, Impl, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTextPattern2_Impl: Sized + IUIAutomationTextPattern_Impl {
    fn RangeFromAnnotation(&self, annotation: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn GetCaretRange(&self, isactive: *mut super::super::Foundation::BOOL, range: *mut ::core::option::Option<IUIAutomationTextRange>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationTextPattern2 {}
impl IUIAutomationTextPattern2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: isize>() -> IUIAutomationTextPattern2_Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RangeFromAnnotation(::windows_core::from_raw_borrowed(&annotation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaretRange(::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&range)).into()
        }
        Self {
            base__: IUIAutomationTextPattern_Vtbl::new::<Identity, Impl, OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Identity, Impl, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationTextPattern as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationTextRange_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn Compare(&self, range: ::core::option::Option<&IUIAutomationTextRange>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareEndpoints(&self, srcendpoint: TextPatternRangeEndpoint, range: ::core::option::Option<&IUIAutomationTextRange>, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::Result<i32>;
    fn ExpandToEnclosingUnit(&self, textunit: TextUnit) -> ::windows_core::Result<()>;
    fn FindAttribute(&self, attr: UIA_TEXTATTRIBUTE_ID, val: &super::super::System::Variant::VARIANT, backward: super::super::Foundation::BOOL) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn FindText(&self, text: &::windows_core::BSTR, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn GetAttributeValue(&self, attr: UIA_TEXTATTRIBUTE_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetBoundingRectangles(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEnclosingElement(&self) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetText(&self, maxlength: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Move(&self, unit: TextUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEndpointByRange(&self, srcendpoint: TextPatternRangeEndpoint, range: ::core::option::Option<&IUIAutomationTextRange>, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::Result<()>;
    fn Select(&self) -> ::windows_core::Result<()>;
    fn AddToSelection(&self) -> ::windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows_core::Result<()>;
    fn ScrollIntoView(&self, aligntotop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetChildren(&self) -> ::windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationTextRange {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationTextRange_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>() -> IUIAutomationTextRange_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clonedrange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, aresame: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Compare(::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(aresame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint, compvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareEndpoints(::core::mem::transmute_copy(&srcendpoint), ::windows_core::from_raw_borrowed(&range), ::core::mem::transmute_copy(&targetendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textunit: TextUnit) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExpandToEnclosingUnit(::core::mem::transmute_copy(&textunit)).into()
        }
        unsafe extern "system" fn FindAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Variant::VARIANT, backward: super::super::Foundation::BOOL, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindAttribute(::core::mem::transmute_copy(&attr), ::core::mem::transmute(&val), ::core::mem::transmute_copy(&backward)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindText(::core::mem::transmute(&text), ::core::mem::transmute_copy(&backward), ::core::mem::transmute_copy(&ignorecase)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: UIA_TEXTATTRIBUTE_ID, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeValue(::core::mem::transmute_copy(&attr)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrects: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoundingRectangles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(boundingrects, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosingelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnclosingElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enclosingelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetText(::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Move(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveEndpointByUnit(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveEndpointByRange(::core::mem::transmute_copy(&srcendpoint), ::windows_core::from_raw_borrowed(&range), ::core::mem::transmute_copy(&targetendpoint)).into()
        }
        unsafe extern "system" fn Select<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFromSelection().into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScrollIntoView(::core::mem::transmute_copy(&aligntotop)).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            CompareEndpoints: CompareEndpoints::<Identity, Impl, OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Identity, Impl, OFFSET>,
            FindAttribute: FindAttribute::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, Impl, OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Identity, Impl, OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Identity, Impl, OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextRange as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationTextRange2_Impl: Sized + IUIAutomationTextRange_Impl {
    fn ShowContextMenu(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationTextRange2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationTextRange2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange2_Impl, const OFFSET: isize>() -> IUIAutomationTextRange2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowContextMenu().into()
        }
        Self { base__: IUIAutomationTextRange_Vtbl::new::<Identity, Impl, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextRange2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationTextRange as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationTextRange3_Impl: Sized + IUIAutomationTextRange2_Impl {
    fn GetEnclosingElementBuildCache(&self, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetChildrenBuildCache(&self, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetAttributeValues(&self, attributeids: *const UIA_TEXTATTRIBUTE_ID, attributeidcount: i32) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUIAutomationTextRange3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUIAutomationTextRange3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>() -> IUIAutomationTextRange3_Vtbl {
        unsafe extern "system" fn GetEnclosingElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, enclosingelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnclosingElementBuildCache(::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enclosingelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildrenBuildCache(::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValues<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeids: *const UIA_TEXTATTRIBUTE_ID, attributeidcount: i32, attributevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeValues(::core::mem::transmute_copy(&attributeids), ::core::mem::transmute_copy(&attributeidcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributevalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationTextRange2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetEnclosingElementBuildCache: GetEnclosingElementBuildCache::<Identity, Impl, OFFSET>,
            GetChildrenBuildCache: GetChildrenBuildCache::<Identity, Impl, OFFSET>,
            GetAttributeValues: GetAttributeValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextRange3 as ::windows_core::Interface>::IID || iid == &<IUIAutomationTextRange as ::windows_core::Interface>::IID || iid == &<IUIAutomationTextRange2 as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTextRangeArray_Impl: Sized {
    fn Length(&self) -> ::windows_core::Result<i32>;
    fn GetElement(&self, index: i32) -> ::windows_core::Result<IUIAutomationTextRange>;
}
impl ::windows_core::RuntimeName for IUIAutomationTextRangeArray {}
impl IUIAutomationTextRangeArray_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>() -> IUIAutomationTextRangeArray_Vtbl {
        unsafe extern "system" fn Length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetElement(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Length: Length::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextRangeArray as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTogglePattern_Impl: Sized {
    fn Toggle(&self) -> ::windows_core::Result<()>;
    fn CurrentToggleState(&self) -> ::windows_core::Result<ToggleState>;
    fn CachedToggleState(&self) -> ::windows_core::Result<ToggleState>;
}
impl ::windows_core::RuntimeName for IUIAutomationTogglePattern {}
impl IUIAutomationTogglePattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>() -> IUIAutomationTogglePattern_Vtbl {
        unsafe extern "system" fn Toggle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Toggle().into()
        }
        unsafe extern "system" fn CurrentToggleState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedToggleState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Toggle: Toggle::<Identity, Impl, OFFSET>,
            CurrentToggleState: CurrentToggleState::<Identity, Impl, OFFSET>,
            CachedToggleState: CachedToggleState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTogglePattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTransformPattern_Impl: Sized {
    fn Move(&self, x: f64, y: f64) -> ::windows_core::Result<()>;
    fn Resize(&self, width: f64, height: f64) -> ::windows_core::Result<()>;
    fn Rotate(&self, degrees: f64) -> ::windows_core::Result<()>;
    fn CurrentCanMove(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanResize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanRotate(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanMove(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanResize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanRotate(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IUIAutomationTransformPattern {}
impl IUIAutomationTransformPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>() -> IUIAutomationTransformPattern_Vtbl {
        unsafe extern "system" fn Move<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rotate(::core::mem::transmute_copy(&degrees)).into()
        }
        unsafe extern "system" fn CurrentCanMove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCanMove() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanResize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCanResize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanRotate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCanMove() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanResize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCanResize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanRotate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Move: Move::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            CurrentCanMove: CurrentCanMove::<Identity, Impl, OFFSET>,
            CurrentCanResize: CurrentCanResize::<Identity, Impl, OFFSET>,
            CurrentCanRotate: CurrentCanRotate::<Identity, Impl, OFFSET>,
            CachedCanMove: CachedCanMove::<Identity, Impl, OFFSET>,
            CachedCanResize: CachedCanResize::<Identity, Impl, OFFSET>,
            CachedCanRotate: CachedCanRotate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTransformPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTransformPattern2_Impl: Sized + IUIAutomationTransformPattern_Impl {
    fn Zoom(&self, zoomvalue: f64) -> ::windows_core::Result<()>;
    fn ZoomByUnit(&self, zoomunit: ZoomUnit) -> ::windows_core::Result<()>;
    fn CurrentCanZoom(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanZoom(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentZoomLevel(&self) -> ::windows_core::Result<f64>;
    fn CachedZoomLevel(&self) -> ::windows_core::Result<f64>;
    fn CurrentZoomMinimum(&self) -> ::windows_core::Result<f64>;
    fn CachedZoomMinimum(&self) -> ::windows_core::Result<f64>;
    fn CurrentZoomMaximum(&self) -> ::windows_core::Result<f64>;
    fn CachedZoomMaximum(&self) -> ::windows_core::Result<f64>;
}
impl ::windows_core::RuntimeName for IUIAutomationTransformPattern2 {}
impl IUIAutomationTransformPattern2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>() -> IUIAutomationTransformPattern2_Vtbl {
        unsafe extern "system" fn Zoom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomvalue: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Zoom(::core::mem::transmute_copy(&zoomvalue)).into()
        }
        unsafe extern "system" fn ZoomByUnit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ZoomByUnit(::core::mem::transmute_copy(&zoomunit)).into()
        }
        unsafe extern "system" fn CurrentCanZoom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanZoom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomMinimum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomMinimum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomMaximum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomMaximum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUIAutomationTransformPattern_Vtbl::new::<Identity, Impl, OFFSET>(),
            Zoom: Zoom::<Identity, Impl, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, Impl, OFFSET>,
            CurrentCanZoom: CurrentCanZoom::<Identity, Impl, OFFSET>,
            CachedCanZoom: CachedCanZoom::<Identity, Impl, OFFSET>,
            CurrentZoomLevel: CurrentZoomLevel::<Identity, Impl, OFFSET>,
            CachedZoomLevel: CachedZoomLevel::<Identity, Impl, OFFSET>,
            CurrentZoomMinimum: CurrentZoomMinimum::<Identity, Impl, OFFSET>,
            CachedZoomMinimum: CachedZoomMinimum::<Identity, Impl, OFFSET>,
            CurrentZoomMaximum: CurrentZoomMaximum::<Identity, Impl, OFFSET>,
            CachedZoomMaximum: CachedZoomMaximum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTransformPattern2 as ::windows_core::Interface>::IID || iid == &<IUIAutomationTransformPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationTreeWalker_Impl: Sized {
    fn GetParentElement(&self, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetFirstChildElement(&self, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetLastChildElement(&self, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetNextSiblingElement(&self, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElement(&self, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn NormalizeElement(&self, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetParentElementBuildCache(&self, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetFirstChildElementBuildCache(&self, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetLastChildElementBuildCache(&self, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetNextSiblingElementBuildCache(&self, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElementBuildCache(&self, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn NormalizeElementBuildCache(&self, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn Condition(&self) -> ::windows_core::Result<IUIAutomationCondition>;
}
impl ::windows_core::RuntimeName for IUIAutomationTreeWalker {}
impl IUIAutomationTreeWalker_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>() -> IUIAutomationTreeWalker_Vtbl {
        unsafe extern "system" fn GetParentElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentElement(::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFirstChildElement(::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(first, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChildElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, last: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastChildElement(::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(last, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextSiblingElement(::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(next, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, previous: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreviousSiblingElement(::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previous, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizeElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, normalized: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NormalizeElement(::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(normalized, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentElementBuildCache(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFirstChildElementBuildCache(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(first, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChildElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, last: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastChildElementBuildCache(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(last, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextSiblingElementBuildCache(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(next, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, previous: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreviousSiblingElementBuildCache(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previous, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizeElementBuildCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, normalized: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NormalizeElementBuildCache(::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(normalized, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Condition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Condition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParentElement: GetParentElement::<Identity, Impl, OFFSET>,
            GetFirstChildElement: GetFirstChildElement::<Identity, Impl, OFFSET>,
            GetLastChildElement: GetLastChildElement::<Identity, Impl, OFFSET>,
            GetNextSiblingElement: GetNextSiblingElement::<Identity, Impl, OFFSET>,
            GetPreviousSiblingElement: GetPreviousSiblingElement::<Identity, Impl, OFFSET>,
            NormalizeElement: NormalizeElement::<Identity, Impl, OFFSET>,
            GetParentElementBuildCache: GetParentElementBuildCache::<Identity, Impl, OFFSET>,
            GetFirstChildElementBuildCache: GetFirstChildElementBuildCache::<Identity, Impl, OFFSET>,
            GetLastChildElementBuildCache: GetLastChildElementBuildCache::<Identity, Impl, OFFSET>,
            GetNextSiblingElementBuildCache: GetNextSiblingElementBuildCache::<Identity, Impl, OFFSET>,
            GetPreviousSiblingElementBuildCache: GetPreviousSiblingElementBuildCache::<Identity, Impl, OFFSET>,
            NormalizeElementBuildCache: NormalizeElementBuildCache::<Identity, Impl, OFFSET>,
            Condition: Condition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationTreeWalker as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationValuePattern_Impl: Sized {
    fn SetValue(&self, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CurrentValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentIsReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedIsReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IUIAutomationValuePattern {}
impl IUIAutomationValuePattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>() -> IUIAutomationValuePattern_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationValuePattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationVirtualizedItemPattern_Impl: Sized {
    fn Realize(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUIAutomationVirtualizedItemPattern {}
impl IUIAutomationVirtualizedItemPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationVirtualizedItemPattern_Vtbl {
        unsafe extern "system" fn Realize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Realize().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Realize: Realize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationVirtualizedItemPattern as ::windows_core::Interface>::IID
    }
}
pub trait IUIAutomationWindowPattern_Impl: Sized {
    fn Close(&self) -> ::windows_core::Result<()>;
    fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetWindowVisualState(&self, state: WindowVisualState) -> ::windows_core::Result<()>;
    fn CurrentCanMaximize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanMinimize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsModal(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsTopmost(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentWindowVisualState(&self) -> ::windows_core::Result<WindowVisualState>;
    fn CurrentWindowInteractionState(&self) -> ::windows_core::Result<WindowInteractionState>;
    fn CachedCanMaximize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanMinimize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsModal(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsTopmost(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedWindowVisualState(&self) -> ::windows_core::Result<WindowVisualState>;
    fn CachedWindowInteractionState(&self) -> ::windows_core::Result<WindowInteractionState>;
}
impl ::windows_core::RuntimeName for IUIAutomationWindowPattern {}
impl IUIAutomationWindowPattern_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>() -> IUIAutomationWindowPattern_Vtbl {
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, success: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitForInputIdle(::core::mem::transmute_copy(&milliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(success, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowVisualState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowVisualState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CurrentCanMaximize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanMinimize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsModal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsModal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsTopmost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentIsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWindowVisualState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentWindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWindowInteractionState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentWindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMaximize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMinimize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedCanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsModal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsModal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsTopmost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedIsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedWindowVisualState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedWindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedWindowInteractionState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CachedWindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, Impl, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, Impl, OFFSET>,
            SetWindowVisualState: SetWindowVisualState::<Identity, Impl, OFFSET>,
            CurrentCanMaximize: CurrentCanMaximize::<Identity, Impl, OFFSET>,
            CurrentCanMinimize: CurrentCanMinimize::<Identity, Impl, OFFSET>,
            CurrentIsModal: CurrentIsModal::<Identity, Impl, OFFSET>,
            CurrentIsTopmost: CurrentIsTopmost::<Identity, Impl, OFFSET>,
            CurrentWindowVisualState: CurrentWindowVisualState::<Identity, Impl, OFFSET>,
            CurrentWindowInteractionState: CurrentWindowInteractionState::<Identity, Impl, OFFSET>,
            CachedCanMaximize: CachedCanMaximize::<Identity, Impl, OFFSET>,
            CachedCanMinimize: CachedCanMinimize::<Identity, Impl, OFFSET>,
            CachedIsModal: CachedIsModal::<Identity, Impl, OFFSET>,
            CachedIsTopmost: CachedIsTopmost::<Identity, Impl, OFFSET>,
            CachedWindowVisualState: CachedWindowVisualState::<Identity, Impl, OFFSET>,
            CachedWindowInteractionState: CachedWindowInteractionState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIAutomationWindowPattern as ::windows_core::Interface>::IID
    }
}
pub trait IValueProvider_Impl: Sized {
    fn SetValue(&self, val: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Value(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IValueProvider {}
impl IValueProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IValueProvider_Impl, const OFFSET: isize>() -> IValueProvider_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IValueProvider as ::windows_core::Interface>::IID
    }
}
pub trait IVirtualizedItemProvider_Impl: Sized {
    fn Realize(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVirtualizedItemProvider {}
impl IVirtualizedItemProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualizedItemProvider_Impl, const OFFSET: isize>() -> IVirtualizedItemProvider_Vtbl {
        unsafe extern "system" fn Realize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualizedItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Realize().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Realize: Realize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVirtualizedItemProvider as ::windows_core::Interface>::IID
    }
}
pub trait IWindowProvider_Impl: Sized {
    fn SetVisualState(&self, state: WindowVisualState) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanMaximize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanMinimize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsModal(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn WindowVisualState(&self) -> ::windows_core::Result<WindowVisualState>;
    fn WindowInteractionState(&self) -> ::windows_core::Result<WindowInteractionState>;
    fn IsTopmost(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IWindowProvider {}
impl IWindowProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>() -> IWindowProvider_Vtbl {
        unsafe extern "system" fn SetVisualState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVisualState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitForInputIdle(::core::mem::transmute_copy(&milliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMaximize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMinimize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsModal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsModal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowVisualState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowVisualState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowInteractionState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowInteractionState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTopmost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetVisualState: SetVisualState::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, Impl, OFFSET>,
            CanMaximize: CanMaximize::<Identity, Impl, OFFSET>,
            CanMinimize: CanMinimize::<Identity, Impl, OFFSET>,
            IsModal: IsModal::<Identity, Impl, OFFSET>,
            WindowVisualState: WindowVisualState::<Identity, Impl, OFFSET>,
            WindowInteractionState: WindowInteractionState::<Identity, Impl, OFFSET>,
            IsTopmost: IsTopmost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowProvider as ::windows_core::Interface>::IID
    }
}
