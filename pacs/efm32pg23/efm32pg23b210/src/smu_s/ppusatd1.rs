#[doc = "Register `PPUSATD1` reader"]
pub struct R(crate::R<PPUSATD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUSATD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUSATD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUSATD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUSATD1` writer"]
pub struct W(crate::W<PPUSATD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUSATD1_SPEC>;
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
impl From<crate::W<PPUSATD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUSATD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSRTC` reader - SYSRTC Secure Access"]
pub type SYSRTC_R = crate::BitReader<bool>;
#[doc = "Field `SYSRTC` writer - SYSRTC Secure Access"]
pub type SYSRTC_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 0>;
#[doc = "Field `LCD` reader - LCD Secure Access"]
pub type LCD_R = crate::BitReader<bool>;
#[doc = "Field `LCD` writer - LCD Secure Access"]
pub type LCD_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 1>;
#[doc = "Field `KEYSCAN` reader - KEYSCAN Secure Access"]
pub type KEYSCAN_R = crate::BitReader<bool>;
#[doc = "Field `KEYSCAN` writer - KEYSCAN Secure Access"]
pub type KEYSCAN_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 2>;
#[doc = "Field `DMEM` reader - DMEM Secure Access"]
pub type DMEM_R = crate::BitReader<bool>;
#[doc = "Field `DMEM` writer - DMEM Secure Access"]
pub type DMEM_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 3>;
#[doc = "Field `LCDRF` reader - LCDRF Secure Access"]
pub type LCDRF_R = crate::BitReader<bool>;
#[doc = "Field `LCDRF` writer - LCDRF Secure Access"]
pub type LCDRF_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 4>;
#[doc = "Field `SMU` reader - SMU Secure Access"]
pub type SMU_R = crate::BitReader<bool>;
#[doc = "Field `SMU` writer - SMU Secure Access"]
pub type SMU_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 7>;
#[doc = "Field `SMUCFGNS` reader - SMUCFGNS Secure Access"]
pub type SMUCFGNS_R = crate::BitReader<bool>;
#[doc = "Field `SMUCFGNS` writer - SMUCFGNS Secure Access"]
pub type SMUCFGNS_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 8>;
#[doc = "Field `LETIMER0` reader - LETIMER0 Secure Access"]
pub type LETIMER0_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER0` writer - LETIMER0 Secure Access"]
pub type LETIMER0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 9>;
#[doc = "Field `IADC0` reader - IADC0 Secure Access"]
pub type IADC0_R = crate::BitReader<bool>;
#[doc = "Field `IADC0` writer - IADC0 Secure Access"]
pub type IADC0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 10>;
#[doc = "Field `ACMP0` reader - ACMP0 Secure Access"]
pub type ACMP0_R = crate::BitReader<bool>;
#[doc = "Field `ACMP0` writer - ACMP0 Secure Access"]
pub type ACMP0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 11>;
#[doc = "Field `ACMP1` reader - ACMP1 Secure Access"]
pub type ACMP1_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1` writer - ACMP1 Secure Access"]
pub type ACMP1_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 12>;
#[doc = "Field `AMUXCP0` reader - AMUXCP0 Secure Access"]
pub type AMUXCP0_R = crate::BitReader<bool>;
#[doc = "Field `AMUXCP0` writer - AMUXCP0 Secure Access"]
pub type AMUXCP0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 13>;
#[doc = "Field `VDAC0` reader - VDAC0 Secure Access"]
pub type VDAC0_R = crate::BitReader<bool>;
#[doc = "Field `VDAC0` writer - VDAC0 Secure Access"]
pub type VDAC0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 14>;
#[doc = "Field `PCNT` reader - PCNT Secure Access"]
pub type PCNT_R = crate::BitReader<bool>;
#[doc = "Field `PCNT` writer - PCNT Secure Access"]
pub type PCNT_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 15>;
#[doc = "Field `LESENSE` reader - LESENSE Secure Access"]
pub type LESENSE_R = crate::BitReader<bool>;
#[doc = "Field `LESENSE` writer - LESENSE Secure Access"]
pub type LESENSE_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 16>;
#[doc = "Field `HFRCO1` reader - HFRCO1 Secure Access"]
pub type HFRCO1_R = crate::BitReader<bool>;
#[doc = "Field `HFRCO1` writer - HFRCO1 Secure Access"]
pub type HFRCO1_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 17>;
#[doc = "Field `HFXO0` reader - HFXO0 Secure Access"]
pub type HFXO0_R = crate::BitReader<bool>;
#[doc = "Field `HFXO0` writer - HFXO0 Secure Access"]
pub type HFXO0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 18>;
#[doc = "Field `I2C0` reader - I2C0 Secure Access"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - I2C0 Secure Access"]
pub type I2C0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 19>;
#[doc = "Field `WDOG0` reader - WDOG0 Secure Access"]
pub type WDOG0_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0` writer - WDOG0 Secure Access"]
pub type WDOG0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 20>;
#[doc = "Field `WDOG1` reader - WDOG1 Secure Access"]
pub type WDOG1_R = crate::BitReader<bool>;
#[doc = "Field `WDOG1` writer - WDOG1 Secure Access"]
pub type WDOG1_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 21>;
#[doc = "Field `EUSART0` reader - EUSART0 Secure Access"]
pub type EUSART0_R = crate::BitReader<bool>;
#[doc = "Field `EUSART0` writer - EUSART0 Secure Access"]
pub type EUSART0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 22>;
#[doc = "Field `SEMAILBOX` reader - SEMAILBOX Secure Access"]
pub type SEMAILBOX_R = crate::BitReader<bool>;
#[doc = "Field `SEMAILBOX` writer - SEMAILBOX Secure Access"]
pub type SEMAILBOX_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - SYSRTC Secure Access"]
    #[inline(always)]
    pub fn sysrtc(&self) -> SYSRTC_R {
        SYSRTC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Secure Access"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - KEYSCAN Secure Access"]
    #[inline(always)]
    pub fn keyscan(&self) -> KEYSCAN_R {
        KEYSCAN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMEM Secure Access"]
    #[inline(always)]
    pub fn dmem(&self) -> DMEM_R {
        DMEM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCDRF Secure Access"]
    #[inline(always)]
    pub fn lcdrf(&self) -> LCDRF_R {
        LCDRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - SMU Secure Access"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMUCFGNS Secure Access"]
    #[inline(always)]
    pub fn smucfgns(&self) -> SMUCFGNS_R {
        SMUCFGNS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LETIMER0 Secure Access"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IADC0 Secure Access"]
    #[inline(always)]
    pub fn iadc0(&self) -> IADC0_R {
        IADC0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ACMP0 Secure Access"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ACMP1 Secure Access"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AMUXCP0 Secure Access"]
    #[inline(always)]
    pub fn amuxcp0(&self) -> AMUXCP0_R {
        AMUXCP0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VDAC0 Secure Access"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCNT Secure Access"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LESENSE Secure Access"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HFRCO1 Secure Access"]
    #[inline(always)]
    pub fn hfrco1(&self) -> HFRCO1_R {
        HFRCO1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HFXO0 Secure Access"]
    #[inline(always)]
    pub fn hfxo0(&self) -> HFXO0_R {
        HFXO0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C0 Secure Access"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WDOG0 Secure Access"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDOG1 Secure Access"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EUSART0 Secure Access"]
    #[inline(always)]
    pub fn eusart0(&self) -> EUSART0_R {
        EUSART0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SEMAILBOX Secure Access"]
    #[inline(always)]
    pub fn semailbox(&self) -> SEMAILBOX_R {
        SEMAILBOX_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRTC Secure Access"]
    #[inline(always)]
    pub fn sysrtc(&mut self) -> SYSRTC_W {
        SYSRTC_W::new(self)
    }
    #[doc = "Bit 1 - LCD Secure Access"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W {
        LCD_W::new(self)
    }
    #[doc = "Bit 2 - KEYSCAN Secure Access"]
    #[inline(always)]
    pub fn keyscan(&mut self) -> KEYSCAN_W {
        KEYSCAN_W::new(self)
    }
    #[doc = "Bit 3 - DMEM Secure Access"]
    #[inline(always)]
    pub fn dmem(&mut self) -> DMEM_W {
        DMEM_W::new(self)
    }
    #[doc = "Bit 4 - LCDRF Secure Access"]
    #[inline(always)]
    pub fn lcdrf(&mut self) -> LCDRF_W {
        LCDRF_W::new(self)
    }
    #[doc = "Bit 7 - SMU Secure Access"]
    #[inline(always)]
    pub fn smu(&mut self) -> SMU_W {
        SMU_W::new(self)
    }
    #[doc = "Bit 8 - SMUCFGNS Secure Access"]
    #[inline(always)]
    pub fn smucfgns(&mut self) -> SMUCFGNS_W {
        SMUCFGNS_W::new(self)
    }
    #[doc = "Bit 9 - LETIMER0 Secure Access"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W::new(self)
    }
    #[doc = "Bit 10 - IADC0 Secure Access"]
    #[inline(always)]
    pub fn iadc0(&mut self) -> IADC0_W {
        IADC0_W::new(self)
    }
    #[doc = "Bit 11 - ACMP0 Secure Access"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> ACMP0_W {
        ACMP0_W::new(self)
    }
    #[doc = "Bit 12 - ACMP1 Secure Access"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> ACMP1_W {
        ACMP1_W::new(self)
    }
    #[doc = "Bit 13 - AMUXCP0 Secure Access"]
    #[inline(always)]
    pub fn amuxcp0(&mut self) -> AMUXCP0_W {
        AMUXCP0_W::new(self)
    }
    #[doc = "Bit 14 - VDAC0 Secure Access"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> VDAC0_W {
        VDAC0_W::new(self)
    }
    #[doc = "Bit 15 - PCNT Secure Access"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PCNT_W {
        PCNT_W::new(self)
    }
    #[doc = "Bit 16 - LESENSE Secure Access"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LESENSE_W {
        LESENSE_W::new(self)
    }
    #[doc = "Bit 17 - HFRCO1 Secure Access"]
    #[inline(always)]
    pub fn hfrco1(&mut self) -> HFRCO1_W {
        HFRCO1_W::new(self)
    }
    #[doc = "Bit 18 - HFXO0 Secure Access"]
    #[inline(always)]
    pub fn hfxo0(&mut self) -> HFXO0_W {
        HFXO0_W::new(self)
    }
    #[doc = "Bit 19 - I2C0 Secure Access"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W::new(self)
    }
    #[doc = "Bit 20 - WDOG0 Secure Access"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> WDOG0_W {
        WDOG0_W::new(self)
    }
    #[doc = "Bit 21 - WDOG1 Secure Access"]
    #[inline(always)]
    pub fn wdog1(&mut self) -> WDOG1_W {
        WDOG1_W::new(self)
    }
    #[doc = "Bit 22 - EUSART0 Secure Access"]
    #[inline(always)]
    pub fn eusart0(&mut self) -> EUSART0_W {
        EUSART0_W::new(self)
    }
    #[doc = "Bit 23 - SEMAILBOX Secure Access"]
    #[inline(always)]
    pub fn semailbox(&mut self) -> SEMAILBOX_W {
        SEMAILBOX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set peripheral bits to 1 to mark as secure access only\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppusatd1](index.html) module"]
pub struct PPUSATD1_SPEC;
impl crate::RegisterSpec for PPUSATD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppusatd1::R](R) reader structure"]
impl crate::Readable for PPUSATD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppusatd1::W](W) writer structure"]
impl crate::Writable for PPUSATD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPUSATD1 to value 0x01ff_ffff"]
impl crate::Resettable for PPUSATD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01ff_ffff
    }
}