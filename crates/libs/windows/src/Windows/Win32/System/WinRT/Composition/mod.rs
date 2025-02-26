::windows_core::imp::com_interface!(ICompositionCapabilitiesInteropFactory, ICompositionCapabilitiesInteropFactory_Vtbl, 0x2c9db356_e70d_4642_8298_bc4aa5b4865c);
::windows_core::imp::interface_hierarchy!(ICompositionCapabilitiesInteropFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ICompositionCapabilitiesInteropFactory {
    #[doc = "Required features: `\"UI_Composition\"`"]
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn GetForWindow<P0>(&self, hwnd: P0) -> ::windows_core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionCapabilitiesInteropFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetForWindow: usize,
}
::windows_core::imp::com_interface!(ICompositionDrawingSurfaceInterop, ICompositionDrawingSurfaceInterop_Vtbl, 0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee);
::windows_core::imp::interface_hierarchy!(ICompositionDrawingSurfaceInterop, ::windows_core::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    pub unsafe fn BeginDraw<T>(&self, updaterect: ::core::option::Option<*const super::super::super::Foundation::RECT>, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).BeginDraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updaterect.unwrap_or(::std::ptr::null())), &T::IID, &mut result__, updateoffset).from_abi(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resize(&self, sizepixels: super::super::super::Foundation::SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sizepixels)).ok()
    }
    pub unsafe fn Scroll(&self, scrollrect: ::core::option::Option<*const super::super::super::Foundation::RECT>, cliprect: ::core::option::Option<*const super::super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Scroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scrollrect.unwrap_or(::std::ptr::null())), ::core::mem::transmute(cliprect.unwrap_or(::std::ptr::null())), offsetx, offsety).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResumeDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SuspendDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> ::windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICompositionDrawingSurfaceInterop2, ICompositionDrawingSurfaceInterop2_Vtbl, 0x41e64aae_98c0_4239_8e95_a330dd6aa18b);
::windows_core::imp::interface_hierarchy!(ICompositionDrawingSurfaceInterop2, ::windows_core::IUnknown, ICompositionDrawingSurfaceInterop);
impl ICompositionDrawingSurfaceInterop2 {
    pub unsafe fn BeginDraw<T>(&self, updaterect: ::core::option::Option<*const super::super::super::Foundation::RECT>, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows_core::Result<T>
    where
        T: ::windows_core::Interface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.BeginDraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updaterect.unwrap_or(::std::ptr::null())), &T::IID, &mut result__, updateoffset).from_abi(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resize(&self, sizepixels: super::super::super::Foundation::SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Resize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sizepixels)).ok()
    }
    pub unsafe fn Scroll(&self, scrollrect: ::core::option::Option<*const super::super::super::Foundation::RECT>, cliprect: ::core::option::Option<*const super::super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Scroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scrollrect.unwrap_or(::std::ptr::null())), ::core::mem::transmute(cliprect.unwrap_or(::std::ptr::null())), offsetx, offsety).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ResumeDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SuspendDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CopySurface<P0>(&self, destinationresource: P0, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: ::core::option::Option<*const super::super::super::Foundation::RECT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CopySurface)(::windows_core::Interface::as_raw(self), destinationresource.into_param().abi(), destinationoffsetx, destinationoffsety, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop2_Vtbl {
    pub base__: ICompositionDrawingSurfaceInterop_Vtbl,
    pub CopySurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICompositionGraphicsDeviceInterop, ICompositionGraphicsDeviceInterop_Vtbl, 0xa116ff71_f8bf_4c8a_9c98_70779a32a9c8);
::windows_core::imp::interface_hierarchy!(ICompositionGraphicsDeviceInterop, ::windows_core::IUnknown);
impl ICompositionGraphicsDeviceInterop {
    pub unsafe fn GetRenderingDevice(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRenderingDevice)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRenderingDevice<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetRenderingDevice)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionGraphicsDeviceInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetRenderingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRenderingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICompositorDesktopInterop, ICompositorDesktopInterop_Vtbl, 0x29e691fa_4567_4dca_b319_d0f207eb6807);
::windows_core::imp::interface_hierarchy!(ICompositorDesktopInterop, ::windows_core::IUnknown);
impl ICompositorDesktopInterop {
    #[doc = "Required features: `\"UI_Composition_Desktop\"`"]
    #[cfg(feature = "UI_Composition_Desktop")]
    pub unsafe fn CreateDesktopWindowTarget<P0, P1>(&self, hwndtarget: P0, istopmost: P1) -> ::windows_core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDesktopWindowTarget)(::windows_core::Interface::as_raw(self), hwndtarget.into_param().abi(), istopmost.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnsureOnThread(&self, threadid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnsureOnThread)(::windows_core::Interface::as_raw(self), threadid).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorDesktopInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "UI_Composition_Desktop")]
    pub CreateDesktopWindowTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition_Desktop"))]
    CreateDesktopWindowTarget: usize,
    pub EnsureOnThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICompositorInterop, ICompositorInterop_Vtbl, 0x25297d5c_3ad4_4c9c_b5cf_e36a38512330);
::windows_core::imp::interface_hierarchy!(ICompositorInterop, ::windows_core::IUnknown);
impl ICompositorInterop {
    #[doc = "Required features: `\"UI_Composition\"`"]
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForHandle<P0>(&self, swapchain: P0) -> ::windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateCompositionSurfaceForHandle)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"UI_Composition\"`"]
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForSwapChain<P0>(&self, swapchain: P0) -> ::windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateCompositionSurfaceForSwapChain)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"UI_Composition\"`"]
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateGraphicsDevice<P0>(&self, renderingdevice: P0) -> ::windows_core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateGraphicsDevice)(::windows_core::Interface::as_raw(self), renderingdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForHandle: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForSwapChain: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateGraphicsDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateGraphicsDevice: usize,
}
::windows_core::imp::com_interface!(IDesktopWindowTargetInterop, IDesktopWindowTargetInterop_Vtbl, 0x35dbf59e_e3f9_45b0_81e7_fe75f4145dc9);
::windows_core::imp::interface_hierarchy!(IDesktopWindowTargetInterop, ::windows_core::IUnknown);
impl IDesktopWindowTargetInterop {
    pub unsafe fn Hwnd(&self) -> ::windows_core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Hwnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTargetInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Hwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVisualInteractionSourceInterop, IVisualInteractionSourceInterop_Vtbl, 0x11f62cd1_2f9d_42d3_b05f_d6790d9e9f8e);
::windows_core::imp::interface_hierarchy!(IVisualInteractionSourceInterop, ::windows_core::IUnknown);
impl IVisualInteractionSourceInterop {
    #[doc = "Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TryRedirectForManipulation)(::windows_core::Interface::as_raw(self), pointerinfo).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))]
    TryRedirectForManipulation: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
