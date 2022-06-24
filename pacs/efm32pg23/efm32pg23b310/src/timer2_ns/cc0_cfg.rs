#[doc = "Register `CC0_CFG` reader"]
pub struct R(crate::R<CC0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC0_CFG` writer"]
pub struct W(crate::W<CC0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CC0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Compare/Capture channel turned off"]
    OFF = 0,
    #[doc = "1: Input Capture"]
    INPUTCAPTURE = 1,
    #[doc = "2: Output Compare"]
    OUTPUTCOMPARE = 2,
    #[doc = "3: Pulse-Width Modulation"]
    PWM = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::OFF,
            1 => MODE_A::INPUTCAPTURE,
            2 => MODE_A::OUTPUTCOMPARE,
            3 => MODE_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `INPUTCAPTURE`"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == MODE_A::INPUTCAPTURE
    }
    #[doc = "Checks if the value of the field is `OUTPUTCOMPARE`"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == MODE_A::OUTPUTCOMPARE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A::PWM
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type MODE_W<'a> = crate::FieldWriterSafe<'a, u32, CC0_CFG_SPEC, u8, MODE_A, 2, 0>;
impl<'a> MODE_W<'a> {
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
    #[doc = "Input Capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut W {
        self.variant(MODE_A::INPUTCAPTURE)
    }
    #[doc = "Output Compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut W {
        self.variant(MODE_A::OUTPUTCOMPARE)
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE_A::PWM)
    }
}
#[doc = "Field `COIST` reader - Compare Output Initial State"]
pub type COIST_R = crate::BitReader<bool>;
#[doc = "Field `COIST` writer - Compare Output Initial State"]
pub type COIST_W<'a> = crate::BitWriter<'a, u32, CC0_CFG_SPEC, bool, 4>;
#[doc = "Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INSEL_A {
    #[doc = "0: TIMERnCCx pin is selected"]
    PIN = 0,
    #[doc = "1: Synchornous PRS selected"]
    PRSSYNC = 1,
    #[doc = "2: Asynchronous Level PRS selected"]
    PRSASYNCLEVEL = 2,
    #[doc = "3: Asynchronous Pulse PRS selected"]
    PRSASYNCPULSE = 3,
}
impl From<INSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INSEL` reader - Input Selection"]
pub type INSEL_R = crate::FieldReader<u8, INSEL_A>;
impl INSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INSEL_A {
        match self.bits {
            0 => INSEL_A::PIN,
            1 => INSEL_A::PRSSYNC,
            2 => INSEL_A::PRSASYNCLEVEL,
            3 => INSEL_A::PRSASYNCPULSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == INSEL_A::PIN
    }
    #[doc = "Checks if the value of the field is `PRSSYNC`"]
    #[inline(always)]
    pub fn is_prssync(&self) -> bool {
        *self == INSEL_A::PRSSYNC
    }
    #[doc = "Checks if the value of the field is `PRSASYNCLEVEL`"]
    #[inline(always)]
    pub fn is_prsasynclevel(&self) -> bool {
        *self == INSEL_A::PRSASYNCLEVEL
    }
    #[doc = "Checks if the value of the field is `PRSASYNCPULSE`"]
    #[inline(always)]
    pub fn is_prsasyncpulse(&self) -> bool {
        *self == INSEL_A::PRSASYNCPULSE
    }
}
#[doc = "Field `INSEL` writer - Input Selection"]
pub type INSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CC0_CFG_SPEC, u8, INSEL_A, 2, 17>;
impl<'a> INSEL_W<'a> {
    #[doc = "TIMERnCCx pin is selected"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut W {
        self.variant(INSEL_A::PIN)
    }
    #[doc = "Synchornous PRS selected"]
    #[inline(always)]
    pub fn prssync(self) -> &'a mut W {
        self.variant(INSEL_A::PRSSYNC)
    }
    #[doc = "Asynchronous Level PRS selected"]
    #[inline(always)]
    pub fn prsasynclevel(self) -> &'a mut W {
        self.variant(INSEL_A::PRSASYNCLEVEL)
    }
    #[doc = "Asynchronous Pulse PRS selected"]
    #[inline(always)]
    pub fn prsasyncpulse(self) -> &'a mut W {
        self.variant(INSEL_A::PRSASYNCPULSE)
    }
}
#[doc = "PRS Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSCONF_A {
    #[doc = "0: Each CC event will generate a one EM01GRPACLK cycle high pulse"]
    PULSE = 0,
    #[doc = "1: The PRS channel will follow CC out"]
    LEVEL = 1,
}
impl From<PRSCONF_A> for bool {
    #[inline(always)]
    fn from(variant: PRSCONF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRSCONF` reader - PRS Configuration"]
pub type PRSCONF_R = crate::BitReader<PRSCONF_A>;
impl PRSCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSCONF_A {
        match self.bits {
            false => PRSCONF_A::PULSE,
            true => PRSCONF_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == PRSCONF_A::PULSE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PRSCONF_A::LEVEL
    }
}
#[doc = "Field `PRSCONF` writer - PRS Configuration"]
pub type PRSCONF_W<'a> = crate::BitWriter<'a, u32, CC0_CFG_SPEC, PRSCONF_A, 19>;
impl<'a> PRSCONF_W<'a> {
    #[doc = "Each CC event will generate a one EM01GRPACLK cycle high pulse"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(PRSCONF_A::PULSE)
    }
    #[doc = "The PRS channel will follow CC out"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(PRSCONF_A::LEVEL)
    }
}
#[doc = "Digital Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILT_A {
    #[doc = "0: Digital Filter Disabled"]
    DISABLE = 0,
    #[doc = "1: Digital Filter Enabled"]
    ENABLE = 1,
}
impl From<FILT_A> for bool {
    #[inline(always)]
    fn from(variant: FILT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILT` reader - Digital Filter"]
pub type FILT_R = crate::BitReader<FILT_A>;
impl FILT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILT_A {
        match self.bits {
            false => FILT_A::DISABLE,
            true => FILT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FILT_A::ENABLE
    }
}
#[doc = "Field `FILT` writer - Digital Filter"]
pub type FILT_W<'a> = crate::BitWriter<'a, u32, CC0_CFG_SPEC, FILT_A, 20>;
impl<'a> FILT_W<'a> {
    #[doc = "Digital Filter Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FILT_A::DISABLE)
    }
    #[doc = "Digital Filter Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FILT_A::ENABLE)
    }
}
#[doc = "Field `ICFWL` reader - Input Capture FIFO watermark level"]
pub type ICFWL_R = crate::BitReader<bool>;
#[doc = "Field `ICFWL` writer - Input Capture FIFO watermark level"]
pub type ICFWL_W<'a> = crate::BitWriter<'a, u32, CC0_CFG_SPEC, bool, 21>;
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&self) -> COIST_R {
        COIST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Input Selection"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - PRS Configuration"]
    #[inline(always)]
    pub fn prsconf(&self) -> PRSCONF_R {
        PRSCONF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input Capture FIFO watermark level"]
    #[inline(always)]
    pub fn icfwl(&self) -> ICFWL_R {
        ICFWL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&mut self) -> COIST_W {
        COIST_W::new(self)
    }
    #[doc = "Bits 17:18 - Input Selection"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W::new(self)
    }
    #[doc = "Bit 19 - PRS Configuration"]
    #[inline(always)]
    pub fn prsconf(&mut self) -> PRSCONF_W {
        PRSCONF_W::new(self)
    }
    #[doc = "Bit 20 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FILT_W {
        FILT_W::new(self)
    }
    #[doc = "Bit 21 - Input Capture FIFO watermark level"]
    #[inline(always)]
    pub fn icfwl(&mut self) -> ICFWL_W {
        ICFWL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0_cfg](index.html) module"]
pub struct CC0_CFG_SPEC;
impl crate::RegisterSpec for CC0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc0_cfg::R](R) reader structure"]
impl crate::Readable for CC0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc0_cfg::W](W) writer structure"]
impl crate::Writable for CC0_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC0_CFG to value 0"]
impl crate::Resettable for CC0_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
