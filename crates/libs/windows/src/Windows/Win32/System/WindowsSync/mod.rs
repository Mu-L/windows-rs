::windows_core::imp::com_interface!(IAsynchronousDataRetriever, IAsynchronousDataRetriever_Vtbl, 0x9fc7e470_61ea_4a88_9be4_df56a27cfef2);
::windows_core::imp::interface_hierarchy!(IAsynchronousDataRetriever, ::windows_core::IUnknown);
impl IAsynchronousDataRetriever {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdParameters)(::windows_core::Interface::as_raw(self), pidparameters).ok()
    }
    pub unsafe fn RegisterCallback<P0>(&self, pdataretrievercallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDataRetrieverCallback>,
    {
        (::windows_core::Interface::vtable(self).RegisterCallback)(::windows_core::Interface::as_raw(self), pdataretrievercallback.into_param().abi()).ok()
    }
    pub unsafe fn RevokeCallback<P0>(&self, pdataretrievercallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDataRetrieverCallback>,
    {
        (::windows_core::Interface::vtable(self).RevokeCallback)(::windows_core::Interface::as_raw(self), pdataretrievercallback.into_param().abi()).ok()
    }
    pub unsafe fn LoadChangeData<P0>(&self, ploadchangecontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ILoadChangeContext>,
    {
        (::windows_core::Interface::vtable(self).LoadChangeData)(::windows_core::Interface::as_raw(self), ploadchangecontext.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsynchronousDataRetriever_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT,
    pub RegisterCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataretrievercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RevokeCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataretrievercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LoadChangeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploadchangecontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IChangeConflict, IChangeConflict_Vtbl, 0x014ebf97_9f20_4f7a_bdd4_25979c77c002);
::windows_core::imp::interface_hierarchy!(IChangeConflict, ::windows_core::IUnknown);
impl IChangeConflict {
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderConflictingChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceProviderConflictingChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderConflictingData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceProviderConflictingData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceProviderConflictingData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolveActionForChange(&self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetResolveActionForChange)(::windows_core::Interface::as_raw(self), presolveaction).ok()
    }
    pub unsafe fn SetResolveActionForChange(&self, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetResolveActionForChange)(::windows_core::Interface::as_raw(self), resolveaction).ok()
    }
    pub unsafe fn GetResolveActionForChangeUnit<P0>(&self, pchangeunit: P0, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChangeUnit>,
    {
        (::windows_core::Interface::vtable(self).GetResolveActionForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), presolveaction).ok()
    }
    pub unsafe fn SetResolveActionForChangeUnit<P0>(&self, pchangeunit: P0, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChangeUnit>,
    {
        (::windows_core::Interface::vtable(self).SetResolveActionForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), resolveaction).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeConflict_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub SetResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub GetResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub SetResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IChangeUnitException, IChangeUnitException_Vtbl, 0x0cd7ee7c_fec0_4021_99ee_f0e5348f2a5f);
::windows_core::imp::interface_hierarchy!(IChangeUnitException, ::windows_core::IUnknown);
impl IChangeUnitException {
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetItemId)(::windows_core::Interface::as_raw(self), pbitemid, pcbidsize).ok()
    }
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitId)(::windows_core::Interface::as_raw(self), pbchangeunitid, pcbidsize).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVector)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeUnitException_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IChangeUnitListFilterInfo, IChangeUnitListFilterInfo_Vtbl, 0xf2837671_0bdf_43fa_b502_232375fb50c2);
::windows_core::imp::interface_hierarchy!(IChangeUnitListFilterInfo, ::windows_core::IUnknown, ISyncFilterInfo);
impl IChangeUnitListFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), pbbuffer, pcbbuffer).ok()
    }
    pub unsafe fn Initialize(&self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), ppbchangeunitids, dwchangeunitcount).ok()
    }
    pub unsafe fn GetChangeUnitIdCount(&self, pdwchangeunitidcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitIdCount)(::windows_core::Interface::as_raw(self), pdwchangeunitidcount).ok()
    }
    pub unsafe fn GetChangeUnitId(&self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitId)(::windows_core::Interface::as_raw(self), dwchangeunitidindex, pbchangeunitid, pcbidsize).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeUnitListFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows_core::HRESULT,
    pub GetChangeUnitIdCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IClockVector, IClockVector_Vtbl, 0x14b2274a_8698_4cc6_9333_f89bd1d47bc4);
::windows_core::imp::interface_hierarchy!(IClockVector, ::windows_core::IUnknown);
impl IClockVector {
    pub unsafe fn GetClockVectorElements(&self, riid: *const ::windows_core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVectorElements)(::windows_core::Interface::as_raw(self), riid, ppienumclockvector).ok()
    }
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVectorElementCount)(::windows_core::Interface::as_raw(self), pdwcount).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockVector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetClockVectorElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetClockVectorElementCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IClockVectorElement, IClockVectorElement_Vtbl, 0xe71c4250_adf8_4a07_8fae_5669596909c1);
::windows_core::imp::interface_hierarchy!(IClockVectorElement, ::windows_core::IUnknown);
impl IClockVectorElement {
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetReplicaKey)(::windows_core::Interface::as_raw(self), pdwreplicakey).ok()
    }
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTickCount)(::windows_core::Interface::as_raw(self), pulltickcount).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockVectorElement_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetReplicaKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT,
    pub GetTickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICombinedFilterInfo, ICombinedFilterInfo_Vtbl, 0x11f9de71_2818_4779_b2ac_42d450565f45);
::windows_core::imp::interface_hierarchy!(ICombinedFilterInfo, ::windows_core::IUnknown, ISyncFilterInfo);
impl ICombinedFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), pbbuffer, pcbbuffer).ok()
    }
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterCount)(::windows_core::Interface::as_raw(self), pdwfiltercount).ok()
    }
    pub unsafe fn GetFilterInfo(&self, dwfilterindex: u32) -> ::windows_core::Result<ISyncFilterInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterInfo)(::windows_core::Interface::as_raw(self), dwfilterindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFilterCombinationType(&self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterCombinationType)(::windows_core::Interface::as_raw(self), pfiltercombinationtype).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICombinedFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetFilterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows_core::HRESULT,
    pub GetFilterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilterCombinationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IConstraintConflict, IConstraintConflict_Vtbl, 0x00d2302e_1cf8_4835_b85f_b7ca4f799e0a);
::windows_core::imp::interface_hierarchy!(IConstraintConflict, ::windows_core::IUnknown);
impl IConstraintConflict {
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderConflictingChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceProviderConflictingChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDestinationProviderOriginalChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderOriginalChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderConflictingData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceProviderConflictingData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceProviderConflictingData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDestinationProviderOriginalData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderOriginalData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConstraintResolveActionForChange(&self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConstraintResolveActionForChange)(::windows_core::Interface::as_raw(self), pconstraintresolveaction).ok()
    }
    pub unsafe fn SetConstraintResolveActionForChange(&self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConstraintResolveActionForChange)(::windows_core::Interface::as_raw(self), constraintresolveaction).ok()
    }
    pub unsafe fn GetConstraintResolveActionForChangeUnit<P0>(&self, pchangeunit: P0, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChangeUnit>,
    {
        (::windows_core::Interface::vtable(self).GetConstraintResolveActionForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), pconstraintresolveaction).ok()
    }
    pub unsafe fn SetConstraintResolveActionForChangeUnit<P0>(&self, pchangeunit: P0, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChangeUnit>,
    {
        (::windows_core::Interface::vtable(self).SetConstraintResolveActionForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), constraintresolveaction).ok()
    }
    pub unsafe fn GetConstraintConflictReason(&self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConstraintConflictReason)(::windows_core::Interface::as_raw(self), pconstraintconflictreason).ok()
    }
    pub unsafe fn IsTemporary(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsTemporary)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstraintConflict_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDestinationProviderOriginalChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporiginalchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDestinationProviderOriginalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetConstraintResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub SetConstraintResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub GetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub SetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub GetConstraintConflictReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows_core::HRESULT,
    pub IsTemporary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IConstructReplicaKeyMap, IConstructReplicaKeyMap_Vtbl, 0xded10970_ec85_4115_b52c_4405845642a5);
::windows_core::imp::interface_hierarchy!(IConstructReplicaKeyMap, ::windows_core::IUnknown);
impl IConstructReplicaKeyMap {
    pub unsafe fn FindOrAddReplica(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindOrAddReplica)(::windows_core::Interface::as_raw(self), pbreplicaid, pdwreplicakey).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstructReplicaKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FindOrAddReplica: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICoreFragment, ICoreFragment_Vtbl, 0x613b2ab5_b304_47d9_9c31_ce6c54401a15);
::windows_core::imp::interface_hierarchy!(ICoreFragment, ::windows_core::IUnknown);
impl ICoreFragment {
    pub unsafe fn NextColumn(&self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextColumn)(::windows_core::Interface::as_raw(self), pchangeunitid, pchangeunitidsize).ok()
    }
    pub unsafe fn NextRange(&self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::core::option::Option<IClockVector>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextRange)(::windows_core::Interface::as_raw(self), pitemid, pitemidsize, ::core::mem::transmute(piclockvector)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetColumnCount(&self, pcolumncount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetColumnCount)(::windows_core::Interface::as_raw(self), pcolumncount).ok()
    }
    pub unsafe fn GetRangeCount(&self, prangecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRangeCount)(::windows_core::Interface::as_raw(self), prangecount).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFragment_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NextColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows_core::HRESULT,
    pub NextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows_core::HRESULT,
    pub GetRangeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICoreFragmentInspector, ICoreFragmentInspector_Vtbl, 0xf7fcc5fd_ae26_4679_ba16_96aac583c134);
::windows_core::imp::interface_hierarchy!(ICoreFragmentInspector, ::windows_core::IUnknown);
impl ICoreFragmentInspector {
    pub unsafe fn NextCoreFragments(&self, requestedcount: u32, ppicorefragments: *mut ::core::option::Option<ICoreFragment>, pfetchedcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextCoreFragments)(::windows_core::Interface::as_raw(self), requestedcount, ::core::mem::transmute(ppicorefragments), pfetchedcount).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFragmentInspector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NextCoreFragments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut *mut ::core::ffi::c_void, pfetchedcount: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICustomFilterInfo, ICustomFilterInfo_Vtbl, 0x1d335dff_6f88_4e4d_91a8_a3f351cfd473);
::windows_core::imp::interface_hierarchy!(ICustomFilterInfo, ::windows_core::IUnknown, ISyncFilterInfo);
impl ICustomFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), pbbuffer, pcbbuffer).ok()
    }
    pub unsafe fn GetSyncFilter(&self) -> ::windows_core::Result<ISyncFilter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncFilter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetSyncFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDataRetrieverCallback, IDataRetrieverCallback_Vtbl, 0x71b4863b_f969_4676_bbc3_3d9fdc3fb2c7);
::windows_core::imp::interface_hierarchy!(IDataRetrieverCallback, ::windows_core::IUnknown);
impl IDataRetrieverCallback {
    pub unsafe fn LoadChangeDataComplete<P0>(&self, punkdata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).LoadChangeDataComplete)(::windows_core::Interface::as_raw(self), punkdata.into_param().abi()).ok()
    }
    pub unsafe fn LoadChangeDataError(&self, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadChangeDataError)(::windows_core::Interface::as_raw(self), hrerror).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRetrieverCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub LoadChangeDataComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LoadChangeDataError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumChangeUnitExceptions, IEnumChangeUnitExceptions_Vtbl, 0x3074e802_9319_4420_be21_1022e2e21da8);
::windows_core::imp::interface_hierarchy!(IEnumChangeUnitExceptions, ::windows_core::IUnknown);
impl IEnumChangeUnitExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, ppchangeunitexception: *mut ::core::option::Option<IChangeUnitException>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cexceptions, ::core::mem::transmute(ppchangeunitexception), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumChangeUnitExceptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumChangeUnitExceptions_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumClockVector, IEnumClockVector_Vtbl, 0x525844db_2837_4799_9e80_81a66e02220c);
::windows_core::imp::interface_hierarchy!(IEnumClockVector, ::windows_core::IUnknown);
impl IEnumClockVector {
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IClockVectorElement>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cclockvectorelements, ::core::mem::transmute(ppiclockvectorelements), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, csyncversions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), csyncversions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumClockVector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumClockVector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumFeedClockVector, IEnumFeedClockVector_Vtbl, 0x550f763d_146a_48f6_abeb_6c88c7f70514);
::windows_core::imp::interface_hierarchy!(IEnumFeedClockVector, ::windows_core::IUnknown);
impl IEnumFeedClockVector {
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cclockvectorelements, ::core::mem::transmute(ppiclockvectorelements), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, csyncversions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), csyncversions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumFeedClockVector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFeedClockVector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumItemIds, IEnumItemIds_Vtbl, 0x43aa3f61_4b2e_4b60_83df_b110d3e148f1);
::windows_core::imp::interface_hierarchy!(IEnumItemIds, ::windows_core::IUnknown);
impl IEnumItemIds {
    pub unsafe fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), pbitemid, pcbitemidsize).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumItemIds_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumRangeExceptions, IEnumRangeExceptions_Vtbl, 0x0944439f_ddb1_4176_b703_046ff22a2386);
::windows_core::imp::interface_hierarchy!(IEnumRangeExceptions, ::windows_core::IUnknown);
impl IEnumRangeExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, pprangeexception: *mut ::core::option::Option<IRangeException>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cexceptions, ::core::mem::transmute(pprangeexception), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumRangeExceptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRangeExceptions_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumSingleItemExceptions, IEnumSingleItemExceptions_Vtbl, 0xe563381c_1b4d_4c66_9796_c86faccdcd40);
::windows_core::imp::interface_hierarchy!(IEnumSingleItemExceptions, ::windows_core::IUnknown);
impl IEnumSingleItemExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, ppsingleitemexception: *mut ::core::option::Option<ISingleItemException>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cexceptions, ::core::mem::transmute(ppsingleitemexception), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSingleItemExceptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSingleItemExceptions_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumSyncChangeUnits, IEnumSyncChangeUnits_Vtbl, 0x346b35f1_8703_4c6d_ab1a_4dbca2cff97f);
::windows_core::imp::interface_hierarchy!(IEnumSyncChangeUnits, ::windows_core::IUnknown);
impl IEnumSyncChangeUnits {
    pub unsafe fn Next(&self, cchanges: u32, ppchangeunit: *mut ::core::option::Option<ISyncChangeUnit>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cchanges, ::core::mem::transmute(ppchangeunit), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cchanges: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cchanges).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSyncChangeUnits> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncChangeUnits_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumSyncChanges, IEnumSyncChanges_Vtbl, 0x5f86be4a_5e78_4e32_ac1c_c24fd223ef85);
::windows_core::imp::interface_hierarchy!(IEnumSyncChanges, ::windows_core::IUnknown);
impl IEnumSyncChanges {
    pub unsafe fn Next(&self, cchanges: u32, ppchange: *mut ::core::option::Option<ISyncChange>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cchanges, ::core::mem::transmute(ppchange), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cchanges: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cchanges).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncChanges_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumSyncProviderConfigUIInfos, IEnumSyncProviderConfigUIInfos_Vtbl, 0xf6be2602_17c6_4658_a2d7_68ed3330f641);
::windows_core::imp::interface_hierarchy!(IEnumSyncProviderConfigUIInfos, ::windows_core::IUnknown);
impl IEnumSyncProviderConfigUIInfos {
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, ppsyncproviderconfiguiinfo: &mut [::core::option::Option<ISyncProviderConfigUIInfo>], pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppsyncproviderconfiguiinfo.len().try_into().unwrap(), ::core::mem::transmute(ppsyncproviderconfiguiinfo.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cfactories: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cfactories).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncProviderConfigUIInfos_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumSyncProviderInfos, IEnumSyncProviderInfos_Vtbl, 0xa04ba850_5eb1_460d_a973_393fcb608a11);
::windows_core::imp::interface_hierarchy!(IEnumSyncProviderInfos, ::windows_core::IUnknown);
impl IEnumSyncProviderInfos {
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, ppsyncproviderinfo: &mut [::core::option::Option<ISyncProviderInfo>], pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppsyncproviderinfo.len().try_into().unwrap(), ::core::mem::transmute(ppsyncproviderinfo.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cinstances: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cinstances).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSyncProviderInfos> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncProviderInfos_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFeedClockVector, IFeedClockVector_Vtbl, 0x8d1d98d1_9fb8_4ec9_a553_54dd924e0f67);
::windows_core::imp::interface_hierarchy!(IFeedClockVector, ::windows_core::IUnknown, IClockVector);
impl IFeedClockVector {
    pub unsafe fn GetClockVectorElements(&self, riid: *const ::windows_core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetClockVectorElements)(::windows_core::Interface::as_raw(self), riid, ppienumclockvector).ok()
    }
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetClockVectorElementCount)(::windows_core::Interface::as_raw(self), pdwcount).ok()
    }
    pub unsafe fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUpdateCount)(::windows_core::Interface::as_raw(self), pdwupdatecount).ok()
    }
    pub unsafe fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsNoConflictsSpecified)(::windows_core::Interface::as_raw(self), pfisnoconflictsspecified).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedClockVector_Vtbl {
    pub base__: IClockVector_Vtbl,
    pub GetUpdateCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows_core::HRESULT,
    pub IsNoConflictsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFeedClockVectorElement, IFeedClockVectorElement_Vtbl, 0xa40b46d2_e97b_4156_b6da_991f501b0f05);
::windows_core::imp::interface_hierarchy!(IFeedClockVectorElement, ::windows_core::IUnknown, IClockVectorElement);
impl IFeedClockVectorElement {
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetReplicaKey)(::windows_core::Interface::as_raw(self), pdwreplicakey).ok()
    }
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetTickCount)(::windows_core::Interface::as_raw(self), pulltickcount).ok()
    }
    pub unsafe fn GetSyncTime(&self, psynctime: *mut SYNC_TIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSyncTime)(::windows_core::Interface::as_raw(self), psynctime).ok()
    }
    pub unsafe fn GetFlags(&self, pbflags: *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), pbflags).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedClockVectorElement_Vtbl {
    pub base__: IClockVectorElement_Vtbl,
    pub GetSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFilterKeyMap, IFilterKeyMap_Vtbl, 0xca169652_07c6_4708_a3da_6e4eba8d2297);
::windows_core::imp::interface_hierarchy!(IFilterKeyMap, ::windows_core::IUnknown);
impl IFilterKeyMap {
    pub unsafe fn GetCount(&self, pdwcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), pdwcount).ok()
    }
    pub unsafe fn AddFilter<P0>(&self, pisyncfilter: P0, pdwfilterkey: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncFilter>,
    {
        (::windows_core::Interface::vtable(self).AddFilter)(::windows_core::Interface::as_raw(self), pisyncfilter.into_param().abi(), pdwfilterkey).ok()
    }
    pub unsafe fn GetFilter(&self, dwfilterkey: u32) -> ::windows_core::Result<ISyncFilter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilter)(::windows_core::Interface::as_raw(self), dwfilterkey, &mut result__).from_abi(result__)
    }
    pub unsafe fn Serialize(&self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), pbfilterkeymap, pcbfilterkeymap).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    pub AddFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::core::ffi::c_void, pdwfilterkey: *mut u32) -> ::windows_core::HRESULT,
    pub GetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFilterRequestCallback, IFilterRequestCallback_Vtbl, 0x82df8873_6360_463a_a8a1_ede5e1a1594d);
::windows_core::imp::interface_hierarchy!(IFilterRequestCallback, ::windows_core::IUnknown);
impl IFilterRequestCallback {
    pub unsafe fn RequestFilter<P0>(&self, pfilter: P0, filteringtype: FILTERING_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).RequestFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi(), filteringtype).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterRequestCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RequestFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFilterTrackingProvider, IFilterTrackingProvider_Vtbl, 0x743383c0_fc4e_45ba_ad81_d9d84c7a24f8);
::windows_core::imp::interface_hierarchy!(IFilterTrackingProvider, ::windows_core::IUnknown);
impl IFilterTrackingProvider {
    pub unsafe fn SpecifyTrackedFilters<P0>(&self, pcallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFilterTrackingRequestCallback>,
    {
        (::windows_core::Interface::vtable(self).SpecifyTrackedFilters)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn AddTrackedFilter<P0>(&self, pfilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncFilter>,
    {
        (::windows_core::Interface::vtable(self).AddTrackedFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SpecifyTrackedFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddTrackedFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFilterTrackingRequestCallback, IFilterTrackingRequestCallback_Vtbl, 0x713ca7bb_c858_4674_b4b6_1122436587a9);
::windows_core::imp::interface_hierarchy!(IFilterTrackingRequestCallback, ::windows_core::IUnknown);
impl IFilterTrackingRequestCallback {
    pub unsafe fn RequestTrackedFilter<P0>(&self, pfilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncFilter>,
    {
        (::windows_core::Interface::vtable(self).RequestTrackedFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingRequestCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RequestTrackedFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFilterTrackingSyncChangeBuilder, IFilterTrackingSyncChangeBuilder_Vtbl, 0x295024a0_70da_4c58_883c_ce2afb308d0b);
::windows_core::imp::interface_hierarchy!(IFilterTrackingSyncChangeBuilder, ::windows_core::IUnknown);
impl IFilterTrackingSyncChangeBuilder {
    pub unsafe fn AddFilterChange(&self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddFilterChange)(::windows_core::Interface::as_raw(self), dwfilterkey, pfilterchange).ok()
    }
    pub unsafe fn SetAllChangeUnitsPresentFlag(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllChangeUnitsPresentFlag)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingSyncChangeBuilder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddFilterChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows_core::HRESULT,
    pub SetAllChangeUnitsPresentFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IForgottenKnowledge, IForgottenKnowledge_Vtbl, 0x456e0f96_6036_452b_9f9d_bcc4b4a85db2);
::windows_core::imp::interface_hierarchy!(IForgottenKnowledge, ::windows_core::IUnknown, ISyncKnowledge);
impl IForgottenKnowledge {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetOwnerReplicaId)(::windows_core::Interface::as_raw(self), pbreplicaid, pcbidsize).ok()
    }
    pub unsafe fn Serialize<P0>(&self, fserializereplicakeymap: P0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), fserializereplicakeymap.into_param().abi(), pbknowledge, pcbknowledge).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalTickCount)(::windows_core::Interface::as_raw(self), ulltickcount).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsChange)(::windows_core::Interface::as_raw(self), pbversionownerreplicaid, pgiditemid, psyncversion).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsChangeUnit)(::windows_core::Interface::as_raw(self), pbversionownerreplicaid, pbitemid, pbchangeunitid, psyncversion).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetScopeVector)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows_core::Result<IReplicaKeyMap> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetReplicaKeyMap)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConvertVersion<P0>(&self, pknowledgein: P0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.ConvertVersion)(::windows_core::Interface::as_raw(self), pknowledgein.into_param().abi(), pbcurrentownerid, pversionin, pbnewownerid, pcbidsize, pversionout).ok()
    }
    pub unsafe fn MapRemoteToLocal<P0>(&self, premoteknowledge: P0) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MapRemoteToLocal)(::windows_core::Interface::as_raw(self), premoteknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Union<P0>(&self, pknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.Union)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoItem)(::windows_core::Interface::as_raw(self), pbitemid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoRange)(::windows_core::Interface::as_raw(self), psrngsyncrange, &mut result__).from_abi(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExcludeItem)(::windows_core::Interface::as_raw(self), pbitemid).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExcludeChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid).ok()
    }
    pub unsafe fn ContainsKnowledge<P0>(&self, pknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.ContainsKnowledge)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindMinTickCountForReplica)(::windows_core::Interface::as_raw(self), pbreplicaid, pullreplicatickcount).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRangeExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSingleItemExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetChangeUnitExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindClockVectorForItem)(::windows_core::Interface::as_raw(self), pbitemid, riid, ppunk).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindClockVectorForChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, riid, ppunk).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), pdwversion).ok()
    }
    pub unsafe fn ForgetToVersion<P0>(&self, pknowledge: P0, pversion: *const SYNC_VERSION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).ForgetToVersion)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi(), pversion).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForgottenKnowledge_Vtbl {
    pub base__: ISyncKnowledge_Vtbl,
    pub ForgetToVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pversion: *const SYNC_VERSION) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IKnowledgeSyncProvider, IKnowledgeSyncProvider_Vtbl, 0x43434a49_8da4_47f2_8172_ad7b8b024978);
::windows_core::imp::interface_hierarchy!(IKnowledgeSyncProvider, ::windows_core::IUnknown, ISyncProvider);
impl IKnowledgeSyncProvider {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIdParameters)(::windows_core::Interface::as_raw(self), pidparameters).ok()
    }
    pub unsafe fn BeginSession<P0>(&self, role: SYNC_PROVIDER_ROLE, psessionstate: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncSessionState>,
    {
        (::windows_core::Interface::vtable(self).BeginSession)(::windows_core::Interface::as_raw(self), role, psessionstate.into_param().abi()).ok()
    }
    pub unsafe fn GetSyncBatchParameters(&self, ppsyncknowledge: *mut ::core::option::Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSyncBatchParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppsyncknowledge), pdwrequestedbatchsize).ok()
    }
    pub unsafe fn GetChangeBatch<P0>(&self, dwbatchsize: u32, psyncknowledge: P0, ppsyncchangebatch: *mut ::core::option::Option<ISyncChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).GetChangeBatch)(::windows_core::Interface::as_raw(self), dwbatchsize, psyncknowledge.into_param().abi(), ::core::mem::transmute(ppsyncchangebatch), ::core::mem::transmute(ppunkdataretriever)).ok()
    }
    pub unsafe fn GetFullEnumerationChangeBatch<P0>(&self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: P0, ppsyncchangebatch: *mut ::core::option::Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).GetFullEnumerationChangeBatch)(::windows_core::Interface::as_raw(self), dwbatchsize, pblowerenumerationbound, psyncknowledge.into_param().abi(), ::core::mem::transmute(ppsyncchangebatch), ::core::mem::transmute(ppunkdataretriever)).ok()
    }
    pub unsafe fn ProcessChangeBatch<P0, P1, P2>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: P0, punkdataretriever: P1, pcallback: P2, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChangeBatch>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<ISyncCallback>,
    {
        (::windows_core::Interface::vtable(self).ProcessChangeBatch)(::windows_core::Interface::as_raw(self), resolutionpolicy, psourcechangebatch.into_param().abi(), punkdataretriever.into_param().abi(), pcallback.into_param().abi(), psyncsessionstatistics).ok()
    }
    pub unsafe fn ProcessFullEnumerationChangeBatch<P0, P1, P2>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: P0, punkdataretriever: P1, pcallback: P2, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncFullEnumerationChangeBatch>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<ISyncCallback>,
    {
        (::windows_core::Interface::vtable(self).ProcessFullEnumerationChangeBatch)(::windows_core::Interface::as_raw(self), resolutionpolicy, psourcechangebatch.into_param().abi(), punkdataretriever.into_param().abi(), pcallback.into_param().abi(), psyncsessionstatistics).ok()
    }
    pub unsafe fn EndSession<P0>(&self, psessionstate: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncSessionState>,
    {
        (::windows_core::Interface::vtable(self).EndSession)(::windows_core::Interface::as_raw(self), psessionstate.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnowledgeSyncProvider_Vtbl {
    pub base__: ISyncProvider_Vtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSyncBatchParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut *mut ::core::ffi::c_void, pdwrequestedbatchsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: *mut ::core::ffi::c_void, ppsyncchangebatch: *mut *mut ::core::ffi::c_void, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFullEnumerationChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: *mut ::core::ffi::c_void, ppsyncchangebatch: *mut *mut ::core::ffi::c_void, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProcessChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut ::core::ffi::c_void, punkdataretriever: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::HRESULT,
    pub ProcessFullEnumerationChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut ::core::ffi::c_void, punkdataretriever: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ILoadChangeContext, ILoadChangeContext_Vtbl, 0x44a4aaca_ec39_46d5_b5c9_d633c0ee67e2);
::windows_core::imp::interface_hierarchy!(ILoadChangeContext, ::windows_core::IUnknown);
impl ILoadChangeContext {
    pub unsafe fn GetSyncChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRecoverableErrorOnChange<P0>(&self, hrerror: ::windows_core::HRESULT, perrordata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IRecoverableErrorData>,
    {
        (::windows_core::Interface::vtable(self).SetRecoverableErrorOnChange)(::windows_core::Interface::as_raw(self), hrerror, perrordata.into_param().abi()).ok()
    }
    pub unsafe fn SetRecoverableErrorOnChangeUnit<P0, P1>(&self, hrerror: ::windows_core::HRESULT, pchangeunit: P0, perrordata: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChangeUnit>,
        P1: ::windows_core::IntoParam<IRecoverableErrorData>,
    {
        (::windows_core::Interface::vtable(self).SetRecoverableErrorOnChangeUnit)(::windows_core::Interface::as_raw(self), hrerror, pchangeunit.into_param().abi(), perrordata.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadChangeContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSyncChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRecoverableErrorOnChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT, perrordata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRecoverableErrorOnChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT, pchangeunit: *mut ::core::ffi::c_void, perrordata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IProviderConverter, IProviderConverter_Vtbl, 0x809b7276_98cf_4957_93a5_0ebdd3dddffd);
::windows_core::imp::interface_hierarchy!(IProviderConverter, ::windows_core::IUnknown);
impl IProviderConverter {
    pub unsafe fn Initialize<P0>(&self, pisyncprovider: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncProvider>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pisyncprovider.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderConverter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncprovider: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IRangeException, IRangeException_Vtbl, 0x75ae8777_6848_49f7_956c_a3a92f5096e8);
::windows_core::imp::interface_hierarchy!(IRangeException, ::windows_core::IUnknown);
impl IRangeException {
    pub unsafe fn GetClosedRangeStart(&self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClosedRangeStart)(::windows_core::Interface::as_raw(self), pbclosedrangestart, pcbidsize).ok()
    }
    pub unsafe fn GetClosedRangeEnd(&self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClosedRangeEnd)(::windows_core::Interface::as_raw(self), pbclosedrangeend, pcbidsize).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVector)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeException_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetClosedRangeStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClosedRangeEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IRecoverableError, IRecoverableError_Vtbl, 0x0f5625e8_0a7b_45ee_9637_1ce13645909e);
::windows_core::imp::interface_hierarchy!(IRecoverableError, ::windows_core::IUnknown);
impl IRecoverableError {
    pub unsafe fn GetStage(&self, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStage)(::windows_core::Interface::as_raw(self), pstage).ok()
    }
    pub unsafe fn GetProvider(&self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProvider)(::windows_core::Interface::as_raw(self), pproviderrole).ok()
    }
    pub unsafe fn GetChangeWithRecoverableError(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetChangeWithRecoverableError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRecoverableErrorDataForChange(&self, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRecoverableErrorDataForChange)(::windows_core::Interface::as_raw(self), phrerror, ::core::mem::transmute(pperrordata)).ok()
    }
    pub unsafe fn GetRecoverableErrorDataForChangeUnit<P0>(&self, pchangeunit: P0, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChangeUnit>,
    {
        (::windows_core::Interface::vtable(self).GetRecoverableErrorDataForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), phrerror, ::core::mem::transmute(pperrordata)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecoverableError_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetStage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows_core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows_core::HRESULT,
    pub GetChangeWithRecoverableError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRecoverableErrorDataForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRecoverableErrorDataForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IRecoverableErrorData, IRecoverableErrorData_Vtbl, 0xb37c4a0a_4b7d_4c2d_9711_3b00d119b1c8);
::windows_core::imp::interface_hierarchy!(IRecoverableErrorData, ::windows_core::IUnknown);
impl IRecoverableErrorData {
    pub unsafe fn Initialize<P0, P1>(&self, pcszitemdisplayname: P0, pcszerrordescription: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pcszitemdisplayname.into_param().abi(), pcszerrordescription.into_param().abi()).ok()
    }
    pub unsafe fn GetItemDisplayName<P0>(&self, pszitemdisplayname: P0, pcchitemdisplayname: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetItemDisplayName)(::windows_core::Interface::as_raw(self), pszitemdisplayname.into_param().abi(), pcchitemdisplayname).ok()
    }
    pub unsafe fn GetErrorDescription<P0>(&self, pszerrordescription: P0, pccherrordescription: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetErrorDescription)(::windows_core::Interface::as_raw(self), pszerrordescription.into_param().abi(), pccherrordescription).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecoverableErrorData_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcszitemdisplayname: ::windows_core::PCWSTR, pcszerrordescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetItemDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszitemdisplayname: ::windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> ::windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszerrordescription: ::windows_core::PCWSTR, pccherrordescription: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IRegisteredSyncProvider, IRegisteredSyncProvider_Vtbl, 0x913bcf76_47c1_40b5_a896_5e8a9c414c14);
::windows_core::imp::interface_hierarchy!(IRegisteredSyncProvider, ::windows_core::IUnknown);
impl IRegisteredSyncProvider {
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Init<P0>(&self, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pcontextpropertystore: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), pguidinstanceid, pguidcontenttype, pcontextpropertystore.into_param().abi()).ok()
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredSyncProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pcontextpropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IReplicaKeyMap, IReplicaKeyMap_Vtbl, 0x2209f4fc_fd10_4ff0_84a8_f0a1982e440e);
::windows_core::imp::interface_hierarchy!(IReplicaKeyMap, ::windows_core::IUnknown);
impl IReplicaKeyMap {
    pub unsafe fn LookupReplicaKey(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LookupReplicaKey)(::windows_core::Interface::as_raw(self), pbreplicaid, pdwreplicakey).ok()
    }
    pub unsafe fn LookupReplicaId(&self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LookupReplicaId)(::windows_core::Interface::as_raw(self), dwreplicakey, pbreplicaid, pcbidsize).ok()
    }
    pub unsafe fn Serialize(&self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), pbreplicakeymap, pcbreplicakeymap).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReplicaKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub LookupReplicaKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT,
    pub LookupReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IRequestFilteredSync, IRequestFilteredSync_Vtbl, 0x2e020184_6d18_46a7_a32a_da4aeb06696c);
::windows_core::imp::interface_hierarchy!(IRequestFilteredSync, ::windows_core::IUnknown);
impl IRequestFilteredSync {
    pub unsafe fn SpecifyFilter<P0>(&self, pcallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFilterRequestCallback>,
    {
        (::windows_core::Interface::vtable(self).SpecifyFilter)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRequestFilteredSync_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SpecifyFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISingleItemException, ISingleItemException_Vtbl, 0x892fb9b0_7c55_4a18_9316_fdf449569b64);
::windows_core::imp::interface_hierarchy!(ISingleItemException, ::windows_core::IUnknown);
impl ISingleItemException {
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetItemId)(::windows_core::Interface::as_raw(self), pbitemid, pcbidsize).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVector)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISingleItemException_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISupportFilteredSync, ISupportFilteredSync_Vtbl, 0x3d128ded_d555_4e0d_bf4b_fb213a8a9302);
::windows_core::imp::interface_hierarchy!(ISupportFilteredSync, ::windows_core::IUnknown);
impl ISupportFilteredSync {
    pub unsafe fn AddFilter<P0>(&self, pfilter: P0, filteringtype: FILTERING_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).AddFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi(), filteringtype).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportFilteredSync_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISupportLastWriteTime, ISupportLastWriteTime_Vtbl, 0xeadf816f_d0bd_43ca_8f40_5acdc6c06f7a);
::windows_core::imp::interface_hierarchy!(ISupportLastWriteTime, ::windows_core::IUnknown);
impl ISupportLastWriteTime {
    pub unsafe fn GetItemChangeTime(&self, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetItemChangeTime)(::windows_core::Interface::as_raw(self), pbitemid, pulltimestamp).ok()
    }
    pub unsafe fn GetChangeUnitChangeTime(&self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitChangeTime)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, pulltimestamp).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportLastWriteTime_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetItemChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::HRESULT,
    pub GetChangeUnitChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncCallback, ISyncCallback_Vtbl, 0x0599797f_5ed9_485c_ae36_0c5d1bf2e7a5);
::windows_core::imp::interface_hierarchy!(ISyncCallback, ::windows_core::IUnknown);
impl ISyncCallback {
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnProgress)(::windows_core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
    pub unsafe fn OnChange<P0>(&self, psyncchange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChange>,
    {
        (::windows_core::Interface::vtable(self).OnChange)(::windows_core::Interface::as_raw(self), psyncchange.into_param().abi()).ok()
    }
    pub unsafe fn OnConflict<P0>(&self, pconflict: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IChangeConflict>,
    {
        (::windows_core::Interface::vtable(self).OnConflict)(::windows_core::Interface::as_raw(self), pconflict.into_param().abi()).ok()
    }
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnFullEnumerationNeeded)(::windows_core::Interface::as_raw(self), pfullenumerationaction).ok()
    }
    pub unsafe fn OnRecoverableError<P0>(&self, precoverableerror: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IRecoverableError>,
    {
        (::windows_core::Interface::vtable(self).OnRecoverableError)(::windows_core::Interface::as_raw(self), precoverableerror.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::HRESULT,
    pub OnChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconflict: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnFullEnumerationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_core::HRESULT,
    pub OnRecoverableError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precoverableerror: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncCallback2, ISyncCallback2_Vtbl, 0x47ce84af_7442_4ead_8630_12015e030ad7);
::windows_core::imp::interface_hierarchy!(ISyncCallback2, ::windows_core::IUnknown, ISyncCallback);
impl ISyncCallback2 {
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnProgress)(::windows_core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
    pub unsafe fn OnChange<P0>(&self, psyncchange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncChange>,
    {
        (::windows_core::Interface::vtable(self).base__.OnChange)(::windows_core::Interface::as_raw(self), psyncchange.into_param().abi()).ok()
    }
    pub unsafe fn OnConflict<P0>(&self, pconflict: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IChangeConflict>,
    {
        (::windows_core::Interface::vtable(self).base__.OnConflict)(::windows_core::Interface::as_raw(self), pconflict.into_param().abi()).ok()
    }
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnFullEnumerationNeeded)(::windows_core::Interface::as_raw(self), pfullenumerationaction).ok()
    }
    pub unsafe fn OnRecoverableError<P0>(&self, precoverableerror: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IRecoverableError>,
    {
        (::windows_core::Interface::vtable(self).base__.OnRecoverableError)(::windows_core::Interface::as_raw(self), precoverableerror.into_param().abi()).ok()
    }
    pub unsafe fn OnChangeApplied(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnChangeApplied)(::windows_core::Interface::as_raw(self), dwchangesapplied, dwchangesfailed).ok()
    }
    pub unsafe fn OnChangeFailed(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnChangeFailed)(::windows_core::Interface::as_raw(self), dwchangesapplied, dwchangesfailed).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncCallback2_Vtbl {
    pub base__: ISyncCallback_Vtbl,
    pub OnChangeApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::HRESULT,
    pub OnChangeFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChange, ISyncChange_Vtbl, 0xa1952beb_0f6b_4711_b136_01da85b968a6);
::windows_core::imp::interface_hierarchy!(ISyncChange, ::windows_core::IUnknown);
impl ISyncChange {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOwnerReplicaId)(::windows_core::Interface::as_raw(self), pbreplicaid, pcbidsize).ok()
    }
    pub unsafe fn GetRootItemId(&self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRootItemId)(::windows_core::Interface::as_raw(self), pbrootitemid, pcbidsize).ok()
    }
    pub unsafe fn GetChangeVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeVersion)(::windows_core::Interface::as_raw(self), pbcurrentreplicaid, pversion).ok()
    }
    pub unsafe fn GetCreationVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCreationVersion)(::windows_core::Interface::as_raw(self), pbcurrentreplicaid, pversion).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn GetWorkEstimate(&self, pdwwork: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWorkEstimate)(::windows_core::Interface::as_raw(self), pdwwork).ok()
    }
    pub unsafe fn GetChangeUnits(&self) -> ::windows_core::Result<IEnumSyncChangeUnits> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetChangeUnits)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMadeWithKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMadeWithKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWorkEstimate(&self, dwwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWorkEstimate)(::windows_core::Interface::as_raw(self), dwwork).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChange_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOwnerReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetRootItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT,
    pub GetCreationVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetWorkEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeUnits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMadeWithKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetWorkEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeBatch, ISyncChangeBatch_Vtbl, 0x70c64dee_380f_4c2e_8f70_31c55bd5f9b3);
::windows_core::imp::interface_hierarchy!(ISyncChangeBatch, ::windows_core::IUnknown, ISyncChangeBatchBase);
impl ISyncChangeBatch {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), pflastbatch).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), pdwworkforbatch).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), pdwremainingworkforsession).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), pblowerbound).ok()
    }
    pub unsafe fn EndOrderedGroup<P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), pbupperbound, pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), pbchangebatch, pcbchangebatch).ok()
    }
    pub unsafe fn BeginUnorderedGroup(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginUnorderedGroup)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndUnorderedGroup<P0, P1>(&self, pmadewithknowledge: P0, fallchangesforknowledge: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).EndUnorderedGroup)(::windows_core::Interface::as_raw(self), pmadewithknowledge.into_param().abi(), fallchangesforknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddLoggedConflict<P0>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: P0) -> ::windows_core::Result<ISyncChangeBuilder>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddLoggedConflict)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, pconflictknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatch_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub BeginUnorderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndUnorderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmadewithknowledge: *mut ::core::ffi::c_void, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub AddLoggedConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: *mut ::core::ffi::c_void, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeBatch2, ISyncChangeBatch2_Vtbl, 0x225f4a33_f5ee_4cc7_b039_67a262b4b2ac);
::windows_core::imp::interface_hierarchy!(ISyncChangeBatch2, ::windows_core::IUnknown, ISyncChangeBatchBase, ISyncChangeBatch);
impl ISyncChangeBatch2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), pflastbatch).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), pdwworkforbatch).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), pdwremainingworkforsession).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), pblowerbound).ok()
    }
    pub unsafe fn EndOrderedGroup<P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), pbupperbound, pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Serialize)(::windows_core::Interface::as_raw(self), pbchangebatch, pcbchangebatch).ok()
    }
    pub unsafe fn BeginUnorderedGroup(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginUnorderedGroup)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndUnorderedGroup<P0, P1>(&self, pmadewithknowledge: P0, fallchangesforknowledge: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.EndUnorderedGroup)(::windows_core::Interface::as_raw(self), pmadewithknowledge.into_param().abi(), fallchangesforknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddLoggedConflict<P0>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: P0) -> ::windows_core::Result<ISyncChangeBuilder>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddLoggedConflict)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, pconflictknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddMergeTombstoneMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbwinneritemid, pbitemid, pchangeversion, pcreationversion, dwworkforchange, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddMergeTombstoneLoggedConflict<P0>(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: P0) -> ::windows_core::Result<ISyncChangeBuilder>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddMergeTombstoneLoggedConflict)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbwinneritemid, pbitemid, pchangeversion, pcreationversion, dwworkforchange, pconflictknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatch2_Vtbl {
    pub base__: ISyncChangeBatch_Vtbl,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddMergeTombstoneLoggedConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: *mut ::core::ffi::c_void, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeBatchAdvanced, ISyncChangeBatchAdvanced_Vtbl, 0x0f1a4995_cbc8_421d_b550_5d0bebf3e9a5);
::windows_core::imp::interface_hierarchy!(ISyncChangeBatchAdvanced, ::windows_core::IUnknown);
impl ISyncChangeBatchAdvanced {
    pub unsafe fn GetFilterInfo(&self) -> ::windows_core::Result<ISyncFilterInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&self) -> ::windows_core::Result<ISyncChangeBatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConvertFullEnumerationChangeBatchToRegularChangeBatch)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUpperBoundItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUpperBoundItemId)(::windows_core::Interface::as_raw(self), pbitemid, pcbidsize).ok()
    }
    pub unsafe fn GetBatchLevelKnowledgeShouldBeApplied(&self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBatchLevelKnowledgeShouldBeApplied)(::windows_core::Interface::as_raw(self), pfbatchknowledgeshouldbeapplied).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchAdvanced_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFilterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertFullEnumerationChangeBatchToRegularChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppchangebatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetUpperBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetBatchLevelKnowledgeShouldBeApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeBatchBase, ISyncChangeBatchBase_Vtbl, 0x52f6e694_6a71_4494_a184_a8311bf5d227);
::windows_core::imp::interface_hierarchy!(ISyncChangeBatchBase, ::windows_core::IUnknown);
impl ISyncChangeBatchBase {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetChangeEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIsLastBatch)(::windows_core::Interface::as_raw(self), pflastbatch).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), pdwworkforbatch).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), pdwremainingworkforsession).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginOrderedGroup)(::windows_core::Interface::as_raw(self), pblowerbound).ok()
    }
    pub unsafe fn EndOrderedGroup<P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).EndOrderedGroup)(::windows_core::Interface::as_raw(self), pbupperbound, pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), pbchangebatch, pcbchangebatch).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchBase_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetChangeEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetIsLastBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetWorkEstimateForBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows_core::HRESULT,
    pub GetRemainingWorkEstimateForSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows_core::HRESULT,
    pub BeginOrderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows_core::HRESULT,
    pub EndOrderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddItemMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLastBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetWorkEstimateForBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows_core::HRESULT,
    pub SetRemainingWorkEstimateForSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeBatchBase2, ISyncChangeBatchBase2_Vtbl, 0x6fdb596a_d755_4584_bd0c_c0c23a548fbf);
::windows_core::imp::interface_hierarchy!(ISyncChangeBatchBase2, ::windows_core::IUnknown, ISyncChangeBatchBase);
impl ISyncChangeBatchBase2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), pflastbatch).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), pdwworkforbatch).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), pdwremainingworkforsession).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), pblowerbound).ok()
    }
    pub unsafe fn EndOrderedGroup<P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), pbupperbound, pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), pbchangebatch, pcbchangebatch).ok()
    }
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SerializeWithOptions)(::windows_core::Interface::as_raw(self), targetformatversion, dwflags, pbbuffer, pdwserializedsize).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchBase2_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub SerializeWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeBatchWithFilterKeyMap, ISyncChangeBatchWithFilterKeyMap_Vtbl, 0xde247002_566d_459a_a6ed_a5aab3459fb7);
::windows_core::imp::interface_hierarchy!(ISyncChangeBatchWithFilterKeyMap, ::windows_core::IUnknown);
impl ISyncChangeBatchWithFilterKeyMap {
    pub unsafe fn GetFilterKeyMap(&self) -> ::windows_core::Result<IFilterKeyMap> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterKeyMap)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFilterKeyMap<P0>(&self, pifilterkeymap: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFilterKeyMap>,
    {
        (::windows_core::Interface::vtable(self).SetFilterKeyMap)(::windows_core::Interface::as_raw(self), pifilterkeymap.into_param().abi()).ok()
    }
    pub unsafe fn SetFilterForgottenKnowledge<P0>(&self, dwfilterkey: u32, pfilterforgottenknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).SetFilterForgottenKnowledge)(::windows_core::Interface::as_raw(self), dwfilterkey, pfilterforgottenknowledge.into_param().abi()).ok()
    }
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), dwfilterkey, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), dwfilterkey, &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchWithFilterKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFilterKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetFilterKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifilterkeymap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeBatchWithPrerequisite, ISyncChangeBatchWithPrerequisite_Vtbl, 0x097f13be_5b92_4048_b3f2_7b42a2515e07);
::windows_core::imp::interface_hierarchy!(ISyncChangeBatchWithPrerequisite, ::windows_core::IUnknown, ISyncChangeBatchBase);
impl ISyncChangeBatchWithPrerequisite {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), pflastbatch).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), pdwworkforbatch).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), pdwremainingworkforsession).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), pblowerbound).ok()
    }
    pub unsafe fn EndOrderedGroup<P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), pbupperbound, pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), pbchangebatch, pcbchangebatch).ok()
    }
    pub unsafe fn SetPrerequisiteKnowledge<P0>(&self, pprerequisiteknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).SetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), pprerequisiteknowledge.into_param().abi()).ok()
    }
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<P0>(&self, pdestinationknowledge: P0) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledgeWithPrerequisite)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchWithPrerequisite_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub SetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pplearnedwithprerequisiteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeBuilder, ISyncChangeBuilder_Vtbl, 0x56f14771_8677_484f_a170_e386e418a676);
::windows_core::imp::interface_hierarchy!(ISyncChangeBuilder, ::windows_core::IUnknown);
impl ISyncChangeBuilder {
    pub unsafe fn AddChangeUnitMetadata(&self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddChangeUnitMetadata)(::windows_core::Interface::as_raw(self), pbchangeunitid, pchangeunitversion).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBuilder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddChangeUnitMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeUnit, ISyncChangeUnit_Vtbl, 0x60edd8ca_7341_4bb7_95ce_fab6394b51cb);
::windows_core::imp::interface_hierarchy!(ISyncChangeUnit, ::windows_core::IUnknown);
impl ISyncChangeUnit {
    pub unsafe fn GetItemChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetItemChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitId)(::windows_core::Interface::as_raw(self), pbchangeunitid, pcbidsize).ok()
    }
    pub unsafe fn GetChangeUnitVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitVersion)(::windows_core::Interface::as_raw(self), pbcurrentreplicaid, pversion).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeUnit_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetItemChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeUnitVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeWithFilterKeyMap, ISyncChangeWithFilterKeyMap_Vtbl, 0xbfe1ef00_e87d_42fd_a4e9_242d70414aef);
::windows_core::imp::interface_hierarchy!(ISyncChangeWithFilterKeyMap, ::windows_core::IUnknown);
impl ISyncChangeWithFilterKeyMap {
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterCount)(::windows_core::Interface::as_raw(self), pdwfiltercount).ok()
    }
    pub unsafe fn GetFilterChange(&self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterChange)(::windows_core::Interface::as_raw(self), dwfilterkey, pfilterchange).ok()
    }
    pub unsafe fn GetAllChangeUnitsPresentFlag(&self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAllChangeUnitsPresentFlag)(::windows_core::Interface::as_raw(self), pfallchangeunitspresent).ok()
    }
    pub unsafe fn GetFilterForgottenKnowledge(&self, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterForgottenKnowledge)(::windows_core::Interface::as_raw(self), dwfilterkey, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), dwfilterkey, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<IEnumItemIds>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), dwfilterkey, &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeWithFilterKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFilterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows_core::HRESULT,
    pub GetFilterChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows_core::HRESULT,
    pub GetAllChangeUnitsPresentFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncChangeWithPrerequisite, ISyncChangeWithPrerequisite_Vtbl, 0x9e38382f_1589_48c3_92e4_05ecdcb4f3f7);
::windows_core::imp::interface_hierarchy!(ISyncChangeWithPrerequisite, ::windows_core::IUnknown);
impl ISyncChangeWithPrerequisite {
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<P0>(&self, pdestinationknowledge: P0) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledgeWithPrerequisite)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeWithPrerequisite_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pplearnedknowledgewithprerequisite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncConstraintCallback, ISyncConstraintCallback_Vtbl, 0x8af3843e_75b3_438c_bb51_6f020d70d3cb);
::windows_core::imp::interface_hierarchy!(ISyncConstraintCallback, ::windows_core::IUnknown);
impl ISyncConstraintCallback {
    pub unsafe fn OnConstraintConflict<P0>(&self, pconflict: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IConstraintConflict>,
    {
        (::windows_core::Interface::vtable(self).OnConstraintConflict)(::windows_core::Interface::as_raw(self), pconflict.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncConstraintCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnConstraintConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconflict: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncDataConverter, ISyncDataConverter_Vtbl, 0x435d4861_68d5_44aa_a0f9_72a0b00ef9cf);
::windows_core::imp::interface_hierarchy!(ISyncDataConverter, ::windows_core::IUnknown);
impl ISyncDataConverter {
    pub unsafe fn ConvertDataRetrieverFromProviderFormat<P0, P1>(&self, punkdataretrieverin: P0, penumsyncchanges: P1) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IEnumSyncChanges>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConvertDataRetrieverFromProviderFormat)(::windows_core::Interface::as_raw(self), punkdataretrieverin.into_param().abi(), penumsyncchanges.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConvertDataRetrieverToProviderFormat<P0, P1>(&self, punkdataretrieverin: P0, penumsyncchanges: P1) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<IEnumSyncChanges>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConvertDataRetrieverToProviderFormat)(::windows_core::Interface::as_raw(self), punkdataretrieverin.into_param().abi(), penumsyncchanges.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConvertDataFromProviderFormat<P0, P1>(&self, pdatacontext: P0, punkdatain: P1) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<ILoadChangeContext>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConvertDataFromProviderFormat)(::windows_core::Interface::as_raw(self), pdatacontext.into_param().abi(), punkdatain.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConvertDataToProviderFormat<P0, P1>(&self, pdatacontext: P0, punkdataout: P1) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<ILoadChangeContext>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConvertDataToProviderFormat)(::windows_core::Interface::as_raw(self), pdatacontext.into_param().abi(), punkdataout.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncDataConverter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ConvertDataRetrieverFromProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertDataRetrieverToProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertDataFromProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatacontext: *mut ::core::ffi::c_void, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertDataToProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatacontext: *mut ::core::ffi::c_void, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncFilter, ISyncFilter_Vtbl, 0x087a3f15_0fcb_44c1_9639_53c14e2b5506);
::windows_core::imp::interface_hierarchy!(ISyncFilter, ::windows_core::IUnknown);
impl ISyncFilter {
    pub unsafe fn IsIdentical<P0>(&self, psyncfilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncFilter>,
    {
        (::windows_core::Interface::vtable(self).IsIdentical)(::windows_core::Interface::as_raw(self), psyncfilter.into_param().abi()).ok()
    }
    pub unsafe fn Serialize(&self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), pbsyncfilter, pcbsyncfilter).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncFilterDeserializer, ISyncFilterDeserializer_Vtbl, 0xb45b7a72_e5c7_46be_9c82_77b8b15dab8a);
::windows_core::imp::interface_hierarchy!(ISyncFilterDeserializer, ::windows_core::IUnknown);
impl ISyncFilterDeserializer {
    pub unsafe fn DeserializeSyncFilter(&self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> ::windows_core::Result<ISyncFilter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeserializeSyncFilter)(::windows_core::Interface::as_raw(self), pbsyncfilter, dwcbsyncfilter, &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterDeserializer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub DeserializeSyncFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncFilterInfo, ISyncFilterInfo_Vtbl, 0x794eaaf8_3f2e_47e6_9728_17e6fcf94cb7);
::windows_core::imp::interface_hierarchy!(ISyncFilterInfo, ::windows_core::IUnknown);
impl ISyncFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), pbbuffer, pcbbuffer).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncFilterInfo2, ISyncFilterInfo2_Vtbl, 0x19b394ba_e3d0_468c_934d_321968b2ab34);
::windows_core::imp::interface_hierarchy!(ISyncFilterInfo2, ::windows_core::IUnknown, ISyncFilterInfo);
impl ISyncFilterInfo2 {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), pbbuffer, pcbbuffer).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), pdwflags).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterInfo2_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncFullEnumerationChange, ISyncFullEnumerationChange_Vtbl, 0x9785e0bd_bdff_40c4_98c5_b34b2f1991b3);
::windows_core::imp::interface_hierarchy!(ISyncFullEnumerationChange, ::windows_core::IUnknown);
impl ISyncFullEnumerationChange {
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChange_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncFullEnumerationChangeBatch, ISyncFullEnumerationChangeBatch_Vtbl, 0xef64197d_4f44_4ea2_b355_4524713e3bed);
::windows_core::imp::interface_hierarchy!(ISyncFullEnumerationChangeBatch, ::windows_core::IUnknown, ISyncChangeBatchBase);
impl ISyncFullEnumerationChangeBatch {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), pflastbatch).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), pdwworkforbatch).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), pdwremainingworkforsession).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), pblowerbound).ok()
    }
    pub unsafe fn EndOrderedGroup<P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), pbupperbound, pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), pbchangebatch, pcbchangebatch).ok()
    }
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClosedLowerBoundItemId)(::windows_core::Interface::as_raw(self), pbclosedlowerbounditemid, pcbidsize).ok()
    }
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClosedUpperBoundItemId)(::windows_core::Interface::as_raw(self), pbclosedupperbounditemid, pcbidsize).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChangeBatch_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetClosedLowerBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClosedUpperBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncFullEnumerationChangeBatch2, ISyncFullEnumerationChangeBatch2_Vtbl, 0xe06449f4_a205_4b65_9724_01b22101eec1);
::windows_core::imp::interface_hierarchy!(ISyncFullEnumerationChangeBatch2, ::windows_core::IUnknown, ISyncChangeBatchBase, ISyncFullEnumerationChangeBatch);
impl ISyncFullEnumerationChangeBatch2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), pflastbatch).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), pdwworkforbatch).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), pdwremainingworkforsession).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), pblowerbound).ok()
    }
    pub unsafe fn EndOrderedGroup<P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), pbupperbound, pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Serialize)(::windows_core::Interface::as_raw(self), pbchangebatch, pcbchangebatch).ok()
    }
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetClosedLowerBoundItemId)(::windows_core::Interface::as_raw(self), pbclosedlowerbounditemid, pcbidsize).ok()
    }
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetClosedUpperBoundItemId)(::windows_core::Interface::as_raw(self), pbclosedupperbounditemid, pcbidsize).ok()
    }
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddMergeTombstoneMetadataToGroup)(::windows_core::Interface::as_raw(self), pbownerreplicaid, pbwinneritemid, pbitemid, pchangeversion, pcreationversion, dwworkforchange, &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChangeBatch2_Vtbl {
    pub base__: ISyncFullEnumerationChangeBatch_Vtbl,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncKnowledge, ISyncKnowledge_Vtbl, 0x615bbb53_c945_4203_bf4b_2cb65919a0aa);
::windows_core::imp::interface_hierarchy!(ISyncKnowledge, ::windows_core::IUnknown);
impl ISyncKnowledge {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOwnerReplicaId)(::windows_core::Interface::as_raw(self), pbreplicaid, pcbidsize).ok()
    }
    pub unsafe fn Serialize<P0>(&self, fserializereplicakeymap: P0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), fserializereplicakeymap.into_param().abi(), pbknowledge, pcbknowledge).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalTickCount)(::windows_core::Interface::as_raw(self), ulltickcount).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContainsChange)(::windows_core::Interface::as_raw(self), pbversionownerreplicaid, pgiditemid, psyncversion).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContainsChangeUnit)(::windows_core::Interface::as_raw(self), pbversionownerreplicaid, pbitemid, pbchangeunitid, psyncversion).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetScopeVector)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows_core::Result<IReplicaKeyMap> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReplicaKeyMap)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConvertVersion<P0>(&self, pknowledgein: P0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).ConvertVersion)(::windows_core::Interface::as_raw(self), pknowledgein.into_param().abi(), pbcurrentownerid, pversionin, pbnewownerid, pcbidsize, pversionout).ok()
    }
    pub unsafe fn MapRemoteToLocal<P0>(&self, premoteknowledge: P0) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MapRemoteToLocal)(::windows_core::Interface::as_raw(self), premoteknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Union<P0>(&self, pknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).Union)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoItem)(::windows_core::Interface::as_raw(self), pbitemid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoRange)(::windows_core::Interface::as_raw(self), psrngsyncrange, &mut result__).from_abi(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExcludeItem)(::windows_core::Interface::as_raw(self), pbitemid).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExcludeChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid).ok()
    }
    pub unsafe fn ContainsKnowledge<P0>(&self, pknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).ContainsKnowledge)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindMinTickCountForReplica)(::windows_core::Interface::as_raw(self), pbreplicaid, pullreplicatickcount).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRangeExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSingleItemExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindClockVectorForItem)(::windows_core::Interface::as_raw(self), pbitemid, riid, ppunk).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindClockVectorForChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, riid, ppunk).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), pdwversion).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncKnowledge_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOwnerReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::HRESULT,
    pub SetLocalTickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows_core::HRESULT,
    pub ContainsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::HRESULT,
    pub ContainsChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::HRESULT,
    pub GetScopeVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetReplicaKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledgein: *mut ::core::ffi::c_void, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::HRESULT,
    pub MapRemoteToLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premoteknowledge: *mut ::core::ffi::c_void, ppmappedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Union: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProjectOntoItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProjectOntoChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProjectOntoRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ExcludeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows_core::HRESULT,
    pub ExcludeChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::HRESULT,
    pub ContainsKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindMinTickCountForReplica: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::HRESULT,
    pub GetRangeExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSingleItemExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetChangeUnitExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindClockVectorForItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindClockVectorForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncKnowledge2, ISyncKnowledge2_Vtbl, 0xed0addc0_3b4b_46a1_9a45_45661d2114c8);
::windows_core::imp::interface_hierarchy!(ISyncKnowledge2, ::windows_core::IUnknown, ISyncKnowledge);
impl ISyncKnowledge2 {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetOwnerReplicaId)(::windows_core::Interface::as_raw(self), pbreplicaid, pcbidsize).ok()
    }
    pub unsafe fn Serialize<P0>(&self, fserializereplicakeymap: P0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), fserializereplicakeymap.into_param().abi(), pbknowledge, pcbknowledge).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalTickCount)(::windows_core::Interface::as_raw(self), ulltickcount).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsChange)(::windows_core::Interface::as_raw(self), pbversionownerreplicaid, pgiditemid, psyncversion).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsChangeUnit)(::windows_core::Interface::as_raw(self), pbversionownerreplicaid, pbitemid, pbchangeunitid, psyncversion).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetScopeVector)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows_core::Result<IReplicaKeyMap> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetReplicaKeyMap)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConvertVersion<P0>(&self, pknowledgein: P0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.ConvertVersion)(::windows_core::Interface::as_raw(self), pknowledgein.into_param().abi(), pbcurrentownerid, pversionin, pbnewownerid, pcbidsize, pversionout).ok()
    }
    pub unsafe fn MapRemoteToLocal<P0>(&self, premoteknowledge: P0) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MapRemoteToLocal)(::windows_core::Interface::as_raw(self), premoteknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Union<P0>(&self, pknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.Union)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoItem)(::windows_core::Interface::as_raw(self), pbitemid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoRange)(::windows_core::Interface::as_raw(self), psrngsyncrange, &mut result__).from_abi(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExcludeItem)(::windows_core::Interface::as_raw(self), pbitemid).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExcludeChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid).ok()
    }
    pub unsafe fn ContainsKnowledge<P0>(&self, pknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).base__.ContainsKnowledge)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindMinTickCountForReplica)(::windows_core::Interface::as_raw(self), pbreplicaid, pullreplicatickcount).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRangeExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSingleItemExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetChangeUnitExceptions)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindClockVectorForItem)(::windows_core::Interface::as_raw(self), pbitemid, riid, ppunk).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindClockVectorForChangeUnit)(::windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, riid, ppunk).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), pdwversion).ok()
    }
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdParameters)(::windows_core::Interface::as_raw(self), pidparameters).ok()
    }
    pub unsafe fn ProjectOntoColumnSet(&self, ppcolumns: *const *const u8, count: u32) -> ::windows_core::Result<ISyncKnowledge2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoColumnSet)(::windows_core::Interface::as_raw(self), ppcolumns, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SerializeWithOptions)(::windows_core::Interface::as_raw(self), targetformatversion, dwflags, pbbuffer, pdwserializedsize).ok()
    }
    pub unsafe fn GetLowestUncontainedId<P0>(&self, pisyncknowledge: P0, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge2>,
    {
        (::windows_core::Interface::vtable(self).GetLowestUncontainedId)(::windows_core::Interface::as_raw(self), pisyncknowledge.into_param().abi(), pbitemid, pcbitemidsize).ok()
    }
    pub unsafe fn GetInspector(&self, riid: *const ::windows_core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInspector)(::windows_core::Interface::as_raw(self), riid, ppiinspector).ok()
    }
    pub unsafe fn GetMinimumSupportedVersion(&self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMinimumSupportedVersion)(::windows_core::Interface::as_raw(self), pversion).ok()
    }
    pub unsafe fn GetStatistics(&self, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatistics)(::windows_core::Interface::as_raw(self), which, pvalue).ok()
    }
    pub unsafe fn ContainsKnowledgeForItem<P0>(&self, pknowledge: P0, pbitemid: *const u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).ContainsKnowledgeForItem)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi(), pbitemid).ok()
    }
    pub unsafe fn ContainsKnowledgeForChangeUnit<P0>(&self, pknowledge: P0, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).ContainsKnowledgeForChangeUnit)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi(), pbitemid, pbchangeunitid).ok()
    }
    pub unsafe fn ProjectOntoKnowledgeWithPrerequisite<P0, P1>(&self, pprerequisiteknowledge: P0, ptemplateknowledge: P1) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
        P1: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoKnowledgeWithPrerequisite)(::windows_core::Interface::as_raw(self), pprerequisiteknowledge.into_param().abi(), ptemplateknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Complement<P0>(&self, psyncknowledge: P0) -> ::windows_core::Result<ISyncKnowledge>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Complement)(::windows_core::Interface::as_raw(self), psyncknowledge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn IntersectsWithKnowledge<P0>(&self, psyncknowledge: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISyncKnowledge>,
    {
        (::windows_core::Interface::vtable(self).IntersectsWithKnowledge)(::windows_core::Interface::as_raw(self), psyncknowledge.into_param().abi()).ok()
    }
    pub unsafe fn GetKnowledgeCookie(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetKnowledgeCookie)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CompareToKnowledgeCookie<P0>(&self, pknowledgecookie: P0, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CompareToKnowledgeCookie)(::windows_core::Interface::as_raw(self), pknowledgecookie.into_param().abi(), presult).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncKnowledge2_Vtbl {
    pub base__: ISyncKnowledge_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT,
    pub ProjectOntoColumnSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SerializeWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetLowestUncontainedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncknowledge: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetInspector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMinimumSupportedVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows_core::HRESULT,
    pub ContainsKnowledgeForItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows_core::HRESULT,
    pub ContainsKnowledgeForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::HRESULT,
    pub ProjectOntoKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: *mut ::core::ffi::c_void, ptemplateknowledge: *mut ::core::ffi::c_void, ppprojectedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Complement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncknowledge: *mut ::core::ffi::c_void, ppcomplementedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IntersectsWithKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetKnowledgeCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CompareToKnowledgeCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncMergeTombstoneChange, ISyncMergeTombstoneChange_Vtbl, 0x6ec62597_0903_484c_ad61_36d6e938f47b);
::windows_core::imp::interface_hierarchy!(ISyncMergeTombstoneChange, ::windows_core::IUnknown);
impl ISyncMergeTombstoneChange {
    pub unsafe fn GetWinnerItemId(&self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWinnerItemId)(::windows_core::Interface::as_raw(self), pbwinneritemid, pcbidsize).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncMergeTombstoneChange_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetWinnerItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncProvider, ISyncProvider_Vtbl, 0x8f657056_2bce_4a17_8c68_c7bb7898b56f);
::windows_core::imp::interface_hierarchy!(ISyncProvider, ::windows_core::IUnknown);
impl ISyncProvider {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdParameters)(::windows_core::Interface::as_raw(self), pidparameters).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncProviderConfigUI, ISyncProviderConfigUI_Vtbl, 0x7b0705f6_cbcd_4071_ab05_3bdc364d4a0c);
::windows_core::imp::interface_hierarchy!(ISyncProviderConfigUI, ::windows_core::IUnknown);
impl ISyncProviderConfigUI {
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Init<P0>(&self, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pconfigurationproperties: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), pguidinstanceid, pguidcontenttype, pconfigurationproperties.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetRegisteredProperties(&self) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRegisteredProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateAndRegisterNewSyncProvider<P0, P1>(&self, hwndparent: P0, punkcontext: P1) -> ::windows_core::Result<ISyncProviderInfo>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateAndRegisterNewSyncProvider)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), punkcontext.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn ModifySyncProvider<P0, P1, P2>(&self, hwndparent: P0, punkcontext: P1, pproviderinfo: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<ISyncProviderInfo>,
    {
        (::windows_core::Interface::vtable(self).ModifySyncProvider)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), punkcontext.into_param().abi(), pproviderinfo.into_param().abi()).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderConfigUI_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pconfigurationproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetRegisteredProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetRegisteredProperties: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateAndRegisterNewSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateAndRegisterNewSyncProvider: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ModifySyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ModifySyncProvider: usize,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    ISyncProviderConfigUIInfo,
    ISyncProviderConfigUIInfo_Vtbl,
    0x214141ae_33d7_4d8d_8e37_f227e880ce50
);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
::windows_core::imp::interface_hierarchy!(ISyncProviderConfigUIInfo, ::windows_core::IUnknown, super::super::UI::Shell::PropertiesSystem::IPropertyStore);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderConfigUIInfo {
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, iprop: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAt)(::windows_core::Interface::as_raw(self), iprop, pkey).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetValue)(::windows_core::Interface::as_raw(self), key, propvar).ok()
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSyncProviderConfigUI(&self, dwclscontext: u32) -> ::windows_core::Result<ISyncProviderConfigUI> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderConfigUI)(::windows_core::Interface::as_raw(self), dwclscontext, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderConfigUIInfo_Vtbl {
    pub base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl,
    pub GetSyncProviderConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    ISyncProviderInfo,
    ISyncProviderInfo_Vtbl,
    0x1ee135de_88a4_4504_b0d0_f7920d7e5ba6
);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
::windows_core::imp::interface_hierarchy!(ISyncProviderInfo, ::windows_core::IUnknown, super::super::UI::Shell::PropertiesSystem::IPropertyStore);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderInfo {
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, iprop: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAt)(::windows_core::Interface::as_raw(self), iprop, pkey).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetValue)(::windows_core::Interface::as_raw(self), key, propvar).ok()
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSyncProvider(&self, dwclscontext: u32) -> ::windows_core::Result<IRegisteredSyncProvider> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProvider)(::windows_core::Interface::as_raw(self), dwclscontext, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderInfo_Vtbl {
    pub base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl,
    pub GetSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncProviderRegistration, ISyncProviderRegistration_Vtbl, 0xcb45953b_7624_47bc_a472_eb8cac6b222e);
::windows_core::imp::interface_hierarchy!(ISyncProviderRegistration, ::windows_core::IUnknown);
impl ISyncProviderRegistration {
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateSyncProviderConfigUIRegistrationInstance(&self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> ::windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSyncProviderConfigUIRegistrationInstance)(::windows_core::Interface::as_raw(self), pconfiguiconfig, &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterSyncProviderConfigUI(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterSyncProviderConfigUI)(::windows_core::Interface::as_raw(self), pguidinstanceid).ok()
    }
    pub unsafe fn EnumerateSyncProviderConfigUIs(&self, pguidcontenttype: ::core::option::Option<*const ::windows_core::GUID>, dwsupportedarchitecture: u32) -> ::windows_core::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateSyncProviderConfigUIs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidcontenttype.unwrap_or(::std::ptr::null())), dwsupportedarchitecture, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateSyncProviderRegistrationInstance(&self, pproviderconfiguration: *const SyncProviderConfiguration) -> ::windows_core::Result<ISyncProviderInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSyncProviderRegistrationInstance)(::windows_core::Interface::as_raw(self), pproviderconfiguration, &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterSyncProvider(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterSyncProvider)(::windows_core::Interface::as_raw(self), pguidinstanceid).ok()
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderConfigUIInfoforProvider(&self, pguidproviderinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderConfigUIInfoforProvider)(::windows_core::Interface::as_raw(self), pguidproviderinstanceid, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerateSyncProviders(&self, pguidcontenttype: ::core::option::Option<*const ::windows_core::GUID>, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows_core::GUID, dwsupportedarchitecture: u32) -> ::windows_core::Result<IEnumSyncProviderInfos> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateSyncProviders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidcontenttype.unwrap_or(::std::ptr::null())), dwstateflagstofiltermask, dwstateflagstofilter, refproviderclsid, dwsupportedarchitecture, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderInfo(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderInfo)(::windows_core::Interface::as_raw(self), pguidinstanceid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSyncProviderFromInstanceId(&self, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32) -> ::windows_core::Result<IRegisteredSyncProvider> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderFromInstanceId)(::windows_core::Interface::as_raw(self), pguidinstanceid, dwclscontext, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderConfigUIInfo(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderConfigUIInfo)(::windows_core::Interface::as_raw(self), pguidinstanceid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSyncProviderConfigUIFromInstanceId(&self, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32) -> ::windows_core::Result<ISyncProviderConfigUI> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderConfigUIFromInstanceId)(::windows_core::Interface::as_raw(self), pguidinstanceid, dwclscontext, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSyncProviderState(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderState)(::windows_core::Interface::as_raw(self), pguidinstanceid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSyncProviderState(&self, pguidinstanceid: *const ::windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSyncProviderState)(::windows_core::Interface::as_raw(self), pguidinstanceid, dwstateflagsmask, dwstateflags).ok()
    }
    pub unsafe fn RegisterForEvent(&self, phevent: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterForEvent)(::windows_core::Interface::as_raw(self), phevent).ok()
    }
    pub unsafe fn RevokeEvent<P0>(&self, hevent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).RevokeEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn GetChange<P0>(&self, hevent: P0) -> ::windows_core::Result<ISyncRegistrationChange>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetChange)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderRegistration_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateSyncProviderConfigUIRegistrationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateSyncProviderConfigUIRegistrationInstance: usize,
    pub UnregisterSyncProviderConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub EnumerateSyncProviderConfigUIs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateSyncProviderRegistrationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateSyncProviderRegistrationInstance: usize,
    pub UnregisterSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfoforProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows_core::GUID, ppproviderconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfoforProvider: usize,
    pub EnumerateSyncProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderInfo: usize,
    pub GetSyncProviderFromInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32, ppsyncprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, ppconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfo: usize,
    pub GetSyncProviderConfigUIFromInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32, ppconfigui: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSyncProviderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pdwstateflags: *mut u32) -> ::windows_core::HRESULT,
    pub SetSyncProviderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows_core::HRESULT,
    pub RegisterForEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    pub RevokeEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    pub GetChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncRegistrationChange, ISyncRegistrationChange_Vtbl, 0xeea0d9ae_6b29_43b4_9e70_e3ae33bb2c3b);
::windows_core::imp::interface_hierarchy!(ISyncRegistrationChange, ::windows_core::IUnknown);
impl ISyncRegistrationChange {
    pub unsafe fn GetEvent(&self) -> ::windows_core::Result<SYNC_REGISTRATION_EVENT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEvent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncRegistrationChange_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows_core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncSessionExtendedErrorInfo, ISyncSessionExtendedErrorInfo_Vtbl, 0x326c6810_790a_409b_b741_6999388761eb);
::windows_core::imp::interface_hierarchy!(ISyncSessionExtendedErrorInfo, ::windows_core::IUnknown);
impl ISyncSessionExtendedErrorInfo {
    pub unsafe fn GetSyncProviderWithError(&self) -> ::windows_core::Result<ISyncProvider> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderWithError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionExtendedErrorInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSyncProviderWithError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncSessionState, ISyncSessionState_Vtbl, 0xb8a940fe_9f01_483b_9434_c37d361225d9);
::windows_core::imp::interface_hierarchy!(ISyncSessionState, ::windows_core::IUnknown);
impl ISyncSessionState {
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsCanceled)(::windows_core::Interface::as_raw(self), pfiscanceled).ok()
    }
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInfoForChangeApplication)(::windows_core::Interface::as_raw(self), pbchangeapplierinfo, pcbchangeapplierinfo).ok()
    }
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadInfoFromChangeApplication)(::windows_core::Interface::as_raw(self), pbchangeapplierinfo, cbchangeapplierinfo).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForgottenKnowledgeRecoveryRangeStart)(::windows_core::Interface::as_raw(self), pbrangestart, pcbrangestart).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForgottenKnowledgeRecoveryRangeEnd)(::windows_core::Interface::as_raw(self), pbrangeend, pcbrangeend).ok()
    }
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetForgottenKnowledgeRecoveryRange)(::windows_core::Interface::as_raw(self), prange).ok()
    }
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnProgress)(::windows_core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionState_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetInfoForChangeApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_core::HRESULT,
    pub LoadInfoFromChangeApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_core::HRESULT,
    pub SetForgottenKnowledgeRecoveryRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISyncSessionState2, ISyncSessionState2_Vtbl, 0x9e37cfa3_9e38_4c61_9ca3_ffe810b45ca2);
::windows_core::imp::interface_hierarchy!(ISyncSessionState2, ::windows_core::IUnknown, ISyncSessionState);
impl ISyncSessionState2 {
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsCanceled)(::windows_core::Interface::as_raw(self), pfiscanceled).ok()
    }
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoForChangeApplication)(::windows_core::Interface::as_raw(self), pbchangeapplierinfo, pcbchangeapplierinfo).ok()
    }
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LoadInfoFromChangeApplication)(::windows_core::Interface::as_raw(self), pbchangeapplierinfo, cbchangeapplierinfo).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetForgottenKnowledgeRecoveryRangeStart)(::windows_core::Interface::as_raw(self), pbrangestart, pcbrangestart).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetForgottenKnowledgeRecoveryRangeEnd)(::windows_core::Interface::as_raw(self), pbrangeend, pcbrangeend).ok()
    }
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetForgottenKnowledgeRecoveryRange)(::windows_core::Interface::as_raw(self), prange).ok()
    }
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnProgress)(::windows_core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
    pub unsafe fn SetProviderWithError<P0>(&self, fself: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetProviderWithError)(::windows_core::Interface::as_raw(self), fself.into_param().abi()).ok()
    }
    pub unsafe fn GetSessionErrorStatus(&self, phrsessionerror: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSessionErrorStatus)(::windows_core::Interface::as_raw(self), phrsessionerror).ok()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionState2_Vtbl {
    pub base__: ISyncSessionState_Vtbl,
    pub SetProviderWithError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fself: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetSessionErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISynchronousDataRetriever, ISynchronousDataRetriever_Vtbl, 0x9b22f2a9_a4cd_4648_9d8e_3a510d4da04b);
::windows_core::imp::interface_hierarchy!(ISynchronousDataRetriever, ::windows_core::IUnknown);
impl ISynchronousDataRetriever {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdParameters)(::windows_core::Interface::as_raw(self), pidparameters).ok()
    }
    pub unsafe fn LoadChangeData<P0>(&self, ploadchangecontext: P0) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<ILoadChangeContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoadChangeData)(::windows_core::Interface::as_raw(self), ploadchangecontext.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronousDataRetriever_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT,
    pub LoadChangeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploadchangecontext: *mut ::core::ffi::c_void, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub const CCR_COLLISION: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(1i32);
pub const CCR_IDENTITY: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(3i32);
pub const CCR_NOPARENT: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(2i32);
pub const CCR_OTHER: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(0i32);
pub const CRP_DESTINATION_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(1i32);
pub const CRP_LAST: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(3i32);
pub const CRP_NONE: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(0i32);
pub const CRP_SOURCE_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(2i32);
pub const FCT_INTERSECTION: FILTER_COMBINATION_TYPE = FILTER_COMBINATION_TYPE(0i32);
pub const FT_CURRENT_ITEMS_AND_VERSIONS_FOR_MOVED_OUT_ITEMS: FILTERING_TYPE = FILTERING_TYPE(1i32);
pub const FT_CURRENT_ITEMS_ONLY: FILTERING_TYPE = FILTERING_TYPE(0i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINED: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(1i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINS: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(2i32);
pub const KCCR_COOKIE_KNOWLEDGE_EQUAL: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(0i32);
pub const KCCR_COOKIE_KNOWLEDGE_NOT_COMPARABLE: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(3i32);
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 5 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 3 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 4 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 9 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 11 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 2 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_IS_GLOBAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 7 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 13 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM_NOUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 12 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 8 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 6 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 10 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 6 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 3 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONFIGUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 4 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 5 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 9 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 11 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 2 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 8 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 7 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 10 };
pub const SCC_CAN_CREATE_WITHOUT_UI: u32 = 1u32;
pub const SCC_CAN_MODIFY_WITHOUT_UI: u32 = 2u32;
pub const SCC_CREATE_NOT_SUPPORTED: u32 = 4u32;
pub const SCC_DEFAULT: u32 = 0u32;
pub const SCC_MODIFY_NOT_SUPPORTED: u32 = 8u32;
pub const SCRA_ACCEPT_DESTINATION_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(1i32);
pub const SCRA_ACCEPT_SOURCE_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(2i32);
pub const SCRA_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(0i32);
pub const SCRA_MERGE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(4i32);
pub const SCRA_RENAME_DESTINATION: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(6i32);
pub const SCRA_RENAME_SOURCE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(5i32);
pub const SCRA_TRANSFER_AND_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(3i32);
pub const SFEA_ABORT: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(2i32);
pub const SFEA_FULL_ENUMERATION: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(0i32);
pub const SFEA_PARTIAL_SYNC: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(1i32);
pub const SPC_DEFAULT: u32 = 0u32;
pub const SPR_DESTINATION: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(1i32);
pub const SPR_SOURCE: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(0i32);
pub const SPS_CHANGE_APPLICATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(2i32);
pub const SPS_CHANGE_DETECTION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(0i32);
pub const SPS_CHANGE_ENUMERATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(1i32);
pub const SRA_ACCEPT_DESTINATION_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(1i32);
pub const SRA_ACCEPT_SOURCE_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(2i32);
pub const SRA_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(0i32);
pub const SRA_LAST: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(5i32);
pub const SRA_MERGE: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(3i32);
pub const SRA_TRANSFER_AND_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(4i32);
pub const SRE_CONFIGUI_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(4i32);
pub const SRE_CONFIGUI_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(5i32);
pub const SRE_CONFIGUI_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(6i32);
pub const SRE_PROVIDER_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(0i32);
pub const SRE_PROVIDER_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(1i32);
pub const SRE_PROVIDER_STATE_CHANGED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(3i32);
pub const SRE_PROVIDER_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(2i32);
pub const SYNC_32_BIT_SUPPORTED: u32 = 1u32;
pub const SYNC_64_BIT_SUPPORTED: u32 = 2u32;
pub const SYNC_CHANGE_FLAG_DELETED: u32 = 1u32;
pub const SYNC_CHANGE_FLAG_DOES_NOT_EXIST: u32 = 2u32;
pub const SYNC_CHANGE_FLAG_GHOST: u32 = 4u32;
pub const SYNC_FILTER_INFO_COMBINED: u32 = 8u32;
pub const SYNC_FILTER_INFO_FLAG_CHANGE_UNIT_LIST: u32 = 2u32;
pub const SYNC_FILTER_INFO_FLAG_CUSTOM: u32 = 4u32;
pub const SYNC_FILTER_INFO_FLAG_ITEM_LIST: u32 = 1u32;
pub const SYNC_PROVIDER_CONFIGUI_CONFIGURATION_VERSION: u32 = 1u32;
pub const SYNC_PROVIDER_CONFIGURATION_VERSION: u32 = 1u32;
pub const SYNC_PROVIDER_STATE_DIRTY: u32 = 2u32;
pub const SYNC_PROVIDER_STATE_ENABLED: u32 = 1u32;
pub const SYNC_SERIALIZATION_VERSION_V1: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(1i32);
pub const SYNC_SERIALIZATION_VERSION_V2: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(4i32);
pub const SYNC_SERIALIZATION_VERSION_V3: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(5i32);
pub const SYNC_SERIALIZE_REPLICA_KEY_MAP: u32 = 1u32;
pub const SYNC_STATISTICS_RANGE_COUNT: SYNC_STATISTICS = SYNC_STATISTICS(0i32);
pub const SYNC_VERSION_FLAG_FROM_FEED: u32 = 1u32;
pub const SYNC_VERSION_FLAG_HAS_BY: u32 = 2u32;
pub const SyncProviderRegistration: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf82b4ef1_93a9_4dde_8015_f7950a1a6e31);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct CONFLICT_RESOLUTION_POLICY(pub i32);
impl ::windows_core::TypeKind for CONFLICT_RESOLUTION_POLICY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CONFLICT_RESOLUTION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONFLICT_RESOLUTION_POLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct CONSTRAINT_CONFLICT_REASON(pub i32);
impl ::windows_core::TypeKind for CONSTRAINT_CONFLICT_REASON {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CONSTRAINT_CONFLICT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSTRAINT_CONFLICT_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct FILTERING_TYPE(pub i32);
impl ::windows_core::TypeKind for FILTERING_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FILTERING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTERING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct FILTER_COMBINATION_TYPE(pub i32);
impl ::windows_core::TypeKind for FILTER_COMBINATION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FILTER_COMBINATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTER_COMBINATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct KNOWLEDGE_COOKIE_COMPARISON_RESULT(pub i32);
impl ::windows_core::TypeKind for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KNOWLEDGE_COOKIE_COMPARISON_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SYNC_CONSTRAINT_RESOLVE_ACTION(pub i32);
impl ::windows_core::TypeKind for SYNC_CONSTRAINT_RESOLVE_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_CONSTRAINT_RESOLVE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SYNC_FULL_ENUMERATION_ACTION(pub i32);
impl ::windows_core::TypeKind for SYNC_FULL_ENUMERATION_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_FULL_ENUMERATION_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_FULL_ENUMERATION_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SYNC_PROGRESS_STAGE(pub i32);
impl ::windows_core::TypeKind for SYNC_PROGRESS_STAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_PROGRESS_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_PROGRESS_STAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SYNC_PROVIDER_ROLE(pub i32);
impl ::windows_core::TypeKind for SYNC_PROVIDER_ROLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_PROVIDER_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_PROVIDER_ROLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SYNC_REGISTRATION_EVENT(pub i32);
impl ::windows_core::TypeKind for SYNC_REGISTRATION_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_REGISTRATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_REGISTRATION_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SYNC_RESOLVE_ACTION(pub i32);
impl ::windows_core::TypeKind for SYNC_RESOLVE_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_RESOLVE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_RESOLVE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SYNC_SERIALIZATION_VERSION(pub i32);
impl ::windows_core::TypeKind for SYNC_SERIALIZATION_VERSION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_SERIALIZATION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_SERIALIZATION_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SYNC_STATISTICS(pub i32);
impl ::windows_core::TypeKind for SYNC_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_STATISTICS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct ID_PARAMETERS {
    pub dwSize: u32,
    pub replicaId: ID_PARAMETER_PAIR,
    pub itemId: ID_PARAMETER_PAIR,
    pub changeUnitId: ID_PARAMETER_PAIR,
}
impl ::core::marker::Copy for ID_PARAMETERS {}
impl ::core::clone::Clone for ID_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ID_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ID_PARAMETERS").field("dwSize", &self.dwSize).field("replicaId", &self.replicaId).field("itemId", &self.itemId).field("changeUnitId", &self.changeUnitId).finish()
    }
}
impl ::windows_core::TypeKind for ID_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ID_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.replicaId == other.replicaId && self.itemId == other.itemId && self.changeUnitId == other.changeUnitId
    }
}
impl ::core::cmp::Eq for ID_PARAMETERS {}
impl ::core::default::Default for ID_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ID_PARAMETER_PAIR {
    pub fIsVariable: super::super::Foundation::BOOL,
    pub cbIdSize: u16,
}
impl ::core::marker::Copy for ID_PARAMETER_PAIR {}
impl ::core::clone::Clone for ID_PARAMETER_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ID_PARAMETER_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ID_PARAMETER_PAIR").field("fIsVariable", &self.fIsVariable).field("cbIdSize", &self.cbIdSize).finish()
    }
}
impl ::windows_core::TypeKind for ID_PARAMETER_PAIR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ID_PARAMETER_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.fIsVariable == other.fIsVariable && self.cbIdSize == other.cbIdSize
    }
}
impl ::core::cmp::Eq for ID_PARAMETER_PAIR {}
impl ::core::default::Default for ID_PARAMETER_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SYNC_FILTER_CHANGE {
    pub fMoveIn: super::super::Foundation::BOOL,
    pub moveVersion: SYNC_VERSION,
}
impl ::core::marker::Copy for SYNC_FILTER_CHANGE {}
impl ::core::clone::Clone for SYNC_FILTER_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_FILTER_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_FILTER_CHANGE").field("fMoveIn", &self.fMoveIn).field("moveVersion", &self.moveVersion).finish()
    }
}
impl ::windows_core::TypeKind for SYNC_FILTER_CHANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SYNC_FILTER_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.fMoveIn == other.fMoveIn && self.moveVersion == other.moveVersion
    }
}
impl ::core::cmp::Eq for SYNC_FILTER_CHANGE {}
impl ::core::default::Default for SYNC_FILTER_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SYNC_RANGE {
    pub pbClosedLowerBound: *mut u8,
    pub pbClosedUpperBound: *mut u8,
}
impl ::core::marker::Copy for SYNC_RANGE {}
impl ::core::clone::Clone for SYNC_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_RANGE").field("pbClosedLowerBound", &self.pbClosedLowerBound).field("pbClosedUpperBound", &self.pbClosedUpperBound).finish()
    }
}
impl ::windows_core::TypeKind for SYNC_RANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SYNC_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.pbClosedLowerBound == other.pbClosedLowerBound && self.pbClosedUpperBound == other.pbClosedUpperBound
    }
}
impl ::core::cmp::Eq for SYNC_RANGE {}
impl ::core::default::Default for SYNC_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SYNC_SESSION_STATISTICS {
    pub dwChangesApplied: u32,
    pub dwChangesFailed: u32,
}
impl ::core::marker::Copy for SYNC_SESSION_STATISTICS {}
impl ::core::clone::Clone for SYNC_SESSION_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_SESSION_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_SESSION_STATISTICS").field("dwChangesApplied", &self.dwChangesApplied).field("dwChangesFailed", &self.dwChangesFailed).finish()
    }
}
impl ::windows_core::TypeKind for SYNC_SESSION_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SYNC_SESSION_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.dwChangesApplied == other.dwChangesApplied && self.dwChangesFailed == other.dwChangesFailed
    }
}
impl ::core::cmp::Eq for SYNC_SESSION_STATISTICS {}
impl ::core::default::Default for SYNC_SESSION_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SYNC_TIME {
    pub dwDate: u32,
    pub dwTime: u32,
}
impl ::core::marker::Copy for SYNC_TIME {}
impl ::core::clone::Clone for SYNC_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_TIME").field("dwDate", &self.dwDate).field("dwTime", &self.dwTime).finish()
    }
}
impl ::windows_core::TypeKind for SYNC_TIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SYNC_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.dwDate == other.dwDate && self.dwTime == other.dwTime
    }
}
impl ::core::cmp::Eq for SYNC_TIME {}
impl ::core::default::Default for SYNC_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SYNC_VERSION {
    pub dwLastUpdatingReplicaKey: u32,
    pub ullTickCount: u64,
}
impl ::core::marker::Copy for SYNC_VERSION {}
impl ::core::clone::Clone for SYNC_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_VERSION").field("dwLastUpdatingReplicaKey", &self.dwLastUpdatingReplicaKey).field("ullTickCount", &self.ullTickCount).finish()
    }
}
impl ::windows_core::TypeKind for SYNC_VERSION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SYNC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwLastUpdatingReplicaKey == other.dwLastUpdatingReplicaKey && self.ullTickCount == other.ullTickCount
    }
}
impl ::core::cmp::Eq for SYNC_VERSION {}
impl ::core::default::Default for SYNC_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SyncProviderConfigUIConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows_core::GUID,
    pub clsidConfigUI: ::windows_core::GUID,
    pub guidContentType: ::windows_core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
    pub fIsGlobal: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for SyncProviderConfigUIConfiguration {}
impl ::core::clone::Clone for SyncProviderConfigUIConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SyncProviderConfigUIConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SyncProviderConfigUIConfiguration").field("dwVersion", &self.dwVersion).field("guidInstanceId", &self.guidInstanceId).field("clsidConfigUI", &self.clsidConfigUI).field("guidContentType", &self.guidContentType).field("dwCapabilities", &self.dwCapabilities).field("dwSupportedArchitecture", &self.dwSupportedArchitecture).field("fIsGlobal", &self.fIsGlobal).finish()
    }
}
impl ::windows_core::TypeKind for SyncProviderConfigUIConfiguration {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SyncProviderConfigUIConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.guidInstanceId == other.guidInstanceId && self.clsidConfigUI == other.clsidConfigUI && self.guidContentType == other.guidContentType && self.dwCapabilities == other.dwCapabilities && self.dwSupportedArchitecture == other.dwSupportedArchitecture && self.fIsGlobal == other.fIsGlobal
    }
}
impl ::core::cmp::Eq for SyncProviderConfigUIConfiguration {}
impl ::core::default::Default for SyncProviderConfigUIConfiguration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SyncProviderConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows_core::GUID,
    pub clsidProvider: ::windows_core::GUID,
    pub guidConfigUIInstanceId: ::windows_core::GUID,
    pub guidContentType: ::windows_core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
}
impl ::core::marker::Copy for SyncProviderConfiguration {}
impl ::core::clone::Clone for SyncProviderConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SyncProviderConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SyncProviderConfiguration").field("dwVersion", &self.dwVersion).field("guidInstanceId", &self.guidInstanceId).field("clsidProvider", &self.clsidProvider).field("guidConfigUIInstanceId", &self.guidConfigUIInstanceId).field("guidContentType", &self.guidContentType).field("dwCapabilities", &self.dwCapabilities).field("dwSupportedArchitecture", &self.dwSupportedArchitecture).finish()
    }
}
impl ::windows_core::TypeKind for SyncProviderConfiguration {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SyncProviderConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.guidInstanceId == other.guidInstanceId && self.clsidProvider == other.clsidProvider && self.guidConfigUIInstanceId == other.guidConfigUIInstanceId && self.guidContentType == other.guidContentType && self.dwCapabilities == other.dwCapabilities && self.dwSupportedArchitecture == other.dwSupportedArchitecture
    }
}
impl ::core::cmp::Eq for SyncProviderConfiguration {}
impl ::core::default::Default for SyncProviderConfiguration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
