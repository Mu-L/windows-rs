#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIAssociatedIdentityProvider_Impl: Sized {
    fn Begin_AssociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Finish_AssociateIdentity(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_DisassociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_DisassociateIdentity(&self) -> ::windows_core::Result<()>;
    fn Begin_ChangeCredential(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_ChangeCredential(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::RuntimeName for AsyncIAssociatedIdentityProvider {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIAssociatedIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>() -> AsyncIAssociatedIdentityProvider_Vtbl {
        unsafe extern "system" fn Begin_AssociateIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_AssociateIdentity(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn Finish_AssociateIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_AssociateIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_DisassociateIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_DisassociateIdentity(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_DisassociateIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_DisassociateIdentity().into()
        }
        unsafe extern "system" fn Begin_ChangeCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_ChangeCredential(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_ChangeCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_ChangeCredential().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_AssociateIdentity: Begin_AssociateIdentity::<Identity, Impl, OFFSET>,
            Finish_AssociateIdentity: Finish_AssociateIdentity::<Identity, Impl, OFFSET>,
            Begin_DisassociateIdentity: Begin_DisassociateIdentity::<Identity, Impl, OFFSET>,
            Finish_DisassociateIdentity: Finish_DisassociateIdentity::<Identity, Impl, OFFSET>,
            Begin_ChangeCredential: Begin_ChangeCredential::<Identity, Impl, OFFSET>,
            Finish_ChangeCredential: Finish_ChangeCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<AsyncIAssociatedIdentityProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait AsyncIConnectedIdentityProvider_Impl: Sized {
    fn Begin_ConnectIdentity(&self, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::Result<()>;
    fn Finish_ConnectIdentity(&self) -> ::windows_core::Result<()>;
    fn Begin_DisconnectIdentity(&self) -> ::windows_core::Result<()>;
    fn Finish_DisconnectIdentity(&self) -> ::windows_core::Result<()>;
    fn Begin_IsConnected(&self) -> ::windows_core::Result<()>;
    fn Finish_IsConnected(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn Begin_GetUrl(&self, identifier: IDENTITY_URL, context: ::core::option::Option<&super::super::super::super::System::Com::IBindCtx>) -> ::windows_core::Result<()>;
    fn Finish_GetUrl(&self, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn Begin_GetAccountState(&self) -> ::windows_core::Result<()>;
    fn Finish_GetAccountState(&self) -> ::windows_core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for AsyncIConnectedIdentityProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl AsyncIConnectedIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>() -> AsyncIConnectedIdentityProvider_Vtbl {
        unsafe extern "system" fn Begin_ConnectIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_ConnectIdentity(::core::mem::transmute_copy(&authbuffer), ::core::mem::transmute_copy(&authbuffersize)).into()
        }
        unsafe extern "system" fn Finish_ConnectIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_ConnectIdentity().into()
        }
        unsafe extern "system" fn Begin_DisconnectIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_DisconnectIdentity().into()
        }
        unsafe extern "system" fn Finish_DisconnectIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_DisconnectIdentity().into()
        }
        unsafe extern "system" fn Begin_IsConnected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_IsConnected().into()
        }
        unsafe extern "system" fn Finish_IsConnected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_GetUrl(::core::mem::transmute_copy(&identifier), ::windows_core::from_raw_borrowed(&context)).into()
        }
        unsafe extern "system" fn Finish_GetUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_GetUrl(::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn Begin_GetAccountState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_GetAccountState().into()
        }
        unsafe extern "system" fn Finish_GetAccountState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_GetAccountState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_ConnectIdentity: Begin_ConnectIdentity::<Identity, Impl, OFFSET>,
            Finish_ConnectIdentity: Finish_ConnectIdentity::<Identity, Impl, OFFSET>,
            Begin_DisconnectIdentity: Begin_DisconnectIdentity::<Identity, Impl, OFFSET>,
            Finish_DisconnectIdentity: Finish_DisconnectIdentity::<Identity, Impl, OFFSET>,
            Begin_IsConnected: Begin_IsConnected::<Identity, Impl, OFFSET>,
            Finish_IsConnected: Finish_IsConnected::<Identity, Impl, OFFSET>,
            Begin_GetUrl: Begin_GetUrl::<Identity, Impl, OFFSET>,
            Finish_GetUrl: Finish_GetUrl::<Identity, Impl, OFFSET>,
            Begin_GetAccountState: Begin_GetAccountState::<Identity, Impl, OFFSET>,
            Finish_GetAccountState: Finish_GetAccountState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<AsyncIConnectedIdentityProvider as ::windows_core::Interface>::IID
    }
}
pub trait AsyncIIdentityAdvise_Impl: Sized {
    fn Begin_IdentityUpdated(&self, dwidentityupdateevents: u32, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_IdentityUpdated(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for AsyncIIdentityAdvise {}
impl AsyncIIdentityAdvise_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: isize>() -> AsyncIIdentityAdvise_Vtbl {
        unsafe extern "system" fn Begin_IdentityUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_IdentityUpdated(::core::mem::transmute_copy(&dwidentityupdateevents), ::core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_IdentityUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_IdentityUpdated().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_IdentityUpdated: Begin_IdentityUpdated::<Identity, Impl, OFFSET>,
            Finish_IdentityUpdated: Finish_IdentityUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityAdvise as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIIdentityAuthentication_Impl: Sized {
    fn Begin_SetIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::Result<()>;
    fn Finish_SetIdentityCredential(&self) -> ::windows_core::Result<()>;
    fn Begin_ValidateIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Finish_ValidateIdentityCredential(&self, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::RuntimeName for AsyncIIdentityAuthentication {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIIdentityAuthentication_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>() -> AsyncIIdentityAuthentication_Vtbl {
        unsafe extern "system" fn Begin_SetIdentityCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_SetIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength)).into()
        }
        unsafe extern "system" fn Finish_SetIdentityCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_SetIdentityCredential().into()
        }
        unsafe extern "system" fn Begin_ValidateIdentityCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_ValidateIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength), ::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        unsafe extern "system" fn Finish_ValidateIdentityCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_ValidateIdentityCredential(::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_SetIdentityCredential: Begin_SetIdentityCredential::<Identity, Impl, OFFSET>,
            Finish_SetIdentityCredential: Finish_SetIdentityCredential::<Identity, Impl, OFFSET>,
            Begin_ValidateIdentityCredential: Begin_ValidateIdentityCredential::<Identity, Impl, OFFSET>,
            Finish_ValidateIdentityCredential: Finish_ValidateIdentityCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityAuthentication as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityProvider_Impl: Sized {
    fn Begin_GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Finish_GetIdentityEnum(&self) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Create(&self, lpszusername: &::windows_core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Finish_Create(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Import(&self, ppropertystore: ::core::option::Option<&super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Finish_Import(&self) -> ::windows_core::Result<()>;
    fn Begin_Delete(&self, lpszuniqueid: &::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Finish_Delete(&self) -> ::windows_core::Result<()>;
    fn Begin_FindByUniqueID(&self, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_FindByUniqueID(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_GetProviderPropertyStore(&self) -> ::windows_core::Result<()>;
    fn Finish_GetProviderPropertyStore(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Advise(&self, pidentityadvise: ::core::option::Option<&IIdentityAdvise>, dwidentityupdateevents: u32) -> ::windows_core::Result<()>;
    fn Finish_Advise(&self) -> ::windows_core::Result<u32>;
    fn Begin_UnAdvise(&self, dwcookie: u32) -> ::windows_core::Result<()>;
    fn Finish_UnAdvise(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for AsyncIIdentityProvider {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>() -> AsyncIIdentityProvider_Vtbl {
        unsafe extern "system" fn Begin_GetIdentityEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_GetIdentityEnum(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)).into()
        }
        unsafe extern "system" fn Finish_GetIdentityEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_GetIdentityEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidentityenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Create<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: ::windows_core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Create(::core::mem::transmute(&lpszusername), ::core::mem::transmute_copy(&pkeywordstoadd)).into()
        }
        unsafe extern "system" fn Finish_Create<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_Create() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Import<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Import(::windows_core::from_raw_borrowed(&ppropertystore)).into()
        }
        unsafe extern "system" fn Finish_Import<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Import().into()
        }
        unsafe extern "system" fn Begin_Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Delete(::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&pkeywordstodelete)).into()
        }
        unsafe extern "system" fn Finish_Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Delete().into()
        }
        unsafe extern "system" fn Begin_FindByUniqueID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_FindByUniqueID(::core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_FindByUniqueID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_FindByUniqueID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetProviderPropertyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_GetProviderPropertyStore().into()
        }
        unsafe extern "system" fn Finish_GetProviderPropertyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_GetProviderPropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Advise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Advise(::windows_core::from_raw_borrowed(&pidentityadvise), ::core::mem::transmute_copy(&dwidentityupdateevents)).into()
        }
        unsafe extern "system" fn Finish_Advise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_Advise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_UnAdvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_UnAdvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn Finish_UnAdvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_UnAdvise().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_GetIdentityEnum: Begin_GetIdentityEnum::<Identity, Impl, OFFSET>,
            Finish_GetIdentityEnum: Finish_GetIdentityEnum::<Identity, Impl, OFFSET>,
            Begin_Create: Begin_Create::<Identity, Impl, OFFSET>,
            Finish_Create: Finish_Create::<Identity, Impl, OFFSET>,
            Begin_Import: Begin_Import::<Identity, Impl, OFFSET>,
            Finish_Import: Finish_Import::<Identity, Impl, OFFSET>,
            Begin_Delete: Begin_Delete::<Identity, Impl, OFFSET>,
            Finish_Delete: Finish_Delete::<Identity, Impl, OFFSET>,
            Begin_FindByUniqueID: Begin_FindByUniqueID::<Identity, Impl, OFFSET>,
            Finish_FindByUniqueID: Finish_FindByUniqueID::<Identity, Impl, OFFSET>,
            Begin_GetProviderPropertyStore: Begin_GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Finish_GetProviderPropertyStore: Finish_GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Begin_Advise: Begin_Advise::<Identity, Impl, OFFSET>,
            Finish_Advise: Finish_Advise::<Identity, Impl, OFFSET>,
            Begin_UnAdvise: Begin_UnAdvise::<Identity, Impl, OFFSET>,
            Finish_UnAdvise: Finish_UnAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityStore_Impl: Sized {
    fn Begin_GetCount(&self) -> ::windows_core::Result<()>;
    fn Finish_GetCount(&self) -> ::windows_core::Result<u32>;
    fn Begin_GetAt(&self, dwprovider: u32, pprovguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_GetAt(&self, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Begin_AddToCache(&self, lpszuniqueid: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_AddToCache(&self) -> ::windows_core::Result<()>;
    fn Begin_ConvertToSid(&self, lpszuniqueid: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8) -> ::windows_core::Result<()>;
    fn Finish_ConvertToSid(&self, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::Result<()>;
    fn Begin_EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Finish_EnumerateIdentities(&self) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Reset(&self) -> ::windows_core::Result<()>;
    fn Finish_Reset(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for AsyncIIdentityStore {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>() -> AsyncIIdentityStore_Vtbl {
        unsafe extern "system" fn Begin_GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_GetCount().into()
        }
        unsafe extern "system" fn Finish_GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwproviders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_GetAt(::core::mem::transmute_copy(&dwprovider), ::core::mem::transmute_copy(&pprovguid)).into()
        }
        unsafe extern "system" fn Finish_GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_GetAt(::core::mem::transmute_copy(&pprovguid), ::core::mem::transmute_copy(&ppidentityprovider)).into()
        }
        unsafe extern "system" fn Begin_AddToCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_AddToCache(::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_AddToCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_AddToCache().into()
        }
        unsafe extern "system" fn Begin_ConvertToSid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_ConvertToSid(::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid), ::core::mem::transmute_copy(&cbsid), ::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn Finish_ConvertToSid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_ConvertToSid(::core::mem::transmute_copy(&psid), ::core::mem::transmute_copy(&pcbrequiredsid)).into()
        }
        unsafe extern "system" fn Begin_EnumerateIdentities<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_EnumerateIdentities(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)).into()
        }
        unsafe extern "system" fn Finish_EnumerateIdentities<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Finish_EnumerateIdentities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidentityenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Reset().into()
        }
        unsafe extern "system" fn Finish_Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Reset().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_GetCount: Begin_GetCount::<Identity, Impl, OFFSET>,
            Finish_GetCount: Finish_GetCount::<Identity, Impl, OFFSET>,
            Begin_GetAt: Begin_GetAt::<Identity, Impl, OFFSET>,
            Finish_GetAt: Finish_GetAt::<Identity, Impl, OFFSET>,
            Begin_AddToCache: Begin_AddToCache::<Identity, Impl, OFFSET>,
            Finish_AddToCache: Finish_AddToCache::<Identity, Impl, OFFSET>,
            Begin_ConvertToSid: Begin_ConvertToSid::<Identity, Impl, OFFSET>,
            Finish_ConvertToSid: Finish_ConvertToSid::<Identity, Impl, OFFSET>,
            Begin_EnumerateIdentities: Begin_EnumerateIdentities::<Identity, Impl, OFFSET>,
            Finish_EnumerateIdentities: Finish_EnumerateIdentities::<Identity, Impl, OFFSET>,
            Begin_Reset: Begin_Reset::<Identity, Impl, OFFSET>,
            Finish_Reset: Finish_Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityStore as ::windows_core::Interface>::IID
    }
}
pub trait AsyncIIdentityStoreEx_Impl: Sized {
    fn Begin_CreateConnectedIdentity(&self, localname: &::windows_core::PCWSTR, connectedname: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_CreateConnectedIdentity(&self) -> ::windows_core::Result<()>;
    fn Begin_DeleteConnectedIdentity(&self, connectedname: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_DeleteConnectedIdentity(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for AsyncIIdentityStoreEx {}
impl AsyncIIdentityStoreEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>() -> AsyncIIdentityStoreEx_Vtbl {
        unsafe extern "system" fn Begin_CreateConnectedIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: ::windows_core::PCWSTR, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_CreateConnectedIdentity(::core::mem::transmute(&localname), ::core::mem::transmute(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_CreateConnectedIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_CreateConnectedIdentity().into()
        }
        unsafe extern "system" fn Begin_DeleteConnectedIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_DeleteConnectedIdentity(::core::mem::transmute(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_DeleteConnectedIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_DeleteConnectedIdentity().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_CreateConnectedIdentity: Begin_CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            Finish_CreateConnectedIdentity: Finish_CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            Begin_DeleteConnectedIdentity: Begin_DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
            Finish_DeleteConnectedIdentity: Finish_DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityStoreEx as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAssociatedIdentityProvider_Impl: Sized {
    fn AssociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn DisassociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ChangeCredential(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::RuntimeName for IAssociatedIdentityProvider {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAssociatedIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: isize>() -> IAssociatedIdentityProvider_Vtbl {
        unsafe extern "system" fn AssociateIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AssociateIdentity(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisassociateIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisassociateIdentity(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn ChangeCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeCredential(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&lpszuniqueid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AssociateIdentity: AssociateIdentity::<Identity, Impl, OFFSET>,
            DisassociateIdentity: DisassociateIdentity::<Identity, Impl, OFFSET>,
            ChangeCredential: ChangeCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAssociatedIdentityProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IConnectedIdentityProvider_Impl: Sized {
    fn ConnectIdentity(&self, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::Result<()>;
    fn DisconnectIdentity(&self) -> ::windows_core::Result<()>;
    fn IsConnected(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn GetUrl(&self, identifier: IDENTITY_URL, context: ::core::option::Option<&super::super::super::super::System::Com::IBindCtx>, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetAccountState(&self) -> ::windows_core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IConnectedIdentityProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IConnectedIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>() -> IConnectedIdentityProvider_Vtbl {
        unsafe extern "system" fn ConnectIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectIdentity(::core::mem::transmute_copy(&authbuffer), ::core::mem::transmute_copy(&authbuffersize)).into()
        }
        unsafe extern "system" fn DisconnectIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectIdentity().into()
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUrl(::core::mem::transmute_copy(&identifier), ::windows_core::from_raw_borrowed(&context), ::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn GetAccountState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccountState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectIdentity: ConnectIdentity::<Identity, Impl, OFFSET>,
            DisconnectIdentity: DisconnectIdentity::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetUrl: GetUrl::<Identity, Impl, OFFSET>,
            GetAccountState: GetAccountState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConnectedIdentityProvider as ::windows_core::Interface>::IID
    }
}
pub trait IIdentityAdvise_Impl: Sized {
    fn IdentityUpdated(&self, dwidentityupdateevents: &IdentityUpdateEvent, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IIdentityAdvise {}
impl IIdentityAdvise_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityAdvise_Impl, const OFFSET: isize>() -> IIdentityAdvise_Vtbl {
        unsafe extern "system" fn IdentityUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityAdvise_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IdentityUpdated(::core::mem::transmute(&dwidentityupdateevents), ::core::mem::transmute(&lpszuniqueid)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IdentityUpdated: IdentityUpdated::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIdentityAdvise as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IIdentityAuthentication_Impl: Sized {
    fn SetIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::Result<()>;
    fn ValidateIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::RuntimeName for IIdentityAuthentication {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IIdentityAuthentication_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityAuthentication_Impl, const OFFSET: isize>() -> IIdentityAuthentication_Vtbl {
        unsafe extern "system" fn SetIdentityCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength)).into()
        }
        unsafe extern "system" fn ValidateIdentityCredential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ValidateIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength), ::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIdentityCredential: SetIdentityCredential::<Identity, Impl, OFFSET>,
            ValidateIdentityCredential: ValidateIdentityCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIdentityAuthentication as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityProvider_Impl: Sized {
    fn GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Create(&self, lpszusername: &::windows_core::PCWSTR, pppropertystore: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Import(&self, ppropertystore: ::core::option::Option<&super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Delete(&self, lpszuniqueid: &::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn FindByUniqueID(&self, lpszuniqueid: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetProviderPropertyStore(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Advise(&self, pidentityadvise: ::core::option::Option<&IIdentityAdvise>, dwidentityupdateevents: &IdentityUpdateEvent) -> ::windows_core::Result<u32>;
    fn UnAdvise(&self, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for IIdentityProvider {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>() -> IIdentityProvider_Vtbl {
        unsafe extern "system" fn GetIdentityEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIdentityEnum(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidentityenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: ::windows_core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute(&lpszusername), ::core::mem::transmute_copy(&pppropertystore), ::core::mem::transmute_copy(&pkeywordstoadd)).into()
        }
        unsafe extern "system" fn Import<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Import(::windows_core::from_raw_borrowed(&ppropertystore)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&pkeywordstodelete)).into()
        }
        unsafe extern "system" fn FindByUniqueID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindByUniqueID(::core::mem::transmute(&lpszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderPropertyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProviderPropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Advise(::windows_core::from_raw_borrowed(&pidentityadvise), ::core::mem::transmute(&dwidentityupdateevents)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnAdvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdentityEnum: GetIdentityEnum::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            FindByUniqueID: FindByUniqueID::<Identity, Impl, OFFSET>,
            GetProviderPropertyStore: GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            UnAdvise: UnAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIdentityProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityStore_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, dwprovider: u32, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn AddToCache(&self, lpszuniqueid: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ConvertToSid(&self, lpszuniqueid: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::Result<()>;
    fn EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Reset(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for IIdentityStore {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: isize>() -> IIdentityStore_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwproviders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAt(::core::mem::transmute_copy(&dwprovider), ::core::mem::transmute_copy(&pprovguid), ::core::mem::transmute_copy(&ppidentityprovider)).into()
        }
        unsafe extern "system" fn AddToCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToCache(::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn ConvertToSid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertToSid(::core::mem::transmute(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid), ::core::mem::transmute_copy(&cbsid), ::core::mem::transmute_copy(&psid), ::core::mem::transmute_copy(&pcbrequiredsid)).into()
        }
        unsafe extern "system" fn EnumerateIdentities<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateIdentities(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidentityenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            AddToCache: AddToCache::<Identity, Impl, OFFSET>,
            ConvertToSid: ConvertToSid::<Identity, Impl, OFFSET>,
            EnumerateIdentities: EnumerateIdentities::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIdentityStore as ::windows_core::Interface>::IID
    }
}
pub trait IIdentityStoreEx_Impl: Sized {
    fn CreateConnectedIdentity(&self, localname: &::windows_core::PCWSTR, connectedname: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn DeleteConnectedIdentity(&self, connectedname: &::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IIdentityStoreEx {}
impl IIdentityStoreEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStoreEx_Impl, const OFFSET: isize>() -> IIdentityStoreEx_Vtbl {
        unsafe extern "system" fn CreateConnectedIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: ::windows_core::PCWSTR, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateConnectedIdentity(::core::mem::transmute(&localname), ::core::mem::transmute(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn DeleteConnectedIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteConnectedIdentity(::core::mem::transmute(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateConnectedIdentity: CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            DeleteConnectedIdentity: DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIdentityStoreEx as ::windows_core::Interface>::IID
    }
}
