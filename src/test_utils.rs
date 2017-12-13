use Device;
use NVML;
#[cfg(not(feature = "test-local"))]
use NvLink;
use Unit;
use bitmasks::device::*;
use bitmasks::event::*;
use enum_wrappers::device::*;
use enums::unit::*;
use error::*;
use event::EventSet;
use std::fmt::Debug;
use struct_wrappers::device::*;
use struct_wrappers::event::*;
#[cfg(not(feature = "test-local"))]
use struct_wrappers::nv_link::*;
use struct_wrappers::unit::*;
use structs::device::*;
#[cfg(not(feature = "test-local"))]
use structs::nv_link::*;

#[cfg(target_os = "windows")]
use structs::device::DriverModelState;

pub trait ShouldPrint: Debug {
    fn should_print(&self) -> bool {
        true
    }
}

impl ShouldPrint for () {
    fn should_print(&self) -> bool {
        false
    }
}

impl<'nvml> ShouldPrint for Device<'nvml> {
    fn should_print(&self) -> bool {
        false
    }
}

impl<'nvml> ShouldPrint for Unit<'nvml> {
    fn should_print(&self) -> bool {
        false
    }
}

impl<'nvml> ShouldPrint for EventSet<'nvml> {
    fn should_print(&self) -> bool {
        false
    }
}

impl ShouldPrint for bool {}
impl ShouldPrint for u32 {}
impl ShouldPrint for i32 {}
impl ShouldPrint for (u32, u32) {}
impl ShouldPrint for u64 {}
impl ShouldPrint for String {}
impl ShouldPrint for Brand {}
impl ShouldPrint for [i8; 16] {}
impl ShouldPrint for Vec<ProcessInfo> {}
impl ShouldPrint for Vec<ProcessUtilizationSample> {}
impl<'nvml> ShouldPrint for Vec<Device<'nvml>> {}
impl ShouldPrint for Vec<u32> {}
impl ShouldPrint for Vec<u64> {}
impl ShouldPrint for Vec<Sample> {}
impl ShouldPrint for Vec<HwbcEntry> {}
impl ShouldPrint for Utilization {}
impl ShouldPrint for EncoderStats {}
impl ShouldPrint for Vec<EncoderSessionInfo> {}
impl ShouldPrint for AutoBoostClocksEnabledInfo {}
impl ShouldPrint for BAR1MemoryInfo {}
impl ShouldPrint for BridgeChipHierarchy {}
impl ShouldPrint for ComputeMode {}
impl ShouldPrint for UtilizationInfo {}
impl ShouldPrint for EccModeState {}
impl ShouldPrint for OperationModeState {}
impl ShouldPrint for InfoRom {}
impl ShouldPrint for MemoryInfo {}
impl ShouldPrint for PciInfo {}
impl ShouldPrint for PerformanceState {}
impl ShouldPrint for PowerManagementConstraints {}
impl ShouldPrint for ThrottleReasons {}
impl ShouldPrint for ViolationTime {}
impl ShouldPrint for AccountingStats {}
impl ShouldPrint for EventTypes {}
impl<'nvml> ShouldPrint for EventData<'nvml> {}
impl ShouldPrint for FansInfo {}
impl ShouldPrint for LedState {}
impl ShouldPrint for PsuInfo {}
impl ShouldPrint for UnitInfo {}
#[cfg(not(feature = "test-local"))]
impl ShouldPrint for UtilizationControl {}
#[cfg(not(feature = "test-local"))]
impl ShouldPrint for UtilizationCounter {}

#[cfg(target_os = "windows")]
impl ShouldPrint for DriverModelState {}

pub fn nvml() -> NVML {
    NVML::init().expect("initialized library")
}

pub fn device<'nvml>(nvml: &'nvml NVML) -> Device<'nvml> {
    nvml.device_by_index(0).expect("device")
}

#[cfg(not(feature = "test-local"))]
pub fn unit<'nvml>(nvml: &'nvml NVML) -> Unit<'nvml> {
    nvml.unit_by_index(0).expect("unit")
}

pub fn assert_send<T: Send>() {}
pub fn assert_sync<T: Sync>() {}

/// Run all testing methods for the given test.
pub fn test<T, R>(reps: usize, test: T)
where
    T: Fn() -> (Result<R>),
    R: ShouldPrint,
{
    single(|| test());

    multi(reps, || test());
}

pub fn test_with_device<T, R>(reps: usize, nvml: &NVML, test: T)
where
    T: Fn(&Device) -> (Result<R>),
    R: ShouldPrint,
{
    let device = device(nvml);

    single(|| test(&device));

    multi(reps, || test(&device));
}

#[cfg(not(feature = "test-local"))]
pub fn test_with_unit<T, R>(reps: usize, nvml: &NVML, test: T)
where
    T: Fn(&Unit) -> (Result<R>),
    R: ShouldPrint,
{
    let unit = unit(nvml);

    single(|| test(&unit));

    multi(reps, || test(&unit));
}

#[cfg(not(feature = "test-local"))]
pub fn test_with_link<T, R>(reps: usize, nvml: &NVML, test: T)
where
    T: Fn(&NvLink) -> (Result<R>),
    R: ShouldPrint,
{
    // Is 0 a good default???
    let device = device(&nvml);
    let link = device.link_wrapper_for(0);

    single(|| test(&link));

    multi(reps, || test(&link));
}

/// Run the given test once.
pub fn single<T, R>(test: T)
where
    T: Fn() -> (Result<R>),
    R: ShouldPrint,
{
    let res = test().expect("successful single test");

    if res.should_print() {
        print!("{:?} ... ", res);
    }
}

/// Run the given test multiple times.
pub fn multi<T, R>(count: usize, test: T)
where
    T: Fn() -> (Result<R>),
    R: ShouldPrint,
{
    for i in 0..count {
        test().expect(&format!("successful multi call #{}", i));
    }
}
