pub trait IActiveXUIHandlerSite_Impl: Sized {
    fn CreateScrollableContextMenu(&self) -> ::windows_core::Result<IScrollableContextMenu>;
    fn PickFileAndGetResult(&self, filepicker: ::core::option::Option<&::windows_core::IUnknown>, allowmultipleselections: super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for IActiveXUIHandlerSite {}
impl IActiveXUIHandlerSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveXUIHandlerSite_Impl, const OFFSET: isize>() -> IActiveXUIHandlerSite_Vtbl {
        unsafe extern "system" fn CreateScrollableContextMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveXUIHandlerSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollablecontextmenu: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateScrollableContextMenu() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scrollablecontextmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickFileAndGetResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveXUIHandlerSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepicker: *mut ::core::ffi::c_void, allowmultipleselections: super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PickFileAndGetResult(::windows_core::from_raw_borrowed(&filepicker), ::core::mem::transmute_copy(&allowmultipleselections)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateScrollableContextMenu: CreateScrollableContextMenu::<Identity, Impl, OFFSET>,
            PickFileAndGetResult: PickFileAndGetResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite as ::windows_core::Interface>::IID
    }
}
pub trait IActiveXUIHandlerSite2_Impl: Sized {
    fn AddSuspensionExemption(&self) -> ::windows_core::Result<u64>;
    fn RemoveSuspensionExemption(&self, ullcookie: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveXUIHandlerSite2 {}
impl IActiveXUIHandlerSite2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveXUIHandlerSite2_Impl, const OFFSET: isize>() -> IActiveXUIHandlerSite2_Vtbl {
        unsafe extern "system" fn AddSuspensionExemption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveXUIHandlerSite2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullcookie: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddSuspensionExemption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuspensionExemption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveXUIHandlerSite2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullcookie: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSuspensionExemption(::core::mem::transmute_copy(&ullcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddSuspensionExemption: AddSuspensionExemption::<Identity, Impl, OFFSET>,
            RemoveSuspensionExemption: RemoveSuspensionExemption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite2 as ::windows_core::Interface>::IID
    }
}
pub trait IActiveXUIHandlerSite3_Impl: Sized {
    fn MessageBoxW(&self, hwnd: super::super::Foundation::HWND, text: &::windows_core::PCWSTR, caption: &::windows_core::PCWSTR, r#type: u32) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IActiveXUIHandlerSite3 {}
impl IActiveXUIHandlerSite3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveXUIHandlerSite3_Impl, const OFFSET: isize>() -> IActiveXUIHandlerSite3_Vtbl {
        unsafe extern "system" fn MessageBoxW<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveXUIHandlerSite3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, text: ::windows_core::PCWSTR, caption: ::windows_core::PCWSTR, r#type: u32, result: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageBoxW(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&text), ::core::mem::transmute(&caption), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MessageBoxW: MessageBoxW::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite3 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAnchorClick_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ProcOnClick(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAnchorClick {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAnchorClick_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchorClick_Impl, const OFFSET: isize>() -> IAnchorClick_Vtbl {
        unsafe extern "system" fn ProcOnClick<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchorClick_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcOnClick().into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ProcOnClick: ProcOnClick::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAnchorClick as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IAudioSessionSite_Impl: Sized {
    fn GetAudioSessionGuid(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn OnAudioStreamCreated(&self, endpointid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnAudioStreamDestroyed(&self, endpointid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IAudioSessionSite {}
impl IAudioSessionSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionSite_Impl, const OFFSET: isize>() -> IAudioSessionSite_Vtbl {
        unsafe extern "system" fn GetAudioSessionGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosessionguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAudioSessionGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiosessionguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAudioStreamCreated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAudioStreamCreated(::core::mem::transmute(&endpointid)).into()
        }
        unsafe extern "system" fn OnAudioStreamDestroyed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAudioStreamDestroyed(::core::mem::transmute(&endpointid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAudioSessionGuid: GetAudioSessionGuid::<Identity, Impl, OFFSET>,
            OnAudioStreamCreated: OnAudioStreamCreated::<Identity, Impl, OFFSET>,
            OnAudioStreamDestroyed: OnAudioStreamDestroyed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAudioSessionSite as ::windows_core::Interface>::IID
    }
}
pub trait ICaretPositionProvider_Impl: Sized {
    fn GetCaretPosition(&self, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICaretPositionProvider {}
impl ICaretPositionProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICaretPositionProvider_Impl, const OFFSET: isize>() -> ICaretPositionProvider_Vtbl {
        unsafe extern "system" fn GetCaretPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICaretPositionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaretPosition(::core::mem::transmute_copy(&pptcaret), ::core::mem::transmute_copy(&pflheight)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCaretPosition: GetCaretPosition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICaretPositionProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDeviceRect_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDeviceRect {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDeviceRect_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceRect_Impl, const OFFSET: isize>() -> IDeviceRect_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDeviceRect as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDithererImpl_Impl: Sized {
    fn SetDestColorTable(&self, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> ::windows_core::Result<()>;
    fn SetEventSink(&self, peventsink: ::core::option::Option<&IImageDecodeEventSink>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDithererImpl {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDithererImpl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDithererImpl_Impl, const OFFSET: isize>() -> IDithererImpl_Vtbl {
        unsafe extern "system" fn SetDestColorTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDithererImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestColorTable(::core::mem::transmute_copy(&ncolors), ::core::mem::transmute_copy(&prgbcolors)).into()
        }
        unsafe extern "system" fn SetEventSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDithererImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventSink(::windows_core::from_raw_borrowed(&peventsink)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDestColorTable: SetDestColorTable::<Identity, Impl, OFFSET>,
            SetEventSink: SetEventSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDithererImpl as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IDocObjectService_Impl: Sized {
    fn FireBeforeNavigate2(&self, pdispatch: ::core::option::Option<&super::super::System::Com::IDispatch>, lpszurl: &::windows_core::PCWSTR, dwflags: u32, lpszframename: &::windows_core::PCWSTR, ppostdata: *const u8, cbpostdata: u32, lpszheaders: &::windows_core::PCWSTR, fplaynavsound: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn FireNavigateComplete2(&self, phtmlwindow2: ::core::option::Option<&super::MsHtml::IHTMLWindow2>, dwflags: u32) -> ::windows_core::Result<()>;
    fn FireDownloadBegin(&self) -> ::windows_core::Result<()>;
    fn FireDownloadComplete(&self) -> ::windows_core::Result<()>;
    fn FireDocumentComplete(&self, phtmlwindow: ::core::option::Option<&super::MsHtml::IHTMLWindow2>, dwflags: u32) -> ::windows_core::Result<()>;
    fn UpdateDesktopComponent(&self, phtmlwindow: ::core::option::Option<&super::MsHtml::IHTMLWindow2>) -> ::windows_core::Result<()>;
    fn GetPendingUrl(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ActiveElementChanged(&self, phtmlelement: ::core::option::Option<&super::MsHtml::IHTMLElement>) -> ::windows_core::Result<()>;
    fn GetUrlSearchComponent(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsErrorUrl(&self, lpszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl ::windows_core::RuntimeName for IDocObjectService {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IDocObjectService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>() -> IDocObjectService_Vtbl {
        unsafe extern "system" fn FireBeforeNavigate2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispatch: *mut ::core::ffi::c_void, lpszurl: ::windows_core::PCWSTR, dwflags: u32, lpszframename: ::windows_core::PCWSTR, ppostdata: *const u8, cbpostdata: u32, lpszheaders: ::windows_core::PCWSTR, fplaynavsound: super::super::Foundation::BOOL, pfcancel: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FireBeforeNavigate2(::windows_core::from_raw_borrowed(&pdispatch), ::core::mem::transmute(&lpszurl), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&lpszframename), ::core::mem::transmute_copy(&ppostdata), ::core::mem::transmute_copy(&cbpostdata), ::core::mem::transmute(&lpszheaders), ::core::mem::transmute_copy(&fplaynavsound)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireNavigateComplete2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtmlwindow2: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireNavigateComplete2(::windows_core::from_raw_borrowed(&phtmlwindow2), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FireDownloadBegin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireDownloadBegin().into()
        }
        unsafe extern "system" fn FireDownloadComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireDownloadComplete().into()
        }
        unsafe extern "system" fn FireDocumentComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtmlwindow: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireDocumentComplete(::windows_core::from_raw_borrowed(&phtmlwindow), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn UpdateDesktopComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtmlwindow: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateDesktopComponent(::windows_core::from_raw_borrowed(&phtmlwindow)).into()
        }
        unsafe extern "system" fn GetPendingUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpendingurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPendingUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpendingurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveElementChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtmlelement: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActiveElementChanged(::windows_core::from_raw_borrowed(&phtmlelement)).into()
        }
        unsafe extern "system" fn GetUrlSearchComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsearch: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUrlSearchComponent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsearch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsErrorUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszurl: ::windows_core::PCWSTR, pfiserror: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsErrorUrl(::core::mem::transmute(&lpszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiserror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FireBeforeNavigate2: FireBeforeNavigate2::<Identity, Impl, OFFSET>,
            FireNavigateComplete2: FireNavigateComplete2::<Identity, Impl, OFFSET>,
            FireDownloadBegin: FireDownloadBegin::<Identity, Impl, OFFSET>,
            FireDownloadComplete: FireDownloadComplete::<Identity, Impl, OFFSET>,
            FireDocumentComplete: FireDocumentComplete::<Identity, Impl, OFFSET>,
            UpdateDesktopComponent: UpdateDesktopComponent::<Identity, Impl, OFFSET>,
            GetPendingUrl: GetPendingUrl::<Identity, Impl, OFFSET>,
            ActiveElementChanged: ActiveElementChanged::<Identity, Impl, OFFSET>,
            GetUrlSearchComponent: GetUrlSearchComponent::<Identity, Impl, OFFSET>,
            IsErrorUrl: IsErrorUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDocObjectService as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadBehavior_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn startDownload(&self, bstrurl: &::windows_core::BSTR, pdispcallback: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDownloadBehavior {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDownloadBehavior_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadBehavior_Impl, const OFFSET: isize>() -> IDownloadBehavior_Vtbl {
        unsafe extern "system" fn startDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdispcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDownload(::core::mem::transmute(&bstrurl), ::windows_core::from_raw_borrowed(&pdispcallback)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), startDownload: startDownload::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadBehavior as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDownloadManager_Impl: Sized {
    fn Download(&self, pmk: ::core::option::Option<&super::super::System::Com::IMoniker>, pbc: ::core::option::Option<&super::super::System::Com::IBindCtx>, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: &::windows_core::PCWSTR, pszredir: &::windows_core::PCWSTR, uicp: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::RuntimeName for IDownloadManager {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IDownloadManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadManager_Impl, const OFFSET: isize>() -> IDownloadManager_Vtbl {
        unsafe extern "system" fn Download<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: ::windows_core::PCWSTR, pszredir: ::windows_core::PCWSTR, uicp: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Download(::windows_core::from_raw_borrowed(&pmk), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&dwbindverb), ::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute(&pszheaders), ::core::mem::transmute(&pszredir), ::core::mem::transmute_copy(&uicp)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Download: Download::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadManager as ::windows_core::Interface>::IID
    }
}
pub trait IEnumManagerFrames_Impl: Sized {
    fn Next(&self, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Count(&self) -> ::windows_core::Result<u32>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumManagerFrames>;
}
impl ::windows_core::RuntimeName for IEnumManagerFrames {}
impl IEnumManagerFrames_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: isize>() -> IEnumManagerFrames_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppwindows), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumManagerFrames as ::windows_core::Interface>::IID
    }
}
pub trait IEnumOpenServiceActivity_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IOpenServiceActivity>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumOpenServiceActivity>;
}
impl ::windows_core::RuntimeName for IEnumOpenServiceActivity {}
impl IEnumOpenServiceActivity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: isize>() -> IEnumOpenServiceActivity_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumOpenServiceActivity as ::windows_core::Interface>::IID
    }
}
pub trait IEnumOpenServiceActivityCategory_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IOpenServiceActivityCategory>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumOpenServiceActivityCategory>;
}
impl ::windows_core::RuntimeName for IEnumOpenServiceActivityCategory {}
impl IEnumOpenServiceActivityCategory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>() -> IEnumOpenServiceActivityCategory_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumOpenServiceActivityCategory as ::windows_core::Interface>::IID
    }
}
pub trait IEnumSTATURL_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumSTATURL>;
    fn SetFilter(&self, poszfilter: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumSTATURL {}
impl IEnumSTATURL_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: isize>() -> IEnumSTATURL_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poszfilter: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFilter(::core::mem::transmute(&poszfilter), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumSTATURL as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IExtensionValidation_Impl: Sized {
    fn Validate(&self, extensionguid: *const ::windows_core::GUID, extensionmodulepath: &::windows_core::PCWSTR, extensionfileversionms: u32, extensionfileversionls: u32, htmldocumenttop: ::core::option::Option<&super::MsHtml::IHTMLDocument2>, htmldocumentsubframe: ::core::option::Option<&super::MsHtml::IHTMLDocument2>, htmlelement: ::core::option::Option<&super::MsHtml::IHTMLElement>, contexts: ExtensionValidationContexts) -> ::windows_core::Result<ExtensionValidationResults>;
    fn DisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl ::windows_core::RuntimeName for IExtensionValidation {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IExtensionValidation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtensionValidation_Impl, const OFFSET: isize>() -> IExtensionValidation_Vtbl {
        unsafe extern "system" fn Validate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtensionValidation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionguid: *const ::windows_core::GUID, extensionmodulepath: ::windows_core::PCWSTR, extensionfileversionms: u32, extensionfileversionls: u32, htmldocumenttop: *mut ::core::ffi::c_void, htmldocumentsubframe: *mut ::core::ffi::c_void, htmlelement: *mut ::core::ffi::c_void, contexts: ExtensionValidationContexts, results: *mut ExtensionValidationResults) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Validate(::core::mem::transmute_copy(&extensionguid), ::core::mem::transmute(&extensionmodulepath), ::core::mem::transmute_copy(&extensionfileversionms), ::core::mem::transmute_copy(&extensionfileversionls), ::windows_core::from_raw_borrowed(&htmldocumenttop), ::windows_core::from_raw_borrowed(&htmldocumentsubframe), ::windows_core::from_raw_borrowed(&htmlelement), ::core::mem::transmute_copy(&contexts)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(results, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtensionValidation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Validate: Validate::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IExtensionValidation as ::windows_core::Interface>::IID
    }
}
pub trait IHTMLPersistData_Impl: Sized {
    fn save(&self, punk: ::core::option::Option<&::windows_core::IUnknown>, ltype: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn load(&self, punk: ::core::option::Option<&::windows_core::IUnknown>, ltype: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn queryType(&self, ltype: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
impl ::windows_core::RuntimeName for IHTMLPersistData {}
impl IHTMLPersistData_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistData_Impl, const OFFSET: isize>() -> IHTMLPersistData_Vtbl {
        unsafe extern "system" fn save<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, ltype: i32, fcontinuebroacast: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.save(::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&ltype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fcontinuebroacast, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn load<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, ltype: i32, fdodefault: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.load(::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&ltype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fdodefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn queryType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltype: i32, pfsupportstype: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.queryType(::core::mem::transmute_copy(&ltype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportstype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            save: save::<Identity, Impl, OFFSET>,
            load: load::<Identity, Impl, OFFSET>,
            queryType: queryType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHTMLPersistData as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHTMLPersistDataOM_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn getAttribute(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn setAttribute(&self, name: &::windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn removeAttribute(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IHTMLPersistDataOM {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHTMLPersistDataOM_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: isize>() -> IHTMLPersistDataOM_Vtbl {
        unsafe extern "system" fn XMLDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.XMLDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttribute(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute(&name)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHTMLPersistDataOM as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHTMLUserDataOM_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn save(&self, strname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn load(&self, strname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getAttribute(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn setAttribute(&self, name: &::windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn removeAttribute(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Setexpires(&self, bstr: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn expires(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IHTMLUserDataOM {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHTMLUserDataOM_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>() -> IHTMLUserDataOM_Vtbl {
        unsafe extern "system" fn XMLDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.XMLDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn save<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.save(::core::mem::transmute(&strname)).into()
        }
        unsafe extern "system" fn load<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.load(::core::mem::transmute(&strname)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttribute(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Setexpires<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setexpires(::core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn expires<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.expires() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, Impl, OFFSET>,
            save: save::<Identity, Impl, OFFSET>,
            load: load::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            Setexpires: Setexpires::<Identity, Impl, OFFSET>,
            expires: expires::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHTMLUserDataOM as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHeaderFooter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn htmlHead(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn htmlFoot(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettextHead(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn textHead(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettextFoot(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn textFoot(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setpage(&self, v: u32) -> ::windows_core::Result<()>;
    fn page(&self) -> ::windows_core::Result<u32>;
    fn SetpageTotal(&self, v: u32) -> ::windows_core::Result<()>;
    fn pageTotal(&self) -> ::windows_core::Result<u32>;
    fn SetURL(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn URL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Settitle(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn title(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetdateShort(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn dateShort(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetdateLong(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn dateLong(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettimeShort(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn timeShort(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettimeLong(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn timeLong(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IHeaderFooter {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHeaderFooter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>() -> IHeaderFooter_Vtbl {
        unsafe extern "system" fn htmlHead<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.htmlHead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn htmlFoot<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.htmlFoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettextHead<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SettextHead(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn textHead<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.textHead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettextFoot<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SettextFoot(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn textFoot<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.textFoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setpage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setpage(::core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn page<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.page() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetpageTotal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetpageTotal(::core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn pageTotal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.pageTotal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetURL(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn URL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.URL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settitle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Settitle(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn title<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdateShort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetdateShort(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn dateShort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dateShort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdateLong<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetdateLong(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn dateLong<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dateLong() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettimeShort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SettimeShort(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn timeShort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.timeShort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettimeLong<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SettimeLong(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn timeLong<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.timeLong() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            htmlHead: htmlHead::<Identity, Impl, OFFSET>,
            htmlFoot: htmlFoot::<Identity, Impl, OFFSET>,
            SettextHead: SettextHead::<Identity, Impl, OFFSET>,
            textHead: textHead::<Identity, Impl, OFFSET>,
            SettextFoot: SettextFoot::<Identity, Impl, OFFSET>,
            textFoot: textFoot::<Identity, Impl, OFFSET>,
            Setpage: Setpage::<Identity, Impl, OFFSET>,
            page: page::<Identity, Impl, OFFSET>,
            SetpageTotal: SetpageTotal::<Identity, Impl, OFFSET>,
            pageTotal: pageTotal::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            Settitle: Settitle::<Identity, Impl, OFFSET>,
            title: title::<Identity, Impl, OFFSET>,
            SetdateShort: SetdateShort::<Identity, Impl, OFFSET>,
            dateShort: dateShort::<Identity, Impl, OFFSET>,
            SetdateLong: SetdateLong::<Identity, Impl, OFFSET>,
            dateLong: dateLong::<Identity, Impl, OFFSET>,
            SettimeShort: SettimeShort::<Identity, Impl, OFFSET>,
            timeShort: timeShort::<Identity, Impl, OFFSET>,
            SettimeLong: SettimeLong::<Identity, Impl, OFFSET>,
            timeLong: timeLong::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHeaderFooter as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHeaderFooter2_Impl: Sized + IHeaderFooter_Impl {
    fn Setfont(&self, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn font(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IHeaderFooter2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHeaderFooter2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter2_Impl, const OFFSET: isize>() -> IHeaderFooter2_Vtbl {
        unsafe extern "system" fn Setfont<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setfont(::core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn font<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderFooter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.font() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IHeaderFooter_Vtbl::new::<Identity, Impl, OFFSET>(), Setfont: Setfont::<Identity, Impl, OFFSET>, font: font::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHeaderFooter2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IHeaderFooter as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHomePage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn navigateHomePage(&self) -> ::windows_core::Result<()>;
    fn setHomePage(&self, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn isHomePage(&self, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IHomePage {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHomePage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHomePage_Impl, const OFFSET: isize>() -> IHomePage_Vtbl {
        unsafe extern "system" fn navigateHomePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHomePage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.navigateHomePage().into()
        }
        unsafe extern "system" fn setHomePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHomePage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setHomePage(::core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn isHomePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHomePage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isHomePage(::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            navigateHomePage: navigateHomePage::<Identity, Impl, OFFSET>,
            setHomePage: setHomePage::<Identity, Impl, OFFSET>,
            isHomePage: isHomePage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHomePage as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IHomePageSetting_Impl: Sized {
    fn SetHomePage(&self, hwnd: super::super::Foundation::HWND, homepageuri: &::windows_core::PCWSTR, brandingmessage: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsHomePage(&self, uri: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetHomePageToBrowserDefault(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHomePageSetting {}
impl IHomePageSetting_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHomePageSetting_Impl, const OFFSET: isize>() -> IHomePageSetting_Vtbl {
        unsafe extern "system" fn SetHomePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHomePageSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, homepageuri: ::windows_core::PCWSTR, brandingmessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHomePage(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&homepageuri), ::core::mem::transmute(&brandingmessage)).into()
        }
        unsafe extern "system" fn IsHomePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHomePageSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, isdefault: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsHomePage(::core::mem::transmute(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePageToBrowserDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHomePageSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHomePageToBrowserDefault().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHomePage: SetHomePage::<Identity, Impl, OFFSET>,
            IsHomePage: IsHomePage::<Identity, Impl, OFFSET>,
            SetHomePageToBrowserDefault: SetHomePageToBrowserDefault::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHomePageSetting as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIEWebDriverManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExecuteCommand(&self, command: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IIEWebDriverManager {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIEWebDriverManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIEWebDriverManager_Impl, const OFFSET: isize>() -> IIEWebDriverManager_Vtbl {
        unsafe extern "system" fn ExecuteCommand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIEWebDriverManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: ::windows_core::PCWSTR, response: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecuteCommand(::core::mem::transmute(&command)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(response, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ExecuteCommand: ExecuteCommand::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIEWebDriverManager as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIEWebDriverSite_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WindowOperation(&self, operationcode: u32, hwnd: u32) -> ::windows_core::Result<()>;
    fn DetachWebdriver(&self, punkwd: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetCapabilityValue(&self, punkwd: ::core::option::Option<&::windows_core::IUnknown>, capname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IIEWebDriverSite {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIEWebDriverSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIEWebDriverSite_Impl, const OFFSET: isize>() -> IIEWebDriverSite_Vtbl {
        unsafe extern "system" fn WindowOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIEWebDriverSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationcode: u32, hwnd: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WindowOperation(::core::mem::transmute_copy(&operationcode), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn DetachWebdriver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIEWebDriverSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkwd: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DetachWebdriver(::windows_core::from_raw_borrowed(&punkwd)).into()
        }
        unsafe extern "system" fn GetCapabilityValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIEWebDriverSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkwd: *mut ::core::ffi::c_void, capname: ::windows_core::PCWSTR, capvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilityValue(::windows_core::from_raw_borrowed(&punkwd), ::core::mem::transmute(&capname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WindowOperation: WindowOperation::<Identity, Impl, OFFSET>,
            DetachWebdriver: DetachWebdriver::<Identity, Impl, OFFSET>,
            GetCapabilityValue: GetCapabilityValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIEWebDriverSite as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IImageDecodeEventSink_Impl: Sized {
    fn GetSurface(&self, nwidth: i32, nheight: i32, bfid: *const ::windows_core::GUID, npasses: u32, dwhints: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OnBeginDecode(&self, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnBitsComplete(&self) -> ::windows_core::Result<()>;
    fn OnDecodeComplete(&self, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnPalette(&self) -> ::windows_core::Result<()>;
    fn OnProgress(&self, pbounds: *const super::super::Foundation::RECT, bcomplete: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IImageDecodeEventSink {}
impl IImageDecodeEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: isize>() -> IImageDecodeEventSink_Vtbl {
        unsafe extern "system" fn GetSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nwidth: i32, nheight: i32, bfid: *const ::windows_core::GUID, npasses: u32, dwhints: u32, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSurface(::core::mem::transmute_copy(&nwidth), ::core::mem::transmute_copy(&nheight), ::core::mem::transmute_copy(&bfid), ::core::mem::transmute_copy(&npasses), ::core::mem::transmute_copy(&dwhints)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnBeginDecode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBeginDecode(::core::mem::transmute_copy(&pdwevents), ::core::mem::transmute_copy(&pnformats), ::core::mem::transmute_copy(&ppformats)).into()
        }
        unsafe extern "system" fn OnBitsComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBitsComplete().into()
        }
        unsafe extern "system" fn OnDecodeComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDecodeComplete(::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn OnPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPalette().into()
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbounds: *const super::super::Foundation::RECT, bcomplete: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnProgress(::core::mem::transmute_copy(&pbounds), ::core::mem::transmute_copy(&bcomplete)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSurface: GetSurface::<Identity, Impl, OFFSET>,
            OnBeginDecode: OnBeginDecode::<Identity, Impl, OFFSET>,
            OnBitsComplete: OnBitsComplete::<Identity, Impl, OFFSET>,
            OnDecodeComplete: OnDecodeComplete::<Identity, Impl, OFFSET>,
            OnPalette: OnPalette::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IImageDecodeEventSink as ::windows_core::Interface>::IID
    }
}
pub trait IImageDecodeEventSink2_Impl: Sized + IImageDecodeEventSink_Impl {
    fn IsAlphaPremultRequired(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for IImageDecodeEventSink2 {}
impl IImageDecodeEventSink2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink2_Impl, const OFFSET: isize>() -> IImageDecodeEventSink2_Vtbl {
        unsafe extern "system" fn IsAlphaPremultRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeEventSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpremultalpha: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAlphaPremultRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfpremultalpha, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IImageDecodeEventSink_Vtbl::new::<Identity, Impl, OFFSET>(), IsAlphaPremultRequired: IsAlphaPremultRequired::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IImageDecodeEventSink2 as ::windows_core::Interface>::IID || iid == &<IImageDecodeEventSink as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IImageDecodeFilter_Impl: Sized {
    fn Initialize(&self, peventsink: ::core::option::Option<&IImageDecodeEventSink>) -> ::windows_core::Result<()>;
    fn Process(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Terminate(&self, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IImageDecodeFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IImageDecodeFilter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeFilter_Impl, const OFFSET: isize>() -> IImageDecodeFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&peventsink)).into()
        }
        unsafe extern "system" fn Process<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Process(::windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageDecodeFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate(::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IImageDecodeFilter as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIntelliForms_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Setenabled(&self, bval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IIntelliForms {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIntelliForms_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIntelliForms_Impl, const OFFSET: isize>() -> IIntelliForms_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIntelliForms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIntelliForms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setenabled(::core::mem::transmute_copy(&bval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIntelliForms as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IInternetExplorerManager_Impl: Sized {
    fn CreateObject(&self, dwconfig: u32, pszurl: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IInternetExplorerManager {}
impl IInternetExplorerManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInternetExplorerManager_Impl, const OFFSET: isize>() -> IInternetExplorerManager_Vtbl {
        unsafe extern "system" fn CreateObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInternetExplorerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconfig: u32, pszurl: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateObject(::core::mem::transmute_copy(&dwconfig), ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObject: CreateObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInternetExplorerManager as ::windows_core::Interface>::IID
    }
}
pub trait IInternetExplorerManager2_Impl: Sized {
    fn EnumFrameWindows(&self) -> ::windows_core::Result<IEnumManagerFrames>;
}
impl ::windows_core::RuntimeName for IInternetExplorerManager2 {}
impl IInternetExplorerManager2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInternetExplorerManager2_Impl, const OFFSET: isize>() -> IInternetExplorerManager2_Vtbl {
        unsafe extern "system" fn EnumFrameWindows<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInternetExplorerManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFrameWindows() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumFrameWindows: EnumFrameWindows::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInternetExplorerManager2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILayoutRect_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetnextRect(&self, bstrelementid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn nextRect(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetcontentSrc(&self, varcontentsrc: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn contentSrc(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SethonorPageBreaks(&self, v: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn honorPageBreaks(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SethonorPageRules(&self, v: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn honorPageRules(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetnextRectElement(&self, pelem: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn nextRectElement(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn contentDocument(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ILayoutRect {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ILayoutRect_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>() -> ILayoutRect_Vtbl {
        unsafe extern "system" fn SetnextRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrelementid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetnextRect(::core::mem::transmute(&bstrelementid)).into()
        }
        unsafe extern "system" fn nextRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrelementid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nextRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrelementid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcontentSrc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varcontentsrc: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcontentSrc(::core::mem::transmute(&varcontentsrc)).into()
        }
        unsafe extern "system" fn contentSrc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcontentsrc: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentSrc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcontentsrc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethonorPageBreaks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SethonorPageBreaks(::core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn honorPageBreaks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.honorPageBreaks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethonorPageRules<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SethonorPageRules(::core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn honorPageRules<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.honorPageRules() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetnextRectElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetnextRectElement(::windows_core::from_raw_borrowed(&pelem)).into()
        }
        unsafe extern "system" fn nextRectElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nextRectElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdoc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetnextRect: SetnextRect::<Identity, Impl, OFFSET>,
            nextRect: nextRect::<Identity, Impl, OFFSET>,
            SetcontentSrc: SetcontentSrc::<Identity, Impl, OFFSET>,
            contentSrc: contentSrc::<Identity, Impl, OFFSET>,
            SethonorPageBreaks: SethonorPageBreaks::<Identity, Impl, OFFSET>,
            honorPageBreaks: honorPageBreaks::<Identity, Impl, OFFSET>,
            SethonorPageRules: SethonorPageRules::<Identity, Impl, OFFSET>,
            honorPageRules: honorPageRules::<Identity, Impl, OFFSET>,
            SetnextRectElement: SetnextRectElement::<Identity, Impl, OFFSET>,
            nextRectElement: nextRectElement::<Identity, Impl, OFFSET>,
            contentDocument: contentDocument::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILayoutRect as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IMapMIMEToCLSID_Impl: Sized {
    fn EnableDefaultMappings(&self, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn MapMIMEToCLSID(&self, pszmimetype: &::windows_core::PCWSTR, pclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetMapping(&self, pszmimetype: &::windows_core::PCWSTR, dwmapmode: u32, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IMapMIMEToCLSID {}
impl IMapMIMEToCLSID_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMapMIMEToCLSID_Impl, const OFFSET: isize>() -> IMapMIMEToCLSID_Vtbl {
        unsafe extern "system" fn EnableDefaultMappings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMapMIMEToCLSID_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableDefaultMappings(::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn MapMIMEToCLSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMapMIMEToCLSID_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmimetype: ::windows_core::PCWSTR, pclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapMIMEToCLSID(::core::mem::transmute(&pszmimetype), ::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetMapping<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMapMIMEToCLSID_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmimetype: ::windows_core::PCWSTR, dwmapmode: u32, clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMapping(::core::mem::transmute(&pszmimetype), ::core::mem::transmute_copy(&dwmapmode), ::core::mem::transmute_copy(&clsid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableDefaultMappings: EnableDefaultMappings::<Identity, Impl, OFFSET>,
            MapMIMEToCLSID: MapMIMEToCLSID::<Identity, Impl, OFFSET>,
            SetMapping: SetMapping::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMapMIMEToCLSID as ::windows_core::Interface>::IID
    }
}
pub trait IMediaActivityNotifySite_Impl: Sized {
    fn OnMediaActivityStarted(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows_core::Result<()>;
    fn OnMediaActivityStopped(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IMediaActivityNotifySite {}
impl IMediaActivityNotifySite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaActivityNotifySite_Impl, const OFFSET: isize>() -> IMediaActivityNotifySite_Vtbl {
        unsafe extern "system" fn OnMediaActivityStarted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaActivityNotifySite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMediaActivityStarted(::core::mem::transmute_copy(&mediaactivitytype)).into()
        }
        unsafe extern "system" fn OnMediaActivityStopped<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaActivityNotifySite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMediaActivityStopped(::core::mem::transmute_copy(&mediaactivitytype)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMediaActivityStarted: OnMediaActivityStarted::<Identity, Impl, OFFSET>,
            OnMediaActivityStopped: OnMediaActivityStopped::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMediaActivityNotifySite as ::windows_core::Interface>::IID
    }
}
pub trait IOpenService_Impl: Sized {
    fn IsDefault(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDefault(&self, fdefault: super::super::Foundation::BOOL, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GetID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IOpenService {}
impl IOpenService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenService_Impl, const OFFSET: isize>() -> IOpenService_Vtbl {
        unsafe extern "system" fn IsDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdefault: super::super::Foundation::BOOL, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefault(::core::mem::transmute_copy(&fdefault), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            SetDefault: SetDefault::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOpenService as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IOpenServiceActivity_Impl: Sized + IOpenService_Impl {
    fn Execute(&self, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<()>;
    fn CanExecute(&self, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanExecuteType(&self, r#type: OpenServiceActivityContentType) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Preview(&self, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<()>;
    fn CanPreview(&self, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanPreviewType(&self, r#type: OpenServiceActivityContentType) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetStatusText(&self, pinput: ::core::option::Option<&IOpenServiceActivityInput>) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHomepageUrl(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCategoryName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetIconPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetIcon(&self, fsmallicon: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn GetDescriptionFilePath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDownloadUrl(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInstallUrl(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsEnabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::RuntimeName for IOpenServiceActivity {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IOpenServiceActivity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>() -> IOpenServiceActivity_Vtbl {
        unsafe extern "system" fn Execute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Execute(::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn CanExecute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, pfcanexecute: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanExecute(::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanexecute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanExecuteType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanexecute: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanExecuteType(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanexecute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preview<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Preview(::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn CanPreview<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, pfcanpreview: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanPreview(::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanpreview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPreviewType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanpreview: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanPreviewType(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanpreview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatusText(::windows_core::from_raw_borrowed(&pinput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHomepageUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhomepageurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHomepageUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrhomepageurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategoryName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcategoryname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCategoryName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcategoryname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstriconpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIconPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstriconpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsmallicon: super::super::Foundation::BOOL, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIcon(::core::mem::transmute_copy(&fsmallicon)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionFilePath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxmlpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescriptionFilePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxmlpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDownloadUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxmluri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxmluri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstallUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinstalluri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInstallUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrinstalluri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: IOpenService_Vtbl::new::<Identity, Impl, OFFSET>(),
            Execute: Execute::<Identity, Impl, OFFSET>,
            CanExecute: CanExecute::<Identity, Impl, OFFSET>,
            CanExecuteType: CanExecuteType::<Identity, Impl, OFFSET>,
            Preview: Preview::<Identity, Impl, OFFSET>,
            CanPreview: CanPreview::<Identity, Impl, OFFSET>,
            CanPreviewType: CanPreviewType::<Identity, Impl, OFFSET>,
            GetStatusText: GetStatusText::<Identity, Impl, OFFSET>,
            GetHomepageUrl: GetHomepageUrl::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetCategoryName: GetCategoryName::<Identity, Impl, OFFSET>,
            GetIconPath: GetIconPath::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetDescriptionFilePath: GetDescriptionFilePath::<Identity, Impl, OFFSET>,
            GetDownloadUrl: GetDownloadUrl::<Identity, Impl, OFFSET>,
            GetInstallUrl: GetInstallUrl::<Identity, Impl, OFFSET>,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivity as ::windows_core::Interface>::IID || iid == &<IOpenService as ::windows_core::Interface>::IID
    }
}
pub trait IOpenServiceActivityCategory_Impl: Sized {
    fn HasDefaultActivity(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetDefaultActivity(&self) -> ::windows_core::Result<IOpenServiceActivity>;
    fn SetDefaultActivity(&self, pactivity: ::core::option::Option<&IOpenServiceActivity>, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetActivityEnumerator(&self, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<IEnumOpenServiceActivity>;
}
impl ::windows_core::RuntimeName for IOpenServiceActivityCategory {}
impl IOpenServiceActivityCategory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: isize>() -> IOpenServiceActivityCategory_Vtbl {
        unsafe extern "system" fn HasDefaultActivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasdefaultactivity: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasDefaultActivity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasdefaultactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultActivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdefaultactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultActivity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdefaultactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultActivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivity: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultActivity(::windows_core::from_raw_borrowed(&pactivity), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, ppenumactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityEnumerator(::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HasDefaultActivity: HasDefaultActivity::<Identity, Impl, OFFSET>,
            GetDefaultActivity: GetDefaultActivity::<Identity, Impl, OFFSET>,
            SetDefaultActivity: SetDefaultActivity::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetActivityEnumerator: GetActivityEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityCategory as ::windows_core::Interface>::IID
    }
}
pub trait IOpenServiceActivityInput_Impl: Sized {
    fn GetVariable(&self, pwzvariablename: &::windows_core::PCWSTR, pwzvariabletype: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HasVariable(&self, pwzvariablename: &::windows_core::PCWSTR, pwzvariabletype: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetType(&self) -> ::windows_core::Result<OpenServiceActivityContentType>;
}
impl ::windows_core::RuntimeName for IOpenServiceActivityInput {}
impl IOpenServiceActivityInput_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityInput_Impl, const OFFSET: isize>() -> IOpenServiceActivityInput_Vtbl {
        unsafe extern "system" fn GetVariable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzvariablename: ::windows_core::PCWSTR, pwzvariabletype: ::windows_core::PCWSTR, pbstrvariablecontent: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVariable(::core::mem::transmute(&pwzvariablename), ::core::mem::transmute(&pwzvariabletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvariablecontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasVariable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzvariablename: ::windows_core::PCWSTR, pwzvariabletype: ::windows_core::PCWSTR, pfhasvariable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasVariable(::core::mem::transmute(&pwzvariablename), ::core::mem::transmute(&pwzvariabletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasvariable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut OpenServiceActivityContentType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVariable: GetVariable::<Identity, Impl, OFFSET>,
            HasVariable: HasVariable::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityInput as ::windows_core::Interface>::IID
    }
}
pub trait IOpenServiceActivityManager_Impl: Sized {
    fn GetCategoryEnumerator(&self, etype: OpenServiceActivityContentType) -> ::windows_core::Result<IEnumOpenServiceActivityCategory>;
    fn GetActivityByID(&self, pwzactivityid: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpenServiceActivity>;
    fn GetActivityByHomepageAndCategory(&self, pwzhomepage: &::windows_core::PCWSTR, pwzcategory: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpenServiceActivity>;
    fn GetVersionCookie(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IOpenServiceActivityManager {}
impl IOpenServiceActivityManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: isize>() -> IOpenServiceActivityManager_Vtbl {
        unsafe extern "system" fn GetCategoryEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, etype: OpenServiceActivityContentType, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCategoryEnumerator(::core::mem::transmute_copy(&etype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityByID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzactivityid: ::windows_core::PCWSTR, ppactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityByID(::core::mem::transmute(&pwzactivityid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityByHomepageAndCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzhomepage: ::windows_core::PCWSTR, pwzcategory: ::windows_core::PCWSTR, ppactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityByHomepageAndCategory(::core::mem::transmute(&pwzhomepage), ::core::mem::transmute(&pwzcategory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersionCookie<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversioncookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersionCookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversioncookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCategoryEnumerator: GetCategoryEnumerator::<Identity, Impl, OFFSET>,
            GetActivityByID: GetActivityByID::<Identity, Impl, OFFSET>,
            GetActivityByHomepageAndCategory: GetActivityByHomepageAndCategory::<Identity, Impl, OFFSET>,
            GetVersionCookie: GetVersionCookie::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityManager as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOpenServiceActivityOutputContext_Impl: Sized {
    fn Navigate(&self, pwzuri: &::windows_core::PCWSTR, pwzmethod: &::windows_core::PCWSTR, pwzheaders: &::windows_core::PCWSTR, ppostdata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CanNavigate(&self, pwzuri: &::windows_core::PCWSTR, pwzmethod: &::windows_core::PCWSTR, pwzheaders: &::windows_core::PCWSTR, ppostdata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IOpenServiceActivityOutputContext {}
#[cfg(feature = "Win32_System_Com")]
impl IOpenServiceActivityOutputContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityOutputContext_Impl, const OFFSET: isize>() -> IOpenServiceActivityOutputContext_Vtbl {
        unsafe extern "system" fn Navigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityOutputContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzuri: ::windows_core::PCWSTR, pwzmethod: ::windows_core::PCWSTR, pwzheaders: ::windows_core::PCWSTR, ppostdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Navigate(::core::mem::transmute(&pwzuri), ::core::mem::transmute(&pwzmethod), ::core::mem::transmute(&pwzheaders), ::windows_core::from_raw_borrowed(&ppostdata)).into()
        }
        unsafe extern "system" fn CanNavigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceActivityOutputContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzuri: ::windows_core::PCWSTR, pwzmethod: ::windows_core::PCWSTR, pwzheaders: ::windows_core::PCWSTR, ppostdata: *mut ::core::ffi::c_void, pfcannavigate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanNavigate(::core::mem::transmute(&pwzuri), ::core::mem::transmute(&pwzmethod), ::core::mem::transmute(&pwzheaders), ::windows_core::from_raw_borrowed(&ppostdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcannavigate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Navigate: Navigate::<Identity, Impl, OFFSET>,
            CanNavigate: CanNavigate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityOutputContext as ::windows_core::Interface>::IID
    }
}
pub trait IOpenServiceManager_Impl: Sized {
    fn InstallService(&self, pwzserviceurl: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpenService>;
    fn UninstallService(&self, pservice: ::core::option::Option<&IOpenService>) -> ::windows_core::Result<()>;
    fn GetServiceByID(&self, pwzid: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpenService>;
}
impl ::windows_core::RuntimeName for IOpenServiceManager {}
impl IOpenServiceManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceManager_Impl, const OFFSET: isize>() -> IOpenServiceManager_Vtbl {
        unsafe extern "system" fn InstallService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzserviceurl: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstallService(::core::mem::transmute(&pwzserviceurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UninstallService(::windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn GetServiceByID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IOpenServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzid: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceByID(::core::mem::transmute(&pwzid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InstallService: InstallService::<Identity, Impl, OFFSET>,
            UninstallService: UninstallService::<Identity, Impl, OFFSET>,
            GetServiceByID: GetServiceByID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IOpenServiceManager as ::windows_core::Interface>::IID
    }
}
pub trait IPeerFactory_Impl: Sized {}
impl ::windows_core::RuntimeName for IPeerFactory {}
impl IPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPeerFactory_Impl, const OFFSET: isize>() -> IPeerFactory_Vtbl {
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPeerFactory as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistHistory_Impl: Sized + super::super::System::Com::IPersist_Impl {
    fn LoadHistory(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>, pbc: ::core::option::Option<&super::super::System::Com::IBindCtx>) -> ::windows_core::Result<()>;
    fn SaveHistory(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetPositionCookie(&self, dwpositioncookie: u32) -> ::windows_core::Result<()>;
    fn GetPositionCookie(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IPersistHistory {}
#[cfg(feature = "Win32_System_Com")]
impl IPersistHistory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: isize>() -> IPersistHistory_Vtbl {
        unsafe extern "system" fn LoadHistory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadHistory(::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&pbc)).into()
        }
        unsafe extern "system" fn SaveHistory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveHistory(::windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn SetPositionCookie<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpositioncookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPositionCookie(::core::mem::transmute_copy(&dwpositioncookie)).into()
        }
        unsafe extern "system" fn GetPositionCookie<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpositioncookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPositionCookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpositioncookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            LoadHistory: LoadHistory::<Identity, Impl, OFFSET>,
            SaveHistory: SaveHistory::<Identity, Impl, OFFSET>,
            SetPositionCookie: SetPositionCookie::<Identity, Impl, OFFSET>,
            GetPositionCookie: GetPositionCookie::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPersistHistory as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersist as ::windows_core::Interface>::IID
    }
}
pub trait IPrintTaskRequestFactory_Impl: Sized {
    fn CreatePrintTaskRequest(&self, pprinttaskrequesthandler: ::core::option::Option<&IPrintTaskRequestHandler>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintTaskRequestFactory {}
impl IPrintTaskRequestFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskRequestFactory_Impl, const OFFSET: isize>() -> IPrintTaskRequestFactory_Vtbl {
        unsafe extern "system" fn CreatePrintTaskRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskRequestFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprinttaskrequesthandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePrintTaskRequest(::windows_core::from_raw_borrowed(&pprinttaskrequesthandler)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreatePrintTaskRequest: CreatePrintTaskRequest::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrintTaskRequestFactory as ::windows_core::Interface>::IID
    }
}
pub trait IPrintTaskRequestHandler_Impl: Sized {
    fn HandlePrintTaskRequest(&self, pprinttaskrequest: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintTaskRequestHandler {}
impl IPrintTaskRequestHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskRequestHandler_Impl, const OFFSET: isize>() -> IPrintTaskRequestHandler_Vtbl {
        unsafe extern "system" fn HandlePrintTaskRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskRequestHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprinttaskrequest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandlePrintTaskRequest(::windows_core::from_raw_borrowed(&pprinttaskrequest)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandlePrintTaskRequest: HandlePrintTaskRequest::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrintTaskRequestHandler as ::windows_core::Interface>::IID
    }
}
pub trait IScrollableContextMenu_Impl: Sized {
    fn AddItem(&self, itemtext: &::windows_core::PCWSTR, cmdid: u32) -> ::windows_core::Result<()>;
    fn ShowModal(&self, x: i32, y: i32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IScrollableContextMenu {}
impl IScrollableContextMenu_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollableContextMenu_Impl, const OFFSET: isize>() -> IScrollableContextMenu_Vtbl {
        unsafe extern "system" fn AddItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollableContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemtext: ::windows_core::PCWSTR, cmdid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute(&itemtext), ::core::mem::transmute_copy(&cmdid)).into()
        }
        unsafe extern "system" fn ShowModal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollableContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, cmdid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShowModal(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cmdid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            ShowModal: ShowModal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IScrollableContextMenu as ::windows_core::Interface>::IID
    }
}
pub trait IScrollableContextMenu2_Impl: Sized + IScrollableContextMenu_Impl {
    fn AddSeparator(&self) -> ::windows_core::Result<()>;
    fn SetPlacement(&self, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IScrollableContextMenu2 {}
impl IScrollableContextMenu2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollableContextMenu2_Impl, const OFFSET: isize>() -> IScrollableContextMenu2_Vtbl {
        unsafe extern "system" fn AddSeparator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollableContextMenu2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSeparator().into()
        }
        unsafe extern "system" fn SetPlacement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollableContextMenu2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlacement(::core::mem::transmute_copy(&scmp)).into()
        }
        Self {
            base__: IScrollableContextMenu_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddSeparator: AddSeparator::<Identity, Impl, OFFSET>,
            SetPlacement: SetPlacement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IScrollableContextMenu2 as ::windows_core::Interface>::IID || iid == &<IScrollableContextMenu as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISniffStream_Impl: Sized {
    fn Init(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Peek(&self, pbuffer: *mut ::core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ISniffStream {}
#[cfg(feature = "Win32_System_Com")]
impl ISniffStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISniffStream_Impl, const OFFSET: isize>() -> ISniffStream_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISniffStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Peek<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISniffStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Peek(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&nbytes), ::core::mem::transmute_copy(&pnbytesread)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Init: Init::<Identity, Impl, OFFSET>, Peek: Peek::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISniffStream as ::windows_core::Interface>::IID
    }
}
pub trait ISurfacePresenterFlip_Impl: Sized {
    fn Present(&self) -> ::windows_core::Result<()>;
    fn GetBuffer(&self, backbufferindex: u32, riid: *const ::windows_core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISurfacePresenterFlip {}
impl ISurfacePresenterFlip_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfacePresenterFlip_Impl, const OFFSET: isize>() -> ISurfacePresenterFlip_Vtbl {
        unsafe extern "system" fn Present<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfacePresenterFlip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Present().into()
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfacePresenterFlip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backbufferindex: u32, riid: *const ::windows_core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBuffer(::core::mem::transmute_copy(&backbufferindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppbuffer)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlip as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ISurfacePresenterFlip2_Impl: Sized {
    fn SetRotation(&self, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::RuntimeName for ISurfacePresenterFlip2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ISurfacePresenterFlip2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfacePresenterFlip2_Impl, const OFFSET: isize>() -> ISurfacePresenterFlip2_Vtbl {
        unsafe extern "system" fn SetRotation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfacePresenterFlip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRotation(::core::mem::transmute_copy(&dxgirotation)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRotation: SetRotation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlip2 as ::windows_core::Interface>::IID
    }
}
pub trait ISurfacePresenterFlipBuffer_Impl: Sized {
    fn BeginDraw(&self, riid: *const ::windows_core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EndDraw(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISurfacePresenterFlipBuffer {}
impl ISurfacePresenterFlipBuffer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfacePresenterFlipBuffer_Impl, const OFFSET: isize>() -> ISurfacePresenterFlipBuffer_Vtbl {
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfacePresenterFlipBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDraw(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfacePresenterFlipBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDraw().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlipBuffer as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetContainer_Impl: Sized {
    fn GetFrameUrl(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> ::windows_core::Result<super::super::System::Ole::IOleContainer>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::RuntimeName for ITargetContainer {}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetContainer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetContainer_Impl, const OFFSET: isize>() -> ITargetContainer_Vtbl {
        unsafe extern "system" fn GetFrameUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframesrc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFramesContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameUrl: GetFrameUrl::<Identity, Impl, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITargetContainer as ::windows_core::Interface>::IID
    }
}
pub trait ITargetEmbedding_Impl: Sized {
    fn GetTargetFrame(&self) -> ::windows_core::Result<ITargetFrame>;
}
impl ::windows_core::RuntimeName for ITargetEmbedding {}
impl ITargetEmbedding_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetEmbedding_Impl, const OFFSET: isize>() -> ITargetEmbedding_Vtbl {
        unsafe extern "system" fn GetTargetFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetEmbedding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetFrame() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTargetFrame: GetTargetFrame::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITargetEmbedding as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetFrame_Impl: Sized {
    fn SetFrameName(&self, pszframename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFrameName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetParentFrame(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn FindFrame(&self, psztargetname: &::windows_core::PCWSTR, ppunkcontextframe: ::core::option::Option<&::windows_core::IUnknown>, dwflags: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetFrameSrc(&self, pszframesrc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFrameSrc(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> ::windows_core::Result<super::super::System::Ole::IOleContainer>;
    fn SetFrameOptions(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetFrameOptions(&self) -> ::windows_core::Result<u32>;
    fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> ::windows_core::Result<()>;
    fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::Result<()>;
    fn RemoteNavigate(&self, clength: u32, puldata: *const u32) -> ::windows_core::Result<()>;
    fn OnChildFrameActivate(&self, punkchildframe: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnChildFrameDeactivate(&self, punkchildframe: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::RuntimeName for ITargetFrame {}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetFrame_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>() -> ITargetFrame_Vtbl {
        unsafe extern "system" fn SetFrameName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszframename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrameName(::core::mem::transmute(&pszframename)).into()
        }
        unsafe extern "system" fn GetFrameName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszframename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentFrame() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, ppunkcontextframe: *mut ::core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFrame(::core::mem::transmute(&psztargetname), ::windows_core::from_raw_borrowed(&ppunkcontextframe), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunktargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameSrc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszframesrc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrameSrc(::core::mem::transmute(&pszframesrc)).into()
        }
        unsafe extern "system" fn GetFrameSrc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameSrc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframesrc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFramesContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrameOptions(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFrameOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameMargins<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwidth: u32, dwheight: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrameMargins(::core::mem::transmute_copy(&dwwidth), ::core::mem::transmute_copy(&dwheight)).into()
        }
        unsafe extern "system" fn GetFrameMargins<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameMargins(::core::mem::transmute_copy(&pdwwidth), ::core::mem::transmute_copy(&pdwheight)).into()
        }
        unsafe extern "system" fn RemoteNavigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clength: u32, puldata: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoteNavigate(::core::mem::transmute_copy(&clength), ::core::mem::transmute_copy(&puldata)).into()
        }
        unsafe extern "system" fn OnChildFrameActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChildFrameActivate(::windows_core::from_raw_borrowed(&punkchildframe)).into()
        }
        unsafe extern "system" fn OnChildFrameDeactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChildFrameDeactivate(::windows_core::from_raw_borrowed(&punkchildframe)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFrameName: SetFrameName::<Identity, Impl, OFFSET>,
            GetFrameName: GetFrameName::<Identity, Impl, OFFSET>,
            GetParentFrame: GetParentFrame::<Identity, Impl, OFFSET>,
            FindFrame: FindFrame::<Identity, Impl, OFFSET>,
            SetFrameSrc: SetFrameSrc::<Identity, Impl, OFFSET>,
            GetFrameSrc: GetFrameSrc::<Identity, Impl, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, Impl, OFFSET>,
            SetFrameOptions: SetFrameOptions::<Identity, Impl, OFFSET>,
            GetFrameOptions: GetFrameOptions::<Identity, Impl, OFFSET>,
            SetFrameMargins: SetFrameMargins::<Identity, Impl, OFFSET>,
            GetFrameMargins: GetFrameMargins::<Identity, Impl, OFFSET>,
            RemoteNavigate: RemoteNavigate::<Identity, Impl, OFFSET>,
            OnChildFrameActivate: OnChildFrameActivate::<Identity, Impl, OFFSET>,
            OnChildFrameDeactivate: OnChildFrameDeactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITargetFrame as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetFrame2_Impl: Sized {
    fn SetFrameName(&self, pszframename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFrameName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetParentFrame(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetFrameSrc(&self, pszframesrc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFrameSrc(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> ::windows_core::Result<super::super::System::Ole::IOleContainer>;
    fn SetFrameOptions(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetFrameOptions(&self) -> ::windows_core::Result<u32>;
    fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> ::windows_core::Result<()>;
    fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::Result<()>;
    fn FindFrame(&self, psztargetname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetTargetAlias(&self, psztargetname: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::RuntimeName for ITargetFrame2 {}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetFrame2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>() -> ITargetFrame2_Vtbl {
        unsafe extern "system" fn SetFrameName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszframename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrameName(::core::mem::transmute(&pszframename)).into()
        }
        unsafe extern "system" fn GetFrameName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszframename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentFrame() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameSrc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszframesrc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrameSrc(::core::mem::transmute(&pszframesrc)).into()
        }
        unsafe extern "system" fn GetFrameSrc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameSrc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframesrc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFramesContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrameOptions(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFrameOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameMargins<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwidth: u32, dwheight: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrameMargins(::core::mem::transmute_copy(&dwwidth), ::core::mem::transmute_copy(&dwheight)).into()
        }
        unsafe extern "system" fn GetFrameMargins<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameMargins(::core::mem::transmute_copy(&pdwwidth), ::core::mem::transmute_copy(&pdwheight)).into()
        }
        unsafe extern "system" fn FindFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFrame(::core::mem::transmute(&psztargetname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunktargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetAlias<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, ppsztargetalias: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetAlias(::core::mem::transmute(&psztargetname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztargetalias, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFrameName: SetFrameName::<Identity, Impl, OFFSET>,
            GetFrameName: GetFrameName::<Identity, Impl, OFFSET>,
            GetParentFrame: GetParentFrame::<Identity, Impl, OFFSET>,
            SetFrameSrc: SetFrameSrc::<Identity, Impl, OFFSET>,
            GetFrameSrc: GetFrameSrc::<Identity, Impl, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, Impl, OFFSET>,
            SetFrameOptions: SetFrameOptions::<Identity, Impl, OFFSET>,
            GetFrameOptions: GetFrameOptions::<Identity, Impl, OFFSET>,
            SetFrameMargins: SetFrameMargins::<Identity, Impl, OFFSET>,
            GetFrameMargins: GetFrameMargins::<Identity, Impl, OFFSET>,
            FindFrame: FindFrame::<Identity, Impl, OFFSET>,
            GetTargetAlias: GetTargetAlias::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITargetFrame2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITargetFramePriv_Impl: Sized {
    fn FindFrameDownwards(&self, psztargetname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn FindFrameInContext(&self, psztargetname: &::windows_core::PCWSTR, punkcontextframe: ::core::option::Option<&::windows_core::IUnknown>, dwflags: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OnChildFrameActivate(&self, punkchildframe: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnChildFrameDeactivate(&self, punkchildframe: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn NavigateHack(&self, grfhlnf: u32, pbc: ::core::option::Option<&super::super::System::Com::IBindCtx>, pibsc: ::core::option::Option<&super::super::System::Com::IBindStatusCallback>, psztargetname: &::windows_core::PCWSTR, pszurl: &::windows_core::PCWSTR, pszlocation: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FindBrowserByIndex(&self, dwid: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITargetFramePriv {}
#[cfg(feature = "Win32_System_Com")]
impl ITargetFramePriv_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: isize>() -> ITargetFramePriv_Vtbl {
        unsafe extern "system" fn FindFrameDownwards<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFrameDownwards(::core::mem::transmute(&psztargetname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunktargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFrameInContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, punkcontextframe: *mut ::core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFrameInContext(::core::mem::transmute(&psztargetname), ::windows_core::from_raw_borrowed(&punkcontextframe), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunktargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnChildFrameActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChildFrameActivate(::windows_core::from_raw_borrowed(&punkchildframe)).into()
        }
        unsafe extern "system" fn OnChildFrameDeactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChildFrameDeactivate(::windows_core::from_raw_borrowed(&punkchildframe)).into()
        }
        unsafe extern "system" fn NavigateHack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfhlnf: u32, pbc: *mut ::core::ffi::c_void, pibsc: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, pszurl: ::windows_core::PCWSTR, pszlocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NavigateHack(::core::mem::transmute_copy(&grfhlnf), ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pibsc), ::core::mem::transmute(&psztargetname), ::core::mem::transmute(&pszurl), ::core::mem::transmute(&pszlocation)).into()
        }
        unsafe extern "system" fn FindBrowserByIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32, ppunkbrowser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindBrowserByIndex(::core::mem::transmute_copy(&dwid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkbrowser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFrameDownwards: FindFrameDownwards::<Identity, Impl, OFFSET>,
            FindFrameInContext: FindFrameInContext::<Identity, Impl, OFFSET>,
            OnChildFrameActivate: OnChildFrameActivate::<Identity, Impl, OFFSET>,
            OnChildFrameDeactivate: OnChildFrameDeactivate::<Identity, Impl, OFFSET>,
            NavigateHack: NavigateHack::<Identity, Impl, OFFSET>,
            FindBrowserByIndex: FindBrowserByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITargetFramePriv as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITargetFramePriv2_Impl: Sized + ITargetFramePriv_Impl {
    fn AggregatedNavigation2(&self, grfhlnf: u32, pbc: ::core::option::Option<&super::super::System::Com::IBindCtx>, pibsc: ::core::option::Option<&super::super::System::Com::IBindStatusCallback>, psztargetname: &::windows_core::PCWSTR, puri: ::core::option::Option<&super::super::System::Com::IUri>, pszlocation: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITargetFramePriv2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITargetFramePriv2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv2_Impl, const OFFSET: isize>() -> ITargetFramePriv2_Vtbl {
        unsafe extern "system" fn AggregatedNavigation2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetFramePriv2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfhlnf: u32, pbc: *mut ::core::ffi::c_void, pibsc: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, puri: *mut ::core::ffi::c_void, pszlocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AggregatedNavigation2(::core::mem::transmute_copy(&grfhlnf), ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pibsc), ::core::mem::transmute(&psztargetname), ::windows_core::from_raw_borrowed(&puri), ::core::mem::transmute(&pszlocation)).into()
        }
        Self { base__: ITargetFramePriv_Vtbl::new::<Identity, Impl, OFFSET>(), AggregatedNavigation2: AggregatedNavigation2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITargetFramePriv2 as ::windows_core::Interface>::IID || iid == &<ITargetFramePriv as ::windows_core::Interface>::IID
    }
}
pub trait ITargetNotify_Impl: Sized {
    fn OnCreate(&self, punkdestination: ::core::option::Option<&::windows_core::IUnknown>, cbcookie: u32) -> ::windows_core::Result<()>;
    fn OnReuse(&self, punkdestination: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITargetNotify {}
impl ITargetNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetNotify_Impl, const OFFSET: isize>() -> ITargetNotify_Vtbl {
        unsafe extern "system" fn OnCreate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdestination: *mut ::core::ffi::c_void, cbcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCreate(::windows_core::from_raw_borrowed(&punkdestination), ::core::mem::transmute_copy(&cbcookie)).into()
        }
        unsafe extern "system" fn OnReuse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdestination: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnReuse(::windows_core::from_raw_borrowed(&punkdestination)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCreate: OnCreate::<Identity, Impl, OFFSET>,
            OnReuse: OnReuse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITargetNotify as ::windows_core::Interface>::IID
    }
}
pub trait ITargetNotify2_Impl: Sized + ITargetNotify_Impl {
    fn GetOptionString(&self, pbstroptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITargetNotify2 {}
impl ITargetNotify2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetNotify2_Impl, const OFFSET: isize>() -> ITargetNotify2_Vtbl {
        unsafe extern "system" fn GetOptionString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstroptions: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOptionString(::core::mem::transmute_copy(&pbstroptions)).into()
        }
        Self { base__: ITargetNotify_Vtbl::new::<Identity, Impl, OFFSET>(), GetOptionString: GetOptionString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITargetNotify2 as ::windows_core::Interface>::IID || iid == &<ITargetNotify as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimer_Impl: Sized {
    fn Advise(&self, vtimemin: &super::super::System::Variant::VARIANT, vtimemax: &super::super::System::Variant::VARIANT, vtimeinterval: &super::super::System::Variant::VARIANT, dwflags: u32, ptimersink: ::core::option::Option<&ITimerSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> ::windows_core::Result<()>;
    fn Freeze(&self, ffreeze: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetTime(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ITimer {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITimer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: isize>() -> ITimer_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtimemin: super::super::System::Variant::VARIANT, vtimemax: super::super::System::Variant::VARIANT, vtimeinterval: super::super::System::Variant::VARIANT, dwflags: u32, ptimersink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Advise(::core::mem::transmute(&vtimemin), ::core::mem::transmute(&vtimemax), ::core::mem::transmute(&vtimeinterval), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&ptimersink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn Freeze<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffreeze: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Freeze(::core::mem::transmute_copy(&ffreeze)).into()
        }
        unsafe extern "system" fn GetTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtime: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            Freeze: Freeze::<Identity, Impl, OFFSET>,
            GetTime: GetTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITimer as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimerEx_Impl: Sized + ITimer_Impl {
    fn SetMode(&self, dwmode: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ITimerEx {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITimerEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimerEx_Impl, const OFFSET: isize>() -> ITimerEx_Vtbl {
        unsafe extern "system" fn SetMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMode(::core::mem::transmute_copy(&dwmode)).into()
        }
        Self { base__: ITimer_Vtbl::new::<Identity, Impl, OFFSET>(), SetMode: SetMode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITimerEx as ::windows_core::Interface>::IID || iid == &<ITimer as ::windows_core::Interface>::IID
    }
}
pub trait ITimerService_Impl: Sized {
    fn CreateTimer(&self, preferencetimer: ::core::option::Option<&ITimer>) -> ::windows_core::Result<ITimer>;
    fn GetNamedTimer(&self, rguidname: *const ::windows_core::GUID) -> ::windows_core::Result<ITimer>;
    fn SetNamedTimerReference(&self, rguidname: *const ::windows_core::GUID, preferencetimer: ::core::option::Option<&ITimer>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITimerService {}
impl ITimerService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimerService_Impl, const OFFSET: isize>() -> ITimerService_Vtbl {
        unsafe extern "system" fn CreateTimer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimerService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferencetimer: *mut ::core::ffi::c_void, ppnewtimer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTimer(::windows_core::from_raw_borrowed(&preferencetimer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewtimer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedTimer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimerService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidname: *const ::windows_core::GUID, pptimer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNamedTimer(::core::mem::transmute_copy(&rguidname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptimer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedTimerReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimerService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidname: *const ::windows_core::GUID, preferencetimer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamedTimerReference(::core::mem::transmute_copy(&rguidname), ::windows_core::from_raw_borrowed(&preferencetimer)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTimer: CreateTimer::<Identity, Impl, OFFSET>,
            GetNamedTimer: GetNamedTimer::<Identity, Impl, OFFSET>,
            SetNamedTimerReference: SetNamedTimerReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITimerService as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimerSink_Impl: Sized {
    fn OnTimer(&self, vtimeadvise: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ITimerSink {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITimerSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimerSink_Impl, const OFFSET: isize>() -> ITimerSink_Vtbl {
        unsafe extern "system" fn OnTimer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITimerSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtimeadvise: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTimer(::core::mem::transmute(&vtimeadvise)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTimer: OnTimer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITimerSink as ::windows_core::Interface>::IID
    }
}
pub trait ITridentTouchInput_Impl: Sized {
    fn OnPointerMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
impl ::windows_core::RuntimeName for ITridentTouchInput {}
impl ITridentTouchInput_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITridentTouchInput_Impl, const OFFSET: isize>() -> ITridentTouchInput_Vtbl {
        unsafe extern "system" fn OnPointerMessage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITridentTouchInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfallowmanipulations: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnPointerMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowmanipulations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPointerMessage: OnPointerMessage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITridentTouchInput as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Web_MsHtml\"`"]
#[cfg(feature = "Win32_Web_MsHtml")]
pub trait ITridentTouchInputSite_Impl: Sized {
    fn SetManipulationMode(&self, mstouchaction: super::MsHtml::styleMsTouchAction) -> ::windows_core::Result<()>;
    fn ZoomToPoint(&self, x: i32, y: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Web_MsHtml")]
impl ::windows_core::RuntimeName for ITridentTouchInputSite {}
#[cfg(feature = "Win32_Web_MsHtml")]
impl ITridentTouchInputSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITridentTouchInputSite_Impl, const OFFSET: isize>() -> ITridentTouchInputSite_Vtbl {
        unsafe extern "system" fn SetManipulationMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITridentTouchInputSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstouchaction: super::MsHtml::styleMsTouchAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetManipulationMode(::core::mem::transmute_copy(&mstouchaction)).into()
        }
        unsafe extern "system" fn ZoomToPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITridentTouchInputSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ZoomToPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetManipulationMode: SetManipulationMode::<Identity, Impl, OFFSET>,
            ZoomToPoint: ZoomToPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITridentTouchInputSite as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUrlHistoryNotify_Impl: Sized + super::super::System::Ole::IOleCommandTarget_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUrlHistoryNotify {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUrlHistoryNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryNotify_Impl, const OFFSET: isize>() -> IUrlHistoryNotify_Vtbl {
        Self { base__: super::super::System::Ole::IOleCommandTarget_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUrlHistoryNotify as ::windows_core::Interface>::IID || iid == &<super::super::System::Ole::IOleCommandTarget as ::windows_core::Interface>::IID
    }
}
pub trait IUrlHistoryStg_Impl: Sized {
    fn AddUrl(&self, pocsurl: &::windows_core::PCWSTR, pocstitle: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn DeleteUrl(&self, pocsurl: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn QueryUrl(&self, pocsurl: &::windows_core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> ::windows_core::Result<()>;
    fn BindToObject(&self, pocsurl: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppvout: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnumUrls(&self) -> ::windows_core::Result<IEnumSTATURL>;
}
impl ::windows_core::RuntimeName for IUrlHistoryStg {}
impl IUrlHistoryStg_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: isize>() -> IUrlHistoryStg_Vtbl {
        unsafe extern "system" fn AddUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, pocstitle: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddUrl(::core::mem::transmute(&pocsurl), ::core::mem::transmute(&pocstitle), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteUrl(::core::mem::transmute(&pocsurl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn QueryUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryUrl(::core::mem::transmute(&pocsurl), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&lpstaturl)).into()
        }
        unsafe extern "system" fn BindToObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppvout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToObject(::core::mem::transmute(&pocsurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvout)).into()
        }
        unsafe extern "system" fn EnumUrls<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumUrls() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddUrl: AddUrl::<Identity, Impl, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, Impl, OFFSET>,
            QueryUrl: QueryUrl::<Identity, Impl, OFFSET>,
            BindToObject: BindToObject::<Identity, Impl, OFFSET>,
            EnumUrls: EnumUrls::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUrlHistoryStg as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IUrlHistoryStg2_Impl: Sized + IUrlHistoryStg_Impl {
    fn AddUrlAndNotify(&self, pocsurl: &::windows_core::PCWSTR, pocstitle: &::windows_core::PCWSTR, dwflags: u32, fwritehistory: super::super::Foundation::BOOL, poctnotify: ::core::option::Option<&super::super::System::Ole::IOleCommandTarget>, punkisfolder: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn ClearHistory(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::RuntimeName for IUrlHistoryStg2 {}
#[cfg(feature = "Win32_System_Ole")]
impl IUrlHistoryStg2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg2_Impl, const OFFSET: isize>() -> IUrlHistoryStg2_Vtbl {
        unsafe extern "system" fn AddUrlAndNotify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, pocstitle: ::windows_core::PCWSTR, dwflags: u32, fwritehistory: super::super::Foundation::BOOL, poctnotify: *mut ::core::ffi::c_void, punkisfolder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddUrlAndNotify(::core::mem::transmute(&pocsurl), ::core::mem::transmute(&pocstitle), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&fwritehistory), ::windows_core::from_raw_borrowed(&poctnotify), ::windows_core::from_raw_borrowed(&punkisfolder)).into()
        }
        unsafe extern "system" fn ClearHistory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUrlHistoryStg2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearHistory().into()
        }
        Self {
            base__: IUrlHistoryStg_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddUrlAndNotify: AddUrlAndNotify::<Identity, Impl, OFFSET>,
            ClearHistory: ClearHistory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUrlHistoryStg2 as ::windows_core::Interface>::IID || iid == &<IUrlHistoryStg as ::windows_core::Interface>::IID
    }
}
pub trait IViewObjectPresentFlip_Impl: Sized {
    fn NotifyRender(&self, frecreatepresenter: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RenderObjectToBitmap(&self, pbitmap: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RenderObjectToSharedBuffer(&self, pbuffer: ::core::option::Option<&ISurfacePresenterFlipBuffer>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IViewObjectPresentFlip {}
impl IViewObjectPresentFlip_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlip_Impl, const OFFSET: isize>() -> IViewObjectPresentFlip_Vtbl {
        unsafe extern "system" fn NotifyRender<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecreatepresenter: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyRender(::core::mem::transmute_copy(&frecreatepresenter)).into()
        }
        unsafe extern "system" fn RenderObjectToBitmap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenderObjectToBitmap(::windows_core::from_raw_borrowed(&pbitmap)).into()
        }
        unsafe extern "system" fn RenderObjectToSharedBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenderObjectToSharedBuffer(::windows_core::from_raw_borrowed(&pbuffer)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifyRender: NotifyRender::<Identity, Impl, OFFSET>,
            RenderObjectToBitmap: RenderObjectToBitmap::<Identity, Impl, OFFSET>,
            RenderObjectToSharedBuffer: RenderObjectToSharedBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlip as ::windows_core::Interface>::IID
    }
}
pub trait IViewObjectPresentFlip2_Impl: Sized {
    fn NotifyLeavingView(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IViewObjectPresentFlip2 {}
impl IViewObjectPresentFlip2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlip2_Impl, const OFFSET: isize>() -> IViewObjectPresentFlip2_Vtbl {
        unsafe extern "system" fn NotifyLeavingView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyLeavingView().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyLeavingView: NotifyLeavingView::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlip2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Web_MsHtml\"`"]
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml"))]
pub trait IViewObjectPresentFlipSite_Impl: Sized {
    fn CreateSurfacePresenterFlip(&self, pdevice: ::core::option::Option<&::windows_core::IUnknown>, width: u32, height: u32, backbuffercount: u32, format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT, mode: super::MsHtml::VIEW_OBJECT_ALPHA_MODE) -> ::windows_core::Result<ISurfacePresenterFlip>;
    fn GetDeviceLuid(&self) -> ::windows_core::Result<super::super::Foundation::LUID>;
    fn EnterFullScreen(&self) -> ::windows_core::Result<()>;
    fn ExitFullScreen(&self) -> ::windows_core::Result<()>;
    fn IsFullScreen(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetBoundingRect(&self) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetMetrics(&self, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> ::windows_core::Result<()>;
    fn GetFullScreenSize(&self) -> ::windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml"))]
impl ::windows_core::RuntimeName for IViewObjectPresentFlipSite {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml"))]
impl IViewObjectPresentFlipSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>() -> IViewObjectPresentFlipSite_Vtbl {
        unsafe extern "system" fn CreateSurfacePresenterFlip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, width: u32, height: u32, backbuffercount: u32, format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT, mode: super::MsHtml::VIEW_OBJECT_ALPHA_MODE, ppspflip: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSurfacePresenterFlip(::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&backbuffercount), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspflip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceLuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pluid: *mut super::super::Foundation::LUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceLuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pluid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterFullScreen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnterFullScreen().into()
        }
        unsafe extern "system" fn ExitFullScreen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExitFullScreen().into()
        }
        unsafe extern "system" fn IsFullScreen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFullScreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffullscreen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetrics(::core::mem::transmute_copy(&ppos), ::core::mem::transmute_copy(&psize), ::core::mem::transmute_copy(&pscalex), ::core::mem::transmute_copy(&pscaley)).into()
        }
        unsafe extern "system" fn GetFullScreenSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFullScreenSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSurfacePresenterFlip: CreateSurfacePresenterFlip::<Identity, Impl, OFFSET>,
            GetDeviceLuid: GetDeviceLuid::<Identity, Impl, OFFSET>,
            EnterFullScreen: EnterFullScreen::<Identity, Impl, OFFSET>,
            ExitFullScreen: ExitFullScreen::<Identity, Impl, OFFSET>,
            IsFullScreen: IsFullScreen::<Identity, Impl, OFFSET>,
            GetBoundingRect: GetBoundingRect::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            GetFullScreenSize: GetFullScreenSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlipSite as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IViewObjectPresentFlipSite2_Impl: Sized {
    fn GetRotationForCurrentOutput(&self) -> ::windows_core::Result<super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::RuntimeName for IViewObjectPresentFlipSite2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IViewObjectPresentFlipSite2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite2_Impl, const OFFSET: isize>() -> IViewObjectPresentFlipSite2_Vtbl {
        unsafe extern "system" fn GetRotationForCurrentOutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewObjectPresentFlipSite2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdxgirotation: *mut super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRotationForCurrentOutput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdxgirotation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRotationForCurrentOutput: GetRotationForCurrentOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlipSite2 as ::windows_core::Interface>::IID
    }
}
pub trait IWebBrowserEventsService_Impl: Sized {
    fn FireBeforeNavigate2Event(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn FireNavigateComplete2Event(&self) -> ::windows_core::Result<()>;
    fn FireDownloadBeginEvent(&self) -> ::windows_core::Result<()>;
    fn FireDownloadCompleteEvent(&self) -> ::windows_core::Result<()>;
    fn FireDocumentCompleteEvent(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWebBrowserEventsService {}
impl IWebBrowserEventsService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: isize>() -> IWebBrowserEventsService_Vtbl {
        unsafe extern "system" fn FireBeforeNavigate2Event<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancel: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FireBeforeNavigate2Event() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireNavigateComplete2Event<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireNavigateComplete2Event().into()
        }
        unsafe extern "system" fn FireDownloadBeginEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireDownloadBeginEvent().into()
        }
        unsafe extern "system" fn FireDownloadCompleteEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireDownloadCompleteEvent().into()
        }
        unsafe extern "system" fn FireDocumentCompleteEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireDocumentCompleteEvent().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FireBeforeNavigate2Event: FireBeforeNavigate2Event::<Identity, Impl, OFFSET>,
            FireNavigateComplete2Event: FireNavigateComplete2Event::<Identity, Impl, OFFSET>,
            FireDownloadBeginEvent: FireDownloadBeginEvent::<Identity, Impl, OFFSET>,
            FireDownloadCompleteEvent: FireDownloadCompleteEvent::<Identity, Impl, OFFSET>,
            FireDocumentCompleteEvent: FireDocumentCompleteEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWebBrowserEventsService as ::windows_core::Interface>::IID
    }
}
pub trait IWebBrowserEventsUrlService_Impl: Sized {
    fn GetUrlForEvents(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IWebBrowserEventsUrlService {}
impl IWebBrowserEventsUrlService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebBrowserEventsUrlService_Impl, const OFFSET: isize>() -> IWebBrowserEventsUrlService_Vtbl {
        unsafe extern "system" fn GetUrlForEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebBrowserEventsUrlService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUrlForEvents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(purl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUrlForEvents: GetUrlForEvents::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWebBrowserEventsUrlService as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Iwfolders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn navigate(&self, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn navigateFrame(&self, bstrurl: &::windows_core::BSTR, bstrtargetframe: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn navigateNoSite(&self, bstrurl: &::windows_core::BSTR, bstrtargetframe: &::windows_core::BSTR, dwhwnd: u32, pwb: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Iwfolders {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Iwfolders_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Iwfolders_Impl, const OFFSET: isize>() -> Iwfolders_Vtbl {
        unsafe extern "system" fn navigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Iwfolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.navigate(::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn navigateFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Iwfolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtargetframe: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.navigateFrame(::core::mem::transmute(&bstrurl), ::core::mem::transmute(&bstrtargetframe)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn navigateNoSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Iwfolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtargetframe: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwhwnd: u32, pwb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.navigateNoSite(::core::mem::transmute(&bstrurl), ::core::mem::transmute(&bstrtargetframe), ::core::mem::transmute_copy(&dwhwnd), ::windows_core::from_raw_borrowed(&pwb)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            navigate: navigate::<Identity, Impl, OFFSET>,
            navigateFrame: navigateFrame::<Identity, Impl, OFFSET>,
            navigateNoSite: navigateNoSite::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Iwfolders as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
