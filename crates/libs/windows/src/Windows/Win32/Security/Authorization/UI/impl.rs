pub trait IEffectivePermission_Impl: Sized {
    fn GetEffectivePermission(&self, pguidobjecttype: *const ::windows_core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: &::windows_core::PCWSTR, psd: super::super::PSECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEffectivePermission {}
impl IEffectivePermission_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEffectivePermission_Impl, const OFFSET: isize>() -> IEffectivePermission_Vtbl {
        unsafe extern "system" fn GetEffectivePermission<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEffectivePermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows_core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: ::windows_core::PCWSTR, psd: super::super::PSECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEffectivePermission(::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&pusersid), ::core::mem::transmute(&pszservername), ::core::mem::transmute_copy(&psd), ::core::mem::transmute_copy(&ppobjecttypelist), ::core::mem::transmute_copy(&pcobjecttypelistlength), ::core::mem::transmute_copy(&ppgrantedaccesslist), ::core::mem::transmute_copy(&pcgrantedaccesslistlength)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetEffectivePermission: GetEffectivePermission::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEffectivePermission as ::windows_core::Interface>::IID
    }
}
pub trait IEffectivePermission2_Impl: Sized {
    fn ComputeEffectivePermissionWithSecondarySecurity(
        &self,
        psid: super::super::super::Foundation::PSID,
        pdevicesid: super::super::super::Foundation::PSID,
        pszservername: &::windows_core::PCWSTR,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: *const super::super::TOKEN_GROUPS,
        pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pdevicegroups: *const super::super::TOKEN_GROUPS,
        pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEffectivePermission2 {}
impl IEffectivePermission2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEffectivePermission2_Impl, const OFFSET: isize>() -> IEffectivePermission2_Vtbl {
        unsafe extern "system" fn ComputeEffectivePermissionWithSecondarySecurity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEffectivePermission2_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            psid: super::super::super::Foundation::PSID,
            pdevicesid: super::super::super::Foundation::PSID,
            pszservername: ::windows_core::PCWSTR,
            psecurityobjects: *mut SECURITY_OBJECT,
            dwsecurityobjectcount: u32,
            pusergroups: *const super::super::TOKEN_GROUPS,
            pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
            pdevicegroups: *const super::super::TOKEN_GROUPS,
            pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
            pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
            pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
            pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
            pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
            peffpermresultlists: *mut EFFPERM_RESULT_LIST,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ComputeEffectivePermissionWithSecondarySecurity(
                ::core::mem::transmute_copy(&psid),
                ::core::mem::transmute_copy(&pdevicesid),
                ::core::mem::transmute(&pszservername),
                ::core::mem::transmute_copy(&psecurityobjects),
                ::core::mem::transmute_copy(&dwsecurityobjectcount),
                ::core::mem::transmute_copy(&pusergroups),
                ::core::mem::transmute_copy(&pauthzusergroupsoperations),
                ::core::mem::transmute_copy(&pdevicegroups),
                ::core::mem::transmute_copy(&pauthzdevicegroupsoperations),
                ::core::mem::transmute_copy(&pauthzuserclaims),
                ::core::mem::transmute_copy(&pauthzuserclaimsoperations),
                ::core::mem::transmute_copy(&pauthzdeviceclaims),
                ::core::mem::transmute_copy(&pauthzdeviceclaimsoperations),
                ::core::mem::transmute_copy(&peffpermresultlists),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ComputeEffectivePermissionWithSecondarySecurity: ComputeEffectivePermissionWithSecondarySecurity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEffectivePermission2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_UI_Controls\"`"]
#[cfg(feature = "Win32_UI_Controls")]
pub trait ISecurityInformation_Impl: Sized {
    fn GetObjectInformation(&self, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows_core::Result<()>;
    fn GetSecurity(&self, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetSecurity(&self, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::PSECURITY_DESCRIPTOR) -> ::windows_core::Result<()>;
    fn GetAccessRights(&self, pguidobjecttype: *const ::windows_core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows_core::Result<()>;
    fn MapGeneric(&self, pguidobjecttype: *const ::windows_core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows_core::Result<()>;
    fn GetInheritTypes(&self, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows_core::Result<()>;
    fn PropertySheetPageCallback(&self, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Controls")]
impl ::windows_core::RuntimeName for ISecurityInformation {}
#[cfg(feature = "Win32_UI_Controls")]
impl ISecurityInformation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: isize>() -> ISecurityInformation_Vtbl {
        unsafe extern "system" fn GetObjectInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInformation(::core::mem::transmute_copy(&pobjectinfo)).into()
        }
        unsafe extern "system" fn GetSecurity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSecurity(::core::mem::transmute_copy(&requestedinformation), ::core::mem::transmute_copy(&ppsecuritydescriptor), ::core::mem::transmute_copy(&fdefault)).into()
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::PSECURITY_DESCRIPTOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurity(::core::mem::transmute_copy(&securityinformation), ::core::mem::transmute_copy(&psecuritydescriptor)).into()
        }
        unsafe extern "system" fn GetAccessRights<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows_core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAccessRights(::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaccess), ::core::mem::transmute_copy(&pcaccesses), ::core::mem::transmute_copy(&pidefaultaccess)).into()
        }
        unsafe extern "system" fn MapGeneric<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows_core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapGeneric(::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&paceflags), ::core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetInheritTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInheritTypes(::core::mem::transmute_copy(&ppinherittypes), ::core::mem::transmute_copy(&pcinherittypes)).into()
        }
        unsafe extern "system" fn PropertySheetPageCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PropertySheetPageCallback(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&upage)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInformation: GetObjectInformation::<Identity, Impl, OFFSET>,
            GetSecurity: GetSecurity::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            GetAccessRights: GetAccessRights::<Identity, Impl, OFFSET>,
            MapGeneric: MapGeneric::<Identity, Impl, OFFSET>,
            GetInheritTypes: GetInheritTypes::<Identity, Impl, OFFSET>,
            PropertySheetPageCallback: PropertySheetPageCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityInformation as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityInformation2_Impl: Sized {
    fn IsDaclCanonical(&self, pdacl: *const super::super::ACL) -> super::super::super::Foundation::BOOL;
    fn LookupSids(&self, csids: u32, rgpsids: *const super::super::super::Foundation::PSID) -> ::windows_core::Result<super::super::super::System::Com::IDataObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ISecurityInformation2 {}
#[cfg(feature = "Win32_System_Com")]
impl ISecurityInformation2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation2_Impl, const OFFSET: isize>() -> ISecurityInformation2_Vtbl {
        unsafe extern "system" fn IsDaclCanonical<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdacl: *const super::super::ACL) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDaclCanonical(::core::mem::transmute_copy(&pdacl))
        }
        unsafe extern "system" fn LookupSids<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csids: u32, rgpsids: *const super::super::super::Foundation::PSID, ppdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupSids(::core::mem::transmute_copy(&csids), ::core::mem::transmute_copy(&rgpsids)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsDaclCanonical: IsDaclCanonical::<Identity, Impl, OFFSET>,
            LookupSids: LookupSids::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityInformation2 as ::windows_core::Interface>::IID
    }
}
pub trait ISecurityInformation3_Impl: Sized {
    fn GetFullResourceName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn OpenElevatedEditor(&self, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISecurityInformation3 {}
impl ISecurityInformation3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation3_Impl, const OFFSET: isize>() -> ISecurityInformation3_Vtbl {
        unsafe extern "system" fn GetFullResourceName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszresourcename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFullResourceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszresourcename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenElevatedEditor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenElevatedEditor(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&upage)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFullResourceName: GetFullResourceName::<Identity, Impl, OFFSET>,
            OpenElevatedEditor: OpenElevatedEditor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityInformation3 as ::windows_core::Interface>::IID
    }
}
pub trait ISecurityInformation4_Impl: Sized {
    fn GetSecondarySecurity(&self, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISecurityInformation4 {}
impl ISecurityInformation4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation4_Impl, const OFFSET: isize>() -> ISecurityInformation4_Vtbl {
        unsafe extern "system" fn GetSecondarySecurity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInformation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSecondarySecurity(::core::mem::transmute_copy(&psecurityobjects), ::core::mem::transmute_copy(&psecurityobjectcount)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSecondarySecurity: GetSecondarySecurity::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityInformation4 as ::windows_core::Interface>::IID
    }
}
pub trait ISecurityObjectTypeInfo_Impl: Sized {
    fn GetInheritSource(&self, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISecurityObjectTypeInfo {}
impl ISecurityObjectTypeInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityObjectTypeInfo_Impl, const OFFSET: isize>() -> ISecurityObjectTypeInfo_Vtbl {
        unsafe extern "system" fn GetInheritSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityObjectTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInheritSource(::core::mem::transmute_copy(&si), ::core::mem::transmute_copy(&pacl), ::core::mem::transmute_copy(&ppinheritarray)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInheritSource: GetInheritSource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityObjectTypeInfo as ::windows_core::Interface>::IID
    }
}
