#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait AppEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for AppEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl AppEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AppEvents_Impl, const OFFSET: isize>() -> AppEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<AppEvents as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Column_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Width(&self) -> ::windows_core::Result<i32>;
    fn SetWidth(&self, width: i32) -> ::windows_core::Result<()>;
    fn DisplayPosition(&self) -> ::windows_core::Result<i32>;
    fn SetDisplayPosition(&self, index: i32) -> ::windows_core::Result<()>;
    fn Hidden(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetHidden(&self, hidden: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetAsSortColumn(&self, sortorder: _ColumnSortOrder) -> ::windows_core::Result<()>;
    fn IsSortColumn(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Column {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Column_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>() -> Column_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Width() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(width, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWidth(::core::mem::transmute_copy(&width)).into()
        }
        unsafe extern "system" fn DisplayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayposition: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayPosition(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Hidden<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Hidden() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hidden, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHidden(::core::mem::transmute_copy(&hidden)).into()
        }
        unsafe extern "system" fn SetAsSortColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sortorder: _ColumnSortOrder) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAsSortColumn(::core::mem::transmute_copy(&sortorder)).into()
        }
        unsafe extern "system" fn IsSortColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Column_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issortcolumn: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSortColumn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issortcolumn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            SetWidth: SetWidth::<Identity, Impl, OFFSET>,
            DisplayPosition: DisplayPosition::<Identity, Impl, OFFSET>,
            SetDisplayPosition: SetDisplayPosition::<Identity, Impl, OFFSET>,
            Hidden: Hidden::<Identity, Impl, OFFSET>,
            SetHidden: SetHidden::<Identity, Impl, OFFSET>,
            SetAsSortColumn: SetAsSortColumn::<Identity, Impl, OFFSET>,
            IsSortColumn: IsSortColumn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Column as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Columns_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows_core::Result<Column>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Columns {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Columns_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Columns_Impl, const OFFSET: isize>() -> Columns_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Columns_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, column: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(column, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Columns_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Columns_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Columns as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextMenu_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(&self, indexorpath: &super::Variant::VARIANT) -> ::windows_core::Result<MenuItem>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ContextMenu {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ContextMenu_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextMenu_Impl, const OFFSET: isize>() -> ContextMenu_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexorpath: super::Variant::VARIANT, menuitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&indexorpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(menuitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
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
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ContextMenu as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Document_Impl: Sized + super::Com::IDispatch_Impl {
    fn Save(&self) -> ::windows_core::Result<()>;
    fn SaveAs(&self, filename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Close(&self, savechanges: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Views(&self) -> ::windows_core::Result<Views>;
    fn SnapIns(&self) -> ::windows_core::Result<SnapIns>;
    fn ActiveView(&self) -> ::windows_core::Result<View>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Location(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsSaved(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Mode(&self) -> ::windows_core::Result<_DocumentMode>;
    fn SetMode(&self, mode: _DocumentMode) -> ::windows_core::Result<()>;
    fn RootNode(&self) -> ::windows_core::Result<Node>;
    fn ScopeNamespace(&self) -> ::windows_core::Result<ScopeNamespace>;
    fn CreateProperties(&self) -> ::windows_core::Result<Properties>;
    fn Application(&self) -> ::windows_core::Result<_Application>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Document {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Document_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>() -> Document_Vtbl {
        unsafe extern "system" fn Save<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn SaveAs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveAs(::core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, savechanges: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close(::core::mem::transmute_copy(&savechanges)).into()
        }
        unsafe extern "system" fn Views<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, views: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Views() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(views, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapIns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapins: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SnapIns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(view, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Location<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Location() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(location, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSaved<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issaved: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSaved() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issaved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut _DocumentMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Mode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: _DocumentMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn RootNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeNamespace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScopeNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scopenamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Application<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Document_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Application() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(application, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Save: Save::<Identity, Impl, OFFSET>,
            SaveAs: SaveAs::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Views: Views::<Identity, Impl, OFFSET>,
            SnapIns: SnapIns::<Identity, Impl, OFFSET>,
            ActiveView: ActiveView::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            IsSaved: IsSaved::<Identity, Impl, OFFSET>,
            Mode: Mode::<Identity, Impl, OFFSET>,
            SetMode: SetMode::<Identity, Impl, OFFSET>,
            RootNode: RootNode::<Identity, Impl, OFFSET>,
            ScopeNamespace: ScopeNamespace::<Identity, Impl, OFFSET>,
            CreateProperties: CreateProperties::<Identity, Impl, OFFSET>,
            Application: Application::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Document as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Extension_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Vendor(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Version(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Extensions(&self) -> ::windows_core::Result<Extensions>;
    fn SnapinCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnableAllExtensions(&self, enable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Enable(&self, enable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Extension {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Extension_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extension_Impl, const OFFSET: isize>() -> Extension_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Vendor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vendor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinCLSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SnapinCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapinclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAllExtensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableAllExtensions(::core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable(::core::mem::transmute_copy(&enable)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Vendor: Vendor::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            Extensions: Extensions::<Identity, Impl, OFFSET>,
            SnapinCLSID: SnapinCLSID::<Identity, Impl, OFFSET>,
            EnableAllExtensions: EnableAllExtensions::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Extension as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Extensions_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(&self, index: i32) -> ::windows_core::Result<Extension>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Extensions {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Extensions_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extensions_Impl, const OFFSET: isize>() -> Extensions_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, extension: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Extensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
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
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Extensions as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Frame_Impl: Sized + super::Com::IDispatch_Impl {
    fn Maximize(&self) -> ::windows_core::Result<()>;
    fn Minimize(&self) -> ::windows_core::Result<()>;
    fn Restore(&self) -> ::windows_core::Result<()>;
    fn Top(&self) -> ::windows_core::Result<i32>;
    fn SetTop(&self, top: i32) -> ::windows_core::Result<()>;
    fn Bottom(&self) -> ::windows_core::Result<i32>;
    fn SetBottom(&self, bottom: i32) -> ::windows_core::Result<()>;
    fn Left(&self) -> ::windows_core::Result<i32>;
    fn SetLeft(&self, left: i32) -> ::windows_core::Result<()>;
    fn Right(&self) -> ::windows_core::Result<i32>;
    fn SetRight(&self, right: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Frame {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Frame_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>() -> Frame_Vtbl {
        unsafe extern "system" fn Maximize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Maximize().into()
        }
        unsafe extern "system" fn Minimize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Minimize().into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn Top<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Top() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(top, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTop(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn Bottom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Bottom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bottom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBottom(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn Left<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Left() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(left, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLeft(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn Right<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Right() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(right, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Frame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRight(::core::mem::transmute_copy(&right)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Maximize: Maximize::<Identity, Impl, OFFSET>,
            Minimize: Minimize::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            Top: Top::<Identity, Impl, OFFSET>,
            SetTop: SetTop::<Identity, Impl, OFFSET>,
            Bottom: Bottom::<Identity, Impl, OFFSET>,
            SetBottom: SetBottom::<Identity, Impl, OFFSET>,
            Left: Left::<Identity, Impl, OFFSET>,
            SetLeft: SetLeft::<Identity, Impl, OFFSET>,
            Right: Right::<Identity, Impl, OFFSET>,
            SetRight: SetRight::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Frame as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IColumnData_Impl: Sized {
    fn SetColumnConfigData(&self, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows_core::Result<()>;
    fn GetColumnConfigData(&self, pcolid: *const SColumnSetID) -> ::windows_core::Result<*mut MMC_COLUMN_SET_DATA>;
    fn SetColumnSortData(&self, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows_core::Result<()>;
    fn GetColumnSortData(&self, pcolid: *const SColumnSetID) -> ::windows_core::Result<*mut MMC_SORT_SET_DATA>;
}
impl ::windows_core::RuntimeName for IColumnData {}
impl IColumnData_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: isize>() -> IColumnData_Vtbl {
        unsafe extern "system" fn SetColumnConfigData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumnConfigData(::core::mem::transmute_copy(&pcolid), ::core::mem::transmute_copy(&pcolsetdata)).into()
        }
        unsafe extern "system" fn GetColumnConfigData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnConfigData(::core::mem::transmute_copy(&pcolid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolsetdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumnSortData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumnSortData(::core::mem::transmute_copy(&pcolid), ::core::mem::transmute_copy(&pcolsortdata)).into()
        }
        unsafe extern "system" fn GetColumnSortData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnSortData(::core::mem::transmute_copy(&pcolid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolsortdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetColumnConfigData: SetColumnConfigData::<Identity, Impl, OFFSET>,
            GetColumnConfigData: GetColumnConfigData::<Identity, Impl, OFFSET>,
            SetColumnSortData: SetColumnSortData::<Identity, Impl, OFFSET>,
            GetColumnSortData: GetColumnSortData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IColumnData as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IComponent_Impl: Sized {
    fn Initialize(&self, lpconsole: ::core::option::Option<&IConsole>) -> ::windows_core::Result<()>;
    fn Notify(&self, lpdataobject: ::core::option::Option<&super::Com::IDataObject>, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn Destroy(&self, cookie: isize) -> ::windows_core::Result<()>;
    fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows_core::Result<super::Com::IDataObject>;
    fn GetResultViewType(&self, cookie: isize, ppviewtype: *mut ::windows_core::PWSTR, pviewoptions: *mut i32) -> ::windows_core::Result<()>;
    fn GetDisplayInfo(&self, presultdataitem: *mut RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn CompareObjects(&self, lpdataobjecta: ::core::option::Option<&super::Com::IDataObject>, lpdataobjectb: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IComponent {}
#[cfg(feature = "Win32_System_Com")]
impl IComponent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: isize>() -> IComponent_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpconsole: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&lpconsole)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify(::windows_core::from_raw_borrowed(&lpdataobject), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Destroy(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn QueryDataObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryDataObject(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultViewType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, ppviewtype: *mut ::windows_core::PWSTR, pviewoptions: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResultViewType(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&ppviewtype), ::core::mem::transmute_copy(&pviewoptions)).into()
        }
        unsafe extern "system" fn GetDisplayInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayInfo(::core::mem::transmute_copy(&presultdataitem)).into()
        }
        unsafe extern "system" fn CompareObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobjecta: *mut ::core::ffi::c_void, lpdataobjectb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompareObjects(::windows_core::from_raw_borrowed(&lpdataobjecta), ::windows_core::from_raw_borrowed(&lpdataobjectb)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            QueryDataObject: QueryDataObject::<Identity, Impl, OFFSET>,
            GetResultViewType: GetResultViewType::<Identity, Impl, OFFSET>,
            GetDisplayInfo: GetDisplayInfo::<Identity, Impl, OFFSET>,
            CompareObjects: CompareObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComponent as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IComponent2_Impl: Sized + IComponent_Impl {
    fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows_core::Result<super::Com::IDispatch>;
    fn GetResultViewType2(&self, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows_core::Result<()>;
    fn RestoreResultView(&self, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IComponent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IComponent2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent2_Impl, const OFFSET: isize>() -> IComponent2_Vtbl {
        unsafe extern "system" fn QueryDispatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryDispatch(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultViewType2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResultViewType2(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&presultviewtype)).into()
        }
        unsafe extern "system" fn RestoreResultView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreResultView(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&presultviewtype)).into()
        }
        Self {
            base__: IComponent_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryDispatch: QueryDispatch::<Identity, Impl, OFFSET>,
            GetResultViewType2: GetResultViewType2::<Identity, Impl, OFFSET>,
            RestoreResultView: RestoreResultView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComponent2 as ::windows_core::Interface>::IID || iid == &<IComponent as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentData_Impl: Sized {
    fn Initialize(&self, punknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateComponent(&self) -> ::windows_core::Result<IComponent>;
    fn Notify(&self, lpdataobject: ::core::option::Option<&super::Com::IDataObject>, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn Destroy(&self) -> ::windows_core::Result<()>;
    fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows_core::Result<super::Com::IDataObject>;
    fn GetDisplayInfo(&self, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows_core::Result<()>;
    fn CompareObjects(&self, lpdataobjecta: ::core::option::Option<&super::Com::IDataObject>, lpdataobjectb: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IComponentData {}
#[cfg(feature = "Win32_System_Com")]
impl IComponentData_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: isize>() -> IComponentData_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&punknown)).into()
        }
        unsafe extern "system" fn CreateComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateComponent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomponent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify(::windows_core::from_raw_borrowed(&lpdataobject), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Destroy().into()
        }
        unsafe extern "system" fn QueryDataObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryDataObject(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayInfo(::core::mem::transmute_copy(&pscopedataitem)).into()
        }
        unsafe extern "system" fn CompareObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobjecta: *mut ::core::ffi::c_void, lpdataobjectb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompareObjects(::windows_core::from_raw_borrowed(&lpdataobjecta), ::windows_core::from_raw_borrowed(&lpdataobjectb)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateComponent: CreateComponent::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            QueryDataObject: QueryDataObject::<Identity, Impl, OFFSET>,
            GetDisplayInfo: GetDisplayInfo::<Identity, Impl, OFFSET>,
            CompareObjects: CompareObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComponentData as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentData2_Impl: Sized + IComponentData_Impl {
    fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IComponentData2 {}
#[cfg(feature = "Win32_System_Com")]
impl IComponentData2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData2_Impl, const OFFSET: isize>() -> IComponentData2_Vtbl {
        unsafe extern "system" fn QueryDispatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentData2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryDispatch(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IComponentData_Vtbl::new::<Identity, Impl, OFFSET>(), QueryDispatch: QueryDispatch::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComponentData2 as ::windows_core::Interface>::IID || iid == &<IComponentData as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IConsole_Impl: Sized {
    fn SetHeader(&self, pheader: ::core::option::Option<&IHeaderCtrl>) -> ::windows_core::Result<()>;
    fn SetToolbar(&self, ptoolbar: ::core::option::Option<&IToolbar>) -> ::windows_core::Result<()>;
    fn QueryResultView(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryScopeImageList(&self) -> ::windows_core::Result<IImageList>;
    fn QueryResultImageList(&self) -> ::windows_core::Result<IImageList>;
    fn UpdateAllViews(&self, lpdataobject: ::core::option::Option<&super::Com::IDataObject>, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows_core::Result<()>;
    fn MessageBox(&self, lpsztext: &::windows_core::PCWSTR, lpsztitle: &::windows_core::PCWSTR, fustyle: u32) -> ::windows_core::Result<i32>;
    fn QueryConsoleVerb(&self) -> ::windows_core::Result<IConsoleVerb>;
    fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows_core::Result<()>;
    fn GetMainWindow(&self) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IConsole {}
#[cfg(feature = "Win32_System_Com")]
impl IConsole_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>() -> IConsole_Vtbl {
        unsafe extern "system" fn SetHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHeader(::windows_core::from_raw_borrowed(&pheader)).into()
        }
        unsafe extern "system" fn SetToolbar<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoolbar: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetToolbar(::windows_core::from_raw_borrowed(&ptoolbar)).into()
        }
        unsafe extern "system" fn QueryResultView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryResultView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryScopeImageList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryScopeImageList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimagelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryResultImageList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryResultImageList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimagelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAllViews<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateAllViews(::windows_core::from_raw_borrowed(&lpdataobject), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&hint)).into()
        }
        unsafe extern "system" fn MessageBox<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsztext: ::windows_core::PCWSTR, lpsztitle: ::windows_core::PCWSTR, fustyle: u32, piretval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageBox(::core::mem::transmute(&lpsztext), ::core::mem::transmute(&lpsztitle), ::core::mem::transmute_copy(&fustyle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryConsoleVerb<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryConsoleVerb() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconsoleverb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectScopeItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectScopeItem(::core::mem::transmute_copy(&hscopeitem)).into()
        }
        unsafe extern "system" fn GetMainWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMainWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NewWindow(::core::mem::transmute_copy(&hscopeitem), ::core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHeader: SetHeader::<Identity, Impl, OFFSET>,
            SetToolbar: SetToolbar::<Identity, Impl, OFFSET>,
            QueryResultView: QueryResultView::<Identity, Impl, OFFSET>,
            QueryScopeImageList: QueryScopeImageList::<Identity, Impl, OFFSET>,
            QueryResultImageList: QueryResultImageList::<Identity, Impl, OFFSET>,
            UpdateAllViews: UpdateAllViews::<Identity, Impl, OFFSET>,
            MessageBox: MessageBox::<Identity, Impl, OFFSET>,
            QueryConsoleVerb: QueryConsoleVerb::<Identity, Impl, OFFSET>,
            SelectScopeItem: SelectScopeItem::<Identity, Impl, OFFSET>,
            GetMainWindow: GetMainWindow::<Identity, Impl, OFFSET>,
            NewWindow: NewWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConsole as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IConsole2_Impl: Sized + IConsole_Impl {
    fn Expand(&self, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsTaskpadViewPreferred(&self) -> ::windows_core::Result<()>;
    fn SetStatusText(&self, pszstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IConsole2 {}
#[cfg(feature = "Win32_System_Com")]
impl IConsole2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole2_Impl, const OFFSET: isize>() -> IConsole2_Vtbl {
        unsafe extern "system" fn Expand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Expand(::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&bexpand)).into()
        }
        unsafe extern "system" fn IsTaskpadViewPreferred<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsTaskpadViewPreferred().into()
        }
        unsafe extern "system" fn SetStatusText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatusText(::core::mem::transmute(&pszstatustext)).into()
        }
        Self {
            base__: IConsole_Vtbl::new::<Identity, Impl, OFFSET>(),
            Expand: Expand::<Identity, Impl, OFFSET>,
            IsTaskpadViewPreferred: IsTaskpadViewPreferred::<Identity, Impl, OFFSET>,
            SetStatusText: SetStatusText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConsole2 as ::windows_core::Interface>::IID || iid == &<IConsole as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IConsole3_Impl: Sized + IConsole2_Impl {
    fn RenameScopeItem(&self, hscopeitem: isize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IConsole3 {}
#[cfg(feature = "Win32_System_Com")]
impl IConsole3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole3_Impl, const OFFSET: isize>() -> IConsole3_Vtbl {
        unsafe extern "system" fn RenameScopeItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsole3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenameScopeItem(::core::mem::transmute_copy(&hscopeitem)).into()
        }
        Self { base__: IConsole2_Vtbl::new::<Identity, Impl, OFFSET>(), RenameScopeItem: RenameScopeItem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConsole3 as ::windows_core::Interface>::IID || iid == &<IConsole as ::windows_core::Interface>::IID || iid == &<IConsole2 as ::windows_core::Interface>::IID
    }
}
pub trait IConsoleNameSpace_Impl: Sized {
    fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> ::windows_core::Result<()>;
    fn DeleteItem(&self, hitem: isize, fdeletethis: i32) -> ::windows_core::Result<()>;
    fn SetItem(&self, item: *const SCOPEDATAITEM) -> ::windows_core::Result<()>;
    fn GetItem(&self, item: *mut SCOPEDATAITEM) -> ::windows_core::Result<()>;
    fn GetChildItem(&self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows_core::Result<()>;
    fn GetNextItem(&self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows_core::Result<()>;
    fn GetParentItem(&self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IConsoleNameSpace {}
impl IConsoleNameSpace_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: isize>() -> IConsoleNameSpace_Vtbl {
        unsafe extern "system" fn InsertItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, fdeletethis: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&fdeletethis)).into()
        }
        unsafe extern "system" fn SetItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const SCOPEDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetChildItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChildItem(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemchild), ::core::mem::transmute_copy(&pcookie)).into()
        }
        unsafe extern "system" fn GetNextItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextItem(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemnext), ::core::mem::transmute_copy(&pcookie)).into()
        }
        unsafe extern "system" fn GetParentItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParentItem(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemparent), ::core::mem::transmute_copy(&pcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertItem: InsertItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            SetItem: SetItem::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            GetChildItem: GetChildItem::<Identity, Impl, OFFSET>,
            GetNextItem: GetNextItem::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConsoleNameSpace as ::windows_core::Interface>::IID
    }
}
pub trait IConsoleNameSpace2_Impl: Sized + IConsoleNameSpace_Impl {
    fn Expand(&self, hitem: isize) -> ::windows_core::Result<()>;
    fn AddExtension(&self, hitem: isize, lpclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IConsoleNameSpace2 {}
impl IConsoleNameSpace2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace2_Impl, const OFFSET: isize>() -> IConsoleNameSpace2_Vtbl {
        unsafe extern "system" fn Expand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Expand(::core::mem::transmute_copy(&hitem)).into()
        }
        unsafe extern "system" fn AddExtension<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleNameSpace2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, lpclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddExtension(::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&lpclsid)).into()
        }
        Self {
            base__: IConsoleNameSpace_Vtbl::new::<Identity, Impl, OFFSET>(),
            Expand: Expand::<Identity, Impl, OFFSET>,
            AddExtension: AddExtension::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConsoleNameSpace2 as ::windows_core::Interface>::IID || iid == &<IConsoleNameSpace as ::windows_core::Interface>::IID
    }
}
pub trait IConsolePower_Impl: Sized {
    fn SetExecutionState(&self, dwadd: u32, dwremove: u32) -> ::windows_core::Result<()>;
    fn ResetIdleTimer(&self, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IConsolePower {}
impl IConsolePower_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsolePower_Impl, const OFFSET: isize>() -> IConsolePower_Vtbl {
        unsafe extern "system" fn SetExecutionState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsolePower_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwadd: u32, dwremove: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExecutionState(::core::mem::transmute_copy(&dwadd), ::core::mem::transmute_copy(&dwremove)).into()
        }
        unsafe extern "system" fn ResetIdleTimer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsolePower_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetIdleTimer(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetExecutionState: SetExecutionState::<Identity, Impl, OFFSET>,
            ResetIdleTimer: ResetIdleTimer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConsolePower as ::windows_core::Interface>::IID
    }
}
pub trait IConsolePowerSink_Impl: Sized {
    fn OnPowerBroadcast(&self, nevent: u32, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::LRESULT>;
}
impl ::windows_core::RuntimeName for IConsolePowerSink {}
impl IConsolePowerSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsolePowerSink_Impl, const OFFSET: isize>() -> IConsolePowerSink_Vtbl {
        unsafe extern "system" fn OnPowerBroadcast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsolePowerSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnPowerBroadcast(::core::mem::transmute_copy(&nevent), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plreturn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPowerBroadcast: OnPowerBroadcast::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConsolePowerSink as ::windows_core::Interface>::IID
    }
}
pub trait IConsoleVerb_Impl: Sized {
    fn GetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetDefaultVerb(&self, ecmdid: MMC_CONSOLE_VERB) -> ::windows_core::Result<()>;
    fn GetDefaultVerb(&self) -> ::windows_core::Result<MMC_CONSOLE_VERB>;
}
impl ::windows_core::RuntimeName for IConsoleVerb {}
impl IConsoleVerb_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: isize>() -> IConsoleVerb_Vtbl {
        unsafe extern "system" fn GetVerbState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVerbState(::core::mem::transmute_copy(&ecmdid), ::core::mem::transmute_copy(&nstate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerbState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVerbState(::core::mem::transmute_copy(&ecmdid), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn SetDefaultVerb<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultVerb(::core::mem::transmute_copy(&ecmdid)).into()
        }
        unsafe extern "system" fn GetDefaultVerb<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pecmdid: *mut MMC_CONSOLE_VERB) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultVerb() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pecmdid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVerbState: GetVerbState::<Identity, Impl, OFFSET>,
            SetVerbState: SetVerbState::<Identity, Impl, OFFSET>,
            SetDefaultVerb: SetDefaultVerb::<Identity, Impl, OFFSET>,
            GetDefaultVerb: GetDefaultVerb::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IConsoleVerb as ::windows_core::Interface>::IID
    }
}
pub trait IContextMenuCallback_Impl: Sized {
    fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IContextMenuCallback {}
impl IContextMenuCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuCallback_Impl, const OFFSET: isize>() -> IContextMenuCallback_Vtbl {
        unsafe extern "system" fn AddItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute_copy(&pitem)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddItem: AddItem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContextMenuCallback as ::windows_core::Interface>::IID
    }
}
pub trait IContextMenuCallback2_Impl: Sized {
    fn AddItem(&self, pitem: *const CONTEXTMENUITEM2) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IContextMenuCallback2 {}
impl IContextMenuCallback2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuCallback2_Impl, const OFFSET: isize>() -> IContextMenuCallback2_Vtbl {
        unsafe extern "system" fn AddItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute_copy(&pitem)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddItem: AddItem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContextMenuCallback2 as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IContextMenuProvider_Impl: Sized + IContextMenuCallback_Impl {
    fn EmptyMenuList(&self) -> ::windows_core::Result<()>;
    fn AddPrimaryExtensionItems(&self, piextension: ::core::option::Option<&::windows_core::IUnknown>, pidataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn AddThirdPartyExtensionItems(&self, pidataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn ShowContextMenu(&self, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IContextMenuProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IContextMenuProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: isize>() -> IContextMenuProvider_Vtbl {
        unsafe extern "system" fn EmptyMenuList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EmptyMenuList().into()
        }
        unsafe extern "system" fn AddPrimaryExtensionItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piextension: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPrimaryExtensionItems(::windows_core::from_raw_borrowed(&piextension), ::windows_core::from_raw_borrowed(&pidataobject)).into()
        }
        unsafe extern "system" fn AddThirdPartyExtensionItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddThirdPartyExtensionItems(::windows_core::from_raw_borrowed(&pidataobject)).into()
        }
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShowContextMenu(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plselected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IContextMenuCallback_Vtbl::new::<Identity, Impl, OFFSET>(),
            EmptyMenuList: EmptyMenuList::<Identity, Impl, OFFSET>,
            AddPrimaryExtensionItems: AddPrimaryExtensionItems::<Identity, Impl, OFFSET>,
            AddThirdPartyExtensionItems: AddThirdPartyExtensionItems::<Identity, Impl, OFFSET>,
            ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContextMenuProvider as ::windows_core::Interface>::IID || iid == &<IContextMenuCallback as ::windows_core::Interface>::IID
    }
}
pub trait IControlbar_Impl: Sized {
    fn Create(&self, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: ::core::option::Option<&IExtendControlbar>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Attach(&self, ntype: MMC_CONTROL_TYPE, lpunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Detach(&self, lpunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IControlbar {}
impl IControlbar_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IControlbar_Impl, const OFFSET: isize>() -> IControlbar_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IControlbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&ntype), ::windows_core::from_raw_borrowed(&pextendcontrolbar)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IControlbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, lpunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Attach(::core::mem::transmute_copy(&ntype), ::windows_core::from_raw_borrowed(&lpunknown)).into()
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IControlbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Detach(::windows_core::from_raw_borrowed(&lpunknown)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IControlbar as ::windows_core::Interface>::IID
    }
}
pub trait IDisplayHelp_Impl: Sized {
    fn ShowTopic(&self, pszhelptopic: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDisplayHelp {}
impl IDisplayHelp_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDisplayHelp_Impl, const OFFSET: isize>() -> IDisplayHelp_Vtbl {
        unsafe extern "system" fn ShowTopic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDisplayHelp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelptopic: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowTopic(::core::mem::transmute(&pszhelptopic)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowTopic: ShowTopic::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDisplayHelp as ::windows_core::Interface>::IID
    }
}
pub trait IEnumTASK_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumTASK>;
}
impl ::windows_core::RuntimeName for IEnumTASK {}
impl IEnumTASK_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: isize>() -> IEnumTASK_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
        iid == &<IEnumTASK as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendContextMenu_Impl: Sized {
    fn AddMenuItems(&self, pidataobject: ::core::option::Option<&super::Com::IDataObject>, picallback: ::core::option::Option<&IContextMenuCallback>, pinsertionallowed: *mut i32) -> ::windows_core::Result<()>;
    fn Command(&self, lcommandid: i32, pidataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IExtendContextMenu {}
#[cfg(feature = "Win32_System_Com")]
impl IExtendContextMenu_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendContextMenu_Impl, const OFFSET: isize>() -> IExtendContextMenu_Vtbl {
        unsafe extern "system" fn AddMenuItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void, pinsertionallowed: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMenuItems(::windows_core::from_raw_borrowed(&pidataobject), ::windows_core::from_raw_borrowed(&picallback), ::core::mem::transmute_copy(&pinsertionallowed)).into()
        }
        unsafe extern "system" fn Command<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcommandid: i32, pidataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Command(::core::mem::transmute_copy(&lcommandid), ::windows_core::from_raw_borrowed(&pidataobject)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddMenuItems: AddMenuItems::<Identity, Impl, OFFSET>,
            Command: Command::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IExtendContextMenu as ::windows_core::Interface>::IID
    }
}
pub trait IExtendControlbar_Impl: Sized {
    fn SetControlbar(&self, pcontrolbar: ::core::option::Option<&IControlbar>) -> ::windows_core::Result<()>;
    fn ControlbarNotify(&self, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IExtendControlbar {}
impl IExtendControlbar_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendControlbar_Impl, const OFFSET: isize>() -> IExtendControlbar_Vtbl {
        unsafe extern "system" fn SetControlbar<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendControlbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrolbar: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetControlbar(::windows_core::from_raw_borrowed(&pcontrolbar)).into()
        }
        unsafe extern "system" fn ControlbarNotify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendControlbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ControlbarNotify(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetControlbar: SetControlbar::<Identity, Impl, OFFSET>,
            ControlbarNotify: ControlbarNotify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IExtendControlbar as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendPropertySheet_Impl: Sized {
    fn CreatePropertyPages(&self, lpprovider: ::core::option::Option<&IPropertySheetCallback>, handle: isize, lpidataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn QueryPagesFor(&self, lpdataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IExtendPropertySheet {}
#[cfg(feature = "Win32_System_Com")]
impl IExtendPropertySheet_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendPropertySheet_Impl, const OFFSET: isize>() -> IExtendPropertySheet_Vtbl {
        unsafe extern "system" fn CreatePropertyPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendPropertySheet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpprovider: *mut ::core::ffi::c_void, handle: isize, lpidataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePropertyPages(::windows_core::from_raw_borrowed(&lpprovider), ::core::mem::transmute_copy(&handle), ::windows_core::from_raw_borrowed(&lpidataobject)).into()
        }
        unsafe extern "system" fn QueryPagesFor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendPropertySheet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryPagesFor(::windows_core::from_raw_borrowed(&lpdataobject)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyPages: CreatePropertyPages::<Identity, Impl, OFFSET>,
            QueryPagesFor: QueryPagesFor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IExtendPropertySheet as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IExtendPropertySheet2_Impl: Sized + IExtendPropertySheet_Impl {
    fn GetWatermarks(&self, lpidataobject: ::core::option::Option<&super::Com::IDataObject>, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IExtendPropertySheet2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IExtendPropertySheet2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendPropertySheet2_Impl, const OFFSET: isize>() -> IExtendPropertySheet2_Vtbl {
        unsafe extern "system" fn GetWatermarks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendPropertySheet2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpidataobject: *mut ::core::ffi::c_void, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWatermarks(::windows_core::from_raw_borrowed(&lpidataobject), ::core::mem::transmute_copy(&lphwatermark), ::core::mem::transmute_copy(&lphheader), ::core::mem::transmute_copy(&lphpalette), ::core::mem::transmute_copy(&bstretch)).into()
        }
        Self { base__: IExtendPropertySheet_Vtbl::new::<Identity, Impl, OFFSET>(), GetWatermarks: GetWatermarks::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IExtendPropertySheet2 as ::windows_core::Interface>::IID || iid == &<IExtendPropertySheet as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IExtendTaskPad_Impl: Sized {
    fn TaskNotify(&self, pdo: ::core::option::Option<&super::Com::IDataObject>, arg: *const super::Variant::VARIANT, param2: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn EnumTasks(&self, pdo: ::core::option::Option<&super::Com::IDataObject>, sztaskgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnumTASK>;
    fn GetTitle(&self, pszgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDescriptiveText(&self, pszgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetBackground(&self, pszgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<MMC_TASK_DISPLAY_OBJECT>;
    fn GetListPadInfo(&self, pszgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<MMC_LISTPAD_INFO>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IExtendTaskPad {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IExtendTaskPad_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: isize>() -> IExtendTaskPad_Vtbl {
        unsafe extern "system" fn TaskNotify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdo: *mut ::core::ffi::c_void, arg: *const super::Variant::VARIANT, param2: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TaskNotify(::windows_core::from_raw_borrowed(&pdo), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumTasks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdo: *mut ::core::ffi::c_void, sztaskgroup: ::windows_core::PCWSTR, ppenumtask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumTasks(::windows_core::from_raw_borrowed(&pdo), ::core::mem::transmute(&sztaskgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumtask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: ::windows_core::PCWSTR, psztitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTitle(::core::mem::transmute(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psztitle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptiveText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: ::windows_core::PCWSTR, pszdescriptivetext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescriptiveText(::core::mem::transmute(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescriptivetext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackground<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: ::windows_core::PCWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackground(::core::mem::transmute(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListPadInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: ::windows_core::PCWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetListPadInfo(::core::mem::transmute(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lplistpadinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TaskNotify: TaskNotify::<Identity, Impl, OFFSET>,
            EnumTasks: EnumTasks::<Identity, Impl, OFFSET>,
            GetTitle: GetTitle::<Identity, Impl, OFFSET>,
            GetDescriptiveText: GetDescriptiveText::<Identity, Impl, OFFSET>,
            GetBackground: GetBackground::<Identity, Impl, OFFSET>,
            GetListPadInfo: GetListPadInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IExtendTaskPad as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendView_Impl: Sized {
    fn GetViews(&self, pdataobject: ::core::option::Option<&super::Com::IDataObject>, pviewextensioncallback: ::core::option::Option<&IViewExtensionCallback>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IExtendView {}
#[cfg(feature = "Win32_System_Com")]
impl IExtendView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendView_Impl, const OFFSET: isize>() -> IExtendView_Vtbl {
        unsafe extern "system" fn GetViews<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IExtendView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, pviewextensioncallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetViews(::windows_core::from_raw_borrowed(&pdataobject), ::windows_core::from_raw_borrowed(&pviewextensioncallback)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetViews: GetViews::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IExtendView as ::windows_core::Interface>::IID
    }
}
pub trait IHeaderCtrl_Impl: Sized {
    fn InsertColumn(&self, ncol: i32, title: &::windows_core::PCWSTR, nformat: i32, nwidth: i32) -> ::windows_core::Result<()>;
    fn DeleteColumn(&self, ncol: i32) -> ::windows_core::Result<()>;
    fn SetColumnText(&self, ncol: i32, title: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetColumnText(&self, ncol: i32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> ::windows_core::Result<()>;
    fn GetColumnWidth(&self, ncol: i32) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IHeaderCtrl {}
impl IHeaderCtrl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: isize>() -> IHeaderCtrl_Vtbl {
        unsafe extern "system" fn InsertColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, title: ::windows_core::PCWSTR, nformat: i32, nwidth: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertColumn(::core::mem::transmute_copy(&ncol), ::core::mem::transmute(&title), ::core::mem::transmute_copy(&nformat), ::core::mem::transmute_copy(&nwidth)).into()
        }
        unsafe extern "system" fn DeleteColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteColumn(::core::mem::transmute_copy(&ncol)).into()
        }
        unsafe extern "system" fn SetColumnText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, title: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumnText(::core::mem::transmute_copy(&ncol), ::core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn GetColumnText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, ptext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnText(::core::mem::transmute_copy(&ncol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumnWidth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, nwidth: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumnWidth(::core::mem::transmute_copy(&ncol), ::core::mem::transmute_copy(&nwidth)).into()
        }
        unsafe extern "system" fn GetColumnWidth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnWidth(::core::mem::transmute_copy(&ncol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertColumn: InsertColumn::<Identity, Impl, OFFSET>,
            DeleteColumn: DeleteColumn::<Identity, Impl, OFFSET>,
            SetColumnText: SetColumnText::<Identity, Impl, OFFSET>,
            GetColumnText: GetColumnText::<Identity, Impl, OFFSET>,
            SetColumnWidth: SetColumnWidth::<Identity, Impl, OFFSET>,
            GetColumnWidth: GetColumnWidth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHeaderCtrl as ::windows_core::Interface>::IID
    }
}
pub trait IHeaderCtrl2_Impl: Sized + IHeaderCtrl_Impl {
    fn SetChangeTimeOut(&self, utimeout: u32) -> ::windows_core::Result<()>;
    fn SetColumnFilter(&self, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows_core::Result<()>;
    fn GetColumnFilter(&self, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHeaderCtrl2 {}
impl IHeaderCtrl2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl2_Impl, const OFFSET: isize>() -> IHeaderCtrl2_Vtbl {
        unsafe extern "system" fn SetChangeTimeOut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utimeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChangeTimeOut(::core::mem::transmute_copy(&utimeout)).into()
        }
        unsafe extern "system" fn SetColumnFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumnFilter(::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pfilterdata)).into()
        }
        unsafe extern "system" fn GetColumnFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHeaderCtrl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumnFilter(::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pfilterdata)).into()
        }
        Self {
            base__: IHeaderCtrl_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetChangeTimeOut: SetChangeTimeOut::<Identity, Impl, OFFSET>,
            SetColumnFilter: SetColumnFilter::<Identity, Impl, OFFSET>,
            GetColumnFilter: GetColumnFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHeaderCtrl2 as ::windows_core::Interface>::IID || iid == &<IHeaderCtrl as ::windows_core::Interface>::IID
    }
}
pub trait IImageList_Impl: Sized {
    fn ImageListSetIcon(&self, picon: *const isize, nloc: i32) -> ::windows_core::Result<()>;
    fn ImageListSetStrip(&self, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: super::super::Foundation::COLORREF) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IImageList {}
impl IImageList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>() -> IImageList_Vtbl {
        unsafe extern "system" fn ImageListSetIcon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picon: *const isize, nloc: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImageListSetIcon(::core::mem::transmute_copy(&picon), ::core::mem::transmute_copy(&nloc)).into()
        }
        unsafe extern "system" fn ImageListSetStrip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: super::super::Foundation::COLORREF) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImageListSetStrip(::core::mem::transmute_copy(&pbmapsm), ::core::mem::transmute_copy(&pbmaplg), ::core::mem::transmute_copy(&nstartloc), ::core::mem::transmute_copy(&cmask)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ImageListSetIcon: ImageListSetIcon::<Identity, Impl, OFFSET>,
            ImageListSetStrip: ImageListSetStrip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IImageList as ::windows_core::Interface>::IID
    }
}
pub trait IMMCVersionInfo_Impl: Sized {
    fn GetMMCVersion(&self, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IMMCVersionInfo {}
impl IMMCVersionInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMMCVersionInfo_Impl, const OFFSET: isize>() -> IMMCVersionInfo_Vtbl {
        unsafe extern "system" fn GetMMCVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMMCVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMMCVersion(::core::mem::transmute_copy(&pversionmajor), ::core::mem::transmute_copy(&pversionminor)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMMCVersion: GetMMCVersion::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMMCVersionInfo as ::windows_core::Interface>::IID
    }
}
pub trait IMenuButton_Impl: Sized {
    fn AddButton(&self, idcommand: i32, lpbuttontext: &::windows_core::PCWSTR, lptooltiptext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetButton(&self, idcommand: i32, lpbuttontext: &::windows_core::PCWSTR, lptooltiptext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IMenuButton {}
impl IMenuButton_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMenuButton_Impl, const OFFSET: isize>() -> IMenuButton_Vtbl {
        unsafe extern "system" fn AddButton<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMenuButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: ::windows_core::PCWSTR, lptooltiptext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddButton(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute(&lpbuttontext), ::core::mem::transmute(&lptooltiptext)).into()
        }
        unsafe extern "system" fn SetButton<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMenuButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: ::windows_core::PCWSTR, lptooltiptext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButton(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute(&lpbuttontext), ::core::mem::transmute(&lptooltiptext)).into()
        }
        unsafe extern "system" fn SetButtonState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMenuButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonState(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddButton: AddButton::<Identity, Impl, OFFSET>,
            SetButton: SetButton::<Identity, Impl, OFFSET>,
            SetButtonState: SetButtonState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMenuButton as ::windows_core::Interface>::IID
    }
}
pub trait IMessageView_Impl: Sized {
    fn SetTitleText(&self, psztitletext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetBodyText(&self, pszbodytext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetIcon(&self, id: IconIdentifier) -> ::windows_core::Result<()>;
    fn Clear(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IMessageView {}
impl IMessageView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: isize>() -> IMessageView_Vtbl {
        unsafe extern "system" fn SetTitleText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitletext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTitleText(::core::mem::transmute(&psztitletext)).into()
        }
        unsafe extern "system" fn SetBodyText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbodytext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBodyText(::core::mem::transmute(&pszbodytext)).into()
        }
        unsafe extern "system" fn SetIcon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: IconIdentifier) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIcon(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTitleText: SetTitleText::<Identity, Impl, OFFSET>,
            SetBodyText: SetBodyText::<Identity, Impl, OFFSET>,
            SetIcon: SetIcon::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMessageView as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait INodeProperties_Impl: Sized {
    fn GetProperty(&self, pdataobject: ::core::option::Option<&super::Com::IDataObject>, szpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for INodeProperties {}
#[cfg(feature = "Win32_System_Com")]
impl INodeProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INodeProperties_Impl, const OFFSET: isize>() -> INodeProperties_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INodeProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, szpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute(&szpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProperty: GetProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INodeProperties as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_UI_Controls\"`"]
#[cfg(feature = "Win32_UI_Controls")]
pub trait IPropertySheetCallback_Impl: Sized {
    fn AddPage(&self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::Result<()>;
    fn RemovePage(&self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Controls")]
impl ::windows_core::RuntimeName for IPropertySheetCallback {}
#[cfg(feature = "Win32_UI_Controls")]
impl IPropertySheetCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetCallback_Impl, const OFFSET: isize>() -> IPropertySheetCallback_Vtbl {
        unsafe extern "system" fn AddPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPage(::core::mem::transmute_copy(&hpage)).into()
        }
        unsafe extern "system" fn RemovePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePage(::core::mem::transmute_copy(&hpage)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            RemovePage: RemovePage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertySheetCallback as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPropertySheetProvider_Impl: Sized {
    fn CreatePropertySheet(&self, title: &::windows_core::PCWSTR, r#type: u8, cookie: isize, pidataobjectm: ::core::option::Option<&super::Com::IDataObject>, dwoptions: u32) -> ::windows_core::Result<()>;
    fn FindPropertySheet(&self, hitem: isize, lpcomponent: ::core::option::Option<&IComponent>, lpdataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn AddPrimaryPages(&self, lpunknown: ::core::option::Option<&::windows_core::IUnknown>, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AddExtensionPages(&self) -> ::windows_core::Result<()>;
    fn Show(&self, window: isize, page: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IPropertySheetProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IPropertySheetProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: isize>() -> IPropertySheetProvider_Vtbl {
        unsafe extern "system" fn CreatePropertySheet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::windows_core::PCWSTR, r#type: u8, cookie: isize, pidataobjectm: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePropertySheet(::core::mem::transmute(&title), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&cookie), ::windows_core::from_raw_borrowed(&pidataobjectm), ::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn FindPropertySheet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, lpcomponent: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindPropertySheet(::core::mem::transmute_copy(&hitem), ::windows_core::from_raw_borrowed(&lpcomponent), ::windows_core::from_raw_borrowed(&lpdataobject)).into()
        }
        unsafe extern "system" fn AddPrimaryPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPrimaryPages(::windows_core::from_raw_borrowed(&lpunknown), ::core::mem::transmute_copy(&bcreatehandle), ::core::mem::transmute_copy(&hnotifywindow), ::core::mem::transmute_copy(&bscopepane)).into()
        }
        unsafe extern "system" fn AddExtensionPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddExtensionPages().into()
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: isize, page: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&page)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertySheet: CreatePropertySheet::<Identity, Impl, OFFSET>,
            FindPropertySheet: FindPropertySheet::<Identity, Impl, OFFSET>,
            AddPrimaryPages: AddPrimaryPages::<Identity, Impl, OFFSET>,
            AddExtensionPages: AddExtensionPages::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertySheetProvider as ::windows_core::Interface>::IID
    }
}
pub trait IRequiredExtensions_Impl: Sized {
    fn EnableAllExtensions(&self) -> ::windows_core::Result<()>;
    fn GetFirstExtension(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetNextExtension(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for IRequiredExtensions {}
impl IRequiredExtensions_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRequiredExtensions_Impl, const OFFSET: isize>() -> IRequiredExtensions_Vtbl {
        unsafe extern "system" fn EnableAllExtensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRequiredExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableAllExtensions().into()
        }
        unsafe extern "system" fn GetFirstExtension<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRequiredExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFirstExtension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextExtension<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRequiredExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextExtension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableAllExtensions: EnableAllExtensions::<Identity, Impl, OFFSET>,
            GetFirstExtension: GetFirstExtension::<Identity, Impl, OFFSET>,
            GetNextExtension: GetNextExtension::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRequiredExtensions as ::windows_core::Interface>::IID
    }
}
pub trait IResultData_Impl: Sized {
    fn InsertItem(&self, item: *mut RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn DeleteItem(&self, itemid: isize, ncol: i32) -> ::windows_core::Result<()>;
    fn FindItemByLParam(&self, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<isize>;
    fn DeleteAllRsltItems(&self) -> ::windows_core::Result<()>;
    fn SetItem(&self, item: *const RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn GetItem(&self, item: *mut RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn ModifyItemState(&self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows_core::Result<()>;
    fn ModifyViewStyle(&self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows_core::Result<()>;
    fn SetViewMode(&self, lviewmode: i32) -> ::windows_core::Result<()>;
    fn GetViewMode(&self) -> ::windows_core::Result<i32>;
    fn UpdateItem(&self, itemid: isize) -> ::windows_core::Result<()>;
    fn Sort(&self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetDescBarText(&self, desctext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetItemCount(&self, nitemcount: i32, dwoptions: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IResultData {}
impl IResultData_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>() -> IResultData_Vtbl {
        unsafe extern "system" fn InsertItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize, ncol: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&ncol)).into()
        }
        unsafe extern "system" fn FindItemByLParam<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindItemByLParam(::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAllRsltItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAllRsltItems().into()
        }
        unsafe extern "system" fn SetItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const RESULTDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetNextItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn ModifyItemState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyItemState(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&uadd), ::core::mem::transmute_copy(&uremove)).into()
        }
        unsafe extern "system" fn ModifyViewStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyViewStyle(::core::mem::transmute_copy(&add), ::core::mem::transmute_copy(&remove)).into()
        }
        unsafe extern "system" fn SetViewMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lviewmode: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewMode(::core::mem::transmute_copy(&lviewmode)).into()
        }
        unsafe extern "system" fn GetViewMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lviewmode: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lviewmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateItem(::core::mem::transmute_copy(&itemid)).into()
        }
        unsafe extern "system" fn Sort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Sort(::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwsortoptions), ::core::mem::transmute_copy(&luserparam)).into()
        }
        unsafe extern "system" fn SetDescBarText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desctext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescBarText(::core::mem::transmute(&desctext)).into()
        }
        unsafe extern "system" fn SetItemCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetItemCount(::core::mem::transmute_copy(&nitemcount), ::core::mem::transmute_copy(&dwoptions)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertItem: InsertItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            FindItemByLParam: FindItemByLParam::<Identity, Impl, OFFSET>,
            DeleteAllRsltItems: DeleteAllRsltItems::<Identity, Impl, OFFSET>,
            SetItem: SetItem::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            GetNextItem: GetNextItem::<Identity, Impl, OFFSET>,
            ModifyItemState: ModifyItemState::<Identity, Impl, OFFSET>,
            ModifyViewStyle: ModifyViewStyle::<Identity, Impl, OFFSET>,
            SetViewMode: SetViewMode::<Identity, Impl, OFFSET>,
            GetViewMode: GetViewMode::<Identity, Impl, OFFSET>,
            UpdateItem: UpdateItem::<Identity, Impl, OFFSET>,
            Sort: Sort::<Identity, Impl, OFFSET>,
            SetDescBarText: SetDescBarText::<Identity, Impl, OFFSET>,
            SetItemCount: SetItemCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IResultData as ::windows_core::Interface>::IID
    }
}
pub trait IResultData2_Impl: Sized + IResultData_Impl {
    fn RenameResultItem(&self, itemid: isize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IResultData2 {}
impl IResultData2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData2_Impl, const OFFSET: isize>() -> IResultData2_Vtbl {
        unsafe extern "system" fn RenameResultItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultData2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenameResultItem(::core::mem::transmute_copy(&itemid)).into()
        }
        Self { base__: IResultData_Vtbl::new::<Identity, Impl, OFFSET>(), RenameResultItem: RenameResultItem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IResultData2 as ::windows_core::Interface>::IID || iid == &<IResultData as ::windows_core::Interface>::IID
    }
}
pub trait IResultDataCompare_Impl: Sized {
    fn Compare(&self, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IResultDataCompare {}
impl IResultDataCompare_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultDataCompare_Impl, const OFFSET: isize>() -> IResultDataCompare_Vtbl {
        unsafe extern "system" fn Compare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultDataCompare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compare(::core::mem::transmute_copy(&luserparam), ::core::mem::transmute_copy(&cookiea), ::core::mem::transmute_copy(&cookieb), ::core::mem::transmute_copy(&pnresult)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Compare: Compare::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IResultDataCompare as ::windows_core::Interface>::IID
    }
}
pub trait IResultDataCompareEx_Impl: Sized {
    fn Compare(&self, prdc: *const RDCOMPARE) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IResultDataCompareEx {}
impl IResultDataCompareEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultDataCompareEx_Impl, const OFFSET: isize>() -> IResultDataCompareEx_Vtbl {
        unsafe extern "system" fn Compare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultDataCompareEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdc: *const RDCOMPARE, pnresult: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Compare(::core::mem::transmute_copy(&prdc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Compare: Compare::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IResultDataCompareEx as ::windows_core::Interface>::IID
    }
}
pub trait IResultOwnerData_Impl: Sized {
    fn FindItem(&self, pfindinfo: *const RESULTFINDINFO) -> ::windows_core::Result<i32>;
    fn CacheHint(&self, nstartindex: i32, nendindex: i32) -> ::windows_core::Result<()>;
    fn SortItems(&self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IResultOwnerData {}
impl IResultOwnerData_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultOwnerData_Impl, const OFFSET: isize>() -> IResultOwnerData_Vtbl {
        unsafe extern "system" fn FindItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultOwnerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindItem(::core::mem::transmute_copy(&pfindinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnfoundindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheHint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultOwnerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nstartindex: i32, nendindex: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CacheHint(::core::mem::transmute_copy(&nstartindex), ::core::mem::transmute_copy(&nendindex)).into()
        }
        unsafe extern "system" fn SortItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IResultOwnerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SortItems(::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwsortoptions), ::core::mem::transmute_copy(&luserparam)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindItem: FindItem::<Identity, Impl, OFFSET>,
            CacheHint: CacheHint::<Identity, Impl, OFFSET>,
            SortItems: SortItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IResultOwnerData as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ISnapinAbout_Impl: Sized {
    fn GetSnapinDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetProvider(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSnapinVersion(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSnapinImage(&self) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn GetStaticFolderImage(&self, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for ISnapinAbout {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ISnapinAbout_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: isize>() -> ISnapinAbout_Vtbl {
        unsafe extern "system" fn GetSnapinDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSnapinDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapinVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpversion: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSnapinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapinImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSnapinImage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(happicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStaticFolderImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStaticFolderImage(::core::mem::transmute_copy(&hsmallimage), ::core::mem::transmute_copy(&hsmallimageopen), ::core::mem::transmute_copy(&hlargeimage), ::core::mem::transmute_copy(&cmask)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSnapinDescription: GetSnapinDescription::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            GetSnapinVersion: GetSnapinVersion::<Identity, Impl, OFFSET>,
            GetSnapinImage: GetSnapinImage::<Identity, Impl, OFFSET>,
            GetStaticFolderImage: GetStaticFolderImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISnapinAbout as ::windows_core::Interface>::IID
    }
}
pub trait ISnapinHelp_Impl: Sized {
    fn GetHelpTopic(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::RuntimeName for ISnapinHelp {}
impl ISnapinHelp_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinHelp_Impl, const OFFSET: isize>() -> ISnapinHelp_Vtbl {
        unsafe extern "system" fn GetHelpTopic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinHelp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfile: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHelpTopic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpcompiledhelpfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHelpTopic: GetHelpTopic::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISnapinHelp as ::windows_core::Interface>::IID
    }
}
pub trait ISnapinHelp2_Impl: Sized + ISnapinHelp_Impl {
    fn GetLinkedTopics(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::RuntimeName for ISnapinHelp2 {}
impl ISnapinHelp2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinHelp2_Impl, const OFFSET: isize>() -> ISnapinHelp2_Vtbl {
        unsafe extern "system" fn GetLinkedTopics<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinHelp2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfiles: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLinkedTopics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpcompiledhelpfiles, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ISnapinHelp_Vtbl::new::<Identity, Impl, OFFSET>(), GetLinkedTopics: GetLinkedTopics::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISnapinHelp2 as ::windows_core::Interface>::IID || iid == &<ISnapinHelp as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISnapinProperties_Impl: Sized {
    fn Initialize(&self, pproperties: ::core::option::Option<&Properties>) -> ::windows_core::Result<()>;
    fn QueryPropertyNames(&self, pcallback: ::core::option::Option<&ISnapinPropertiesCallback>) -> ::windows_core::Result<()>;
    fn PropertiesChanged(&self, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISnapinProperties {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISnapinProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinProperties_Impl, const OFFSET: isize>() -> ISnapinProperties_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pproperties)).into()
        }
        unsafe extern "system" fn QueryPropertyNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryPropertyNames(::windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn PropertiesChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PropertiesChanged(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&pproperties)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            QueryPropertyNames: QueryPropertyNames::<Identity, Impl, OFFSET>,
            PropertiesChanged: PropertiesChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISnapinProperties as ::windows_core::Interface>::IID
    }
}
pub trait ISnapinPropertiesCallback_Impl: Sized {
    fn AddPropertyName(&self, pszpropname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISnapinPropertiesCallback {}
impl ISnapinPropertiesCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinPropertiesCallback_Impl, const OFFSET: isize>() -> ISnapinPropertiesCallback_Vtbl {
        unsafe extern "system" fn AddPropertyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISnapinPropertiesCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyName(::core::mem::transmute(&pszpropname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPropertyName: AddPropertyName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISnapinPropertiesCallback as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IStringTable_Impl: Sized {
    fn AddString(&self, pszadd: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn GetString(&self, stringid: u32, cchbuffer: u32, lpbuffer: ::windows_core::PWSTR, pcchout: *mut u32) -> ::windows_core::Result<()>;
    fn GetStringLength(&self, stringid: u32) -> ::windows_core::Result<u32>;
    fn DeleteString(&self, stringid: u32) -> ::windows_core::Result<()>;
    fn DeleteAllStrings(&self) -> ::windows_core::Result<()>;
    fn FindString(&self, pszfind: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn Enumerate(&self) -> ::windows_core::Result<super::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IStringTable {}
#[cfg(feature = "Win32_System_Com")]
impl IStringTable_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: isize>() -> IStringTable_Vtbl {
        unsafe extern "system" fn AddString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszadd: ::windows_core::PCWSTR, pstringid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddString(::core::mem::transmute(&pszadd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstringid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32, cchbuffer: u32, lpbuffer: ::windows_core::PWSTR, pcchout: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetString(::core::mem::transmute_copy(&stringid), ::core::mem::transmute_copy(&cchbuffer), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&pcchout)).into()
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32, pcchstring: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringLength(::core::mem::transmute_copy(&stringid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteString(::core::mem::transmute_copy(&stringid)).into()
        }
        unsafe extern "system" fn DeleteAllStrings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAllStrings().into()
        }
        unsafe extern "system" fn FindString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfind: ::windows_core::PCWSTR, pstringid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindString(::core::mem::transmute(&pszfind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstringid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enumerate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enumerate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddString: AddString::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
            DeleteString: DeleteString::<Identity, Impl, OFFSET>,
            DeleteAllStrings: DeleteAllStrings::<Identity, Impl, OFFSET>,
            FindString: FindString::<Identity, Impl, OFFSET>,
            Enumerate: Enumerate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStringTable as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IToolbar_Impl: Sized {
    fn AddBitmap(&self, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: super::super::Foundation::COLORREF) -> ::windows_core::Result<()>;
    fn AddButtons(&self, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows_core::Result<()>;
    fn InsertButton(&self, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows_core::Result<()>;
    fn DeleteButton(&self, nindex: i32) -> ::windows_core::Result<()>;
    fn GetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for IToolbar {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IToolbar_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: isize>() -> IToolbar_Vtbl {
        unsafe extern "system" fn AddBitmap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: super::super::Foundation::COLORREF) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddBitmap(::core::mem::transmute_copy(&nimages), ::core::mem::transmute_copy(&hbmp), ::core::mem::transmute_copy(&cxsize), ::core::mem::transmute_copy(&cysize), ::core::mem::transmute_copy(&crmask)).into()
        }
        unsafe extern "system" fn AddButtons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddButtons(::core::mem::transmute_copy(&nbuttons), ::core::mem::transmute_copy(&lpbuttons)).into()
        }
        unsafe extern "system" fn InsertButton<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertButton(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&lpbutton)).into()
        }
        unsafe extern "system" fn DeleteButton<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteButton(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn GetButtonState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetButtonState(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonState(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddBitmap: AddBitmap::<Identity, Impl, OFFSET>,
            AddButtons: AddButtons::<Identity, Impl, OFFSET>,
            InsertButton: InsertButton::<Identity, Impl, OFFSET>,
            DeleteButton: DeleteButton::<Identity, Impl, OFFSET>,
            GetButtonState: GetButtonState::<Identity, Impl, OFFSET>,
            SetButtonState: SetButtonState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IToolbar as ::windows_core::Interface>::IID
    }
}
pub trait IViewExtensionCallback_Impl: Sized {
    fn AddView(&self, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IViewExtensionCallback {}
impl IViewExtensionCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewExtensionCallback_Impl, const OFFSET: isize>() -> IViewExtensionCallback_Vtbl {
        unsafe extern "system" fn AddView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IViewExtensionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddView(::core::mem::transmute_copy(&pextviewdata)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddView: AddView::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IViewExtensionCallback as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait MenuItem_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LanguageIndependentName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LanguageIndependentPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Execute(&self) -> ::windows_core::Result<()>;
    fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for MenuItem {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl MenuItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: isize>() -> MenuItem_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn LanguageIndependentName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageindependentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LanguageIndependentName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageindependentname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageIndependentPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageindependentpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LanguageIndependentPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageindependentpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Execute().into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            LanguageIndependentName: LanguageIndependentName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            LanguageIndependentPath: LanguageIndependentPath::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<MenuItem as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Node_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_Property(&self, propertyname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Bookmark(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsScopeNode(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Nodetype(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Node {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Node_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Node_Impl, const OFFSET: isize>() -> Node_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Node_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Property<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Node_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Property(::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Node_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmark: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Bookmark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bookmark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScopeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Node_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscopenode: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsScopeNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isscopenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nodetype<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Node_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Nodetype() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            get_Property: get_Property::<Identity, Impl, OFFSET>,
            Bookmark: Bookmark::<Identity, Impl, OFFSET>,
            IsScopeNode: IsScopeNode::<Identity, Impl, OFFSET>,
            Nodetype: Nodetype::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Node as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Nodes_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(&self, index: i32) -> ::windows_core::Result<Node>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Nodes {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Nodes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Nodes_Impl, const OFFSET: isize>() -> Nodes_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Nodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Nodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Nodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
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
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Nodes as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Properties_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<Property>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Remove(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Properties {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Properties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Properties_Impl, const OFFSET: isize>() -> Properties_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Properties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Properties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Properties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Properties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&name)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Properties as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Property_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, value: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Property {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Property_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Property_Impl, const OFFSET: isize>() -> Property_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Property_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Property_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Property_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Property as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ScopeNamespace_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetParent(&self, node: ::core::option::Option<&Node>) -> ::windows_core::Result<Node>;
    fn GetChild(&self, node: ::core::option::Option<&Node>) -> ::windows_core::Result<Node>;
    fn GetNext(&self, node: ::core::option::Option<&Node>) -> ::windows_core::Result<Node>;
    fn GetRoot(&self) -> ::windows_core::Result<Node>;
    fn Expand(&self, node: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ScopeNamespace {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ScopeNamespace_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: isize>() -> ScopeNamespace_Vtbl {
        unsafe extern "system" fn GetParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParent(::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, child: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChild(::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(child, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNext(::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(next, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoot<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(root, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Expand(::windows_core::from_raw_borrowed(&node)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetParent: GetParent::<Identity, Impl, OFFSET>,
            GetChild: GetChild::<Identity, Impl, OFFSET>,
            GetNext: GetNext::<Identity, Impl, OFFSET>,
            GetRoot: GetRoot::<Identity, Impl, OFFSET>,
            Expand: Expand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ScopeNamespace as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SnapIn_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Vendor(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Version(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Extensions(&self) -> ::windows_core::Result<Extensions>;
    fn SnapinCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Properties(&self) -> ::windows_core::Result<Properties>;
    fn EnableAllExtensions(&self, enable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for SnapIn {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl SnapIn_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: isize>() -> SnapIn_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Vendor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vendor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinCLSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SnapinCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapinclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAllExtensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableAllExtensions(::core::mem::transmute_copy(&enable)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Vendor: Vendor::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            Extensions: Extensions::<Identity, Impl, OFFSET>,
            SnapinCLSID: SnapinCLSID::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            EnableAllExtensions: EnableAllExtensions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<SnapIn as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SnapIns_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(&self, index: i32) -> ::windows_core::Result<SnapIn>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Add(&self, snapinnameorclsid: &::windows_core::BSTR, parentsnapin: &super::Variant::VARIANT, properties: &super::Variant::VARIANT) -> ::windows_core::Result<SnapIn>;
    fn Remove(&self, snapin: ::core::option::Option<&SnapIn>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for SnapIns {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl SnapIns_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: isize>() -> SnapIns_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, snapin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinnameorclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, parentsnapin: super::Variant::VARIANT, properties: super::Variant::VARIANT, snapin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&snapinnameorclsid), ::core::mem::transmute(&parentsnapin), ::core::mem::transmute(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::windows_core::from_raw_borrowed(&snapin)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<SnapIns as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait View_Impl: Sized + super::Com::IDispatch_Impl {
    fn ActiveScopeNode(&self) -> ::windows_core::Result<Node>;
    fn SetActiveScopeNode(&self, node: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
    fn Selection(&self) -> ::windows_core::Result<Nodes>;
    fn ListItems(&self) -> ::windows_core::Result<Nodes>;
    fn SnapinScopeObject(&self, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<super::Com::IDispatch>;
    fn SnapinSelectionObject(&self) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Is(&self, view: ::core::option::Option<&View>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Document(&self) -> ::windows_core::Result<Document>;
    fn SelectAll(&self) -> ::windows_core::Result<()>;
    fn Select(&self, node: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
    fn Deselect(&self, node: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
    fn IsSelected(&self, node: ::core::option::Option<&Node>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DisplayScopeNodePropertySheet(&self, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DisplaySelectionPropertySheet(&self) -> ::windows_core::Result<()>;
    fn CopyScopeNode(&self, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn CopySelection(&self) -> ::windows_core::Result<()>;
    fn DeleteScopeNode(&self, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteSelection(&self) -> ::windows_core::Result<()>;
    fn RenameScopeNode(&self, newname: &::windows_core::BSTR, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RenameSelectedItem(&self, newname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_ScopeNodeContextMenu(&self, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<ContextMenu>;
    fn SelectionContextMenu(&self) -> ::windows_core::Result<ContextMenu>;
    fn RefreshScopeNode(&self, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RefreshSelection(&self) -> ::windows_core::Result<()>;
    fn ExecuteSelectionMenuItem(&self, menuitempath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ExecuteScopeNodeMenuItem(&self, menuitempath: &::windows_core::BSTR, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ExecuteShellCommand(&self, command: &::windows_core::BSTR, directory: &::windows_core::BSTR, parameters: &::windows_core::BSTR, windowstate: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Frame(&self) -> ::windows_core::Result<Frame>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn ScopeTreeVisible(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetScopeTreeVisible(&self, visible: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Back(&self) -> ::windows_core::Result<()>;
    fn Forward(&self) -> ::windows_core::Result<()>;
    fn SetStatusBarText(&self, statusbartext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Memento(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ViewMemento(&self, memento: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Columns(&self) -> ::windows_core::Result<Columns>;
    fn get_CellContents(&self, node: ::core::option::Option<&Node>, column: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ExportList(&self, file: &::windows_core::BSTR, exportoptions: _ExportListOptions) -> ::windows_core::Result<()>;
    fn ListViewMode(&self) -> ::windows_core::Result<_ListViewMode>;
    fn SetListViewMode(&self, mode: _ListViewMode) -> ::windows_core::Result<()>;
    fn ControlObject(&self) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for View {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl View_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>() -> View_Vtbl {
        unsafe extern "system" fn ActiveScopeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActiveScopeNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveScopeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActiveScopeNode(::windows_core::from_raw_borrowed(&node)).into()
        }
        unsafe extern "system" fn Selection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Selection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinScopeObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT, scopenodeobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SnapinScopeObject(::core::mem::transmute(&scopenode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scopenodeobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinSelectionObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SnapinSelectionObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectionobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, thesame: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Is(::windows_core::from_raw_borrowed(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(thesame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Document<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Document() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectAll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectAll().into()
        }
        unsafe extern "system" fn Select<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Select(::windows_core::from_raw_borrowed(&node)).into()
        }
        unsafe extern "system" fn Deselect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deselect(::windows_core::from_raw_borrowed(&node)).into()
        }
        unsafe extern "system" fn IsSelected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, isselected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSelected(::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isselected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayScopeNodePropertySheet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayScopeNodePropertySheet(::core::mem::transmute(&scopenode)).into()
        }
        unsafe extern "system" fn DisplaySelectionPropertySheet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplaySelectionPropertySheet().into()
        }
        unsafe extern "system" fn CopyScopeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyScopeNode(::core::mem::transmute(&scopenode)).into()
        }
        unsafe extern "system" fn CopySelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopySelection().into()
        }
        unsafe extern "system" fn DeleteScopeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteScopeNode(::core::mem::transmute(&scopenode)).into()
        }
        unsafe extern "system" fn DeleteSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteSelection().into()
        }
        unsafe extern "system" fn RenameScopeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenameScopeNode(::core::mem::transmute(&newname), ::core::mem::transmute(&scopenode)).into()
        }
        unsafe extern "system" fn RenameSelectedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenameSelectedItem(::core::mem::transmute(&newname)).into()
        }
        unsafe extern "system" fn get_ScopeNodeContextMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT, contextmenu: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_ScopeNodeContextMenu(::core::mem::transmute(&scopenode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contextmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContextMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextmenu: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectionContextMenu() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contextmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshScopeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshScopeNode(::core::mem::transmute(&scopenode)).into()
        }
        unsafe extern "system" fn RefreshSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshSelection().into()
        }
        unsafe extern "system" fn ExecuteSelectionMenuItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitempath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecuteSelectionMenuItem(::core::mem::transmute(&menuitempath)).into()
        }
        unsafe extern "system" fn ExecuteScopeNodeMenuItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitempath: ::std::mem::MaybeUninit<::windows_core::BSTR>, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecuteScopeNodeMenuItem(::core::mem::transmute(&menuitempath), ::core::mem::transmute(&scopenode)).into()
        }
        unsafe extern "system" fn ExecuteShellCommand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: ::std::mem::MaybeUninit<::windows_core::BSTR>, directory: ::std::mem::MaybeUninit<::windows_core::BSTR>, parameters: ::std::mem::MaybeUninit<::windows_core::BSTR>, windowstate: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecuteShellCommand(::core::mem::transmute(&command), ::core::mem::transmute(&directory), ::core::mem::transmute(&parameters), ::core::mem::transmute(&windowstate)).into()
        }
        unsafe extern "system" fn Frame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Frame() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(frame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn ScopeTreeVisible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScopeTreeVisible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScopeTreeVisible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScopeTreeVisible(::core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn Back<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Back().into()
        }
        unsafe extern "system" fn Forward<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Forward().into()
        }
        unsafe extern "system" fn SetStatusBarText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statusbartext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatusBarText(::core::mem::transmute(&statusbartext)).into()
        }
        unsafe extern "system" fn Memento<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memento: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Memento() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(memento, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewMemento<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memento: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ViewMemento(::core::mem::transmute(&memento)).into()
        }
        unsafe extern "system" fn Columns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, columns: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Columns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(columns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_CellContents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, column: i32, cellcontents: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_CellContents(::windows_core::from_raw_borrowed(&node), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cellcontents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::std::mem::MaybeUninit<::windows_core::BSTR>, exportoptions: _ExportListOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportList(::core::mem::transmute(&file), ::core::mem::transmute_copy(&exportoptions)).into()
        }
        unsafe extern "system" fn ListViewMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut _ListViewMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: _ListViewMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListViewMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn ControlObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, control: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ControlObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(control, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActiveScopeNode: ActiveScopeNode::<Identity, Impl, OFFSET>,
            SetActiveScopeNode: SetActiveScopeNode::<Identity, Impl, OFFSET>,
            Selection: Selection::<Identity, Impl, OFFSET>,
            ListItems: ListItems::<Identity, Impl, OFFSET>,
            SnapinScopeObject: SnapinScopeObject::<Identity, Impl, OFFSET>,
            SnapinSelectionObject: SnapinSelectionObject::<Identity, Impl, OFFSET>,
            Is: Is::<Identity, Impl, OFFSET>,
            Document: Document::<Identity, Impl, OFFSET>,
            SelectAll: SelectAll::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            Deselect: Deselect::<Identity, Impl, OFFSET>,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            DisplayScopeNodePropertySheet: DisplayScopeNodePropertySheet::<Identity, Impl, OFFSET>,
            DisplaySelectionPropertySheet: DisplaySelectionPropertySheet::<Identity, Impl, OFFSET>,
            CopyScopeNode: CopyScopeNode::<Identity, Impl, OFFSET>,
            CopySelection: CopySelection::<Identity, Impl, OFFSET>,
            DeleteScopeNode: DeleteScopeNode::<Identity, Impl, OFFSET>,
            DeleteSelection: DeleteSelection::<Identity, Impl, OFFSET>,
            RenameScopeNode: RenameScopeNode::<Identity, Impl, OFFSET>,
            RenameSelectedItem: RenameSelectedItem::<Identity, Impl, OFFSET>,
            get_ScopeNodeContextMenu: get_ScopeNodeContextMenu::<Identity, Impl, OFFSET>,
            SelectionContextMenu: SelectionContextMenu::<Identity, Impl, OFFSET>,
            RefreshScopeNode: RefreshScopeNode::<Identity, Impl, OFFSET>,
            RefreshSelection: RefreshSelection::<Identity, Impl, OFFSET>,
            ExecuteSelectionMenuItem: ExecuteSelectionMenuItem::<Identity, Impl, OFFSET>,
            ExecuteScopeNodeMenuItem: ExecuteScopeNodeMenuItem::<Identity, Impl, OFFSET>,
            ExecuteShellCommand: ExecuteShellCommand::<Identity, Impl, OFFSET>,
            Frame: Frame::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            ScopeTreeVisible: ScopeTreeVisible::<Identity, Impl, OFFSET>,
            SetScopeTreeVisible: SetScopeTreeVisible::<Identity, Impl, OFFSET>,
            Back: Back::<Identity, Impl, OFFSET>,
            Forward: Forward::<Identity, Impl, OFFSET>,
            SetStatusBarText: SetStatusBarText::<Identity, Impl, OFFSET>,
            Memento: Memento::<Identity, Impl, OFFSET>,
            ViewMemento: ViewMemento::<Identity, Impl, OFFSET>,
            Columns: Columns::<Identity, Impl, OFFSET>,
            get_CellContents: get_CellContents::<Identity, Impl, OFFSET>,
            ExportList: ExportList::<Identity, Impl, OFFSET>,
            ListViewMode: ListViewMode::<Identity, Impl, OFFSET>,
            SetListViewMode: SetListViewMode::<Identity, Impl, OFFSET>,
            ControlObject: ControlObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<View as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Views_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows_core::Result<View>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Add(&self, node: ::core::option::Option<&Node>, viewoptions: _ViewOptions) -> ::windows_core::Result<()>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for Views {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Views_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Views_Impl, const OFFSET: isize>() -> Views_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Views_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, view: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(view, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Views_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Views_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, viewoptions: _ViewOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows_core::from_raw_borrowed(&node), ::core::mem::transmute_copy(&viewoptions)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: Views_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<Views as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _AppEvents_Impl: Sized + super::Com::IDispatch_Impl {
    fn OnQuit(&self, application: ::core::option::Option<&_Application>) -> ::windows_core::Result<()>;
    fn OnDocumentOpen(&self, document: ::core::option::Option<&Document>, new: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnDocumentClose(&self, document: ::core::option::Option<&Document>) -> ::windows_core::Result<()>;
    fn OnSnapInAdded(&self, document: ::core::option::Option<&Document>, snapin: ::core::option::Option<&SnapIn>) -> ::windows_core::Result<()>;
    fn OnSnapInRemoved(&self, document: ::core::option::Option<&Document>, snapin: ::core::option::Option<&SnapIn>) -> ::windows_core::Result<()>;
    fn OnNewView(&self, view: ::core::option::Option<&View>) -> ::windows_core::Result<()>;
    fn OnViewClose(&self, view: ::core::option::Option<&View>) -> ::windows_core::Result<()>;
    fn OnViewChange(&self, view: ::core::option::Option<&View>, newownernode: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
    fn OnSelectionChange(&self, view: ::core::option::Option<&View>, newnodes: ::core::option::Option<&Nodes>) -> ::windows_core::Result<()>;
    fn OnContextMenuExecuted(&self, menuitem: ::core::option::Option<&MenuItem>) -> ::windows_core::Result<()>;
    fn OnToolbarButtonClicked(&self) -> ::windows_core::Result<()>;
    fn OnListUpdated(&self, view: ::core::option::Option<&View>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for _AppEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _AppEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>() -> _AppEvents_Vtbl {
        unsafe extern "system" fn OnQuit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQuit(::windows_core::from_raw_borrowed(&application)).into()
        }
        unsafe extern "system" fn OnDocumentOpen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, new: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDocumentOpen(::windows_core::from_raw_borrowed(&document), ::core::mem::transmute_copy(&new)).into()
        }
        unsafe extern "system" fn OnDocumentClose<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDocumentClose(::windows_core::from_raw_borrowed(&document)).into()
        }
        unsafe extern "system" fn OnSnapInAdded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSnapInAdded(::windows_core::from_raw_borrowed(&document), ::windows_core::from_raw_borrowed(&snapin)).into()
        }
        unsafe extern "system" fn OnSnapInRemoved<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSnapInRemoved(::windows_core::from_raw_borrowed(&document), ::windows_core::from_raw_borrowed(&snapin)).into()
        }
        unsafe extern "system" fn OnNewView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNewView(::windows_core::from_raw_borrowed(&view)).into()
        }
        unsafe extern "system" fn OnViewClose<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnViewClose(::windows_core::from_raw_borrowed(&view)).into()
        }
        unsafe extern "system" fn OnViewChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, newownernode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnViewChange(::windows_core::from_raw_borrowed(&view), ::windows_core::from_raw_borrowed(&newownernode)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, newnodes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSelectionChange(::windows_core::from_raw_borrowed(&view), ::windows_core::from_raw_borrowed(&newnodes)).into()
        }
        unsafe extern "system" fn OnContextMenuExecuted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnContextMenuExecuted(::windows_core::from_raw_borrowed(&menuitem)).into()
        }
        unsafe extern "system" fn OnToolbarButtonClicked<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnToolbarButtonClicked().into()
        }
        unsafe extern "system" fn OnListUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnListUpdated(::windows_core::from_raw_borrowed(&view)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnQuit: OnQuit::<Identity, Impl, OFFSET>,
            OnDocumentOpen: OnDocumentOpen::<Identity, Impl, OFFSET>,
            OnDocumentClose: OnDocumentClose::<Identity, Impl, OFFSET>,
            OnSnapInAdded: OnSnapInAdded::<Identity, Impl, OFFSET>,
            OnSnapInRemoved: OnSnapInRemoved::<Identity, Impl, OFFSET>,
            OnNewView: OnNewView::<Identity, Impl, OFFSET>,
            OnViewClose: OnViewClose::<Identity, Impl, OFFSET>,
            OnViewChange: OnViewChange::<Identity, Impl, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, Impl, OFFSET>,
            OnContextMenuExecuted: OnContextMenuExecuted::<Identity, Impl, OFFSET>,
            OnToolbarButtonClicked: OnToolbarButtonClicked::<Identity, Impl, OFFSET>,
            OnListUpdated: OnListUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<_AppEvents as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _Application_Impl: Sized + super::Com::IDispatch_Impl {
    fn Help(&self);
    fn Quit(&self);
    fn Document(&self) -> ::windows_core::Result<Document>;
    fn Load(&self, filename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Frame(&self) -> ::windows_core::Result<Frame>;
    fn Visible(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Show(&self) -> ::windows_core::Result<()>;
    fn Hide(&self) -> ::windows_core::Result<()>;
    fn UserControl(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUserControl(&self, usercontrol: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn VersionMajor(&self) -> ::windows_core::Result<i32>;
    fn VersionMinor(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for _Application {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _Application_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>() -> _Application_Vtbl {
        unsafe extern "system" fn Help<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Help()
        }
        unsafe extern "system" fn Quit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Quit()
        }
        unsafe extern "system" fn Document<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Document() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn Frame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Frame() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(frame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Visible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show().into()
        }
        unsafe extern "system" fn Hide<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Hide().into()
        }
        unsafe extern "system" fn UserControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontrol: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usercontrol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontrol: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserControl(::core::mem::transmute_copy(&usercontrol)).into()
        }
        unsafe extern "system" fn VersionMajor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, versionmajor: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VersionMajor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(versionmajor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VersionMinor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _Application_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, versionminor: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VersionMinor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(versionminor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Help: Help::<Identity, Impl, OFFSET>,
            Quit: Quit::<Identity, Impl, OFFSET>,
            Document: Document::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Frame: Frame::<Identity, Impl, OFFSET>,
            Visible: Visible::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            Hide: Hide::<Identity, Impl, OFFSET>,
            UserControl: UserControl::<Identity, Impl, OFFSET>,
            SetUserControl: SetUserControl::<Identity, Impl, OFFSET>,
            VersionMajor: VersionMajor::<Identity, Impl, OFFSET>,
            VersionMinor: VersionMinor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<_Application as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _EventConnector_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectTo(&self, application: ::core::option::Option<&_Application>) -> ::windows_core::Result<()>;
    fn Disconnect(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for _EventConnector {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _EventConnector_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _EventConnector_Impl, const OFFSET: isize>() -> _EventConnector_Vtbl {
        unsafe extern "system" fn ConnectTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _EventConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectTo(::windows_core::from_raw_borrowed(&application)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _EventConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectTo: ConnectTo::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<_EventConnector as ::windows_core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
