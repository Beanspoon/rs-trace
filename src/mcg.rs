#![allow(unused)]
use core::ptr;

use modify_derive::Modify;

#[repr(packed)]
struct MCG {
    control1: u8,
    control2: u8,
    control3: u8,
    control4: u8,
    res_1: u8,
    control6: u8,
    status: u8,
    res_2: u8,
    status_and_control: u8,
    res_3: u8,
    auto_trim_compare_value: u16,
}

#[derive(Clone, Copy, Modify)]
#[modify(width = 2, position = 6, register_type = u8)]
enum ClockSource {
    FLL = 0b00,
    Internal = 0b01,
    External = 0b10,
}

#[derive(Modify)]
#[modify(width = 3, position = 3, register_type = u8)]
enum FllExternalReferenceDivider {
    Div1or32 = 0b000,
    Div2or64 = 0b001,
    Div4or128 = 0b010,
    Div8or256 = 0b011,
    Div16or512 = 0b100,
    Div32or1024 = 0b101,
    Div64or1280 = 0b110,
    Div128or1536 = 0b111,
}

#[derive(Modify)]
#[modify(width = 1, position = 2, register_type = u8)]
enum ReferenceSelect {
    External,
    Internal,
}

#[derive(Modify)]
#[modify(width = 1, position = 1, register_type = u8)]
enum InternalReferenceClockEnable {
    Inactive,
    Active,
}

#[derive(Modify)]
#[modify(width = 1, position = 0, register_type = u8)]
enum InternalReferenceStopEnable {
    Disabled,
    Enabled,
}

#[derive(Modify)]
#[modify(width = 1, position = 7, register_type = u8)]
enum LossOfClockResetEnable {
    Interrupt,
    Reset,
}

#[derive(Modify)]
#[modify(width = 1, position = 6, register_type = u8)]
enum FastInternalReferenceClockFineTrim {
    Clear,
    Set,
}

#[derive(Modify)]
#[modify(width = 2, position = 4, register_type = u8)]
enum FrequencyRangeSelect {
    Low,
    High,
    VeryHigh,
}

#[derive(Modify)]
#[modify(width = 1, position = 3, register_type = u8)]
enum HighGainOscillatorSelect {
    LowPowerMode,
    HighGainMode,
}

#[derive(Modify)]
#[modify(width = 1, position = 2, register_type = u8)]
enum ExternalReferenceSelect {
    ExternalReferenceClock,
    Oscillator,
}

#[derive(Modify)]
#[modify(width = 1, position = 1, register_type = u8)]
enum FllLowPowerDisable {
    Enabled,
    Disabled,
}

#[derive(Modify)]
#[modify(width = 1, position = 2, register_type = u8)]
enum InternalReferenceClockSelect {
    Slow,
    Fast,
}

macro_rules! read_modify_write {
    ($fn_name: ident, $register: ident, $setting_type: ty) => {
        pub unsafe fn $fn_name(&mut self, value: $setting_type) {
            let read = ptr::read_volatile(&self.$register);
            ptr::write_volatile(&mut self.$register, value.modify(read));
        }
    };
}

impl MCG {
    read_modify_write!(set_clock_source, control1, ClockSource);
    read_modify_write!(
        set_fll_external_reference_divider,
        control1,
        FllExternalReferenceDivider
    );
    read_modify_write!(select_reference, control1, ReferenceSelect);
    read_modify_write!(
        set_internal_reference_clock_enable,
        control1,
        InternalReferenceClockEnable
    );
    read_modify_write!(
        set_internal_reference_clock_enable_during_stop,
        control1,
        InternalReferenceStopEnable
    );

    read_modify_write!(
        set_loss_of_clock_reset_enable,
        control2,
        LossOfClockResetEnable
    );
    read_modify_write!(
        set_fast_internal_clock_fine_trim,
        control2,
        FastInternalReferenceClockFineTrim
    );
    read_modify_write!(set_frequency_range, control2, FrequencyRangeSelect);
    read_modify_write!(
        select_high_gain_oscillator_mode,
        control2,
        HighGainOscillatorSelect
    );
    read_modify_write!(select_external_reference, control2, ExternalReferenceSelect);
    read_modify_write!(select_fll_low_power_disable, control2, FllLowPowerDisable);
    read_modify_write!(
        select_internal_reference,
        control2,
        InternalReferenceClockSelect
    );
}
