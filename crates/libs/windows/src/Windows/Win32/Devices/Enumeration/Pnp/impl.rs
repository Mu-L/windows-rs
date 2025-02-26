pub trait IUPnPAddressFamilyControl_Impl: Sized {
    fn SetAddressFamily(&self, dwflags: i32) -> ::windows_core::Result<()>;
    fn GetAddressFamily(&self) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IUPnPAddressFamilyControl {}
impl IUPnPAddressFamilyControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>() -> IUPnPAddressFamilyControl_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAddressFamily(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetAddressFamily<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAddressFamily() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, Impl, OFFSET>,
            GetAddressFamily: GetAddressFamily::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPAddressFamilyControl as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPAsyncResult_Impl: Sized {
    fn AsyncOperationComplete(&self, ullrequestid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUPnPAsyncResult {}
impl IUPnPAsyncResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAsyncResult_Impl, const OFFSET: isize>() -> IUPnPAsyncResult_Vtbl {
        unsafe extern "system" fn AsyncOperationComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncOperationComplete(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPAsyncResult as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDescriptionDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn ReadyState(&self) -> ::windows_core::Result<i32>;
    fn Load(&self, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LoadAsync(&self, bstrurl: &::windows_core::BSTR, punkcallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn LoadResult(&self) -> ::windows_core::Result<i32>;
    fn Abort(&self) -> ::windows_core::Result<()>;
    fn RootDevice(&self) -> ::windows_core::Result<IUPnPDevice>;
    fn DeviceByUDN(&self, bstrudn: &::windows_core::BSTR) -> ::windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPDescriptionDocument {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPDescriptionDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>() -> IUPnPDescriptionDocument_Vtbl {
        unsafe extern "system" fn ReadyState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plreadystate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn LoadAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadAsync(::core::mem::transmute(&bstrurl), ::windows_core::from_raw_borrowed(&punkcallback)).into()
        }
        unsafe extern "system" fn LoadResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn RootDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudrootdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceByUDN<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppuddevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceByUDN(::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuddevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReadyState: ReadyState::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            LoadAsync: LoadAsync::<Identity, Impl, OFFSET>,
            LoadResult: LoadResult::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            RootDevice: RootDevice::<Identity, Impl, OFFSET>,
            DeviceByUDN: DeviceByUDN::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocument as ::windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPDescriptionDocumentCallback_Impl: Sized {
    fn LoadComplete(&self, hrloadresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUPnPDescriptionDocumentCallback {}
impl IUPnPDescriptionDocumentCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: isize>() -> IUPnPDescriptionDocumentCallback_Vtbl {
        unsafe extern "system" fn LoadComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrloadresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadComplete(::core::mem::transmute_copy(&hrloadresult)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadComplete: LoadComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocumentCallback as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDevice_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn IsRootDevice(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn RootDevice(&self) -> ::windows_core::Result<IUPnPDevice>;
    fn ParentDevice(&self) -> ::windows_core::Result<IUPnPDevice>;
    fn HasChildren(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Children(&self) -> ::windows_core::Result<IUPnPDevices>;
    fn UniqueDeviceName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Type(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PresentationURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ManufacturerName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ManufacturerURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ModelName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ModelNumber(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ModelURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UPC(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SerialNumber(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IconURL(&self, bstrencodingformat: &::windows_core::BSTR, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Services(&self) -> ::windows_core::Result<IUPnPServices>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPDevice {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPDevice_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>() -> IUPnPDevice_Vtbl {
        unsafe extern "system" fn IsRootDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudrootdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParentDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuddeviceparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueDeviceName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UniqueDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PresentationURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ManufacturerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ManufacturerURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModelURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UPC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UPC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrencodingformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IconURL(::core::mem::transmute(&bstrencodingformat), ::core::mem::transmute_copy(&lsizex), ::core::mem::transmute_copy(&lsizey), ::core::mem::transmute_copy(&lbitdepth)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstriconurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppusservices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Services() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppusservices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsRootDevice: IsRootDevice::<Identity, Impl, OFFSET>,
            RootDevice: RootDevice::<Identity, Impl, OFFSET>,
            ParentDevice: ParentDevice::<Identity, Impl, OFFSET>,
            HasChildren: HasChildren::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            UniqueDeviceName: UniqueDeviceName::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            PresentationURL: PresentationURL::<Identity, Impl, OFFSET>,
            ManufacturerName: ManufacturerName::<Identity, Impl, OFFSET>,
            ManufacturerURL: ManufacturerURL::<Identity, Impl, OFFSET>,
            ModelName: ModelName::<Identity, Impl, OFFSET>,
            ModelNumber: ModelNumber::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            ModelURL: ModelURL::<Identity, Impl, OFFSET>,
            UPC: UPC::<Identity, Impl, OFFSET>,
            SerialNumber: SerialNumber::<Identity, Impl, OFFSET>,
            IconURL: IconURL::<Identity, Impl, OFFSET>,
            Services: Services::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDevice as ::windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceControl_Impl: Sized {
    fn Initialize(&self, bstrxmldesc: &::windows_core::BSTR, bstrdeviceidentifier: &::windows_core::BSTR, bstrinitstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetServiceObject(&self, bstrudn: &::windows_core::BSTR, bstrserviceid: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUPnPDeviceControl {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: isize>() -> IUPnPDeviceControl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrinitstring)).into()
        }
        unsafe extern "system" fn GetServiceObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrserviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdispservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceObject(::core::mem::transmute(&bstrudn), ::core::mem::transmute(&bstrserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetServiceObject: GetServiceObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceControl as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPDeviceControlHttpHeaders_Impl: Sized {
    fn GetAdditionalResponseHeaders(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IUPnPDeviceControlHttpHeaders {}
impl IUPnPDeviceControlHttpHeaders_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: isize>() -> IUPnPDeviceControlHttpHeaders_Vtbl {
        unsafe extern "system" fn GetAdditionalResponseHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAdditionalResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrhttpresponseheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAdditionalResponseHeaders: GetAdditionalResponseHeaders::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceControlHttpHeaders as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPDeviceDocumentAccess_Impl: Sized {
    fn GetDocumentURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IUPnPDeviceDocumentAccess {}
impl IUPnPDeviceDocumentAccess_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceDocumentAccess_Impl, const OFFSET: isize>() -> IUPnPDeviceDocumentAccess_Vtbl {
        unsafe extern "system" fn GetDocumentURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDocumentURL: GetDocumentURL::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccess as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPDeviceDocumentAccessEx_Impl: Sized {
    fn GetDocument(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IUPnPDeviceDocumentAccessEx {}
impl IUPnPDeviceDocumentAccessEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: isize>() -> IUPnPDeviceDocumentAccessEx_Vtbl {
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDocument: GetDocument::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccessEx as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDeviceFinder_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn FindByType(&self, bstrtypeuri: &::windows_core::BSTR, dwflags: u32) -> ::windows_core::Result<IUPnPDevices>;
    fn CreateAsyncFind(&self, bstrtypeuri: &::windows_core::BSTR, dwflags: u32, punkdevicefindercallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<i32>;
    fn StartAsyncFind(&self, lfinddata: i32) -> ::windows_core::Result<()>;
    fn CancelAsyncFind(&self, lfinddata: i32) -> ::windows_core::Result<()>;
    fn FindByUDN(&self, bstrudn: &::windows_core::BSTR) -> ::windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPDeviceFinder {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPDeviceFinder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>() -> IUPnPDeviceFinder_Vtbl {
        unsafe extern "system" fn FindByType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32, pdevices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindByType(::core::mem::transmute(&bstrtypeuri), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsyncFind<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAsyncFind(::core::mem::transmute(&bstrtypeuri), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&punkdevicefindercallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfinddata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsyncFind<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartAsyncFind(::core::mem::transmute_copy(&lfinddata)).into()
        }
        unsafe extern "system" fn CancelAsyncFind<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncFind(::core::mem::transmute_copy(&lfinddata)).into()
        }
        unsafe extern "system" fn FindByUDN<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindByUDN(::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FindByType: FindByType::<Identity, Impl, OFFSET>,
            CreateAsyncFind: CreateAsyncFind::<Identity, Impl, OFFSET>,
            StartAsyncFind: StartAsyncFind::<Identity, Impl, OFFSET>,
            CancelAsyncFind: CancelAsyncFind::<Identity, Impl, OFFSET>,
            FindByUDN: FindByUDN::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinder as ::windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderAddCallbackWithInterface_Impl: Sized {
    fn DeviceAddedWithInterface(&self, lfinddata: i32, pdevice: ::core::option::Option<&IUPnPDevice>, pguidinterface: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUPnPDeviceFinderAddCallbackWithInterface {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: isize>() -> IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
        unsafe extern "system" fn DeviceAddedWithInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void, pguidinterface: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceAddedWithInterface(::core::mem::transmute_copy(&lfinddata), ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pguidinterface)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DeviceAddedWithInterface: DeviceAddedWithInterface::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderAddCallbackWithInterface as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderCallback_Impl: Sized {
    fn DeviceAdded(&self, lfinddata: i32, pdevice: ::core::option::Option<&IUPnPDevice>) -> ::windows_core::Result<()>;
    fn DeviceRemoved(&self, lfinddata: i32, bstrudn: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SearchComplete(&self, lfinddata: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IUPnPDeviceFinderCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>() -> IUPnPDeviceFinderCallback_Vtbl {
        unsafe extern "system" fn DeviceAdded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceAdded(::core::mem::transmute_copy(&lfinddata), ::windows_core::from_raw_borrowed(&pdevice)).into()
        }
        unsafe extern "system" fn DeviceRemoved<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceRemoved(::core::mem::transmute_copy(&lfinddata), ::core::mem::transmute(&bstrudn)).into()
        }
        unsafe extern "system" fn SearchComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SearchComplete(::core::mem::transmute_copy(&lfinddata)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceAdded: DeviceAdded::<Identity, Impl, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, Impl, OFFSET>,
            SearchComplete: SearchComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderCallback as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPDeviceProvider_Impl: Sized {
    fn Start(&self, bstrinitstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUPnPDeviceProvider {}
impl IUPnPDeviceProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: isize>() -> IUPnPDeviceProvider_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute(&bstrinitstring)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Start: Start::<Identity, Impl, OFFSET>, Stop: Stop::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDevices_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(&self, bstrudn: &::windows_core::BSTR) -> ::windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPDevices {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPDevices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: isize>() -> IUPnPDevices_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPDevices as ::windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPEventSink_Impl: Sized {
    fn OnStateChanged(&self, cchanges: u32, rgdispidchanges: *const i32) -> ::windows_core::Result<()>;
    fn OnStateChangedSafe(&self, varsadispidchanges: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPEventSink {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: isize>() -> IUPnPEventSink_Vtbl {
        unsafe extern "system" fn OnStateChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStateChanged(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&rgdispidchanges)).into()
        }
        unsafe extern "system" fn OnStateChangedSafe<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsadispidchanges: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStateChangedSafe(::core::mem::transmute(&varsadispidchanges)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnStateChangedSafe: OnStateChangedSafe::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPEventSink as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPEventSource_Impl: Sized {
    fn Advise(&self, pessubscriber: ::core::option::Option<&IUPnPEventSink>) -> ::windows_core::Result<()>;
    fn Unadvise(&self, pessubscriber: ::core::option::Option<&IUPnPEventSink>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUPnPEventSource {}
impl IUPnPEventSource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: isize>() -> IUPnPEventSource_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Advise(::windows_core::from_raw_borrowed(&pessubscriber)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::windows_core::from_raw_borrowed(&pessubscriber)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPEventSource as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPHttpHeaderControl_Impl: Sized {
    fn AddRequestHeaders(&self, bstrhttpheaders: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUPnPHttpHeaderControl {}
impl IUPnPHttpHeaderControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPHttpHeaderControl_Impl, const OFFSET: isize>() -> IUPnPHttpHeaderControl_Vtbl {
        unsafe extern "system" fn AddRequestHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPHttpHeaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRequestHeaders(::core::mem::transmute(&bstrhttpheaders)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddRequestHeaders: AddRequestHeaders::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPHttpHeaderControl as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPRegistrar_Impl: Sized {
    fn RegisterDevice(&self, bstrxmldesc: &::windows_core::BSTR, bstrprogiddevicecontrolclass: &::windows_core::BSTR, bstrinitstring: &::windows_core::BSTR, bstrcontainerid: &::windows_core::BSTR, bstrresourcepath: &::windows_core::BSTR, nlifetime: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RegisterRunningDevice(&self, bstrxmldesc: &::windows_core::BSTR, punkdevicecontrol: ::core::option::Option<&::windows_core::IUnknown>, bstrinitstring: &::windows_core::BSTR, bstrresourcepath: &::windows_core::BSTR, nlifetime: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RegisterDeviceProvider(&self, bstrprovidername: &::windows_core::BSTR, bstrprogidproviderclass: &::windows_core::BSTR, bstrinitstring: &::windows_core::BSTR, bstrcontainerid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetUniqueDeviceName(&self, bstrdeviceidentifier: &::windows_core::BSTR, bstrtemplateudn: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UnregisterDevice(&self, bstrdeviceidentifier: &::windows_core::BSTR, fpermanent: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UnregisterDeviceProvider(&self, bstrprovidername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUPnPRegistrar {}
impl IUPnPRegistrar_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>() -> IUPnPRegistrar_Vtbl {
        unsafe extern "system" fn RegisterDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogiddevicecontrolclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterDevice(::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrprogiddevicecontrolclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdeviceidentifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRunningDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterRunningDevice(::core::mem::transmute(&bstrxmldesc), ::windows_core::from_raw_borrowed(&punkdevicecontrol), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdeviceidentifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogidproviderclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterDeviceProvider(::core::mem::transmute(&bstrprovidername), ::core::mem::transmute(&bstrprogidproviderclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid)).into()
        }
        unsafe extern "system" fn GetUniqueDeviceName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtemplateudn: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrudn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUniqueDeviceName(::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrtemplateudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrudn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDevice(::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute_copy(&fpermanent)).into()
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDeviceProvider(::core::mem::transmute(&bstrprovidername)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDevice: RegisterDevice::<Identity, Impl, OFFSET>,
            RegisterRunningDevice: RegisterRunningDevice::<Identity, Impl, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, Impl, OFFSET>,
            GetUniqueDeviceName: GetUniqueDeviceName::<Identity, Impl, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, Impl, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPRegistrar as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPRemoteEndpointInfo_Impl: Sized {
    fn GetDwordValue(&self, bstrvaluename: &::windows_core::BSTR) -> ::windows_core::Result<u32>;
    fn GetStringValue(&self, bstrvaluename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetGuidValue(&self, bstrvaluename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for IUPnPRemoteEndpointInfo {}
impl IUPnPRemoteEndpointInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>() -> IUPnPRemoteEndpointInfo_Vtbl {
        unsafe extern "system" fn GetDwordValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwvalue: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDwordValue(::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringValue(::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuidValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pguidvalue: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGuidValue(::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDwordValue: GetDwordValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPRemoteEndpointInfo as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPReregistrar_Impl: Sized {
    fn ReregisterDevice(&self, bstrdeviceidentifier: &::windows_core::BSTR, bstrxmldesc: &::windows_core::BSTR, bstrprogiddevicecontrolclass: &::windows_core::BSTR, bstrinitstring: &::windows_core::BSTR, bstrcontainerid: &::windows_core::BSTR, bstrresourcepath: &::windows_core::BSTR, nlifetime: i32) -> ::windows_core::Result<()>;
    fn ReregisterRunningDevice(&self, bstrdeviceidentifier: &::windows_core::BSTR, bstrxmldesc: &::windows_core::BSTR, punkdevicecontrol: ::core::option::Option<&::windows_core::IUnknown>, bstrinitstring: &::windows_core::BSTR, bstrresourcepath: &::windows_core::BSTR, nlifetime: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUPnPReregistrar {}
impl IUPnPReregistrar_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: isize>() -> IUPnPReregistrar_Vtbl {
        unsafe extern "system" fn ReregisterDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogiddevicecontrolclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcontainerid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReregisterDevice(::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrprogiddevicecontrolclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)).into()
        }
        unsafe extern "system" fn ReregisterRunningDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrxmldesc: ::std::mem::MaybeUninit<::windows_core::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresourcepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, nlifetime: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReregisterRunningDevice(::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrxmldesc), ::windows_core::from_raw_borrowed(&punkdevicecontrol), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReregisterDevice: ReregisterDevice::<Identity, Impl, OFFSET>,
            ReregisterRunningDevice: ReregisterRunningDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPReregistrar as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPService_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn QueryStateVariable(&self, bstrvariablename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn InvokeAction(&self, bstractionname: &::windows_core::BSTR, vinactionargs: &super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ServiceTypeIdentifier(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddCallback(&self, punkcallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastTransportStatus(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPService {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>() -> IUPnPService_Vtbl {
        unsafe extern "system" fn QueryStateVariable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryStateVariable(::core::mem::transmute(&bstrvariablename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstractionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vinactionargs: super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeAction(::core::mem::transmute(&bstractionname), ::core::mem::transmute(&vinactionargs), ::core::mem::transmute_copy(&pvoutactionargs), ::core::mem::transmute_copy(&pvretval)).into()
        }
        unsafe extern "system" fn ServiceTypeIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceTypeIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddCallback(::windows_core::from_raw_borrowed(&punkcallback)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTransportStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastTransportStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryStateVariable: QueryStateVariable::<Identity, Impl, OFFSET>,
            InvokeAction: InvokeAction::<Identity, Impl, OFFSET>,
            ServiceTypeIdentifier: ServiceTypeIdentifier::<Identity, Impl, OFFSET>,
            AddCallback: AddCallback::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            LastTransportStatus: LastTransportStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPService as ::windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServiceAsync_Impl: Sized {
    fn BeginInvokeAction(&self, bstractionname: &::windows_core::BSTR, vinactionargs: &super::super::super::System::Variant::VARIANT, pasyncresult: ::core::option::Option<&IUPnPAsyncResult>) -> ::windows_core::Result<u64>;
    fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn BeginQueryStateVariable(&self, bstrvariablename: &::windows_core::BSTR, pasyncresult: ::core::option::Option<&IUPnPAsyncResult>) -> ::windows_core::Result<u64>;
    fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn BeginSubscribeToEvents(&self, punkcallback: ::core::option::Option<&::windows_core::IUnknown>, pasyncresult: ::core::option::Option<&IUPnPAsyncResult>) -> ::windows_core::Result<u64>;
    fn EndSubscribeToEvents(&self, ullrequestid: u64) -> ::windows_core::Result<()>;
    fn BeginSCPDDownload(&self, pasyncresult: ::core::option::Option<&IUPnPAsyncResult>) -> ::windows_core::Result<u64>;
    fn EndSCPDDownload(&self, ullrequestid: u64) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CancelAsyncOperation(&self, ullrequestid: u64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPServiceAsync {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPServiceAsync_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>() -> IUPnPServiceAsync_Vtbl {
        unsafe extern "system" fn BeginInvokeAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstractionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vinactionargs: super::super::super::System::Variant::VARIANT, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginInvokeAction(::core::mem::transmute(&bstractionname), ::core::mem::transmute(&vinactionargs), ::windows_core::from_raw_borrowed(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInvokeAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndInvokeAction(::core::mem::transmute_copy(&ullrequestid), ::core::mem::transmute_copy(&pvoutactionargs), ::core::mem::transmute_copy(&pvretval)).into()
        }
        unsafe extern "system" fn BeginQueryStateVariable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginQueryStateVariable(::core::mem::transmute(&bstrvariablename), ::windows_core::from_raw_borrowed(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQueryStateVariable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndQueryStateVariable(::core::mem::transmute_copy(&ullrequestid), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn BeginSubscribeToEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSubscribeToEvents(::windows_core::from_raw_borrowed(&punkcallback), ::windows_core::from_raw_borrowed(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSubscribeToEvents(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        unsafe extern "system" fn BeginSCPDDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSCPDDownload(::windows_core::from_raw_borrowed(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSCPDDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndSCPDDownload(::core::mem::transmute_copy(&ullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrscpddoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncOperation(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginInvokeAction: BeginInvokeAction::<Identity, Impl, OFFSET>,
            EndInvokeAction: EndInvokeAction::<Identity, Impl, OFFSET>,
            BeginQueryStateVariable: BeginQueryStateVariable::<Identity, Impl, OFFSET>,
            EndQueryStateVariable: EndQueryStateVariable::<Identity, Impl, OFFSET>,
            BeginSubscribeToEvents: BeginSubscribeToEvents::<Identity, Impl, OFFSET>,
            EndSubscribeToEvents: EndSubscribeToEvents::<Identity, Impl, OFFSET>,
            BeginSCPDDownload: BeginSCPDDownload::<Identity, Impl, OFFSET>,
            EndSCPDDownload: EndSCPDDownload::<Identity, Impl, OFFSET>,
            CancelAsyncOperation: CancelAsyncOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPServiceAsync as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServiceCallback_Impl: Sized {
    fn StateVariableChanged(&self, pus: ::core::option::Option<&IUPnPService>, pcwszstatevarname: &::windows_core::PCWSTR, vavalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ServiceInstanceDied(&self, pus: ::core::option::Option<&IUPnPService>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPServiceCallback {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPServiceCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: isize>() -> IUPnPServiceCallback_Vtbl {
        unsafe extern "system" fn StateVariableChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void, pcwszstatevarname: ::windows_core::PCWSTR, vavalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StateVariableChanged(::windows_core::from_raw_borrowed(&pus), ::core::mem::transmute(&pcwszstatevarname), ::core::mem::transmute(&vavalue)).into()
        }
        unsafe extern "system" fn ServiceInstanceDied<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ServiceInstanceDied(::windows_core::from_raw_borrowed(&pus)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StateVariableChanged: StateVariableChanged::<Identity, Impl, OFFSET>,
            ServiceInstanceDied: ServiceInstanceDied::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPServiceCallback as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPServiceDocumentAccess_Impl: Sized {
    fn GetDocumentURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDocument(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IUPnPServiceDocumentAccess {}
impl IUPnPServiceDocumentAccess_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>() -> IUPnPServiceDocumentAccess_Vtbl {
        unsafe extern "system" fn GetDocumentURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdoc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentURL: GetDocumentURL::<Identity, Impl, OFFSET>,
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPServiceDocumentAccess as ::windows_core::Interface>::IID
    }
}
pub trait IUPnPServiceEnumProperty_Impl: Sized {
    fn SetServiceEnumProperty(&self, dwmask: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUPnPServiceEnumProperty {}
impl IUPnPServiceEnumProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceEnumProperty_Impl, const OFFSET: isize>() -> IUPnPServiceEnumProperty_Vtbl {
        unsafe extern "system" fn SetServiceEnumProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceEnumProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceEnumProperty(::core::mem::transmute_copy(&dwmask)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetServiceEnumProperty: SetServiceEnumProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPServiceEnumProperty as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServices_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(&self, bstrserviceid: &::windows_core::BSTR) -> ::windows_core::Result<IUPnPService>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUPnPServices {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPServices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: isize>() -> IUPnPServices_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&bstrserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUPnPServices as ::windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
