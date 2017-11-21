#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TI1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1SR {
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"] TI1INPUT,
    #[doc = "The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"] ALLTI1INPUT,
}
impl TI1SR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            TI1SR::TI1INPUT => false,
            TI1SR::ALLTI1INPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> TI1SR {
        match value {
            false => TI1SR::TI1INPUT,
            true => TI1SR::ALLTI1INPUT,
        }
    }
    #[doc = "Checks if the value of the field is `TI1INPUT`"]
    #[inline(always)]
    pub fn is_ti1input(&self) -> bool {
        *self == TI1SR::TI1INPUT
    }
    #[doc = "Checks if the value of the field is `ALLTI1INPUT`"]
    #[inline(always)]
    pub fn is_all_ti1input(&self) -> bool {
        *self == TI1SR::ALLTI1INPUT
    }
}
#[doc = "Possible values of the field `MMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMSR {
    #[doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."] RESET,
    #[doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)."] ENABLE,
    #[doc = "The update event is selected as trigger output (TRGO)."] UPDATE,
    #[doc = "The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.(TRGO)."] COMPAREPULSE,
    #[doc = "OC1REF signal is used as trigger output (TRGO)"] COMPAREOC1REF,
    #[doc = "OC2REF signal is used as trigger output (TRGO)"] COMPAREOC2REF,
    #[doc = "OC3REF signal is used as trigger output (TRGO)"] COMPAREOC3REF,
    #[doc = "OC4REF signal is used as trigger output (TRGO)"] COMPAREOC4REF,
}
impl MMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            MMSR::RESET => 0,
            MMSR::ENABLE => 1,
            MMSR::UPDATE => 2,
            MMSR::COMPAREPULSE => 3,
            MMSR::COMPAREOC1REF => 4,
            MMSR::COMPAREOC2REF => 5,
            MMSR::COMPAREOC3REF => 6,
            MMSR::COMPAREOC4REF => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> MMSR {
        match value {
            0 => MMSR::RESET,
            1 => MMSR::ENABLE,
            2 => MMSR::UPDATE,
            3 => MMSR::COMPAREPULSE,
            4 => MMSR::COMPAREOC1REF,
            5 => MMSR::COMPAREOC2REF,
            6 => MMSR::COMPAREOC3REF,
            7 => MMSR::COMPAREOC4REF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMSR::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMSR::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMSR::UPDATE
    }
    #[doc = "Checks if the value of the field is `COMPAREPULSE`"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMSR::COMPAREPULSE
    }
    #[doc = "Checks if the value of the field is `COMPAREOC1REF`"]
    #[inline(always)]
    pub fn is_compare_oc1ref(&self) -> bool {
        *self == MMSR::COMPAREOC1REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC2REF`"]
    #[inline(always)]
    pub fn is_compare_oc2ref(&self) -> bool {
        *self == MMSR::COMPAREOC2REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC3REF`"]
    #[inline(always)]
    pub fn is_compare_oc3ref(&self) -> bool {
        *self == MMSR::COMPAREOC3REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC4REF`"]
    #[inline(always)]
    pub fn is_compare_oc4ref(&self) -> bool {
        *self == MMSR::COMPAREOC4REF
    }
}
#[doc = "Possible values of the field `CCDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDSR {
    #[doc = "CCx DMA request sent when CCx event occurs"] CCXEVENT,
    #[doc = "CCx DMA requests sent when update event occurs"] UPDATE,
}
impl CCDSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            CCDSR::CCXEVENT => false,
            CCDSR::UPDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CCDSR {
        match value {
            false => CCDSR::CCXEVENT,
            true => CCDSR::UPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `CCXEVENT`"]
    #[inline(always)]
    pub fn is_ccx_event(&self) -> bool {
        *self == CCDSR::CCXEVENT
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == CCDSR::UPDATE
    }
}
#[doc = "Values that can be written to the field `TI1S`"]
pub enum TI1SW {
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"] TI1INPUT,
    #[doc = "The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"] ALLTI1INPUT,
}
impl TI1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TI1SW::TI1INPUT => false,
            TI1SW::ALLTI1INPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TI1SW<'a> {
    w: &'a mut W,
}
impl<'a> _TI1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn ti1input(self) -> &'a mut W {
        self.variant(TI1SW::TI1INPUT)
    }
    #[doc = "The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub fn all_ti1input(self) -> &'a mut W {
        self.variant(TI1SW::ALLTI1INPUT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MMS`"]
pub enum MMSW {
    #[doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."] RESET,
    #[doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)."] ENABLE,
    #[doc = "The update event is selected as trigger output (TRGO)."] UPDATE,
    #[doc = "The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.(TRGO)."] COMPAREPULSE,
    #[doc = "OC1REF signal is used as trigger output (TRGO)"] COMPAREOC1REF,
    #[doc = "OC2REF signal is used as trigger output (TRGO)"] COMPAREOC2REF,
    #[doc = "OC3REF signal is used as trigger output (TRGO)"] COMPAREOC3REF,
    #[doc = "OC4REF signal is used as trigger output (TRGO)"] COMPAREOC4REF,
}
impl MMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MMSW::RESET => 0,
            MMSW::ENABLE => 1,
            MMSW::UPDATE => 2,
            MMSW::COMPAREPULSE => 3,
            MMSW::COMPAREOC1REF => 4,
            MMSW::COMPAREOC2REF => 5,
            MMSW::COMPAREOC3REF => 6,
            MMSW::COMPAREOC4REF => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMSW<'a> {
    w: &'a mut W,
}
impl<'a> _MMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMSW::RESET)
    }
    #[doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMSW::ENABLE)
    }
    #[doc = "The update event is selected as trigger output (TRGO)."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMSW::UPDATE)
    }
    #[doc = "The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.(TRGO)."]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMSW::COMPAREPULSE)
    }
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn compare_oc1ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC1REF)
    }
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn compare_oc2ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC2REF)
    }
    #[doc = "OC3REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn compare_oc3ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC3REF)
    }
    #[doc = "OC4REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn compare_oc4ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC4REF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCDS`"]
pub enum CCDSW {
    #[doc = "CCx DMA request sent when CCx event occurs"] CCXEVENT,
    #[doc = "CCx DMA requests sent when update event occurs"] UPDATE,
}
impl CCDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCDSW::CCXEVENT => false,
            CCDSW::UPDATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCDSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn ccx_event(self) -> &'a mut W {
        self.variant(CCDSW::CCXEVENT)
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(CCDSW::UPDATE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1SR {
        TI1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMSR {
        MMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDSR {
        CCDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> _TI1SW {
        _TI1SW { w: self }
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&mut self) -> _MMSW {
        _MMSW { w: self }
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> _CCDSW {
        _CCDSW { w: self }
    }
}
