#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscovery_Impl: Sized {
    fn GetInstanceCollection(&self, pszcategory: &::windows_core::PCWSTR, pszsubcategory: &::windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL) -> ::windows_core::Result<IFunctionInstanceCollection>;
    fn GetInstance(&self, pszfunctioninstanceidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(&self, pszcategory: &::windows_core::PCWSTR, pszsubcategory: &::windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::core::option::Option<IFunctionInstanceCollectionQuery>) -> ::windows_core::Result<()>;
    fn CreateInstanceQuery(&self, pszfunctioninstanceidentity: &::windows_core::PCWSTR, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::core::option::Option<IFunctionInstanceQuery>) -> ::windows_core::Result<()>;
    fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: &::windows_core::PCWSTR, pszsubcategory: &::windows_core::PCWSTR, pszcategoryidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: &::windows_core::PCWSTR, pszsubcategory: &::windows_core::PCWSTR, pszcategoryidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IFunctionDiscovery {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscovery_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>() -> IFunctionDiscovery_Vtbl {
        unsafe extern "system" fn GetInstanceCollection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInstanceCollection(::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute_copy(&fincludeallsubcategories)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstancecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInstance(::core::mem::transmute(&pszfunctioninstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstanceCollectionQuery(::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute_copy(&fincludeallsubcategories), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&pfdqcquerycontext), ::core::mem::transmute_copy(&ppifunctioninstancecollectionquery)).into()
        }
        unsafe extern "system" fn CreateInstanceQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows_core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstanceQuery(::core::mem::transmute(&pszfunctioninstanceidentity), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&pfdqcquerycontext), ::core::mem::transmute_copy(&ppifunctioninstancequery)).into()
        }
        unsafe extern "system" fn AddInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, pszcategoryidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddInstance(::core::mem::transmute_copy(&enumsystemvisibility), ::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszcategoryidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, pszcategoryidentity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveInstance(::core::mem::transmute_copy(&enumsystemvisibility), ::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszcategoryidentity)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInstanceCollection: GetInstanceCollection::<Identity, Impl, OFFSET>,
            GetInstance: GetInstance::<Identity, Impl, OFFSET>,
            CreateInstanceCollectionQuery: CreateInstanceCollectionQuery::<Identity, Impl, OFFSET>,
            CreateInstanceQuery: CreateInstanceQuery::<Identity, Impl, OFFSET>,
            AddInstance: AddInstance::<Identity, Impl, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionDiscovery as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryNotification_Impl: Sized {
    fn OnUpdate(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::core::option::Option<&IFunctionInstance>) -> ::windows_core::Result<()>;
    fn OnError(&self, hr: ::windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnEvent(&self, dweventid: u32, fdqcquerycontext: u64, pszprovider: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IFunctionDiscoveryNotification {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscoveryNotification_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>() -> IFunctionDiscoveryNotification_Vtbl {
        unsafe extern "system" fn OnUpdate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUpdate(::core::mem::transmute_copy(&enumqueryupdateaction), ::core::mem::transmute_copy(&fdqcquerycontext), ::windows_core::from_raw_borrowed(&pifunctioninstance)).into()
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnError(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute(&pszprovider)).into()
        }
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEvent(::core::mem::transmute_copy(&dweventid), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute(&pszprovider)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
            OnEvent: OnEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryNotification as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProvider_Impl: Sized {
    fn Initialize(&self, pifunctiondiscoveryproviderfactory: ::core::option::Option<&IFunctionDiscoveryProviderFactory>, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>, lciduserdefault: u32) -> ::windows_core::Result<u32>;
    fn Query(&self, pifunctiondiscoveryproviderquery: ::core::option::Option<&IFunctionDiscoveryProviderQuery>) -> ::windows_core::Result<IFunctionInstanceCollection>;
    fn EndQuery(&self) -> ::windows_core::Result<()>;
    fn InstancePropertyStoreValidateAccess(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::Result<()>;
    fn InstancePropertyStoreOpen(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn InstancePropertyStoreFlush(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows_core::Result<()>;
    fn InstanceQueryService(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InstanceReleased(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for IFunctionDiscoveryProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>() -> IFunctionDiscoveryProvider_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderfactory: *mut ::core::ffi::c_void, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Initialize(::windows_core::from_raw_borrowed(&pifunctiondiscoveryproviderfactory), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&lciduserdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstgaccesscapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderquery: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Query(::windows_core::from_raw_borrowed(&pifunctiondiscoveryproviderquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstancecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndQuery().into()
        }
        unsafe extern "system" fn InstancePropertyStoreValidateAccess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstancePropertyStoreValidateAccess(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwstgaccess)).into()
        }
        unsafe extern "system" fn InstancePropertyStoreOpen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstancePropertyStoreOpen(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwstgaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancePropertyStoreFlush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstancePropertyStoreFlush(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)).into()
        }
        unsafe extern "system" fn InstanceQueryService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstanceQueryService(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&guidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceReleased<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstanceReleased(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            EndQuery: EndQuery::<Identity, Impl, OFFSET>,
            InstancePropertyStoreValidateAccess: InstancePropertyStoreValidateAccess::<Identity, Impl, OFFSET>,
            InstancePropertyStoreOpen: InstancePropertyStoreOpen::<Identity, Impl, OFFSET>,
            InstancePropertyStoreFlush: InstancePropertyStoreFlush::<Identity, Impl, OFFSET>,
            InstanceQueryService: InstanceQueryService::<Identity, Impl, OFFSET>,
            InstanceReleased: InstanceReleased::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProviderFactory_Impl: Sized {
    fn CreatePropertyStore(&self) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateInstance(&self, pszsubcategory: &::windows_core::PCWSTR, pszproviderinstanceidentity: &::windows_core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pifunctiondiscoveryprovider: ::core::option::Option<&IFunctionDiscoveryProvider>) -> ::windows_core::Result<IFunctionInstance>;
    fn CreateFunctionInstanceCollection(&self) -> ::windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for IFunctionDiscoveryProviderFactory {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProviderFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>() -> IFunctionDiscoveryProviderFactory_Vtbl {
        unsafe extern "system" fn CreatePropertyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: *mut ::core::ffi::c_void, pifunctiondiscoveryprovider: *mut ::core::ffi::c_void, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity), ::core::mem::transmute_copy(&iproviderinstancecontext), ::windows_core::from_raw_borrowed(&pipropertystore), ::windows_core::from_raw_borrowed(&pifunctiondiscoveryprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFunctionInstanceCollection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFunctionInstanceCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstancecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyStore: CreatePropertyStore::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            CreateFunctionInstanceCollection: CreateFunctionInstanceCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderFactory as ::windows_core::Interface>::IID
    }
}
pub trait IFunctionDiscoveryProviderQuery_Impl: Sized {
    fn IsInstanceQuery(&self, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()>;
    fn IsSubcategoryQuery(&self, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()>;
    fn GetQueryConstraints(&self) -> ::windows_core::Result<IProviderQueryConstraintCollection>;
    fn GetPropertyConstraints(&self) -> ::windows_core::Result<IProviderPropertyConstraintCollection>;
}
impl ::windows_core::RuntimeName for IFunctionDiscoveryProviderQuery {}
impl IFunctionDiscoveryProviderQuery_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>() -> IFunctionDiscoveryProviderQuery_Vtbl {
        unsafe extern "system" fn IsInstanceQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsInstanceQuery(::core::mem::transmute_copy(&pisinstancequery), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn IsSubcategoryQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSubcategoryQuery(::core::mem::transmute_copy(&pissubcategoryquery), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn GetQueryConstraints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderqueryconstraints: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQueryConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiproviderqueryconstraints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyConstraints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderpropertyconstraints: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiproviderpropertyconstraints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsInstanceQuery: IsInstanceQuery::<Identity, Impl, OFFSET>,
            IsSubcategoryQuery: IsSubcategoryQuery::<Identity, Impl, OFFSET>,
            GetQueryConstraints: GetQueryConstraints::<Identity, Impl, OFFSET>,
            GetPropertyConstraints: GetPropertyConstraints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderQuery as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryServiceProvider_Impl: Sized {
    fn Initialize(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IFunctionDiscoveryServiceProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscoveryServiceProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryServiceProvider_Impl, const OFFSET: isize>() -> IFunctionDiscoveryServiceProvider_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionDiscoveryServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryServiceProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstance_Impl: Sized + super::super::System::Com::IServiceProvider_Impl {
    fn GetID(&self) -> ::windows_core::Result<*mut u16>;
    fn GetProviderInstanceID(&self) -> ::windows_core::Result<*mut u16>;
    fn OpenPropertyStore(&self, dwstgaccess: super::super::System::Com::STGM) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for IFunctionInstance {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstance_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: isize>() -> IFunctionInstance_Vtbl {
        unsafe extern "system" fn GetID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemidentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderInstanceID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProviderInstanceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemproviderinstanceidentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstgaccess: super::super::System::Com::STGM, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenPropertyStore(::core::mem::transmute_copy(&dwstgaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCategory(::core::mem::transmute_copy(&ppszcomemcategory), ::core::mem::transmute_copy(&ppszcomemsubcategory)).into()
        }
        Self {
            base__: super::super::System::Com::IServiceProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetProviderInstanceID: GetProviderInstanceID::<Identity, Impl, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionInstance as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IServiceProvider as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: &::windows_core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::core::option::Option<IFunctionInstance>) -> ::windows_core::Result<()>;
    fn Item(&self, dwindex: u32) -> ::windows_core::Result<IFunctionInstance>;
    fn Add(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>) -> ::windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> ::windows_core::Result<IFunctionInstance>;
    fn Delete(&self, dwindex: u32) -> ::windows_core::Result<()>;
    fn DeleteAll(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IFunctionInstanceCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstanceCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>() -> IFunctionInstanceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows_core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute(&pszinstanceidentity), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppifunctioninstance)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows_core::from_raw_borrowed(&pifunctioninstance)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Remove(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute_copy(&dwindex)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAll().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceCollection as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstanceCollectionQuery_Impl: Sized {
    fn AddQueryConstraint(&self, pszconstraintname: &::windows_core::PCWSTR, pszconstraintvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddPropertyConstraint(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows_core::Result<()>;
    fn Execute(&self) -> ::windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for IFunctionInstanceCollectionQuery {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstanceCollectionQuery_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>() -> IFunctionInstanceCollectionQuery_Vtbl {
        unsafe extern "system" fn AddQueryConstraint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows_core::PCWSTR, pszconstraintvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddQueryConstraint(::core::mem::transmute(&pszconstraintname), ::core::mem::transmute(&pszconstraintvalue)).into()
        }
        unsafe extern "system" fn AddPropertyConstraint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyConstraint(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&enumpropertyconstraint)).into()
        }
        unsafe extern "system" fn Execute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Execute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstancecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddQueryConstraint: AddQueryConstraint::<Identity, Impl, OFFSET>,
            AddPropertyConstraint: AddPropertyConstraint::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceCollectionQuery as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceQuery_Impl: Sized {
    fn Execute(&self) -> ::windows_core::Result<IFunctionInstance>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IFunctionInstanceQuery {}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstanceQuery_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceQuery_Impl, const OFFSET: isize>() -> IFunctionInstanceQuery_Vtbl {
        unsafe extern "system" fn Execute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFunctionInstanceQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Execute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Execute: Execute::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceQuery as ::windows_core::Interface>::IID
    }
}
pub trait IPNPXAssociation_Impl: Sized {
    fn Associate(&self, pszsubcategory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Unassociate(&self, pszsubcategory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Delete(&self, pszsubcategory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPNPXAssociation {}
impl IPNPXAssociation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPNPXAssociation_Impl, const OFFSET: isize>() -> IPNPXAssociation_Vtbl {
        unsafe extern "system" fn Associate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Associate(::core::mem::transmute(&pszsubcategory)).into()
        }
        unsafe extern "system" fn Unassociate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unassociate(::core::mem::transmute(&pszsubcategory)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&pszsubcategory)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Associate: Associate::<Identity, Impl, OFFSET>,
            Unassociate: Unassociate::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPNPXAssociation as ::windows_core::Interface>::IID
    }
}
pub trait IPNPXDeviceAssociation_Impl: Sized {
    fn Associate(&self, pszsubcategory: &::windows_core::PCWSTR, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>) -> ::windows_core::Result<()>;
    fn Unassociate(&self, pszsubcategory: &::windows_core::PCWSTR, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>) -> ::windows_core::Result<()>;
    fn Delete(&self, pszsubcategory: &::windows_core::PCWSTR, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPNPXDeviceAssociation {}
impl IPNPXDeviceAssociation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>() -> IPNPXDeviceAssociation_Vtbl {
        unsafe extern "system" fn Associate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Associate(::core::mem::transmute(&pszsubcategory), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into()
        }
        unsafe extern "system" fn Unassociate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unassociate(::core::mem::transmute(&pszsubcategory), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&pszsubcategory), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Associate: Associate::<Identity, Impl, OFFSET>,
            Unassociate: Unassociate::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPNPXDeviceAssociation as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPropertyStoreCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: &::windows_core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Item(&self, dwindex: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Add(&self, pipropertystore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Delete(&self, dwindex: u32) -> ::windows_core::Result<()>;
    fn DeleteAll(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::RuntimeName for IPropertyStoreCollection {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPropertyStoreCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>() -> IPropertyStoreCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows_core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute(&pszinstanceidentity), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppipropertystore)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows_core::from_raw_borrowed(&pipropertystore)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Remove(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute_copy(&dwindex)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAll().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCollection as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderProperties_Impl: Sized {
    fn GetCount(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows_core::Result<u32>;
    fn GetAt(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetValue(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetValue(&self, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for IProviderProperties {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: isize>() -> IProviderProperties_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAt(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProviderProperties as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderPropertyConstraintCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn Get(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()>;
    fn Item(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()>;
    fn Next(&self, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for IProviderPropertyConstraintCollection {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderPropertyConstraintCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>() -> IProviderPropertyConstraintCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Item(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProviderPropertyConstraintCollection as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProviderPublishing_Impl: Sized {
    fn CreateInstance(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: &::windows_core::PCWSTR, pszproviderinstanceidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: &::windows_core::PCWSTR, pszproviderinstanceidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IProviderPublishing {}
#[cfg(feature = "Win32_System_Com")]
impl IProviderPublishing_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPublishing_Impl, const OFFSET: isize>() -> IProviderPublishing_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPublishing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(::core::mem::transmute_copy(&enumvisibilityflags), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderPublishing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveInstance(::core::mem::transmute_copy(&enumvisibilityflags), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProviderPublishing as ::windows_core::Interface>::IID
    }
}
pub trait IProviderQueryConstraintCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn Get(&self, pszconstraintname: &::windows_core::PCWSTR) -> ::windows_core::Result<*mut u16>;
    fn Item(&self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()>;
    fn Next(&self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()>;
    fn Skip(&self) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IProviderQueryConstraintCollection {}
impl IProviderQueryConstraintCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>() -> IProviderQueryConstraintCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows_core::PCWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Get(::core::mem::transmute(&pszconstraintname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszconstraintvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Item(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProviderQueryConstraintCollection as ::windows_core::Interface>::IID
    }
}
