#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInfo(&self) -> ::windows_core::Result<()>;
    fn SetInfo(&self) -> ::windows_core::Result<()>;
    fn Get(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Put(&self, bstrname: &::windows_core::BSTR, vprop: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetEx(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows_core::BSTR, vprop: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetInfoEx(&self, vproperties: &super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADs {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>() -> IADs_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Class<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Class() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Schema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Schema() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfo().into()
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInfo().into()
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Get(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Put(::core::mem::transmute(&bstrname), ::core::mem::transmute(&vprop)).into()
        }
        unsafe extern "system" fn GetEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEx(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutEx(::core::mem::transmute_copy(&lncontrolcode), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&vprop)).into()
        }
        unsafe extern "system" fn GetInfoEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfoEx(::core::mem::transmute(&vproperties), ::core::mem::transmute_copy(&lnreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Class: Class::<Identity, Impl, OFFSET>,
            GUID: GUID::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Schema: Schema::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            GetEx: GetEx::<Identity, Impl, OFFSET>,
            PutEx: PutEx::<Identity, Impl, OFFSET>,
            GetInfoEx: GetInfoEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADs as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsADSystemInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ComputerName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SiteName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DomainShortName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DomainDNSName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ForestDNSName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PDCRoleOwner(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SchemaRoleOwner(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsNativeMode(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAnyDCName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDCSiteName(&self, szserver: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RefreshSchemaCache(&self) -> ::windows_core::Result<()>;
    fn GetTrees(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsADSystemInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsADSystemInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>() -> IADsADSystemInfo_Vtbl {
        unsafe extern "system" fn UserName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiteName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SiteName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainShortName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainShortName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainDNSName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainDNSName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForestDNSName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForestDNSName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PDCRoleOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PDCRoleOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SchemaRoleOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SchemaRoleOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNativeMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsNativeMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnyDCName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdcname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAnyDCName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdcname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDCSiteName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, pszsitename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDCSiteName(::core::mem::transmute(&szserver)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszsitename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshSchemaCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshSchemaCache().into()
        }
        unsafe extern "system" fn GetTrees<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtrees: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTrees() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvtrees, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserName: UserName::<Identity, Impl, OFFSET>,
            ComputerName: ComputerName::<Identity, Impl, OFFSET>,
            SiteName: SiteName::<Identity, Impl, OFFSET>,
            DomainShortName: DomainShortName::<Identity, Impl, OFFSET>,
            DomainDNSName: DomainDNSName::<Identity, Impl, OFFSET>,
            ForestDNSName: ForestDNSName::<Identity, Impl, OFFSET>,
            PDCRoleOwner: PDCRoleOwner::<Identity, Impl, OFFSET>,
            SchemaRoleOwner: SchemaRoleOwner::<Identity, Impl, OFFSET>,
            IsNativeMode: IsNativeMode::<Identity, Impl, OFFSET>,
            GetAnyDCName: GetAnyDCName::<Identity, Impl, OFFSET>,
            GetDCSiteName: GetDCSiteName::<Identity, Impl, OFFSET>,
            RefreshSchemaCache: RefreshSchemaCache::<Identity, Impl, OFFSET>,
            GetTrees: GetTrees::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsADSystemInfo as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsAccessControlEntry_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccessMask(&self) -> ::windows_core::Result<i32>;
    fn SetAccessMask(&self, lnaccessmask: i32) -> ::windows_core::Result<()>;
    fn AceType(&self) -> ::windows_core::Result<i32>;
    fn SetAceType(&self, lnacetype: i32) -> ::windows_core::Result<()>;
    fn AceFlags(&self) -> ::windows_core::Result<i32>;
    fn SetAceFlags(&self, lnaceflags: i32) -> ::windows_core::Result<()>;
    fn Flags(&self) -> ::windows_core::Result<i32>;
    fn SetFlags(&self, lnflags: i32) -> ::windows_core::Result<()>;
    fn ObjectType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetObjectType(&self, bstrobjecttype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InheritedObjectType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetInheritedObjectType(&self, bstrinheritedobjecttype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Trustee(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTrustee(&self, bstrtrustee: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsAccessControlEntry {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsAccessControlEntry_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>() -> IADsAccessControlEntry_Vtbl {
        unsafe extern "system" fn AccessMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessMask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaccessmask: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccessMask(::core::mem::transmute_copy(&lnaccessmask)).into()
        }
        unsafe extern "system" fn AceType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AceType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnacetype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAceType(::core::mem::transmute_copy(&lnacetype)).into()
        }
        unsafe extern "system" fn AceFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AceFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaceflags: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAceFlags(::core::mem::transmute_copy(&lnaceflags)).into()
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Flags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlags(::core::mem::transmute_copy(&lnflags)).into()
        }
        unsafe extern "system" fn ObjectType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjecttype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectType(::core::mem::transmute(&bstrobjecttype)).into()
        }
        unsafe extern "system" fn InheritedObjectType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InheritedObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInheritedObjectType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinheritedobjecttype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInheritedObjectType(::core::mem::transmute(&bstrinheritedobjecttype)).into()
        }
        unsafe extern "system" fn Trustee<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Trustee() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrustee<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrustee(::core::mem::transmute(&bstrtrustee)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccessMask: AccessMask::<Identity, Impl, OFFSET>,
            SetAccessMask: SetAccessMask::<Identity, Impl, OFFSET>,
            AceType: AceType::<Identity, Impl, OFFSET>,
            SetAceType: SetAceType::<Identity, Impl, OFFSET>,
            AceFlags: AceFlags::<Identity, Impl, OFFSET>,
            SetAceFlags: SetAceFlags::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ObjectType: ObjectType::<Identity, Impl, OFFSET>,
            SetObjectType: SetObjectType::<Identity, Impl, OFFSET>,
            InheritedObjectType: InheritedObjectType::<Identity, Impl, OFFSET>,
            SetInheritedObjectType: SetInheritedObjectType::<Identity, Impl, OFFSET>,
            Trustee: Trustee::<Identity, Impl, OFFSET>,
            SetTrustee: SetTrustee::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsAccessControlEntry as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsAccessControlList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AclRevision(&self) -> ::windows_core::Result<i32>;
    fn SetAclRevision(&self, lnaclrevision: i32) -> ::windows_core::Result<()>;
    fn AceCount(&self) -> ::windows_core::Result<i32>;
    fn SetAceCount(&self, lnacecount: i32) -> ::windows_core::Result<()>;
    fn AddAce(&self, paccesscontrolentry: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn RemoveAce(&self, paccesscontrolentry: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn CopyAccessList(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsAccessControlList {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsAccessControlList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>() -> IADsAccessControlList_Vtbl {
        unsafe extern "system" fn AclRevision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AclRevision() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAclRevision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaclrevision: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAclRevision(::core::mem::transmute_copy(&lnaclrevision)).into()
        }
        unsafe extern "system" fn AceCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AceCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnacecount: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAceCount(::core::mem::transmute_copy(&lnacecount)).into()
        }
        unsafe extern "system" fn AddAce<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAce(::windows_core::from_raw_borrowed(&paccesscontrolentry)).into()
        }
        unsafe extern "system" fn RemoveAce<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAce(::windows_core::from_raw_borrowed(&paccesscontrolentry)).into()
        }
        unsafe extern "system" fn CopyAccessList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccesscontrollist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyAccessList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccesscontrollist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AclRevision: AclRevision::<Identity, Impl, OFFSET>,
            SetAclRevision: SetAclRevision::<Identity, Impl, OFFSET>,
            AceCount: AceCount::<Identity, Impl, OFFSET>,
            SetAceCount: SetAceCount::<Identity, Impl, OFFSET>,
            AddAce: AddAce::<Identity, Impl, OFFSET>,
            RemoveAce: RemoveAce::<Identity, Impl, OFFSET>,
            CopyAccessList: CopyAccessList::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsAccessControlList as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsAcl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ProtectedAttrName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProtectedAttrName(&self, bstrprotectedattrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SubjectName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubjectName(&self, bstrsubjectname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Privileges(&self) -> ::windows_core::Result<i32>;
    fn SetPrivileges(&self, lnprivileges: i32) -> ::windows_core::Result<()>;
    fn CopyAcl(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsAcl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsAcl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>() -> IADsAcl_Vtbl {
        unsafe extern "system" fn ProtectedAttrName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProtectedAttrName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectedAttrName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotectedattrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProtectedAttrName(::core::mem::transmute(&bstrprotectedattrname)).into()
        }
        unsafe extern "system" fn SubjectName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubjectName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubjectName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubjectName(::core::mem::transmute(&bstrsubjectname)).into()
        }
        unsafe extern "system" fn Privileges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Privileges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivileges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnprivileges: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivileges(::core::mem::transmute_copy(&lnprivileges)).into()
        }
        unsafe extern "system" fn CopyAcl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyAcl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppacl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProtectedAttrName: ProtectedAttrName::<Identity, Impl, OFFSET>,
            SetProtectedAttrName: SetProtectedAttrName::<Identity, Impl, OFFSET>,
            SubjectName: SubjectName::<Identity, Impl, OFFSET>,
            SetSubjectName: SetSubjectName::<Identity, Impl, OFFSET>,
            Privileges: Privileges::<Identity, Impl, OFFSET>,
            SetPrivileges: SetPrivileges::<Identity, Impl, OFFSET>,
            CopyAcl: CopyAcl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsAcl as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IADsAggregatee_Impl: Sized {
    fn ConnectAsAggregatee(&self, pouterunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DisconnectAsAggregatee(&self) -> ::windows_core::Result<()>;
    fn RelinquishInterface(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreInterface(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IADsAggregatee {}
impl IADsAggregatee_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>() -> IADsAggregatee_Vtbl {
        unsafe extern "system" fn ConnectAsAggregatee<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pouterunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectAsAggregatee(::windows_core::from_raw_borrowed(&pouterunknown)).into()
        }
        unsafe extern "system" fn DisconnectAsAggregatee<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectAsAggregatee().into()
        }
        unsafe extern "system" fn RelinquishInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RelinquishInterface(::core::mem::transmute_copy(&riid)).into()
        }
        unsafe extern "system" fn RestoreInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreInterface(::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectAsAggregatee: ConnectAsAggregatee::<Identity, Impl, OFFSET>,
            DisconnectAsAggregatee: DisconnectAsAggregatee::<Identity, Impl, OFFSET>,
            RelinquishInterface: RelinquishInterface::<Identity, Impl, OFFSET>,
            RestoreInterface: RestoreInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsAggregatee as ::windows_core::Interface>::IID
    }
}
pub trait IADsAggregator_Impl: Sized {
    fn ConnectAsAggregator(&self, paggregatee: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DisconnectAsAggregator(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IADsAggregator {}
impl IADsAggregator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: isize>() -> IADsAggregator_Vtbl {
        unsafe extern "system" fn ConnectAsAggregator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregatee: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectAsAggregator(::windows_core::from_raw_borrowed(&paggregatee)).into()
        }
        unsafe extern "system" fn DisconnectAsAggregator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectAsAggregator().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectAsAggregator: ConnectAsAggregator::<Identity, Impl, OFFSET>,
            DisconnectAsAggregator: DisconnectAsAggregator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsAggregator as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsBackLink_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RemoteID(&self) -> ::windows_core::Result<i32>;
    fn SetRemoteID(&self, lnremoteid: i32) -> ::windows_core::Result<()>;
    fn ObjectName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsBackLink {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsBackLink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>() -> IADsBackLink_Vtbl {
        unsafe extern "system" fn RemoteID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoteID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnremoteid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRemoteID(::core::mem::transmute_copy(&lnremoteid)).into()
        }
        unsafe extern "system" fn ObjectName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectName(::core::mem::transmute(&bstrobjectname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RemoteID: RemoteID::<Identity, Impl, OFFSET>,
            SetRemoteID: SetRemoteID::<Identity, Impl, OFFSET>,
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsBackLink as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsCaseIgnoreList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CaseIgnoreList(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetCaseIgnoreList(&self, vcaseignorelist: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsCaseIgnoreList {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsCaseIgnoreList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>() -> IADsCaseIgnoreList_Vtbl {
        unsafe extern "system" fn CaseIgnoreList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CaseIgnoreList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseIgnoreList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcaseignorelist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCaseIgnoreList(::core::mem::transmute(&vcaseignorelist)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CaseIgnoreList: CaseIgnoreList::<Identity, Impl, OFFSET>,
            SetCaseIgnoreList: SetCaseIgnoreList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsCaseIgnoreList as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsClass_Impl: Sized + IADs_Impl {
    fn PrimaryInterface(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CLSID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCLSID(&self, bstrclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOID(&self, bstroid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Abstract(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAbstract(&self, fabstract: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Auxiliary(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAuxiliary(&self, fauxiliary: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn MandatoryProperties(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetMandatoryProperties(&self, vmandatoryproperties: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn OptionalProperties(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOptionalProperties(&self, voptionalproperties: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn NamingProperties(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetNamingProperties(&self, vnamingproperties: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DerivedFrom(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDerivedFrom(&self, vderivedfrom: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AuxDerivedFrom(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetAuxDerivedFrom(&self, vauxderivedfrom: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PossibleSuperiors(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPossibleSuperiors(&self, vpossiblesuperiors: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Containment(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetContainment(&self, vcontainment: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Container(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetContainer(&self, fcontainer: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn HelpFileName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHelpFileName(&self, bstrhelpfilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HelpFileContext(&self) -> ::windows_core::Result<i32>;
    fn SetHelpFileContext(&self, lnhelpfilecontext: i32) -> ::windows_core::Result<()>;
    fn Qualifiers(&self) -> ::windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsClass {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsClass_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>() -> IADsClass_Vtbl {
        unsafe extern "system" fn PrimaryInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrimaryInterface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLSID(::core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn OID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOID(::core::mem::transmute(&bstroid)).into()
        }
        unsafe extern "system" fn Abstract<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Abstract() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAbstract<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fabstract: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAbstract(::core::mem::transmute_copy(&fabstract)).into()
        }
        unsafe extern "system" fn Auxiliary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Auxiliary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuxiliary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fauxiliary: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuxiliary(::core::mem::transmute_copy(&fauxiliary)).into()
        }
        unsafe extern "system" fn MandatoryProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MandatoryProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMandatoryProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vmandatoryproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMandatoryProperties(::core::mem::transmute(&vmandatoryproperties)).into()
        }
        unsafe extern "system" fn OptionalProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OptionalProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptionalProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voptionalproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOptionalProperties(::core::mem::transmute(&voptionalproperties)).into()
        }
        unsafe extern "system" fn NamingProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NamingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamingProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnamingproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamingProperties(::core::mem::transmute(&vnamingproperties)).into()
        }
        unsafe extern "system" fn DerivedFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DerivedFrom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDerivedFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vderivedfrom: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDerivedFrom(::core::mem::transmute(&vderivedfrom)).into()
        }
        unsafe extern "system" fn AuxDerivedFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuxDerivedFrom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuxDerivedFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vauxderivedfrom: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuxDerivedFrom(::core::mem::transmute(&vauxderivedfrom)).into()
        }
        unsafe extern "system" fn PossibleSuperiors<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PossibleSuperiors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPossibleSuperiors<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpossiblesuperiors: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPossibleSuperiors(::core::mem::transmute(&vpossiblesuperiors)).into()
        }
        unsafe extern "system" fn Containment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Containment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcontainment: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContainment(::core::mem::transmute(&vcontainment)).into()
        }
        unsafe extern "system" fn Container<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Container() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcontainer: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContainer(::core::mem::transmute_copy(&fcontainer)).into()
        }
        unsafe extern "system" fn HelpFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HelpFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhelpfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpFileName(::core::mem::transmute(&bstrhelpfilename)).into()
        }
        unsafe extern "system" fn HelpFileContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HelpFileContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnhelpfilecontext: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHelpFileContext(::core::mem::transmute_copy(&lnhelpfilecontext)).into()
        }
        unsafe extern "system" fn Qualifiers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Qualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualifiers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrimaryInterface: PrimaryInterface::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
            SetCLSID: SetCLSID::<Identity, Impl, OFFSET>,
            OID: OID::<Identity, Impl, OFFSET>,
            SetOID: SetOID::<Identity, Impl, OFFSET>,
            Abstract: Abstract::<Identity, Impl, OFFSET>,
            SetAbstract: SetAbstract::<Identity, Impl, OFFSET>,
            Auxiliary: Auxiliary::<Identity, Impl, OFFSET>,
            SetAuxiliary: SetAuxiliary::<Identity, Impl, OFFSET>,
            MandatoryProperties: MandatoryProperties::<Identity, Impl, OFFSET>,
            SetMandatoryProperties: SetMandatoryProperties::<Identity, Impl, OFFSET>,
            OptionalProperties: OptionalProperties::<Identity, Impl, OFFSET>,
            SetOptionalProperties: SetOptionalProperties::<Identity, Impl, OFFSET>,
            NamingProperties: NamingProperties::<Identity, Impl, OFFSET>,
            SetNamingProperties: SetNamingProperties::<Identity, Impl, OFFSET>,
            DerivedFrom: DerivedFrom::<Identity, Impl, OFFSET>,
            SetDerivedFrom: SetDerivedFrom::<Identity, Impl, OFFSET>,
            AuxDerivedFrom: AuxDerivedFrom::<Identity, Impl, OFFSET>,
            SetAuxDerivedFrom: SetAuxDerivedFrom::<Identity, Impl, OFFSET>,
            PossibleSuperiors: PossibleSuperiors::<Identity, Impl, OFFSET>,
            SetPossibleSuperiors: SetPossibleSuperiors::<Identity, Impl, OFFSET>,
            Containment: Containment::<Identity, Impl, OFFSET>,
            SetContainment: SetContainment::<Identity, Impl, OFFSET>,
            Container: Container::<Identity, Impl, OFFSET>,
            SetContainer: SetContainer::<Identity, Impl, OFFSET>,
            HelpFileName: HelpFileName::<Identity, Impl, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, Impl, OFFSET>,
            HelpFileContext: HelpFileContext::<Identity, Impl, OFFSET>,
            SetHelpFileContext: SetHelpFileContext::<Identity, Impl, OFFSET>,
            Qualifiers: Qualifiers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsClass as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Add(&self, bstrname: &::windows_core::BSTR, vitem: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Remove(&self, bstritemtoberemoved: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetObject(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>() -> IADsCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vitem: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&bstrname), ::core::mem::transmute(&vitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&bstritemtoberemoved)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvitem: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObject(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsCollection as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsComputer_Impl: Sized + IADs_Impl {
    fn ComputerID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Site(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Location(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocation(&self, bstrlocation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PrimaryUser(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPrimaryUser(&self, bstrprimaryuser: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Owner(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOwner(&self, bstrowner: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Division(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDivision(&self, bstrdivision: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Department(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Role(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRole(&self, bstrrole: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OperatingSystem(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOperatingSystem(&self, bstroperatingsystem: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OperatingSystemVersion(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOperatingSystemVersion(&self, bstroperatingsystemversion: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Model(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetModel(&self, bstrmodel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Processor(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProcessor(&self, bstrprocessor: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ProcessorCount(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProcessorCount(&self, bstrprocessorcount: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MemorySize(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMemorySize(&self, bstrmemorysize: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StorageCapacity(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStorageCapacity(&self, bstrstoragecapacity: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NetAddresses(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetNetAddresses(&self, vnetaddresses: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsComputer {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsComputer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>() -> IADsComputer_Vtbl {
        unsafe extern "system" fn ComputerID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputerID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Site<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Site() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Location<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Location() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocation(::core::mem::transmute(&bstrlocation)).into()
        }
        unsafe extern "system" fn PrimaryUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrimaryUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimaryUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprimaryuser: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrimaryUser(::core::mem::transmute(&bstrprimaryuser)).into()
        }
        unsafe extern "system" fn Owner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Owner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrowner: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOwner(::core::mem::transmute(&bstrowner)).into()
        }
        unsafe extern "system" fn Division<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Division() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDivision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdivision: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDivision(::core::mem::transmute(&bstrdivision)).into()
        }
        unsafe extern "system" fn Department<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Department() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDepartment(::core::mem::transmute(&bstrdepartment)).into()
        }
        unsafe extern "system" fn Role<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Role() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRole(::core::mem::transmute(&bstrrole)).into()
        }
        unsafe extern "system" fn OperatingSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OperatingSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperatingSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperatingsystem: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOperatingSystem(::core::mem::transmute(&bstroperatingsystem)).into()
        }
        unsafe extern "system" fn OperatingSystemVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OperatingSystemVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperatingSystemVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperatingsystemversion: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOperatingSystemVersion(::core::mem::transmute(&bstroperatingsystemversion)).into()
        }
        unsafe extern "system" fn Model<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Model() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmodel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModel(::core::mem::transmute(&bstrmodel)).into()
        }
        unsafe extern "system" fn Processor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Processor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprocessor: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProcessor(::core::mem::transmute(&bstrprocessor)).into()
        }
        unsafe extern "system" fn ProcessorCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProcessorCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessorCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprocessorcount: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProcessorCount(::core::mem::transmute(&bstrprocessorcount)).into()
        }
        unsafe extern "system" fn MemorySize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MemorySize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMemorySize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmemorysize: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMemorySize(::core::mem::transmute(&bstrmemorysize)).into()
        }
        unsafe extern "system" fn StorageCapacity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StorageCapacity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageCapacity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstoragecapacity: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStorageCapacity(::core::mem::transmute(&bstrstoragecapacity)).into()
        }
        unsafe extern "system" fn NetAddresses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetAddresses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnetaddresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetAddresses(::core::mem::transmute(&vnetaddresses)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            ComputerID: ComputerID::<Identity, Impl, OFFSET>,
            Site: Site::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            SetLocation: SetLocation::<Identity, Impl, OFFSET>,
            PrimaryUser: PrimaryUser::<Identity, Impl, OFFSET>,
            SetPrimaryUser: SetPrimaryUser::<Identity, Impl, OFFSET>,
            Owner: Owner::<Identity, Impl, OFFSET>,
            SetOwner: SetOwner::<Identity, Impl, OFFSET>,
            Division: Division::<Identity, Impl, OFFSET>,
            SetDivision: SetDivision::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
            SetRole: SetRole::<Identity, Impl, OFFSET>,
            OperatingSystem: OperatingSystem::<Identity, Impl, OFFSET>,
            SetOperatingSystem: SetOperatingSystem::<Identity, Impl, OFFSET>,
            OperatingSystemVersion: OperatingSystemVersion::<Identity, Impl, OFFSET>,
            SetOperatingSystemVersion: SetOperatingSystemVersion::<Identity, Impl, OFFSET>,
            Model: Model::<Identity, Impl, OFFSET>,
            SetModel: SetModel::<Identity, Impl, OFFSET>,
            Processor: Processor::<Identity, Impl, OFFSET>,
            SetProcessor: SetProcessor::<Identity, Impl, OFFSET>,
            ProcessorCount: ProcessorCount::<Identity, Impl, OFFSET>,
            SetProcessorCount: SetProcessorCount::<Identity, Impl, OFFSET>,
            MemorySize: MemorySize::<Identity, Impl, OFFSET>,
            SetMemorySize: SetMemorySize::<Identity, Impl, OFFSET>,
            StorageCapacity: StorageCapacity::<Identity, Impl, OFFSET>,
            SetStorageCapacity: SetStorageCapacity::<Identity, Impl, OFFSET>,
            NetAddresses: NetAddresses::<Identity, Impl, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsComputer as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsComputerOperations_Impl: Sized + IADs_Impl {
    fn Status(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Shutdown(&self, breboot: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsComputerOperations {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsComputerOperations_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: isize>() -> IADsComputerOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breboot: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown(::core::mem::transmute_copy(&breboot)).into()
        }
        Self { base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(), Status: Status::<Identity, Impl, OFFSET>, Shutdown: Shutdown::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsComputerOperations as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsContainer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Filter(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetFilter(&self, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Hints(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetHints(&self, vhints: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetObject(&self, classname: &::windows_core::BSTR, relativename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Create(&self, classname: &::windows_core::BSTR, relativename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&self, bstrclassname: &::windows_core::BSTR, bstrrelativename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CopyHere(&self, sourcename: &::windows_core::BSTR, newname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn MoveHere(&self, sourcename: &::windows_core::BSTR, newname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsContainer {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsContainer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>() -> IADsContainer_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Filter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Filter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFilter(::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn Hints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Hints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhints: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHints(::core::mem::transmute(&vhints)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::std::mem::MaybeUninit<::windows_core::BSTR>, relativename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObject(::core::mem::transmute(&classname), ::core::mem::transmute(&relativename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::std::mem::MaybeUninit<::windows_core::BSTR>, relativename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&classname), ::core::mem::transmute(&relativename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclassname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrelativename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&bstrclassname), ::core::mem::transmute(&bstrrelativename)).into()
        }
        unsafe extern "system" fn CopyHere<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyHere(::core::mem::transmute(&sourcename), ::core::mem::transmute(&newname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveHere<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveHere(::core::mem::transmute(&sourcename), ::core::mem::transmute(&newname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Filter: Filter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
            Hints: Hints::<Identity, Impl, OFFSET>,
            SetHints: SetHints::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            CopyHere: CopyHere::<Identity, Impl, OFFSET>,
            MoveHere: MoveHere::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsContainer as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsDNWithBinary_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BinaryValue(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetBinaryValue(&self, vbinaryvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DNString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsDNWithBinary {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsDNWithBinary_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>() -> IADsDNWithBinary_Vtbl {
        unsafe extern "system" fn BinaryValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BinaryValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinaryValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbinaryvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBinaryValue(::core::mem::transmute(&vbinaryvalue)).into()
        }
        unsafe extern "system" fn DNString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DNString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDNString(::core::mem::transmute(&bstrdnstring)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BinaryValue: BinaryValue::<Identity, Impl, OFFSET>,
            SetBinaryValue: SetBinaryValue::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsDNWithBinary as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsDNWithString_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StringValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStringValue(&self, bstrstringvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DNString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsDNWithString {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsDNWithString_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>() -> IADsDNWithString_Vtbl {
        unsafe extern "system" fn StringValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StringValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstringvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStringValue(::core::mem::transmute(&bstrstringvalue)).into()
        }
        unsafe extern "system" fn DNString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DNString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDNString(::core::mem::transmute(&bstrdnstring)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StringValue: StringValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsDNWithString as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsDeleteOps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DeleteObject(&self, lnflags: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsDeleteOps {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsDeleteOps_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDeleteOps_Impl, const OFFSET: isize>() -> IADsDeleteOps_Vtbl {
        unsafe extern "system" fn DeleteObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDeleteOps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteObject(::core::mem::transmute_copy(&lnflags)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), DeleteObject: DeleteObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsDeleteOps as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsDomain_Impl: Sized + IADs_Impl {
    fn IsWorkgroup(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MinPasswordLength(&self) -> ::windows_core::Result<i32>;
    fn SetMinPasswordLength(&self, lnminpasswordlength: i32) -> ::windows_core::Result<()>;
    fn MinPasswordAge(&self) -> ::windows_core::Result<i32>;
    fn SetMinPasswordAge(&self, lnminpasswordage: i32) -> ::windows_core::Result<()>;
    fn MaxPasswordAge(&self) -> ::windows_core::Result<i32>;
    fn SetMaxPasswordAge(&self, lnmaxpasswordage: i32) -> ::windows_core::Result<()>;
    fn MaxBadPasswordsAllowed(&self) -> ::windows_core::Result<i32>;
    fn SetMaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> ::windows_core::Result<()>;
    fn PasswordHistoryLength(&self) -> ::windows_core::Result<i32>;
    fn SetPasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> ::windows_core::Result<()>;
    fn PasswordAttributes(&self) -> ::windows_core::Result<i32>;
    fn SetPasswordAttributes(&self, lnpasswordattributes: i32) -> ::windows_core::Result<()>;
    fn AutoUnlockInterval(&self) -> ::windows_core::Result<i32>;
    fn SetAutoUnlockInterval(&self, lnautounlockinterval: i32) -> ::windows_core::Result<()>;
    fn LockoutObservationInterval(&self) -> ::windows_core::Result<i32>;
    fn SetLockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsDomain {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsDomain_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>() -> IADsDomain_Vtbl {
        unsafe extern "system" fn IsWorkgroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWorkgroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPasswordLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinPasswordLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminpasswordlength: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinPasswordLength(::core::mem::transmute_copy(&lnminpasswordlength)).into()
        }
        unsafe extern "system" fn MinPasswordAge<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinPasswordAge() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordAge<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminpasswordage: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinPasswordAge(::core::mem::transmute_copy(&lnminpasswordage)).into()
        }
        unsafe extern "system" fn MaxPasswordAge<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxPasswordAge() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPasswordAge<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxpasswordage: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxPasswordAge(::core::mem::transmute_copy(&lnmaxpasswordage)).into()
        }
        unsafe extern "system" fn MaxBadPasswordsAllowed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxBadPasswordsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBadPasswordsAllowed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxBadPasswordsAllowed(::core::mem::transmute_copy(&lnmaxbadpasswordsallowed)).into()
        }
        unsafe extern "system" fn PasswordHistoryLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PasswordHistoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordHistoryLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordhistorylength: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPasswordHistoryLength(::core::mem::transmute_copy(&lnpasswordhistorylength)).into()
        }
        unsafe extern "system" fn PasswordAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PasswordAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordattributes: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPasswordAttributes(::core::mem::transmute_copy(&lnpasswordattributes)).into()
        }
        unsafe extern "system" fn AutoUnlockInterval<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoUnlockInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoUnlockInterval<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnautounlockinterval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoUnlockInterval(::core::mem::transmute_copy(&lnautounlockinterval)).into()
        }
        unsafe extern "system" fn LockoutObservationInterval<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LockoutObservationInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockoutObservationInterval<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlockoutobservationinterval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLockoutObservationInterval(::core::mem::transmute_copy(&lnlockoutobservationinterval)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsWorkgroup: IsWorkgroup::<Identity, Impl, OFFSET>,
            MinPasswordLength: MinPasswordLength::<Identity, Impl, OFFSET>,
            SetMinPasswordLength: SetMinPasswordLength::<Identity, Impl, OFFSET>,
            MinPasswordAge: MinPasswordAge::<Identity, Impl, OFFSET>,
            SetMinPasswordAge: SetMinPasswordAge::<Identity, Impl, OFFSET>,
            MaxPasswordAge: MaxPasswordAge::<Identity, Impl, OFFSET>,
            SetMaxPasswordAge: SetMaxPasswordAge::<Identity, Impl, OFFSET>,
            MaxBadPasswordsAllowed: MaxBadPasswordsAllowed::<Identity, Impl, OFFSET>,
            SetMaxBadPasswordsAllowed: SetMaxBadPasswordsAllowed::<Identity, Impl, OFFSET>,
            PasswordHistoryLength: PasswordHistoryLength::<Identity, Impl, OFFSET>,
            SetPasswordHistoryLength: SetPasswordHistoryLength::<Identity, Impl, OFFSET>,
            PasswordAttributes: PasswordAttributes::<Identity, Impl, OFFSET>,
            SetPasswordAttributes: SetPasswordAttributes::<Identity, Impl, OFFSET>,
            AutoUnlockInterval: AutoUnlockInterval::<Identity, Impl, OFFSET>,
            SetAutoUnlockInterval: SetAutoUnlockInterval::<Identity, Impl, OFFSET>,
            LockoutObservationInterval: LockoutObservationInterval::<Identity, Impl, OFFSET>,
            SetLockoutObservationInterval: SetLockoutObservationInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsDomain as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsEmail_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows_core::Result<i32>;
    fn SetType(&self, lntype: i32) -> ::windows_core::Result<()>;
    fn Address(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAddress(&self, bstraddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsEmail {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsEmail_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>() -> IADsEmail_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&lntype)).into()
        }
        unsafe extern "system" fn Address<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Address() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAddress(::core::mem::transmute(&bstraddress)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsEmail as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsExtension_Impl: Sized {
    fn Operate(&self, dwcode: u32, vardata1: &super::super::System::Variant::VARIANT, vardata2: &super::super::System::Variant::VARIANT, vardata3: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PrivateGetIDsOfNames(&self, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows_core::Result<i32>;
    fn PrivateInvoke(&self, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsExtension {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsExtension_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: isize>() -> IADsExtension_Vtbl {
        unsafe extern "system" fn Operate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcode: u32, vardata1: super::super::System::Variant::VARIANT, vardata2: super::super::System::Variant::VARIANT, vardata3: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Operate(::core::mem::transmute_copy(&dwcode), ::core::mem::transmute(&vardata1), ::core::mem::transmute(&vardata2), ::core::mem::transmute(&vardata3)).into()
        }
        unsafe extern "system" fn PrivateGetIDsOfNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateGetIDsOfNames(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rgdispid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateInvoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrivateInvoke(::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Operate: Operate::<Identity, Impl, OFFSET>,
            PrivateGetIDsOfNames: PrivateGetIDsOfNames::<Identity, Impl, OFFSET>,
            PrivateInvoke: PrivateInvoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsExtension as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsFaxNumber_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TelephoneNumber(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Parameters(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetParameters(&self, vparameters: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsFaxNumber {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsFaxNumber_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>() -> IADsFaxNumber_Vtbl {
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTelephoneNumber(::core::mem::transmute(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn Parameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vparameters: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute(&vparameters)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsFaxNumber as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsFileService_Impl: Sized + IADsService_Impl {
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxUserCount(&self) -> ::windows_core::Result<i32>;
    fn SetMaxUserCount(&self, lnmaxusercount: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsFileService {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsFileService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>() -> IADsFileService_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn MaxUserCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxUserCount(::core::mem::transmute_copy(&lnmaxusercount)).into()
        }
        Self {
            base__: IADsService_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, Impl, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsFileService as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID || iid == &<IADsService as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsFileServiceOperations_Impl: Sized + IADsServiceOperations_Impl {
    fn Sessions(&self) -> ::windows_core::Result<IADsCollection>;
    fn Resources(&self) -> ::windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsFileServiceOperations {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsFileServiceOperations_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>() -> IADsFileServiceOperations_Vtbl {
        unsafe extern "system" fn Sessions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsessions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sessions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsessions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Resources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IADsServiceOperations_Vtbl::new::<Identity, Impl, OFFSET>(),
            Sessions: Sessions::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsFileServiceOperations as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID || iid == &<IADsServiceOperations as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsFileShare_Impl: Sized + IADs_Impl {
    fn CurrentUserCount(&self) -> ::windows_core::Result<i32>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HostComputer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHostComputer(&self, bstrhostcomputer: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxUserCount(&self) -> ::windows_core::Result<i32>;
    fn SetMaxUserCount(&self, lnmaxusercount: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsFileShare {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsFileShare_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>() -> IADsFileShare_Vtbl {
        unsafe extern "system" fn CurrentUserCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn HostComputer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HostComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostComputer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHostComputer(::core::mem::transmute(&bstrhostcomputer)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPath(::core::mem::transmute(&bstrpath)).into()
        }
        unsafe extern "system" fn MaxUserCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxUserCount(::core::mem::transmute_copy(&lnmaxusercount)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentUserCount: CurrentUserCount::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            HostComputer: HostComputer::<Identity, Impl, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, Impl, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsFileShare as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsGroup_Impl: Sized + IADs_Impl {
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Members(&self) -> ::windows_core::Result<IADsMembers>;
    fn IsMember(&self, bstrmember: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(&self, bstrnewitem: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Remove(&self, bstritemtoberemoved: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsGroup {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsGroup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>() -> IADsGroup_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Members<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmembers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Members() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmembers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmember: ::std::mem::MaybeUninit<::windows_core::BSTR>, bmember: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMember(::core::mem::transmute(&bstrmember)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bmember, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewitem: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&bstrnewitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&bstritemtoberemoved)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            IsMember: IsMember::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsGroup as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsHold_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Amount(&self) -> ::windows_core::Result<i32>;
    fn SetAmount(&self, lnamount: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsHold {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsHold_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>() -> IADsHold_Vtbl {
        unsafe extern "system" fn ObjectName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectName(::core::mem::transmute(&bstrobjectname)).into()
        }
        unsafe extern "system" fn Amount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Amount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnamount: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAmount(::core::mem::transmute_copy(&lnamount)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            Amount: Amount::<Identity, Impl, OFFSET>,
            SetAmount: SetAmount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsHold as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsLargeInteger_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn HighPart(&self) -> ::windows_core::Result<i32>;
    fn SetHighPart(&self, lnhighpart: i32) -> ::windows_core::Result<()>;
    fn LowPart(&self) -> ::windows_core::Result<i32>;
    fn SetLowPart(&self, lnlowpart: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsLargeInteger {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsLargeInteger_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>() -> IADsLargeInteger_Vtbl {
        unsafe extern "system" fn HighPart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HighPart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighPart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnhighpart: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHighPart(::core::mem::transmute_copy(&lnhighpart)).into()
        }
        unsafe extern "system" fn LowPart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LowPart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowPart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlowpart: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLowPart(::core::mem::transmute_copy(&lnlowpart)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HighPart: HighPart::<Identity, Impl, OFFSET>,
            SetHighPart: SetHighPart::<Identity, Impl, OFFSET>,
            LowPart: LowPart::<Identity, Impl, OFFSET>,
            SetLowPart: SetLowPart::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsLargeInteger as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsLocality_Impl: Sized + IADs_Impl {
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalityName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PostalAddress(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SeeAlso(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsLocality {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsLocality_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>() -> IADsLocality_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalityName(::core::mem::transmute(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostalAddress(::core::mem::transmute(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSeeAlso(::core::mem::transmute(&vseealso)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsLocality as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsMembers_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Filter(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetFilter(&self, pvfilter: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsMembers {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsMembers_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>() -> IADsMembers_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Filter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Filter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFilter(::core::mem::transmute(&pvfilter)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Filter: Filter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsMembers as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsNameTranslate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetChaseReferral(&self, lnchasereferral: i32) -> ::windows_core::Result<()>;
    fn Init(&self, lnsettype: i32, bstradspath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InitEx(&self, lnsettype: i32, bstradspath: &::windows_core::BSTR, bstruserid: &::windows_core::BSTR, bstrdomain: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Set(&self, lnsettype: i32, bstradspath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Get(&self, lnformattype: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEx(&self, lnformattype: i32, pvar: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetEx(&self, lnformattype: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsNameTranslate {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsNameTranslate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>() -> IADsNameTranslate_Vtbl {
        unsafe extern "system" fn SetChaseReferral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnchasereferral: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChaseReferral(::core::mem::transmute_copy(&lnchasereferral)).into()
        }
        unsafe extern "system" fn Init<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute(&bstradspath)).into()
        }
        unsafe extern "system" fn InitEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstruserid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitEx(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute(&bstradspath), ::core::mem::transmute(&bstruserid), ::core::mem::transmute(&bstrdomain), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn Set<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute(&bstradspath)).into()
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Get(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEx(::core::mem::transmute_copy(&lnformattype), ::core::mem::transmute(&pvar)).into()
        }
        unsafe extern "system" fn GetEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEx(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetChaseReferral: SetChaseReferral::<Identity, Impl, OFFSET>,
            Init: Init::<Identity, Impl, OFFSET>,
            InitEx: InitEx::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            SetEx: SetEx::<Identity, Impl, OFFSET>,
            GetEx: GetEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsNameTranslate as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsNamespaces_Impl: Sized + IADs_Impl {
    fn DefaultContainer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDefaultContainer(&self, bstrdefaultcontainer: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsNamespaces {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsNamespaces_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: isize>() -> IADsNamespaces_Vtbl {
        unsafe extern "system" fn DefaultContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultcontainer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultContainer(::core::mem::transmute(&bstrdefaultcontainer)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            DefaultContainer: DefaultContainer::<Identity, Impl, OFFSET>,
            SetDefaultContainer: SetDefaultContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsNamespaces as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsNetAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddressType(&self) -> ::windows_core::Result<i32>;
    fn SetAddressType(&self, lnaddresstype: i32) -> ::windows_core::Result<()>;
    fn Address(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetAddress(&self, vaddress: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsNetAddress {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsNetAddress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>() -> IADsNetAddress_Vtbl {
        unsafe extern "system" fn AddressType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddressType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddressType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaddresstype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAddressType(::core::mem::transmute_copy(&lnaddresstype)).into()
        }
        unsafe extern "system" fn Address<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Address() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vaddress: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAddress(::core::mem::transmute(&vaddress)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddressType: AddressType::<Identity, Impl, OFFSET>,
            SetAddressType: SetAddressType::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsNetAddress as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsO_Impl: Sized + IADs_Impl {
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalityName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PostalAddress(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TelephoneNumber(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FaxNumber(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SeeAlso(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsO {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsO_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>() -> IADsO_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalityName(::core::mem::transmute(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostalAddress(::core::mem::transmute(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTelephoneNumber(::core::mem::transmute(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFaxNumber(::core::mem::transmute(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSeeAlso(::core::mem::transmute(&vseealso)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsO as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsOU_Impl: Sized + IADs_Impl {
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalityName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PostalAddress(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TelephoneNumber(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FaxNumber(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SeeAlso(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn BusinessCategory(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBusinessCategory(&self, bstrbusinesscategory: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsOU {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsOU_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>() -> IADsOU_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalityName(::core::mem::transmute(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostalAddress(::core::mem::transmute(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTelephoneNumber(::core::mem::transmute(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFaxNumber(::core::mem::transmute(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSeeAlso(::core::mem::transmute(&vseealso)).into()
        }
        unsafe extern "system" fn BusinessCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BusinessCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinesscategory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBusinessCategory(::core::mem::transmute(&bstrbusinesscategory)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
            BusinessCategory: BusinessCategory::<Identity, Impl, OFFSET>,
            SetBusinessCategory: SetBusinessCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsOU as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsObjectOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetOption(&self, lnoption: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOption(&self, lnoption: i32, vvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsObjectOptions {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsObjectOptions_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: isize>() -> IADsObjectOptions_Vtbl {
        unsafe extern "system" fn GetOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoption: i32, pvvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOption(::core::mem::transmute_copy(&lnoption)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoption: i32, vvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOption(::core::mem::transmute_copy(&lnoption), ::core::mem::transmute(&vvalue)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsObjectOptions as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsOctetList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OctetList(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOctetList(&self, voctetlist: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsOctetList {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsOctetList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: isize>() -> IADsOctetList_Vtbl {
        unsafe extern "system" fn OctetList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OctetList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOctetList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voctetlist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOctetList(::core::mem::transmute(&voctetlist)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OctetList: OctetList::<Identity, Impl, OFFSET>,
            SetOctetList: SetOctetList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsOctetList as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsOpenDSObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OpenDSObject(&self, lpszdnname: &::windows_core::BSTR, lpszusername: &::windows_core::BSTR, lpszpassword: &::windows_core::BSTR, lnreserved: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsOpenDSObject {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsOpenDSObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOpenDSObject_Impl, const OFFSET: isize>() -> IADsOpenDSObject_Vtbl {
        unsafe extern "system" fn OpenDSObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOpenDSObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszdnname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnreserved: i32, ppoledsobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenDSObject(::core::mem::transmute(&lpszdnname), ::core::mem::transmute(&lpszusername), ::core::mem::transmute(&lpszpassword), ::core::mem::transmute_copy(&lnreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoledsobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), OpenDSObject: OpenDSObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsOpenDSObject as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPath_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows_core::Result<i32>;
    fn SetType(&self, lntype: i32) -> ::windows_core::Result<()>;
    fn VolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVolumeName(&self, bstrvolumename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPath {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPath_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>() -> IADsPath_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&lntype)).into()
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvolumename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolumeName(::core::mem::transmute(&bstrvolumename)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPath(::core::mem::transmute(&bstrpath)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPath as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPathname_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Set(&self, bstradspath: &::windows_core::BSTR, lnsettype: i32) -> ::windows_core::Result<()>;
    fn SetDisplayType(&self, lndisplaytype: i32) -> ::windows_core::Result<()>;
    fn Retrieve(&self, lnformattype: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetNumElements(&self) -> ::windows_core::Result<i32>;
    fn GetElement(&self, lnelementindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddLeafElement(&self, bstrleafelement: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveLeafElement(&self) -> ::windows_core::Result<()>;
    fn CopyPath(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetEscapedElement(&self, lnreserved: i32, bstrinstr: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EscapedMode(&self) -> ::windows_core::Result<i32>;
    fn SetEscapedMode(&self, lnescapedmode: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPathname {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPathname_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>() -> IADsPathname_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnsettype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute(&bstradspath), ::core::mem::transmute_copy(&lnsettype)).into()
        }
        unsafe extern "system" fn SetDisplayType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lndisplaytype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayType(::core::mem::transmute_copy(&lndisplaytype)).into()
        }
        unsafe extern "system" fn Retrieve<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Retrieve(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumElements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnnumpathelements: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumElements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plnnumpathelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetElement(::core::mem::transmute_copy(&lnelementindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLeafElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrleafelement: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLeafElement(::core::mem::transmute(&bstrleafelement)).into()
        }
        unsafe extern "system" fn RemoveLeafElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLeafElement().into()
        }
        unsafe extern "system" fn CopyPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadspath: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEscapedElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreserved: i32, bstrinstr: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstroutstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEscapedElement(::core::mem::transmute_copy(&lnreserved), ::core::mem::transmute(&bstrinstr)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstroutstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapedMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EscapedMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEscapedMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnescapedmode: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEscapedMode(::core::mem::transmute_copy(&lnescapedmode)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Set: Set::<Identity, Impl, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, Impl, OFFSET>,
            Retrieve: Retrieve::<Identity, Impl, OFFSET>,
            GetNumElements: GetNumElements::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            AddLeafElement: AddLeafElement::<Identity, Impl, OFFSET>,
            RemoveLeafElement: RemoveLeafElement::<Identity, Impl, OFFSET>,
            CopyPath: CopyPath::<Identity, Impl, OFFSET>,
            GetEscapedElement: GetEscapedElement::<Identity, Impl, OFFSET>,
            EscapedMode: EscapedMode::<Identity, Impl, OFFSET>,
            SetEscapedMode: SetEscapedMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPathname as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPostalAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PostalAddress(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPostalAddress(&self, vpostaladdress: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPostalAddress {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPostalAddress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: isize>() -> IADsPostalAddress_Vtbl {
        unsafe extern "system" fn PostalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostaladdress: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostalAddress(::core::mem::transmute(&vpostaladdress)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPostalAddress as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPrintJob_Impl: Sized + IADs_Impl {
    fn HostPrintQueue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn User(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TimeSubmitted(&self) -> ::windows_core::Result<f64>;
    fn TotalPages(&self) -> ::windows_core::Result<i32>;
    fn Size(&self) -> ::windows_core::Result<i32>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Priority(&self) -> ::windows_core::Result<i32>;
    fn SetPriority(&self, lnpriority: i32) -> ::windows_core::Result<()>;
    fn StartTime(&self) -> ::windows_core::Result<f64>;
    fn SetStartTime(&self, dastarttime: f64) -> ::windows_core::Result<()>;
    fn UntilTime(&self) -> ::windows_core::Result<f64>;
    fn SetUntilTime(&self, dauntiltime: f64) -> ::windows_core::Result<()>;
    fn Notify(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNotify(&self, bstrnotify: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NotifyPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNotifyPath(&self, bstrnotifypath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPrintJob {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPrintJob_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>() -> IADsPrintJob_Vtbl {
        unsafe extern "system" fn HostPrintQueue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HostPrintQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.User() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeSubmitted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TimeSubmitted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalPages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&lnpriority)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartTime(::core::mem::transmute_copy(&dastarttime)).into()
        }
        unsafe extern "system" fn UntilTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UntilTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntilTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUntilTime(::core::mem::transmute_copy(&dauntiltime)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Notify() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnotify: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotify(::core::mem::transmute(&bstrnotify)).into()
        }
        unsafe extern "system" fn NotifyPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NotifyPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnotifypath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotifyPath(::core::mem::transmute(&bstrnotifypath)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            HostPrintQueue: HostPrintQueue::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            TimeSubmitted: TimeSubmitted::<Identity, Impl, OFFSET>,
            TotalPages: TotalPages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            UntilTime: UntilTime::<Identity, Impl, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            SetNotify: SetNotify::<Identity, Impl, OFFSET>,
            NotifyPath: NotifyPath::<Identity, Impl, OFFSET>,
            SetNotifyPath: SetNotifyPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPrintJob as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPrintJobOperations_Impl: Sized + IADs_Impl {
    fn Status(&self) -> ::windows_core::Result<i32>;
    fn TimeElapsed(&self) -> ::windows_core::Result<i32>;
    fn PagesPrinted(&self) -> ::windows_core::Result<i32>;
    fn Position(&self) -> ::windows_core::Result<i32>;
    fn SetPosition(&self, lnposition: i32) -> ::windows_core::Result<()>;
    fn Pause(&self) -> ::windows_core::Result<()>;
    fn Resume(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPrintJobOperations {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPrintJobOperations_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>() -> IADsPrintJobOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeElapsed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TimeElapsed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PagesPrinted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PagesPrinted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Position() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnposition: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPosition(::core::mem::transmute_copy(&lnposition)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            TimeElapsed: TimeElapsed::<Identity, Impl, OFFSET>,
            PagesPrinted: PagesPrinted::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPrintJobOperations as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPrintQueue_Impl: Sized + IADs_Impl {
    fn PrinterPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPrinterPath(&self, bstrprinterpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Model(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetModel(&self, bstrmodel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Datatype(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDatatype(&self, bstrdatatype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PrintProcessor(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPrintProcessor(&self, bstrprintprocessor: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Location(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocation(&self, bstrlocation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartTime(&self) -> ::windows_core::Result<f64>;
    fn SetStartTime(&self, dastarttime: f64) -> ::windows_core::Result<()>;
    fn UntilTime(&self) -> ::windows_core::Result<f64>;
    fn SetUntilTime(&self, dauntiltime: f64) -> ::windows_core::Result<()>;
    fn DefaultJobPriority(&self) -> ::windows_core::Result<i32>;
    fn SetDefaultJobPriority(&self, lndefaultjobpriority: i32) -> ::windows_core::Result<()>;
    fn Priority(&self) -> ::windows_core::Result<i32>;
    fn SetPriority(&self, lnpriority: i32) -> ::windows_core::Result<()>;
    fn BannerPage(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBannerPage(&self, bstrbannerpage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PrintDevices(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPrintDevices(&self, vprintdevices: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn NetAddresses(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetNetAddresses(&self, vnetaddresses: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPrintQueue {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPrintQueue_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>() -> IADsPrintQueue_Vtbl {
        unsafe extern "system" fn PrinterPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrinterPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrinterPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprinterpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrinterPath(::core::mem::transmute(&bstrprinterpath)).into()
        }
        unsafe extern "system" fn Model<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Model() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmodel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModel(::core::mem::transmute(&bstrmodel)).into()
        }
        unsafe extern "system" fn Datatype<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Datatype() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDatatype<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdatatype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDatatype(::core::mem::transmute(&bstrdatatype)).into()
        }
        unsafe extern "system" fn PrintProcessor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrintProcessor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintProcessor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprintprocessor: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintProcessor(::core::mem::transmute(&bstrprintprocessor)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Location<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Location() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocation(::core::mem::transmute(&bstrlocation)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartTime(::core::mem::transmute_copy(&dastarttime)).into()
        }
        unsafe extern "system" fn UntilTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UntilTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntilTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUntilTime(::core::mem::transmute_copy(&dauntiltime)).into()
        }
        unsafe extern "system" fn DefaultJobPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultJobPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultJobPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lndefaultjobpriority: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultJobPriority(::core::mem::transmute_copy(&lndefaultjobpriority)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&lnpriority)).into()
        }
        unsafe extern "system" fn BannerPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BannerPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBannerPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbannerpage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBannerPage(::core::mem::transmute(&bstrbannerpage)).into()
        }
        unsafe extern "system" fn PrintDevices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrintDevices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintDevices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vprintdevices: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintDevices(::core::mem::transmute(&vprintdevices)).into()
        }
        unsafe extern "system" fn NetAddresses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetAddresses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnetaddresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetAddresses(::core::mem::transmute(&vnetaddresses)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrinterPath: PrinterPath::<Identity, Impl, OFFSET>,
            SetPrinterPath: SetPrinterPath::<Identity, Impl, OFFSET>,
            Model: Model::<Identity, Impl, OFFSET>,
            SetModel: SetModel::<Identity, Impl, OFFSET>,
            Datatype: Datatype::<Identity, Impl, OFFSET>,
            SetDatatype: SetDatatype::<Identity, Impl, OFFSET>,
            PrintProcessor: PrintProcessor::<Identity, Impl, OFFSET>,
            SetPrintProcessor: SetPrintProcessor::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            SetLocation: SetLocation::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            UntilTime: UntilTime::<Identity, Impl, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, Impl, OFFSET>,
            DefaultJobPriority: DefaultJobPriority::<Identity, Impl, OFFSET>,
            SetDefaultJobPriority: SetDefaultJobPriority::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            BannerPage: BannerPage::<Identity, Impl, OFFSET>,
            SetBannerPage: SetBannerPage::<Identity, Impl, OFFSET>,
            PrintDevices: PrintDevices::<Identity, Impl, OFFSET>,
            SetPrintDevices: SetPrintDevices::<Identity, Impl, OFFSET>,
            NetAddresses: NetAddresses::<Identity, Impl, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPrintQueue as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPrintQueueOperations_Impl: Sized + IADs_Impl {
    fn Status(&self) -> ::windows_core::Result<i32>;
    fn PrintJobs(&self) -> ::windows_core::Result<IADsCollection>;
    fn Pause(&self) -> ::windows_core::Result<()>;
    fn Resume(&self) -> ::windows_core::Result<()>;
    fn Purge(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPrintQueueOperations {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPrintQueueOperations_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>() -> IADsPrintQueueOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintJobs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrintJobs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn Purge<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Purge().into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            PrintJobs: PrintJobs::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPrintQueueOperations as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsProperty_Impl: Sized + IADs_Impl {
    fn OID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOID(&self, bstroid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Syntax(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSyntax(&self, bstrsyntax: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxRange(&self) -> ::windows_core::Result<i32>;
    fn SetMaxRange(&self, lnmaxrange: i32) -> ::windows_core::Result<()>;
    fn MinRange(&self) -> ::windows_core::Result<i32>;
    fn SetMinRange(&self, lnminrange: i32) -> ::windows_core::Result<()>;
    fn MultiValued(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMultiValued(&self, fmultivalued: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Qualifiers(&self) -> ::windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsProperty {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>() -> IADsProperty_Vtbl {
        unsafe extern "system" fn OID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOID(::core::mem::transmute(&bstroid)).into()
        }
        unsafe extern "system" fn Syntax<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Syntax() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyntax<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsyntax: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSyntax(::core::mem::transmute(&bstrsyntax)).into()
        }
        unsafe extern "system" fn MaxRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxrange: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxRange(::core::mem::transmute_copy(&lnmaxrange)).into()
        }
        unsafe extern "system" fn MinRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminrange: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinRange(::core::mem::transmute_copy(&lnminrange)).into()
        }
        unsafe extern "system" fn MultiValued<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MultiValued() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiValued<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmultivalued: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultiValued(::core::mem::transmute_copy(&fmultivalued)).into()
        }
        unsafe extern "system" fn Qualifiers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Qualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualifiers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            OID: OID::<Identity, Impl, OFFSET>,
            SetOID: SetOID::<Identity, Impl, OFFSET>,
            Syntax: Syntax::<Identity, Impl, OFFSET>,
            SetSyntax: SetSyntax::<Identity, Impl, OFFSET>,
            MaxRange: MaxRange::<Identity, Impl, OFFSET>,
            SetMaxRange: SetMaxRange::<Identity, Impl, OFFSET>,
            MinRange: MinRange::<Identity, Impl, OFFSET>,
            SetMinRange: SetMinRange::<Identity, Impl, OFFSET>,
            MultiValued: MultiValued::<Identity, Impl, OFFSET>,
            SetMultiValued: SetMultiValued::<Identity, Impl, OFFSET>,
            Qualifiers: Qualifiers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsProperty as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPropertyEntry_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&self) -> ::windows_core::Result<()>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ADsType(&self) -> ::windows_core::Result<i32>;
    fn SetADsType(&self, lnadstype: i32) -> ::windows_core::Result<()>;
    fn ControlCode(&self) -> ::windows_core::Result<i32>;
    fn SetControlCode(&self, lncontrolcode: i32) -> ::windows_core::Result<()>;
    fn Values(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValues(&self, vvalues: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPropertyEntry {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPropertyEntry_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>() -> IADsPropertyEntry_Vtbl {
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn ADsType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADsType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetADsType(::core::mem::transmute_copy(&lnadstype)).into()
        }
        unsafe extern "system" fn ControlCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ControlCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetControlCode(::core::mem::transmute_copy(&lncontrolcode)).into()
        }
        unsafe extern "system" fn Values<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Values() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValues<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vvalues: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValues(::core::mem::transmute(&vvalues)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Clear: Clear::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            ADsType: ADsType::<Identity, Impl, OFFSET>,
            SetADsType: SetADsType::<Identity, Impl, OFFSET>,
            ControlCode: ControlCode::<Identity, Impl, OFFSET>,
            SetControlCode: SetControlCode::<Identity, Impl, OFFSET>,
            Values: Values::<Identity, Impl, OFFSET>,
            SetValues: SetValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPropertyEntry as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPropertyList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PropertyCount(&self) -> ::windows_core::Result<i32>;
    fn Next(&self, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT;
    fn Skip(&self, celements: i32) -> ::windows_core::HRESULT;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Item(&self, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetPropertyItem(&self, bstrname: &::windows_core::BSTR, lnadstype: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PutPropertyItem(&self, vardata: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ResetPropertyItem(&self, varentry: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PurgePropertyList(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPropertyList {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPropertyList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>() -> IADsPropertyList_Vtbl {
        unsafe extern "system" fn PropertyCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&pvariant))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celements))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnadstype: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyItem(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&lnadstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardata: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutPropertyItem(::core::mem::transmute(&vardata)).into()
        }
        unsafe extern "system" fn ResetPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varentry: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetPropertyItem(::core::mem::transmute(&varentry)).into()
        }
        unsafe extern "system" fn PurgePropertyList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PurgePropertyList().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PropertyCount: PropertyCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetPropertyItem: GetPropertyItem::<Identity, Impl, OFFSET>,
            PutPropertyItem: PutPropertyItem::<Identity, Impl, OFFSET>,
            ResetPropertyItem: ResetPropertyItem::<Identity, Impl, OFFSET>,
            PurgePropertyList: PurgePropertyList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPropertyList as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPropertyValue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&self) -> ::windows_core::Result<()>;
    fn ADsType(&self) -> ::windows_core::Result<i32>;
    fn SetADsType(&self, lnadstype: i32) -> ::windows_core::Result<()>;
    fn DNString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CaseExactString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCaseExactString(&self, bstrcaseexactstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CaseIgnoreString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCaseIgnoreString(&self, bstrcaseignorestring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PrintableString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPrintableString(&self, bstrprintablestring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NumericString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNumericString(&self, bstrnumericstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Boolean(&self) -> ::windows_core::Result<i32>;
    fn SetBoolean(&self, lnboolean: i32) -> ::windows_core::Result<()>;
    fn Integer(&self) -> ::windows_core::Result<i32>;
    fn SetInteger(&self, lninteger: i32) -> ::windows_core::Result<()>;
    fn OctetString(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOctetString(&self, voctetstring: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SecurityDescriptor(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(&self, psecuritydescriptor: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn LargeInteger(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetLargeInteger(&self, plargeinteger: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn UTCTime(&self) -> ::windows_core::Result<f64>;
    fn SetUTCTime(&self, dautctime: f64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPropertyValue {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPropertyValue_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>() -> IADsPropertyValue_Vtbl {
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn ADsType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADsType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetADsType(::core::mem::transmute_copy(&lnadstype)).into()
        }
        unsafe extern "system" fn DNString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DNString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDNString(::core::mem::transmute(&bstrdnstring)).into()
        }
        unsafe extern "system" fn CaseExactString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CaseExactString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseExactString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaseexactstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCaseExactString(::core::mem::transmute(&bstrcaseexactstring)).into()
        }
        unsafe extern "system" fn CaseIgnoreString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CaseIgnoreString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseIgnoreString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaseignorestring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCaseIgnoreString(::core::mem::transmute(&bstrcaseignorestring)).into()
        }
        unsafe extern "system" fn PrintableString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrintableString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintableString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprintablestring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintableString(::core::mem::transmute(&bstrprintablestring)).into()
        }
        unsafe extern "system" fn NumericString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumericString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumericString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnumericstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNumericString(::core::mem::transmute(&bstrnumericstring)).into()
        }
        unsafe extern "system" fn Boolean<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Boolean() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolean<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnboolean: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBoolean(::core::mem::transmute_copy(&lnboolean)).into()
        }
        unsafe extern "system" fn Integer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Integer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInteger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lninteger: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInteger(::core::mem::transmute_copy(&lninteger)).into()
        }
        unsafe extern "system" fn OctetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OctetString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOctetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voctetstring: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOctetString(::core::mem::transmute(&voctetstring)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecuritydescriptor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityDescriptor(::windows_core::from_raw_borrowed(&psecuritydescriptor)).into()
        }
        unsafe extern "system" fn LargeInteger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LargeInteger() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLargeInteger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plargeinteger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLargeInteger(::windows_core::from_raw_borrowed(&plargeinteger)).into()
        }
        unsafe extern "system" fn UTCTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UTCTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dautctime: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUTCTime(::core::mem::transmute_copy(&dautctime)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Clear: Clear::<Identity, Impl, OFFSET>,
            ADsType: ADsType::<Identity, Impl, OFFSET>,
            SetADsType: SetADsType::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
            CaseExactString: CaseExactString::<Identity, Impl, OFFSET>,
            SetCaseExactString: SetCaseExactString::<Identity, Impl, OFFSET>,
            CaseIgnoreString: CaseIgnoreString::<Identity, Impl, OFFSET>,
            SetCaseIgnoreString: SetCaseIgnoreString::<Identity, Impl, OFFSET>,
            PrintableString: PrintableString::<Identity, Impl, OFFSET>,
            SetPrintableString: SetPrintableString::<Identity, Impl, OFFSET>,
            NumericString: NumericString::<Identity, Impl, OFFSET>,
            SetNumericString: SetNumericString::<Identity, Impl, OFFSET>,
            Boolean: Boolean::<Identity, Impl, OFFSET>,
            SetBoolean: SetBoolean::<Identity, Impl, OFFSET>,
            Integer: Integer::<Identity, Impl, OFFSET>,
            SetInteger: SetInteger::<Identity, Impl, OFFSET>,
            OctetString: OctetString::<Identity, Impl, OFFSET>,
            SetOctetString: SetOctetString::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            LargeInteger: LargeInteger::<Identity, Impl, OFFSET>,
            SetLargeInteger: SetLargeInteger::<Identity, Impl, OFFSET>,
            UTCTime: UTCTime::<Identity, Impl, OFFSET>,
            SetUTCTime: SetUTCTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPropertyValue as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPropertyValue2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetObjectProperty(&self, lnadstype: *mut i32, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PutObjectProperty(&self, lnadstype: i32, vprop: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsPropertyValue2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsPropertyValue2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>() -> IADsPropertyValue2_Vtbl {
        unsafe extern "system" fn GetObjectProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectProperty(::core::mem::transmute_copy(&lnadstype), ::core::mem::transmute_copy(&pvprop)).into()
        }
        unsafe extern "system" fn PutObjectProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutObjectProperty(::core::mem::transmute_copy(&lnadstype), ::core::mem::transmute(&vprop)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetObjectProperty: GetObjectProperty::<Identity, Impl, OFFSET>,
            PutObjectProperty: PutObjectProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsPropertyValue2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsReplicaPointer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ServerName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServerName(&self, bstrservername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReplicaType(&self) -> ::windows_core::Result<i32>;
    fn SetReplicaType(&self, lnreplicatype: i32) -> ::windows_core::Result<()>;
    fn ReplicaNumber(&self) -> ::windows_core::Result<i32>;
    fn SetReplicaNumber(&self, lnreplicanumber: i32) -> ::windows_core::Result<()>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn SetCount(&self, lncount: i32) -> ::windows_core::Result<()>;
    fn ReplicaAddressHints(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetReplicaAddressHints(&self, vreplicaaddresshints: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsReplicaPointer {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsReplicaPointer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>() -> IADsReplicaPointer_Vtbl {
        unsafe extern "system" fn ServerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServerName(::core::mem::transmute(&bstrservername)).into()
        }
        unsafe extern "system" fn ReplicaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReplicaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreplicatype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReplicaType(::core::mem::transmute_copy(&lnreplicatype)).into()
        }
        unsafe extern "system" fn ReplicaNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReplicaNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreplicanumber: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReplicaNumber(::core::mem::transmute_copy(&lnreplicanumber)).into()
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncount: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCount(::core::mem::transmute_copy(&lncount)).into()
        }
        unsafe extern "system" fn ReplicaAddressHints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReplicaAddressHints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaAddressHints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vreplicaaddresshints: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReplicaAddressHints(::core::mem::transmute(&vreplicaaddresshints)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ServerName: ServerName::<Identity, Impl, OFFSET>,
            SetServerName: SetServerName::<Identity, Impl, OFFSET>,
            ReplicaType: ReplicaType::<Identity, Impl, OFFSET>,
            SetReplicaType: SetReplicaType::<Identity, Impl, OFFSET>,
            ReplicaNumber: ReplicaNumber::<Identity, Impl, OFFSET>,
            SetReplicaNumber: SetReplicaNumber::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            SetCount: SetCount::<Identity, Impl, OFFSET>,
            ReplicaAddressHints: ReplicaAddressHints::<Identity, Impl, OFFSET>,
            SetReplicaAddressHints: SetReplicaAddressHints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsReplicaPointer as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsResource_Impl: Sized + IADs_Impl {
    fn User(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LockCount(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsResource {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>() -> IADsResource_Vtbl {
        unsafe extern "system" fn User<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.User() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LockCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            LockCount: LockCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsResource as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsSecurityDescriptor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Revision(&self) -> ::windows_core::Result<i32>;
    fn SetRevision(&self, lnrevision: i32) -> ::windows_core::Result<()>;
    fn Control(&self) -> ::windows_core::Result<i32>;
    fn SetControl(&self, lncontrol: i32) -> ::windows_core::Result<()>;
    fn Owner(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOwner(&self, bstrowner: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OwnerDefaulted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOwnerDefaulted(&self, fownerdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Group(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetGroup(&self, bstrgroup: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GroupDefaulted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGroupDefaulted(&self, fgroupdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DiscretionaryAcl(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetDiscretionaryAcl(&self, pdiscretionaryacl: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn DaclDefaulted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDaclDefaulted(&self, fdacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SystemAcl(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetSystemAcl(&self, psystemacl: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn SaclDefaulted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSaclDefaulted(&self, fsacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn CopySecurityDescriptor(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsSecurityDescriptor {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsSecurityDescriptor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>() -> IADsSecurityDescriptor_Vtbl {
        unsafe extern "system" fn Revision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Revision() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnrevision: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRevision(::core::mem::transmute_copy(&lnrevision)).into()
        }
        unsafe extern "system" fn Control<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Control() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrol: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetControl(::core::mem::transmute_copy(&lncontrol)).into()
        }
        unsafe extern "system" fn Owner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Owner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrowner: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOwner(::core::mem::transmute(&bstrowner)).into()
        }
        unsafe extern "system" fn OwnerDefaulted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OwnerDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerDefaulted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fownerdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOwnerDefaulted(::core::mem::transmute_copy(&fownerdefaulted)).into()
        }
        unsafe extern "system" fn Group<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Group() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroup: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroup(::core::mem::transmute(&bstrgroup)).into()
        }
        unsafe extern "system" fn GroupDefaulted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GroupDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupDefaulted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgroupdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroupDefaulted(::core::mem::transmute_copy(&fgroupdefaulted)).into()
        }
        unsafe extern "system" fn DiscretionaryAcl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiscretionaryAcl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscretionaryAcl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdiscretionaryacl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDiscretionaryAcl(::windows_core::from_raw_borrowed(&pdiscretionaryacl)).into()
        }
        unsafe extern "system" fn DaclDefaulted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DaclDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaclDefaulted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaclDefaulted(::core::mem::transmute_copy(&fdacldefaulted)).into()
        }
        unsafe extern "system" fn SystemAcl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SystemAcl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemAcl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psystemacl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSystemAcl(::windows_core::from_raw_borrowed(&psystemacl)).into()
        }
        unsafe extern "system" fn SaclDefaulted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaclDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaclDefaulted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSaclDefaulted(::core::mem::transmute_copy(&fsacldefaulted)).into()
        }
        unsafe extern "system" fn CopySecurityDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecuritydescriptor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopySecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecuritydescriptor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Revision: Revision::<Identity, Impl, OFFSET>,
            SetRevision: SetRevision::<Identity, Impl, OFFSET>,
            Control: Control::<Identity, Impl, OFFSET>,
            SetControl: SetControl::<Identity, Impl, OFFSET>,
            Owner: Owner::<Identity, Impl, OFFSET>,
            SetOwner: SetOwner::<Identity, Impl, OFFSET>,
            OwnerDefaulted: OwnerDefaulted::<Identity, Impl, OFFSET>,
            SetOwnerDefaulted: SetOwnerDefaulted::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            SetGroup: SetGroup::<Identity, Impl, OFFSET>,
            GroupDefaulted: GroupDefaulted::<Identity, Impl, OFFSET>,
            SetGroupDefaulted: SetGroupDefaulted::<Identity, Impl, OFFSET>,
            DiscretionaryAcl: DiscretionaryAcl::<Identity, Impl, OFFSET>,
            SetDiscretionaryAcl: SetDiscretionaryAcl::<Identity, Impl, OFFSET>,
            DaclDefaulted: DaclDefaulted::<Identity, Impl, OFFSET>,
            SetDaclDefaulted: SetDaclDefaulted::<Identity, Impl, OFFSET>,
            SystemAcl: SystemAcl::<Identity, Impl, OFFSET>,
            SetSystemAcl: SetSystemAcl::<Identity, Impl, OFFSET>,
            SaclDefaulted: SaclDefaulted::<Identity, Impl, OFFSET>,
            SetSaclDefaulted: SetSaclDefaulted::<Identity, Impl, OFFSET>,
            CopySecurityDescriptor: CopySecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsSecurityDescriptor as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsSecurityUtility_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetSecurityDescriptor(&self, varpath: &super::super::System::Variant::VARIANT, lpathformat: i32, lformat: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSecurityDescriptor(&self, varpath: &super::super::System::Variant::VARIANT, lpathformat: i32, vardata: &super::super::System::Variant::VARIANT, ldataformat: i32) -> ::windows_core::Result<()>;
    fn ConvertSecurityDescriptor(&self, varsd: &super::super::System::Variant::VARIANT, ldataformat: i32, loutformat: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SecurityMask(&self) -> ::windows_core::Result<i32>;
    fn SetSecurityMask(&self, lnsecuritymask: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsSecurityUtility {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsSecurityUtility_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>() -> IADsSecurityUtility_Vtbl {
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varpath: super::super::System::Variant::VARIANT, lpathformat: i32, lformat: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityDescriptor(::core::mem::transmute(&varpath), ::core::mem::transmute_copy(&lpathformat), ::core::mem::transmute_copy(&lformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varpath: super::super::System::Variant::VARIANT, lpathformat: i32, vardata: super::super::System::Variant::VARIANT, ldataformat: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityDescriptor(::core::mem::transmute(&varpath), ::core::mem::transmute_copy(&lpathformat), ::core::mem::transmute(&vardata), ::core::mem::transmute_copy(&ldataformat)).into()
        }
        unsafe extern "system" fn ConvertSecurityDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsd: super::super::System::Variant::VARIANT, ldataformat: i32, loutformat: i32, presult: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConvertSecurityDescriptor(::core::mem::transmute(&varsd), ::core::mem::transmute_copy(&ldataformat), ::core::mem::transmute_copy(&loutformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SecurityMask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsecuritymask: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityMask(::core::mem::transmute_copy(&lnsecuritymask)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            ConvertSecurityDescriptor: ConvertSecurityDescriptor::<Identity, Impl, OFFSET>,
            SecurityMask: SecurityMask::<Identity, Impl, OFFSET>,
            SetSecurityMask: SetSecurityMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsSecurityUtility as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsService_Impl: Sized + IADs_Impl {
    fn HostComputer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHostComputer(&self, bstrhostcomputer: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDisplayName(&self, bstrdisplayname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Version(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVersion(&self, bstrversion: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceType(&self) -> ::windows_core::Result<i32>;
    fn SetServiceType(&self, lnservicetype: i32) -> ::windows_core::Result<()>;
    fn StartType(&self) -> ::windows_core::Result<i32>;
    fn SetStartType(&self, lnstarttype: i32) -> ::windows_core::Result<()>;
    fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartupParameters(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStartupParameters(&self, bstrstartupparameters: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ErrorControl(&self) -> ::windows_core::Result<i32>;
    fn SetErrorControl(&self, lnerrorcontrol: i32) -> ::windows_core::Result<()>;
    fn LoadOrderGroup(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLoadOrderGroup(&self, bstrloadordergroup: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceAccountName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceAccountName(&self, bstrserviceaccountname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceAccountPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceAccountPath(&self, bstrserviceaccountpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Dependencies(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDependencies(&self, vdependencies: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsService {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>() -> IADsService_Vtbl {
        unsafe extern "system" fn HostComputer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HostComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostComputer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHostComputer(::core::mem::transmute(&bstrhostcomputer)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdisplayname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&bstrdisplayname)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrversion: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVersion(::core::mem::transmute(&bstrversion)).into()
        }
        unsafe extern "system" fn ServiceType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnservicetype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceType(::core::mem::transmute_copy(&lnservicetype)).into()
        }
        unsafe extern "system" fn StartType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnstarttype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartType(::core::mem::transmute_copy(&lnstarttype)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPath(::core::mem::transmute(&bstrpath)).into()
        }
        unsafe extern "system" fn StartupParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartupParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartupParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstartupparameters: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartupParameters(::core::mem::transmute(&bstrstartupparameters)).into()
        }
        unsafe extern "system" fn ErrorControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ErrorControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnerrorcontrol: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetErrorControl(::core::mem::transmute_copy(&lnerrorcontrol)).into()
        }
        unsafe extern "system" fn LoadOrderGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadOrderGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoadOrderGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloadordergroup: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLoadOrderGroup(::core::mem::transmute(&bstrloadordergroup)).into()
        }
        unsafe extern "system" fn ServiceAccountName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceAccountName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceAccountName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceaccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceAccountName(::core::mem::transmute(&bstrserviceaccountname)).into()
        }
        unsafe extern "system" fn ServiceAccountPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceAccountPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceAccountPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceaccountpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceAccountPath(::core::mem::transmute(&bstrserviceaccountpath)).into()
        }
        unsafe extern "system" fn Dependencies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDependencies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdependencies: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDependencies(::core::mem::transmute(&vdependencies)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            HostComputer: HostComputer::<Identity, Impl, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            ServiceType: ServiceType::<Identity, Impl, OFFSET>,
            SetServiceType: SetServiceType::<Identity, Impl, OFFSET>,
            StartType: StartType::<Identity, Impl, OFFSET>,
            SetStartType: SetStartType::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            StartupParameters: StartupParameters::<Identity, Impl, OFFSET>,
            SetStartupParameters: SetStartupParameters::<Identity, Impl, OFFSET>,
            ErrorControl: ErrorControl::<Identity, Impl, OFFSET>,
            SetErrorControl: SetErrorControl::<Identity, Impl, OFFSET>,
            LoadOrderGroup: LoadOrderGroup::<Identity, Impl, OFFSET>,
            SetLoadOrderGroup: SetLoadOrderGroup::<Identity, Impl, OFFSET>,
            ServiceAccountName: ServiceAccountName::<Identity, Impl, OFFSET>,
            SetServiceAccountName: SetServiceAccountName::<Identity, Impl, OFFSET>,
            ServiceAccountPath: ServiceAccountPath::<Identity, Impl, OFFSET>,
            SetServiceAccountPath: SetServiceAccountPath::<Identity, Impl, OFFSET>,
            Dependencies: Dependencies::<Identity, Impl, OFFSET>,
            SetDependencies: SetDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsService as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsServiceOperations_Impl: Sized + IADs_Impl {
    fn Status(&self) -> ::windows_core::Result<i32>;
    fn Start(&self) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
    fn Pause(&self) -> ::windows_core::Result<()>;
    fn Continue(&self) -> ::windows_core::Result<()>;
    fn SetPassword(&self, bstrnewpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsServiceOperations {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsServiceOperations_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>() -> IADsServiceOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Continue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Continue().into()
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPassword(::core::mem::transmute(&bstrnewpassword)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Continue: Continue::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsServiceOperations as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsSession_Impl: Sized + IADs_Impl {
    fn User(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Computer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ComputerPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ConnectTime(&self) -> ::windows_core::Result<i32>;
    fn IdleTime(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsSession {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsSession_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>() -> IADsSession_Vtbl {
        unsafe extern "system" fn User<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.User() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Computer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Computer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputerPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdleTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IdleTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            Computer: Computer::<Identity, Impl, OFFSET>,
            ComputerPath: ComputerPath::<Identity, Impl, OFFSET>,
            ConnectTime: ConnectTime::<Identity, Impl, OFFSET>,
            IdleTime: IdleTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsSession as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsSyntax_Impl: Sized + IADs_Impl {
    fn OleAutoDataType(&self) -> ::windows_core::Result<i32>;
    fn SetOleAutoDataType(&self, lnoleautodatatype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsSyntax {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsSyntax_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: isize>() -> IADsSyntax_Vtbl {
        unsafe extern "system" fn OleAutoDataType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OleAutoDataType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOleAutoDataType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoleautodatatype: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOleAutoDataType(::core::mem::transmute_copy(&lnoleautodatatype)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            OleAutoDataType: OleAutoDataType::<Identity, Impl, OFFSET>,
            SetOleAutoDataType: SetOleAutoDataType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsSyntax as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsTimestamp_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WholeSeconds(&self) -> ::windows_core::Result<i32>;
    fn SetWholeSeconds(&self, lnwholeseconds: i32) -> ::windows_core::Result<()>;
    fn EventID(&self) -> ::windows_core::Result<i32>;
    fn SetEventID(&self, lneventid: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsTimestamp {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsTimestamp_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>() -> IADsTimestamp_Vtbl {
        unsafe extern "system" fn WholeSeconds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WholeSeconds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWholeSeconds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnwholeseconds: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWholeSeconds(::core::mem::transmute_copy(&lnwholeseconds)).into()
        }
        unsafe extern "system" fn EventID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lneventid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventID(::core::mem::transmute_copy(&lneventid)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WholeSeconds: WholeSeconds::<Identity, Impl, OFFSET>,
            SetWholeSeconds: SetWholeSeconds::<Identity, Impl, OFFSET>,
            EventID: EventID::<Identity, Impl, OFFSET>,
            SetEventID: SetEventID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsTimestamp as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsTypedName_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Level(&self) -> ::windows_core::Result<i32>;
    fn SetLevel(&self, lnlevel: i32) -> ::windows_core::Result<()>;
    fn Interval(&self) -> ::windows_core::Result<i32>;
    fn SetInterval(&self, lninterval: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsTypedName {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsTypedName_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>() -> IADsTypedName_Vtbl {
        unsafe extern "system" fn ObjectName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectName(::core::mem::transmute(&bstrobjectname)).into()
        }
        unsafe extern "system" fn Level<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Level() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlevel: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLevel(::core::mem::transmute_copy(&lnlevel)).into()
        }
        unsafe extern "system" fn Interval<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Interval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lninterval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInterval(::core::mem::transmute_copy(&lninterval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            Level: Level::<Identity, Impl, OFFSET>,
            SetLevel: SetLevel::<Identity, Impl, OFFSET>,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsTypedName as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsUser_Impl: Sized + IADs_Impl {
    fn BadLoginAddress(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BadLoginCount(&self) -> ::windows_core::Result<i32>;
    fn LastLogin(&self) -> ::windows_core::Result<f64>;
    fn LastLogoff(&self) -> ::windows_core::Result<f64>;
    fn LastFailedLogin(&self) -> ::windows_core::Result<f64>;
    fn PasswordLastChanged(&self) -> ::windows_core::Result<f64>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Division(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDivision(&self, bstrdivision: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Department(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EmployeeID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEmployeeID(&self, bstremployeeid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FullName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFullName(&self, bstrfullname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FirstName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFirstName(&self, bstrfirstname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LastName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLastName(&self, bstrlastname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OtherName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOtherName(&self, bstrothername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NamePrefix(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNamePrefix(&self, bstrnameprefix: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NameSuffix(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNameSuffix(&self, bstrnamesuffix: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTitle(&self, bstrtitle: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Manager(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetManager(&self, bstrmanager: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TelephoneHome(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetTelephoneHome(&self, vtelephonehome: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn TelephoneMobile(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetTelephoneMobile(&self, vtelephonemobile: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn TelephoneNumber(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetTelephoneNumber(&self, vtelephonenumber: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn TelephonePager(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetTelephonePager(&self, vtelephonepager: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn FaxNumber(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetFaxNumber(&self, vfaxnumber: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn OfficeLocations(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOfficeLocations(&self, vofficelocations: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PostalAddresses(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPostalAddresses(&self, vpostaladdresses: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PostalCodes(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPostalCodes(&self, vpostalcodes: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SeeAlso(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AccountDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAccountDisabled(&self, faccountdisabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AccountExpirationDate(&self) -> ::windows_core::Result<f64>;
    fn SetAccountExpirationDate(&self, daaccountexpirationdate: f64) -> ::windows_core::Result<()>;
    fn GraceLoginsAllowed(&self) -> ::windows_core::Result<i32>;
    fn SetGraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> ::windows_core::Result<()>;
    fn GraceLoginsRemaining(&self) -> ::windows_core::Result<i32>;
    fn SetGraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> ::windows_core::Result<()>;
    fn IsAccountLocked(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsAccountLocked(&self, fisaccountlocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn LoginHours(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetLoginHours(&self, vloginhours: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn LoginWorkstations(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetLoginWorkstations(&self, vloginworkstations: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn MaxLogins(&self) -> ::windows_core::Result<i32>;
    fn SetMaxLogins(&self, lnmaxlogins: i32) -> ::windows_core::Result<()>;
    fn MaxStorage(&self) -> ::windows_core::Result<i32>;
    fn SetMaxStorage(&self, lnmaxstorage: i32) -> ::windows_core::Result<()>;
    fn PasswordExpirationDate(&self) -> ::windows_core::Result<f64>;
    fn SetPasswordExpirationDate(&self, dapasswordexpirationdate: f64) -> ::windows_core::Result<()>;
    fn PasswordMinimumLength(&self) -> ::windows_core::Result<i32>;
    fn SetPasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> ::windows_core::Result<()>;
    fn PasswordRequired(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPasswordRequired(&self, fpasswordrequired: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RequireUniquePassword(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRequireUniquePassword(&self, frequireuniquepassword: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EmailAddress(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEmailAddress(&self, bstremailaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HomeDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHomeDirectory(&self, bstrhomedirectory: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Languages(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetLanguages(&self, vlanguages: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Profile(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProfile(&self, bstrprofile: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LoginScript(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLoginScript(&self, bstrloginscript: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Picture(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPicture(&self, vpicture: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn HomePage(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHomePage(&self, bstrhomepage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Groups(&self) -> ::windows_core::Result<IADsMembers>;
    fn SetPassword(&self, newpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangePassword(&self, bstroldpassword: &::windows_core::BSTR, bstrnewpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsUser {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsUser_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>() -> IADsUser_Vtbl {
        unsafe extern "system" fn BadLoginAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BadLoginAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BadLoginCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BadLoginCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastLogin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastLogin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastLogoff<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastLogoff() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastFailedLogin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastFailedLogin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PasswordLastChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PasswordLastChanged() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Division<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Division() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDivision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdivision: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDivision(::core::mem::transmute(&bstrdivision)).into()
        }
        unsafe extern "system" fn Department<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Department() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDepartment(::core::mem::transmute(&bstrdepartment)).into()
        }
        unsafe extern "system" fn EmployeeID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EmployeeID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmployeeID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremployeeid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEmployeeID(::core::mem::transmute(&bstremployeeid)).into()
        }
        unsafe extern "system" fn FullName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FullName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfullname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFullName(::core::mem::transmute(&bstrfullname)).into()
        }
        unsafe extern "system" fn FirstName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirstName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfirstname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFirstName(::core::mem::transmute(&bstrfirstname)).into()
        }
        unsafe extern "system" fn LastName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlastname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastName(::core::mem::transmute(&bstrlastname)).into()
        }
        unsafe extern "system" fn OtherName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OtherName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrothername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOtherName(::core::mem::transmute(&bstrothername)).into()
        }
        unsafe extern "system" fn NamePrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamePrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnameprefix: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamePrefix(::core::mem::transmute(&bstrnameprefix)).into()
        }
        unsafe extern "system" fn NameSuffix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NameSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNameSuffix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamesuffix: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNameSuffix(::core::mem::transmute(&bstrnamesuffix)).into()
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTitle(::core::mem::transmute(&bstrtitle)).into()
        }
        unsafe extern "system" fn Manager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Manager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmanager: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetManager(::core::mem::transmute(&bstrmanager)).into()
        }
        unsafe extern "system" fn TelephoneHome<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TelephoneHome() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneHome<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonehome: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTelephoneHome(::core::mem::transmute(&vtelephonehome)).into()
        }
        unsafe extern "system" fn TelephoneMobile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TelephoneMobile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneMobile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonemobile: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTelephoneMobile(::core::mem::transmute(&vtelephonemobile)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonenumber: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTelephoneNumber(::core::mem::transmute(&vtelephonenumber)).into()
        }
        unsafe extern "system" fn TelephonePager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TelephonePager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephonePager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonepager: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTelephonePager(::core::mem::transmute(&vtelephonepager)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vfaxnumber: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFaxNumber(::core::mem::transmute(&vfaxnumber)).into()
        }
        unsafe extern "system" fn OfficeLocations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OfficeLocations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficeLocations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vofficelocations: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOfficeLocations(::core::mem::transmute(&vofficelocations)).into()
        }
        unsafe extern "system" fn PostalAddresses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PostalAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddresses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostaladdresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostalAddresses(::core::mem::transmute(&vpostaladdresses)).into()
        }
        unsafe extern "system" fn PostalCodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PostalCodes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalCodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostalcodes: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostalCodes(::core::mem::transmute(&vpostalcodes)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSeeAlso(::core::mem::transmute(&vseealso)).into()
        }
        unsafe extern "system" fn AccountDisabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccountDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountDisabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, faccountdisabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccountDisabled(::core::mem::transmute_copy(&faccountdisabled)).into()
        }
        unsafe extern "system" fn AccountExpirationDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccountExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountExpirationDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, daaccountexpirationdate: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccountExpirationDate(::core::mem::transmute_copy(&daaccountexpirationdate)).into()
        }
        unsafe extern "system" fn GraceLoginsAllowed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GraceLoginsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraceLoginsAllowed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lngraceloginsallowed: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGraceLoginsAllowed(::core::mem::transmute_copy(&lngraceloginsallowed)).into()
        }
        unsafe extern "system" fn GraceLoginsRemaining<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GraceLoginsRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraceLoginsRemaining<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lngraceloginsremaining: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGraceLoginsRemaining(::core::mem::transmute_copy(&lngraceloginsremaining)).into()
        }
        unsafe extern "system" fn IsAccountLocked<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAccountLocked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAccountLocked<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fisaccountlocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsAccountLocked(::core::mem::transmute_copy(&fisaccountlocked)).into()
        }
        unsafe extern "system" fn LoginHours<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoginHours() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginHours<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vloginhours: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLoginHours(::core::mem::transmute(&vloginhours)).into()
        }
        unsafe extern "system" fn LoginWorkstations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoginWorkstations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginWorkstations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vloginworkstations: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLoginWorkstations(::core::mem::transmute(&vloginworkstations)).into()
        }
        unsafe extern "system" fn MaxLogins<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxLogins() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLogins<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxlogins: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxLogins(::core::mem::transmute_copy(&lnmaxlogins)).into()
        }
        unsafe extern "system" fn MaxStorage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxStorage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStorage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxstorage: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxStorage(::core::mem::transmute_copy(&lnmaxstorage)).into()
        }
        unsafe extern "system" fn PasswordExpirationDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PasswordExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordExpirationDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dapasswordexpirationdate: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPasswordExpirationDate(::core::mem::transmute_copy(&dapasswordexpirationdate)).into()
        }
        unsafe extern "system" fn PasswordMinimumLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PasswordMinimumLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordMinimumLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordminimumlength: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPasswordMinimumLength(::core::mem::transmute_copy(&lnpasswordminimumlength)).into()
        }
        unsafe extern "system" fn PasswordRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PasswordRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpasswordrequired: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPasswordRequired(::core::mem::transmute_copy(&fpasswordrequired)).into()
        }
        unsafe extern "system" fn RequireUniquePassword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequireUniquePassword() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireUniquePassword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequireuniquepassword: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequireUniquePassword(::core::mem::transmute_copy(&frequireuniquepassword)).into()
        }
        unsafe extern "system" fn EmailAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EmailAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmailAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremailaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEmailAddress(::core::mem::transmute(&bstremailaddress)).into()
        }
        unsafe extern "system" fn HomeDirectory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HomeDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomeDirectory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHomeDirectory(::core::mem::transmute(&bstrhomedirectory)).into()
        }
        unsafe extern "system" fn Languages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Languages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vlanguages: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguages(::core::mem::transmute(&vlanguages)).into()
        }
        unsafe extern "system" fn Profile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofile: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProfile(::core::mem::transmute(&bstrprofile)).into()
        }
        unsafe extern "system" fn LoginScript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoginScript() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginScript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloginscript: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLoginScript(::core::mem::transmute(&bstrloginscript)).into()
        }
        unsafe extern "system" fn Picture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Picture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPicture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpicture: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPicture(::core::mem::transmute(&vpicture)).into()
        }
        unsafe extern "system" fn HomePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HomePage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomepage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHomePage(::core::mem::transmute(&bstrhomepage)).into()
        }
        unsafe extern "system" fn Groups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Groups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPassword(::core::mem::transmute(&newpassword)).into()
        }
        unsafe extern "system" fn ChangePassword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroldpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangePassword(::core::mem::transmute(&bstroldpassword), ::core::mem::transmute(&bstrnewpassword)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            BadLoginAddress: BadLoginAddress::<Identity, Impl, OFFSET>,
            BadLoginCount: BadLoginCount::<Identity, Impl, OFFSET>,
            LastLogin: LastLogin::<Identity, Impl, OFFSET>,
            LastLogoff: LastLogoff::<Identity, Impl, OFFSET>,
            LastFailedLogin: LastFailedLogin::<Identity, Impl, OFFSET>,
            PasswordLastChanged: PasswordLastChanged::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Division: Division::<Identity, Impl, OFFSET>,
            SetDivision: SetDivision::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            EmployeeID: EmployeeID::<Identity, Impl, OFFSET>,
            SetEmployeeID: SetEmployeeID::<Identity, Impl, OFFSET>,
            FullName: FullName::<Identity, Impl, OFFSET>,
            SetFullName: SetFullName::<Identity, Impl, OFFSET>,
            FirstName: FirstName::<Identity, Impl, OFFSET>,
            SetFirstName: SetFirstName::<Identity, Impl, OFFSET>,
            LastName: LastName::<Identity, Impl, OFFSET>,
            SetLastName: SetLastName::<Identity, Impl, OFFSET>,
            OtherName: OtherName::<Identity, Impl, OFFSET>,
            SetOtherName: SetOtherName::<Identity, Impl, OFFSET>,
            NamePrefix: NamePrefix::<Identity, Impl, OFFSET>,
            SetNamePrefix: SetNamePrefix::<Identity, Impl, OFFSET>,
            NameSuffix: NameSuffix::<Identity, Impl, OFFSET>,
            SetNameSuffix: SetNameSuffix::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            Manager: Manager::<Identity, Impl, OFFSET>,
            SetManager: SetManager::<Identity, Impl, OFFSET>,
            TelephoneHome: TelephoneHome::<Identity, Impl, OFFSET>,
            SetTelephoneHome: SetTelephoneHome::<Identity, Impl, OFFSET>,
            TelephoneMobile: TelephoneMobile::<Identity, Impl, OFFSET>,
            SetTelephoneMobile: SetTelephoneMobile::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            TelephonePager: TelephonePager::<Identity, Impl, OFFSET>,
            SetTelephonePager: SetTelephonePager::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            OfficeLocations: OfficeLocations::<Identity, Impl, OFFSET>,
            SetOfficeLocations: SetOfficeLocations::<Identity, Impl, OFFSET>,
            PostalAddresses: PostalAddresses::<Identity, Impl, OFFSET>,
            SetPostalAddresses: SetPostalAddresses::<Identity, Impl, OFFSET>,
            PostalCodes: PostalCodes::<Identity, Impl, OFFSET>,
            SetPostalCodes: SetPostalCodes::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
            AccountDisabled: AccountDisabled::<Identity, Impl, OFFSET>,
            SetAccountDisabled: SetAccountDisabled::<Identity, Impl, OFFSET>,
            AccountExpirationDate: AccountExpirationDate::<Identity, Impl, OFFSET>,
            SetAccountExpirationDate: SetAccountExpirationDate::<Identity, Impl, OFFSET>,
            GraceLoginsAllowed: GraceLoginsAllowed::<Identity, Impl, OFFSET>,
            SetGraceLoginsAllowed: SetGraceLoginsAllowed::<Identity, Impl, OFFSET>,
            GraceLoginsRemaining: GraceLoginsRemaining::<Identity, Impl, OFFSET>,
            SetGraceLoginsRemaining: SetGraceLoginsRemaining::<Identity, Impl, OFFSET>,
            IsAccountLocked: IsAccountLocked::<Identity, Impl, OFFSET>,
            SetIsAccountLocked: SetIsAccountLocked::<Identity, Impl, OFFSET>,
            LoginHours: LoginHours::<Identity, Impl, OFFSET>,
            SetLoginHours: SetLoginHours::<Identity, Impl, OFFSET>,
            LoginWorkstations: LoginWorkstations::<Identity, Impl, OFFSET>,
            SetLoginWorkstations: SetLoginWorkstations::<Identity, Impl, OFFSET>,
            MaxLogins: MaxLogins::<Identity, Impl, OFFSET>,
            SetMaxLogins: SetMaxLogins::<Identity, Impl, OFFSET>,
            MaxStorage: MaxStorage::<Identity, Impl, OFFSET>,
            SetMaxStorage: SetMaxStorage::<Identity, Impl, OFFSET>,
            PasswordExpirationDate: PasswordExpirationDate::<Identity, Impl, OFFSET>,
            SetPasswordExpirationDate: SetPasswordExpirationDate::<Identity, Impl, OFFSET>,
            PasswordMinimumLength: PasswordMinimumLength::<Identity, Impl, OFFSET>,
            SetPasswordMinimumLength: SetPasswordMinimumLength::<Identity, Impl, OFFSET>,
            PasswordRequired: PasswordRequired::<Identity, Impl, OFFSET>,
            SetPasswordRequired: SetPasswordRequired::<Identity, Impl, OFFSET>,
            RequireUniquePassword: RequireUniquePassword::<Identity, Impl, OFFSET>,
            SetRequireUniquePassword: SetRequireUniquePassword::<Identity, Impl, OFFSET>,
            EmailAddress: EmailAddress::<Identity, Impl, OFFSET>,
            SetEmailAddress: SetEmailAddress::<Identity, Impl, OFFSET>,
            HomeDirectory: HomeDirectory::<Identity, Impl, OFFSET>,
            SetHomeDirectory: SetHomeDirectory::<Identity, Impl, OFFSET>,
            Languages: Languages::<Identity, Impl, OFFSET>,
            SetLanguages: SetLanguages::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            SetProfile: SetProfile::<Identity, Impl, OFFSET>,
            LoginScript: LoginScript::<Identity, Impl, OFFSET>,
            SetLoginScript: SetLoginScript::<Identity, Impl, OFFSET>,
            Picture: Picture::<Identity, Impl, OFFSET>,
            SetPicture: SetPicture::<Identity, Impl, OFFSET>,
            HomePage: HomePage::<Identity, Impl, OFFSET>,
            SetHomePage: SetHomePage::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            ChangePassword: ChangePassword::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsUser as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IADs as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsWinNTSystemInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ComputerName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DomainName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PDC(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IADsWinNTSystemInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsWinNTSystemInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>() -> IADsWinNTSystemInfo_Vtbl {
        unsafe extern "system" fn UserName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PDC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserName: UserName::<Identity, Impl, OFFSET>,
            ComputerName: ComputerName::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
            PDC: PDC::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IADsWinNTSystemInfo as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ICommonQuery_Impl: Sized {
    fn OpenQueryWindow(&self, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::RuntimeName for ICommonQuery {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ICommonQuery_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICommonQuery_Impl, const OFFSET: isize>() -> ICommonQuery_Vtbl {
        unsafe extern "system" fn OpenQueryWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICommonQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenQueryWindow(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pquerywnd), ::core::mem::transmute_copy(&ppdataobject)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenQueryWindow: OpenQueryWindow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICommonQuery as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDirectoryObject_Impl: Sized {
    fn GetObjectInformation(&self) -> ::windows_core::Result<*mut ADS_OBJECT_INFO>;
    fn GetObjectAttributes(&self, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn SetObjectAttributes(&self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows_core::Result<u32>;
    fn CreateDSObject(&self, pszrdnname: &::windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn DeleteDSObject(&self, pszrdnname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDirectoryObject {}
#[cfg(feature = "Win32_System_Com")]
impl IDirectoryObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>() -> IDirectoryObject_Vtbl {
        unsafe extern "system" fn GetObjectInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjinfo: *mut *mut ADS_OBJECT_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObjectInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectAttributes(::core::mem::transmute_copy(&pattributenames), ::core::mem::transmute_copy(&dwnumberattributes), ::core::mem::transmute_copy(&ppattributeentries), ::core::mem::transmute_copy(&pdwnumattributesreturned)).into()
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetObjectAttributes(::core::mem::transmute_copy(&pattributeentries), ::core::mem::transmute_copy(&dwnumattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwnumattributesmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDSObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrdnname: ::windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDSObject(::core::mem::transmute(&pszrdnname), ::core::mem::transmute_copy(&pattributeentries), ::core::mem::transmute_copy(&dwnumattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDSObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrdnname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDSObject(::core::mem::transmute(&pszrdnname)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInformation: GetObjectInformation::<Identity, Impl, OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Identity, Impl, OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Identity, Impl, OFFSET>,
            CreateDSObject: CreateDSObject::<Identity, Impl, OFFSET>,
            DeleteDSObject: DeleteDSObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectoryObject as ::windows_core::Interface>::IID
    }
}
pub trait IDirectorySchemaMgmt_Impl: Sized {
    fn EnumAttributes(&self, ppszattrnames: *const ::windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows_core::Result<()>;
    fn CreateAttributeDefinition(&self, pszattributename: &::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::Result<()>;
    fn WriteAttributeDefinition(&self, pszattributename: &::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::Result<()>;
    fn DeleteAttributeDefinition(&self, pszattributename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumClasses(&self, ppszclassnames: *const ::windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows_core::Result<()>;
    fn WriteClassDefinition(&self, pszclassname: &::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::Result<()>;
    fn CreateClassDefinition(&self, pszclassname: &::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::Result<()>;
    fn DeleteClassDefinition(&self, pszclassname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectorySchemaMgmt {}
impl IDirectorySchemaMgmt_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>() -> IDirectorySchemaMgmt_Vtbl {
        unsafe extern "system" fn EnumAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszattrnames: *const ::windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttributes(::core::mem::transmute_copy(&ppszattrnames), ::core::mem::transmute_copy(&dwnumattributes), ::core::mem::transmute_copy(&ppattrdefinition), ::core::mem::transmute_copy(&pdwnumattributes)).into()
        }
        unsafe extern "system" fn CreateAttributeDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAttributeDefinition(::core::mem::transmute(&pszattributename), ::core::mem::transmute_copy(&pattributedefinition)).into()
        }
        unsafe extern "system" fn WriteAttributeDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteAttributeDefinition(::core::mem::transmute(&pszattributename), ::core::mem::transmute_copy(&pattributedefinition)).into()
        }
        unsafe extern "system" fn DeleteAttributeDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAttributeDefinition(::core::mem::transmute(&pszattributename)).into()
        }
        unsafe extern "system" fn EnumClasses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszclassnames: *const ::windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumClasses(::core::mem::transmute_copy(&ppszclassnames), ::core::mem::transmute_copy(&dwnumclasses), ::core::mem::transmute_copy(&ppclassdefinition), ::core::mem::transmute_copy(&pdwnumclasses)).into()
        }
        unsafe extern "system" fn WriteClassDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteClassDefinition(::core::mem::transmute(&pszclassname), ::core::mem::transmute_copy(&pclassdefinition)).into()
        }
        unsafe extern "system" fn CreateClassDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateClassDefinition(::core::mem::transmute(&pszclassname), ::core::mem::transmute_copy(&pclassdefinition)).into()
        }
        unsafe extern "system" fn DeleteClassDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteClassDefinition(::core::mem::transmute(&pszclassname)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumAttributes: EnumAttributes::<Identity, Impl, OFFSET>,
            CreateAttributeDefinition: CreateAttributeDefinition::<Identity, Impl, OFFSET>,
            WriteAttributeDefinition: WriteAttributeDefinition::<Identity, Impl, OFFSET>,
            DeleteAttributeDefinition: DeleteAttributeDefinition::<Identity, Impl, OFFSET>,
            EnumClasses: EnumClasses::<Identity, Impl, OFFSET>,
            WriteClassDefinition: WriteClassDefinition::<Identity, Impl, OFFSET>,
            CreateClassDefinition: CreateClassDefinition::<Identity, Impl, OFFSET>,
            DeleteClassDefinition: DeleteClassDefinition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectorySchemaMgmt as ::windows_core::Interface>::IID
    }
}
pub trait IDirectorySearch_Impl: Sized {
    fn SetSearchPreference(&self, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> ::windows_core::Result<()>;
    fn ExecuteSearch(&self, pszsearchfilter: &::windows_core::PCWSTR, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32) -> ::windows_core::Result<ADS_SEARCH_HANDLE>;
    fn AbandonSearch(&self, phsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::Result<()>;
    fn GetFirstRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT;
    fn GetNextRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT;
    fn GetPreviousRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT;
    fn GetNextColumnName(&self, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
    fn GetColumn(&self, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: &::windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> ::windows_core::Result<()>;
    fn FreeColumn(&self, psearchcolumn: *const ADS_SEARCH_COLUMN) -> ::windows_core::Result<()>;
    fn CloseSearchHandle(&self, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectorySearch {}
impl IDirectorySearch_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>() -> IDirectorySearch_Vtbl {
        unsafe extern "system" fn SetSearchPreference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSearchPreference(::core::mem::transmute_copy(&psearchprefs), ::core::mem::transmute_copy(&dwnumprefs)).into()
        }
        unsafe extern "system" fn ExecuteSearch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsearchfilter: ::windows_core::PCWSTR, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, phsearchresult: *mut ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecuteSearch(::core::mem::transmute(&pszsearchfilter), ::core::mem::transmute_copy(&pattributenames), ::core::mem::transmute_copy(&dwnumberattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phsearchresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbandonSearch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbandonSearch(::core::mem::transmute_copy(&phsearchresult)).into()
        }
        unsafe extern "system" fn GetFirstRow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFirstRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetNextRow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetPreviousRow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPreviousRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetNextColumnName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextColumnName(::core::mem::transmute_copy(&hsearchhandle), ::core::mem::transmute_copy(&ppszcolumnname))
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: ::windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumn(::core::mem::transmute_copy(&hsearchresult), ::core::mem::transmute(&szcolumnname), ::core::mem::transmute_copy(&psearchcolumn)).into()
        }
        unsafe extern "system" fn FreeColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchcolumn: *const ADS_SEARCH_COLUMN) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeColumn(::core::mem::transmute_copy(&psearchcolumn)).into()
        }
        unsafe extern "system" fn CloseSearchHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseSearchHandle(::core::mem::transmute_copy(&hsearchresult)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSearchPreference: SetSearchPreference::<Identity, Impl, OFFSET>,
            ExecuteSearch: ExecuteSearch::<Identity, Impl, OFFSET>,
            AbandonSearch: AbandonSearch::<Identity, Impl, OFFSET>,
            GetFirstRow: GetFirstRow::<Identity, Impl, OFFSET>,
            GetNextRow: GetNextRow::<Identity, Impl, OFFSET>,
            GetPreviousRow: GetPreviousRow::<Identity, Impl, OFFSET>,
            GetNextColumnName: GetNextColumnName::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            FreeColumn: FreeColumn::<Identity, Impl, OFFSET>,
            CloseSearchHandle: CloseSearchHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectorySearch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDsAdminCreateObj_Impl: Sized {
    fn Initialize(&self, padscontainerobj: ::core::option::Option<&IADsContainer>, padscopysource: ::core::option::Option<&IADs>, lpszclassname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CreateModal(&self, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<IADs>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDsAdminCreateObj {}
#[cfg(feature = "Win32_System_Com")]
impl IDsAdminCreateObj_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>() -> IDsAdminCreateObj_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padscontainerobj: *mut ::core::ffi::c_void, padscopysource: *mut ::core::ffi::c_void, lpszclassname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&padscontainerobj), ::windows_core::from_raw_borrowed(&padscopysource), ::core::mem::transmute(&lpszclassname)).into()
        }
        unsafe extern "system" fn CreateModal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppadsobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateModal(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadsobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateModal: CreateModal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsAdminCreateObj as ::windows_core::Interface>::IID
    }
}
pub trait IDsAdminNewObj_Impl: Sized {
    fn SetButtons(&self, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetPageCounts(&self, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDsAdminNewObj {}
impl IDsAdminNewObj_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>() -> IDsAdminNewObj_Vtbl {
        unsafe extern "system" fn SetButtons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtons(::core::mem::transmute_copy(&ncurrindex), ::core::mem::transmute_copy(&bvalid)).into()
        }
        unsafe extern "system" fn GetPageCounts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPageCounts(::core::mem::transmute_copy(&pntotal), ::core::mem::transmute_copy(&pnstartindex)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetButtons: SetButtons::<Identity, Impl, OFFSET>,
            GetPageCounts: GetPageCounts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsAdminNewObj as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsAdminNewObjExt_Impl: Sized {
    fn Initialize(&self, padscontainerobj: ::core::option::Option<&IADsContainer>, padscopysource: ::core::option::Option<&IADs>, lpszclassname: &::windows_core::PCWSTR, pdsadminnewobj: ::core::option::Option<&IDsAdminNewObj>, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows_core::Result<()>;
    fn AddPages(&self, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetObject(&self, padsobj: ::core::option::Option<&IADs>) -> ::windows_core::Result<()>;
    fn WriteData(&self, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows_core::Result<()>;
    fn OnError(&self, hwnd: super::super::Foundation::HWND, hr: ::windows_core::HRESULT, ucontext: u32) -> ::windows_core::Result<()>;
    fn GetSummaryInfo(&self, pbstrtext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IDsAdminNewObjExt {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDsAdminNewObjExt_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>() -> IDsAdminNewObjExt_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padscontainerobj: *mut ::core::ffi::c_void, padscopysource: *mut ::core::ffi::c_void, lpszclassname: ::windows_core::PCWSTR, pdsadminnewobj: *mut ::core::ffi::c_void, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&padscontainerobj), ::windows_core::from_raw_borrowed(&padscopysource), ::core::mem::transmute(&lpszclassname), ::windows_core::from_raw_borrowed(&pdsadminnewobj), ::core::mem::transmute_copy(&pdispinfo)).into()
        }
        unsafe extern "system" fn AddPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPages(::core::mem::transmute_copy(&lpfnaddpage), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padsobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObject(::windows_core::from_raw_borrowed(&padsobj)).into()
        }
        unsafe extern "system" fn WriteData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteData(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ucontext)).into()
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, hr: ::windows_core::HRESULT, ucontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnError(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&ucontext)).into()
        }
        unsafe extern "system" fn GetSummaryInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSummaryInfo(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            AddPages: AddPages::<Identity, Impl, OFFSET>,
            SetObject: SetObject::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
            GetSummaryInfo: GetSummaryInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsAdminNewObjExt as ::windows_core::Interface>::IID
    }
}
pub trait IDsAdminNewObjPrimarySite_Impl: Sized {
    fn CreateNew(&self, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Commit(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDsAdminNewObjPrimarySite {}
impl IDsAdminNewObjPrimarySite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>() -> IDsAdminNewObjPrimarySite_Vtbl {
        unsafe extern "system" fn CreateNew<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNew(::core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateNew: CreateNew::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsAdminNewObjPrimarySite as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDsAdminNotifyHandler_Impl: Sized {
    fn Initialize(&self, pextrainfo: ::core::option::Option<&super::super::System::Com::IDataObject>, pueventflags: *mut u32) -> ::windows_core::Result<()>;
    fn Begin(&self, uevent: u32, parg1: ::core::option::Option<&super::super::System::Com::IDataObject>, parg2: ::core::option::Option<&super::super::System::Com::IDataObject>, puflags: *mut u32, pbstr: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Notify(&self, nitem: u32, uflags: u32) -> ::windows_core::Result<()>;
    fn End(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDsAdminNotifyHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IDsAdminNotifyHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>() -> IDsAdminNotifyHandler_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextrainfo: *mut ::core::ffi::c_void, pueventflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pextrainfo), ::core::mem::transmute_copy(&pueventflags)).into()
        }
        unsafe extern "system" fn Begin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uevent: u32, parg1: *mut ::core::ffi::c_void, parg2: *mut ::core::ffi::c_void, puflags: *mut u32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin(::core::mem::transmute_copy(&uevent), ::windows_core::from_raw_borrowed(&parg1), ::windows_core::from_raw_borrowed(&parg2), ::core::mem::transmute_copy(&puflags), ::core::mem::transmute_copy(&pbstr)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitem: u32, uflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify(::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn End<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.End().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsAdminNotifyHandler as ::windows_core::Interface>::IID
    }
}
pub trait IDsBrowseDomainTree_Impl: Sized {
    fn BrowseTo(&self, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut ::windows_core::PWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows_core::Result<()>;
    fn FreeDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows_core::Result<()>;
    fn FlushCachedDomains(&self) -> ::windows_core::Result<()>;
    fn SetComputer(&self, pszcomputername: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR, pszpassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDsBrowseDomainTree {}
impl IDsBrowseDomainTree_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>() -> IDsBrowseDomainTree_Vtbl {
        unsafe extern "system" fn BrowseTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut ::windows_core::PWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BrowseTo(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ppsztargetpath), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetDomains<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDomains(::core::mem::transmute_copy(&ppdomaintree), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FreeDomains<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeDomains(::core::mem::transmute_copy(&ppdomaintree)).into()
        }
        unsafe extern "system" fn FlushCachedDomains<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushCachedDomains().into()
        }
        unsafe extern "system" fn SetComputer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcomputername: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetComputer(::core::mem::transmute(&pszcomputername), ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszpassword)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BrowseTo: BrowseTo::<Identity, Impl, OFFSET>,
            GetDomains: GetDomains::<Identity, Impl, OFFSET>,
            FreeDomains: FreeDomains::<Identity, Impl, OFFSET>,
            FlushCachedDomains: FlushCachedDomains::<Identity, Impl, OFFSET>,
            SetComputer: SetComputer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsBrowseDomainTree as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IDsDisplaySpecifier_Impl: Sized {
    fn SetServer(&self, pszserver: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR, pszpassword: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn SetLanguageID(&self, langid: u16) -> ::windows_core::Result<()>;
    fn GetDisplaySpecifier(&self, pszobjectclass: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetIconLocation(&self, pszobjectclass: &::windows_core::PCWSTR, dwflags: u32, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows_core::Result<()>;
    fn GetIcon(&self, pszobjectclass: &::windows_core::PCWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON;
    fn GetFriendlyClassName(&self, pszobjectclass: &::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::Result<()>;
    fn GetFriendlyAttributeName(&self, pszobjectclass: &::windows_core::PCWSTR, pszattributename: &::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: u32) -> ::windows_core::Result<()>;
    fn IsClassContainer(&self, pszobjectclass: &::windows_core::PCWSTR, pszadspath: &::windows_core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    fn GetClassCreationInfo(&self, pszobjectclass: &::windows_core::PCWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows_core::Result<()>;
    fn EnumClassAttributes(&self, pszobjectclass: &::windows_core::PCWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn GetAttributeADsType(&self, pszattributename: &::windows_core::PCWSTR) -> ADSTYPE;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::RuntimeName for IDsDisplaySpecifier {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IDsDisplaySpecifier_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>() -> IDsDisplaySpecifier_Vtbl {
        unsafe extern "system" fn SetServer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserver: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServer(::core::mem::transmute(&pszserver), ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszpassword), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetLanguageID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguageID(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetDisplaySpecifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplaySpecifier(::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetIconLocation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, dwflags: u32, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIconLocation(::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer), ::core::mem::transmute_copy(&presid)).into()
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIcon(::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cxicon), ::core::mem::transmute_copy(&cyicon))
        }
        unsafe extern "system" fn GetFriendlyClassName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFriendlyClassName(::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn GetFriendlyAttributeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszattributename: ::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFriendlyAttributeName(::core::mem::transmute(&pszobjectclass), ::core::mem::transmute(&pszattributename), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn IsClassContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszadspath: ::windows_core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsClassContainer(::core::mem::transmute(&pszobjectclass), ::core::mem::transmute(&pszadspath), ::core::mem::transmute_copy(&dwflags))
        }
        unsafe extern "system" fn GetClassCreationInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassCreationInfo(::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&ppdscci)).into()
        }
        unsafe extern "system" fn EnumClassAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumClassAttributes(::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&pcbenum), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn GetAttributeADsType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR) -> ADSTYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributeADsType(::core::mem::transmute(&pszattributename))
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            SetLanguageID: SetLanguageID::<Identity, Impl, OFFSET>,
            GetDisplaySpecifier: GetDisplaySpecifier::<Identity, Impl, OFFSET>,
            GetIconLocation: GetIconLocation::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetFriendlyClassName: GetFriendlyClassName::<Identity, Impl, OFFSET>,
            GetFriendlyAttributeName: GetFriendlyAttributeName::<Identity, Impl, OFFSET>,
            IsClassContainer: IsClassContainer::<Identity, Impl, OFFSET>,
            GetClassCreationInfo: GetClassCreationInfo::<Identity, Impl, OFFSET>,
            EnumClassAttributes: EnumClassAttributes::<Identity, Impl, OFFSET>,
            GetAttributeADsType: GetAttributeADsType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsDisplaySpecifier as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDsObjectPicker_Impl: Sized {
    fn Initialize(&self, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows_core::Result<()>;
    fn InvokeDialog(&self, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDsObjectPicker {}
#[cfg(feature = "Win32_System_Com")]
impl IDsObjectPicker_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: isize>() -> IDsObjectPicker_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&pinitinfo)).into()
        }
        unsafe extern "system" fn InvokeDialog<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppdoselections: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InvokeDialog(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdoselections, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            InvokeDialog: InvokeDialog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsObjectPicker as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDsObjectPickerCredentials_Impl: Sized + IDsObjectPicker_Impl {
    fn SetCredentials(&self, szusername: &::windows_core::PCWSTR, szpassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDsObjectPickerCredentials {}
#[cfg(feature = "Win32_System_Com")]
impl IDsObjectPickerCredentials_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPickerCredentials_Impl, const OFFSET: isize>() -> IDsObjectPickerCredentials_Vtbl {
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPickerCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szusername: ::windows_core::PCWSTR, szpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentials(::core::mem::transmute(&szusername), ::core::mem::transmute(&szpassword)).into()
        }
        Self { base__: IDsObjectPicker_Vtbl::new::<Identity, Impl, OFFSET>(), SetCredentials: SetCredentials::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDsObjectPickerCredentials as ::windows_core::Interface>::IID || iid == &<IDsObjectPicker as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistQuery_Impl: Sized + super::super::System::Com::IPersist_Impl {
    fn WriteString(&self, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ReadString(&self, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::Result<()>;
    fn WriteInt(&self, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, value: i32) -> ::windows_core::Result<()>;
    fn ReadInt(&self, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pvalue: *mut i32) -> ::windows_core::Result<()>;
    fn WriteStruct(&self, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::Result<()>;
    fn ReadStruct(&self, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::Result<()>;
    fn Clear(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IPersistQuery {}
#[cfg(feature = "Win32_System_Com")]
impl IPersistQuery_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>() -> IPersistQuery_Vtbl {
        unsafe extern "system" fn WriteString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteString(::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn ReadString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadString(::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn WriteInt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteInt(::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadInt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadInt(::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteStruct<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteStruct(::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&pstruct), ::core::mem::transmute_copy(&cbstruct)).into()
        }
        unsafe extern "system" fn ReadStruct<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadStruct(::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&pstruct), ::core::mem::transmute_copy(&cbstruct)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: super::super::System::Com::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            ReadString: ReadString::<Identity, Impl, OFFSET>,
            WriteInt: WriteInt::<Identity, Impl, OFFSET>,
            ReadInt: ReadInt::<Identity, Impl, OFFSET>,
            WriteStruct: WriteStruct::<Identity, Impl, OFFSET>,
            ReadStruct: ReadStruct::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPersistQuery as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersist as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrivateDispatch_Impl: Sized {
    fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> ::windows_core::Result<()>;
    fn ADSIGetTypeInfoCount(&self) -> ::windows_core::Result<u32>;
    fn ADSIGetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows_core::Result<super::super::System::Com::ITypeInfo>;
    fn ADSIGetIDsOfNames(&self, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows_core::Result<i32>;
    fn ADSIInvoke(&self, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrivateDispatch {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrivateDispatch_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>() -> IPrivateDispatch_Vtbl {
        unsafe extern "system" fn ADSIInitializeDispatchManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwextensionid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ADSIInitializeDispatchManager(::core::mem::transmute_copy(&dwextensionid)).into()
        }
        unsafe extern "system" fn ADSIGetTypeInfoCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADSIGetTypeInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIGetTypeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADSIGetTypeInfo(::core::mem::transmute_copy(&itinfo), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIGetIDsOfNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADSIGetIDsOfNames(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rgdispid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIInvoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ADSIInvoke(::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ADSIInitializeDispatchManager: ADSIInitializeDispatchManager::<Identity, Impl, OFFSET>,
            ADSIGetTypeInfoCount: ADSIGetTypeInfoCount::<Identity, Impl, OFFSET>,
            ADSIGetTypeInfo: ADSIGetTypeInfo::<Identity, Impl, OFFSET>,
            ADSIGetIDsOfNames: ADSIGetIDsOfNames::<Identity, Impl, OFFSET>,
            ADSIInvoke: ADSIInvoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrivateDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IPrivateUnknown_Impl: Sized {
    fn ADSIInitializeObject(&self, lpszusername: &::windows_core::BSTR, lpszpassword: &::windows_core::BSTR, lnreserved: i32) -> ::windows_core::Result<()>;
    fn ADSIReleaseObject(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrivateUnknown {}
impl IPrivateUnknown_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: isize>() -> IPrivateUnknown_Vtbl {
        unsafe extern "system" fn ADSIInitializeObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnreserved: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ADSIInitializeObject(::core::mem::transmute(&lpszusername), ::core::mem::transmute(&lpszpassword), ::core::mem::transmute_copy(&lnreserved)).into()
        }
        unsafe extern "system" fn ADSIReleaseObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ADSIReleaseObject().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ADSIInitializeObject: ADSIInitializeObject::<Identity, Impl, OFFSET>,
            ADSIReleaseObject: ADSIReleaseObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrivateUnknown as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Registry\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IQueryForm_Impl: Sized {
    fn Initialize(&self, hkform: super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
    fn AddForms(&self, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn AddPages(&self, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IQueryForm {}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl IQueryForm_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: isize>() -> IQueryForm_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkform: super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&hkform)).into()
        }
        unsafe extern "system" fn AddForms<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddForms(::core::mem::transmute_copy(&paddformsproc), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn AddPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPages(::core::mem::transmute_copy(&paddpagesproc), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            AddForms: AddForms::<Identity, Impl, OFFSET>,
            AddPages: AddPages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IQueryForm as ::windows_core::Interface>::IID
    }
}
