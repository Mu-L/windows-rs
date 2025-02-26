pub trait IDDVideoPortContainer_Impl: Sized {
    fn CreateVideoPort(&self, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::core::option::Option<IDirectDrawVideoPort>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumVideoPorts(&self, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: LPDDENUMVIDEOCALLBACK) -> ::windows_core::Result<()>;
    fn GetVideoPortConnectInfo(&self, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows_core::Result<()>;
    fn QueryVideoPortStatus(&self, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDDVideoPortContainer {}
impl IDDVideoPortContainer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>() -> IDDVideoPortContainer_Vtbl {
        unsafe extern "system" fn CreateVideoPort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateVideoPort(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumVideoPorts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: LPDDENUMVIDEOCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumVideoPorts(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetVideoPortConnectInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVideoPortConnectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&pcinfo), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn QueryVideoPortStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryVideoPortStatus(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateVideoPort: CreateVideoPort::<Identity, Impl, OFFSET>,
            EnumVideoPorts: EnumVideoPorts::<Identity, Impl, OFFSET>,
            GetVideoPortConnectInfo: GetVideoPortConnectInfo::<Identity, Impl, OFFSET>,
            QueryVideoPortStatus: QueryVideoPortStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDDVideoPortContainer as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDraw_Impl: Sized {
    fn Compact(&self) -> ::windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC, param1: *mut ::core::option::Option<IDirectDrawSurface>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows_core::Result<()>;
    fn GetGDISurface(&self) -> ::windows_core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetScanLine(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVerticalBlankStatus(&self, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreDisplayMode(&self) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDraw {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDraw_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>() -> IDirectDraw_Vtbl {
        unsafe extern "system" fn Compact<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DuplicateSurface(::windows_core::from_raw_borrowed(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDraw as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDraw2_Impl: Sized {
    fn Compact(&self) -> ::windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC, param1: *mut ::core::option::Option<IDirectDrawSurface>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows_core::Result<()>;
    fn GetGDISurface(&self) -> ::windows_core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetScanLine(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVerticalBlankStatus(&self, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreDisplayMode(&self) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDraw2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDraw2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>() -> IDirectDraw2_Vtbl {
        unsafe extern "system" fn Compact<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DuplicateSurface(::windows_core::from_raw_borrowed(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDraw2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDraw4_Impl: Sized {
    fn Compact(&self) -> ::windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC2, param1: *mut ::core::option::Option<IDirectDrawSurface4>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface4>) -> ::windows_core::Result<IDirectDrawSurface4>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows_core::Result<()>;
    fn GetGDISurface(&self) -> ::windows_core::Result<IDirectDrawSurface4>;
    fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetScanLine(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVerticalBlankStatus(&self, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreDisplayMode(&self) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows_core::Result<()>;
    fn GetSurfaceFromDC(&self, param0: super::Gdi::HDC) -> ::windows_core::Result<IDirectDrawSurface4>;
    fn RestoreAllSurfaces(&self) -> ::windows_core::Result<()>;
    fn TestCooperativeLevel(&self) -> ::windows_core::Result<()>;
    fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDraw4 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDraw4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>() -> IDirectDraw4_Vtbl {
        unsafe extern "system" fn Compact<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DuplicateSurface(::windows_core::from_raw_borrowed(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSurfaceFromDC(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreAllSurfaces().into()
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TestCooperativeLevel().into()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceIdentifier(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, Impl, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, Impl, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, Impl, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDraw4 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDraw7_Impl: Sized {
    fn Compact(&self) -> ::windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC2, param1: *mut ::core::option::Option<IDirectDrawSurface7>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface7>) -> ::windows_core::Result<IDirectDrawSurface7>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows_core::Result<()>;
    fn GetGDISurface(&self) -> ::windows_core::Result<IDirectDrawSurface7>;
    fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetScanLine(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVerticalBlankStatus(&self, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreDisplayMode(&self) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows_core::Result<()>;
    fn GetSurfaceFromDC(&self, param0: super::Gdi::HDC) -> ::windows_core::Result<IDirectDrawSurface7>;
    fn RestoreAllSurfaces(&self) -> ::windows_core::Result<()>;
    fn TestCooperativeLevel(&self) -> ::windows_core::Result<()>;
    fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows_core::Result<()>;
    fn StartModeTest(&self, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn EvaluateMode(&self, param0: u32, param1: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDraw7 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDraw7_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>() -> IDirectDraw7_Vtbl {
        unsafe extern "system" fn Compact<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compact().into()
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateClipper(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePalette(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DuplicateSurface(::windows_core::from_raw_borrowed(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDisplayModes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlipToGDISurface().into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayMode(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFourCCCodes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGDISurface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMonitorFrequency(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScanLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVerticalBlankStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreDisplayMode().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForVerticalBlank(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAvailableVidMem(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSurfaceFromDC(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreAllSurfaces().into()
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TestCooperativeLevel().into()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceIdentifier(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartModeTest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartModeTest(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EvaluateMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EvaluateMode(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, Impl, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, Impl, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, Impl, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, Impl, OFFSET>,
            StartModeTest: StartModeTest::<Identity, Impl, OFFSET>,
            EvaluateMode: EvaluateMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDraw7 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawClipper_Impl: Sized {
    fn GetClipList(&self, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows_core::Result<()>;
    fn GetHWnd(&self, param0: *mut super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: ::core::option::Option<&IDirectDraw>, param1: u32) -> ::windows_core::Result<()>;
    fn IsClipListChanged(&self, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetClipList(&self, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows_core::Result<()>;
    fn SetHWnd(&self, param0: u32, param1: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDrawClipper {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawClipper_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>() -> IDirectDrawClipper_Vtbl {
        unsafe extern "system" fn GetClipList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClipList(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetHWnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHWnd(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsClipListChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsClipListChanged(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetClipList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipList(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetHWnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHWnd(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClipList: GetClipList::<Identity, Impl, OFFSET>,
            GetHWnd: GetHWnd::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsClipListChanged: IsClipListChanged::<Identity, Impl, OFFSET>,
            SetClipList: SetClipList::<Identity, Impl, OFFSET>,
            SetHWnd: SetHWnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawClipper as ::windows_core::Interface>::IID
    }
}
pub trait IDirectDrawColorControl_Impl: Sized {
    fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows_core::Result<()>;
    fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectDrawColorControl {}
impl IDirectDrawColorControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawColorControl_Impl, const OFFSET: isize>() -> IDirectDrawColorControl_Vtbl {
        unsafe extern "system" fn GetColorControls<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawColorControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetColorControls<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawColorControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColorControls: GetColorControls::<Identity, Impl, OFFSET>,
            SetColorControls: SetColorControls::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawColorControl as ::windows_core::Interface>::IID
    }
}
pub trait IDirectDrawGammaControl_Impl: Sized {
    fn GetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows_core::Result<()>;
    fn SetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectDrawGammaControl {}
impl IDirectDrawGammaControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawGammaControl_Impl, const OFFSET: isize>() -> IDirectDrawGammaControl_Vtbl {
        unsafe extern "system" fn GetGammaRamp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawGammaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGammaRamp(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetGammaRamp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawGammaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGammaRamp(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGammaRamp: GetGammaRamp::<Identity, Impl, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawGammaControl as ::windows_core::Interface>::IID
    }
}
pub trait IDirectDrawKernel_Impl: Sized {
    fn GetCaps(&self, param0: *mut DDKERNELCAPS) -> ::windows_core::Result<()>;
    fn GetKernelHandle(&self, param0: *mut usize) -> ::windows_core::Result<()>;
    fn ReleaseKernelHandle(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectDrawKernel {}
impl IDirectDrawKernel_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawKernel_Impl, const OFFSET: isize>() -> IDirectDrawKernel_Vtbl {
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDKERNELCAPS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetKernelHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKernelHandle(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseKernelHandle().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetKernelHandle: GetKernelHandle::<Identity, Impl, OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawKernel as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawPalette_Impl: Sized {
    fn GetCaps(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetEntries(&self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: ::core::option::Option<&IDirectDraw>, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::Result<()>;
    fn SetEntries(&self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDrawPalette {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawPalette_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>() -> IDirectDrawPalette_Vtbl {
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetEntries<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEntries(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetEntries<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEntries(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetEntries: GetEntries::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetEntries: SetEntries::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawPalette as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface_Impl: Sized {
    fn AddAttachedSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn Flip(&self, param0: ::core::option::Option<&IDirectDrawSurface>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface>) -> ::windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows_core::Result<()>;
    fn GetClipper(&self) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(&self) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn IsLost(&self) -> ::windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(&self) -> ::windows_core::Result<()>;
    fn SetClipper(&self, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(&self, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(&self, param0: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDrawSurface {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>() -> IDirectDrawSurface_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAttachedSurface(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Blt(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flip(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipper(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPalette(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlay(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface2_Impl: Sized {
    fn AddAttachedSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface2>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface2>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface2>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface2>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn Flip(&self, param0: ::core::option::Option<&IDirectDrawSurface2>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface2>) -> ::windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows_core::Result<()>;
    fn GetClipper(&self) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(&self) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn IsLost(&self) -> ::windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(&self) -> ::windows_core::Result<()>;
    fn SetClipper(&self, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(&self, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(&self, param0: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface2>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface2>) -> ::windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> ::windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDrawSurface2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>() -> IDirectDrawSurface2_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAttachedSurface(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Blt(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flip(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipper(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPalette(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlay(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface3_Impl: Sized {
    fn AddAttachedSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface3>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface3>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface3>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface3>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn Flip(&self, param0: ::core::option::Option<&IDirectDrawSurface3>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface3>) -> ::windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows_core::Result<()>;
    fn GetClipper(&self) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(&self) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn IsLost(&self) -> ::windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(&self) -> ::windows_core::Result<()>;
    fn SetClipper(&self, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(&self, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(&self, param0: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface3>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface3>) -> ::windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> ::windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> ::windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDrawSurface3 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>() -> IDirectDrawSurface3_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAttachedSurface(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Blt(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flip(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipper(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPalette(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlay(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface3 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface4_Impl: Sized {
    fn AddAttachedSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface4>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface4>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface4>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface4>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::Result<()>;
    fn Flip(&self, param0: ::core::option::Option<&IDirectDrawSurface4>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut ::core::option::Option<IDirectDrawSurface4>) -> ::windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS2) -> ::windows_core::Result<()>;
    fn GetClipper(&self) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(&self) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn IsLost(&self) -> ::windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(&self) -> ::windows_core::Result<()>;
    fn SetClipper(&self, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(&self, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(&self, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface4>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface4>) -> ::windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> ::windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> ::windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows_core::Result<()>;
    fn SetPrivateData(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows_core::Result<()>;
    fn GetPrivateData(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows_core::Result<()>;
    fn FreePrivateData(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetUniquenessValue(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn ChangeUniquenessValue(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDrawSurface4 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>() -> IDirectDrawSurface4_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAttachedSurface(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Blt(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flip(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipper(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPalette(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlay(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreePrivateData(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUniquenessValue(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeUniquenessValue().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, Impl, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface4 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface7_Impl: Sized {
    fn AddAttachedSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface7>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface7>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface7>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface7>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::Result<()>;
    fn Flip(&self, param0: ::core::option::Option<&IDirectDrawSurface7>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut ::core::option::Option<IDirectDrawSurface7>) -> ::windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS2) -> ::windows_core::Result<()>;
    fn GetClipper(&self) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(&self) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn Initialize(&self, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn IsLost(&self) -> ::windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(&self) -> ::windows_core::Result<()>;
    fn SetClipper(&self, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(&self, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(&self, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface7>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface7>) -> ::windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> ::windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> ::windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows_core::Result<()>;
    fn SetPrivateData(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows_core::Result<()>;
    fn GetPrivateData(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows_core::Result<()>;
    fn FreePrivateData(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetUniquenessValue(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn ChangeUniquenessValue(&self) -> ::windows_core::Result<()>;
    fn SetPriority(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetPriority(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn SetLOD(&self, param0: u32) -> ::windows_core::Result<()>;
    fn GetLOD(&self, param0: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IDirectDrawSurface7 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface7_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>() -> IDirectDrawSurface7_Vtbl {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAttachedSurface(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOverlayDirtyRect(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Blt(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltBatch(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BltFast(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAttachedSurface(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttachedSurfaces(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumOverlayZOrders(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flip(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttachedSurface(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBltStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipper() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFlipStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSurfaceDesc(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLost().into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDC(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipper(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverlayPosition(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPalette(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlay(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayDisplay(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOverlayZOrder(::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDDInterface(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PageLock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PageUnlock(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSurfaceDesc(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrivateData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreePrivateData(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUniquenessValue(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeUniquenessValue().into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPriority(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetLOD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLOD(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetLOD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLOD(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, Impl, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetLOD: SetLOD::<Identity, Impl, OFFSET>,
            GetLOD: GetLOD::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface7 as ::windows_core::Interface>::IID
    }
}
pub trait IDirectDrawSurfaceKernel_Impl: Sized {
    fn GetKernelHandle(&self, param0: *mut usize) -> ::windows_core::Result<()>;
    fn ReleaseKernelHandle(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectDrawSurfaceKernel {}
impl IDirectDrawSurfaceKernel_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: isize>() -> IDirectDrawSurfaceKernel_Vtbl {
        unsafe extern "system" fn GetKernelHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKernelHandle(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseKernelHandle().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetKernelHandle: GetKernelHandle::<Identity, Impl, OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurfaceKernel as ::windows_core::Interface>::IID
    }
}
pub trait IDirectDrawVideoPort_Impl: Sized {
    fn Flip(&self, param0: ::core::option::Option<&IDirectDrawSurface>, param1: u32) -> ::windows_core::Result<()>;
    fn GetBandwidthInfo(&self, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows_core::Result<()>;
    fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows_core::Result<()>;
    fn GetInputFormats(&self, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows_core::Result<()>;
    fn GetOutputFormats(&self, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows_core::Result<()>;
    fn GetFieldPolarity(&self, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetVideoLine(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVideoSignalStatus(&self, param0: *mut u32) -> ::windows_core::Result<()>;
    fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows_core::Result<()>;
    fn SetTargetSurface(&self, param0: ::core::option::Option<&IDirectDrawSurface>, param1: u32) -> ::windows_core::Result<()>;
    fn StartVideo(&self, param0: *mut DDVIDEOPORTINFO) -> ::windows_core::Result<()>;
    fn StopVideo(&self) -> ::windows_core::Result<()>;
    fn UpdateVideo(&self, param0: *mut DDVIDEOPORTINFO) -> ::windows_core::Result<()>;
    fn WaitForSync(&self, param0: u32, param1: u32, param2: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectDrawVideoPort {}
impl IDirectDrawVideoPort_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>() -> IDirectDrawVideoPort_Vtbl {
        unsafe extern "system" fn Flip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flip(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBandwidthInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBandwidthInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetColorControls<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetInputFormats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputFormats(::core::mem::transmute_copy(&lpnumformats), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetOutputFormats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputFormats(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&lpnumformats), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetFieldPolarity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFieldPolarity(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVideoLine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVideoLine(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVideoSignalStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVideoSignalStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetColorControls<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorControls(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetTargetSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTargetSurface(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartVideo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartVideo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn StopVideo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopVideo().into()
        }
        unsafe extern "system" fn UpdateVideo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateVideo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn WaitForSync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForSync(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetBandwidthInfo: GetBandwidthInfo::<Identity, Impl, OFFSET>,
            GetColorControls: GetColorControls::<Identity, Impl, OFFSET>,
            GetInputFormats: GetInputFormats::<Identity, Impl, OFFSET>,
            GetOutputFormats: GetOutputFormats::<Identity, Impl, OFFSET>,
            GetFieldPolarity: GetFieldPolarity::<Identity, Impl, OFFSET>,
            GetVideoLine: GetVideoLine::<Identity, Impl, OFFSET>,
            GetVideoSignalStatus: GetVideoSignalStatus::<Identity, Impl, OFFSET>,
            SetColorControls: SetColorControls::<Identity, Impl, OFFSET>,
            SetTargetSurface: SetTargetSurface::<Identity, Impl, OFFSET>,
            StartVideo: StartVideo::<Identity, Impl, OFFSET>,
            StopVideo: StopVideo::<Identity, Impl, OFFSET>,
            UpdateVideo: UpdateVideo::<Identity, Impl, OFFSET>,
            WaitForSync: WaitForSync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawVideoPort as ::windows_core::Interface>::IID
    }
}
pub trait IDirectDrawVideoPortNotify_Impl: Sized {
    fn AcquireNotification(&self, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows_core::Result<()>;
    fn ReleaseNotification(&self, param0: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectDrawVideoPortNotify {}
impl IDirectDrawVideoPortNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: isize>() -> IDirectDrawVideoPortNotify_Vtbl {
        unsafe extern "system" fn AcquireNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireNotification(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn ReleaseNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseNotification(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireNotification: AcquireNotification::<Identity, Impl, OFFSET>,
            ReleaseNotification: ReleaseNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectDrawVideoPortNotify as ::windows_core::Interface>::IID
    }
}
