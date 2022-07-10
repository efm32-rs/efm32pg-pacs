#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVDDBOD` reader - AVDD BOD Interrupt flag"]
pub type AVDDBOD_R = crate::BitReader<bool>;
#[doc = "Field `AVDDBOD` writer - AVDD BOD Interrupt flag"]
pub type AVDDBOD_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 16>;
#[doc = "Field `IOVDD0BOD` reader - VDDIO0 BOD Interrupt flag"]
pub type IOVDD0BOD_R = crate::BitReader<bool>;
#[doc = "Field `IOVDD0BOD` writer - VDDIO0 BOD Interrupt flag"]
pub type IOVDD0BOD_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 17>;
#[doc = "Field `EM23WAKEUP` reader - EM23 Wake up Interrupt flag"]
pub type EM23WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `EM23WAKEUP` writer - EM23 Wake up Interrupt flag"]
pub type EM23WAKEUP_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 24>;
#[doc = "Field `VSCALEDONE` reader - Vscale done Interrupt flag"]
pub type VSCALEDONE_R = crate::BitReader<bool>;
#[doc = "Field `VSCALEDONE` writer - Vscale done Interrupt flag"]
pub type VSCALEDONE_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 25>;
#[doc = "Field `TEMPAVG` reader - Temperature Average Interrupt flag"]
pub type TEMPAVG_R = crate::BitReader<bool>;
#[doc = "Field `TEMPAVG` writer - Temperature Average Interrupt flag"]
pub type TEMPAVG_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 27>;
#[doc = "Field `TEMP` reader - Temperature Interrupt flag"]
pub type TEMP_R = crate::BitReader<bool>;
#[doc = "Field `TEMP` writer - Temperature Interrupt flag"]
pub type TEMP_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 29>;
#[doc = "Field `TEMPLOW` reader - Temperature low Interrupt flag"]
pub type TEMPLOW_R = crate::BitReader<bool>;
#[doc = "Field `TEMPLOW` writer - Temperature low Interrupt flag"]
pub type TEMPLOW_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 30>;
#[doc = "Field `TEMPHIGH` reader - Temperature high Interrupt flag"]
pub type TEMPHIGH_R = crate::BitReader<bool>;
#[doc = "Field `TEMPHIGH` writer - Temperature high Interrupt flag"]
pub type TEMPHIGH_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 16 - AVDD BOD Interrupt flag"]
    #[inline(always)]
    pub fn avddbod(&self) -> AVDDBOD_R {
        AVDDBOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VDDIO0 BOD Interrupt flag"]
    #[inline(always)]
    pub fn iovdd0bod(&self) -> IOVDD0BOD_R {
        IOVDD0BOD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - EM23 Wake up Interrupt flag"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> EM23WAKEUP_R {
        EM23WAKEUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Vscale done Interrupt flag"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VSCALEDONE_R {
        VSCALEDONE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Temperature Average Interrupt flag"]
    #[inline(always)]
    pub fn tempavg(&self) -> TEMPAVG_R {
        TEMPAVG_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Temperature Interrupt flag"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature low Interrupt flag"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature high Interrupt flag"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - AVDD BOD Interrupt flag"]
    #[inline(always)]
    pub fn avddbod(&mut self) -> AVDDBOD_W {
        AVDDBOD_W::new(self)
    }
    #[doc = "Bit 17 - VDDIO0 BOD Interrupt flag"]
    #[inline(always)]
    pub fn iovdd0bod(&mut self) -> IOVDD0BOD_W {
        IOVDD0BOD_W::new(self)
    }
    #[doc = "Bit 24 - EM23 Wake up Interrupt flag"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W {
        EM23WAKEUP_W::new(self)
    }
    #[doc = "Bit 25 - Vscale done Interrupt flag"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W {
        VSCALEDONE_W::new(self)
    }
    #[doc = "Bit 27 - Temperature Average Interrupt flag"]
    #[inline(always)]
    pub fn tempavg(&mut self) -> TEMPAVG_W {
        TEMPAVG_W::new(self)
    }
    #[doc = "Bit 29 - Temperature Interrupt flag"]
    #[inline(always)]
    pub fn temp(&mut self) -> TEMP_W {
        TEMP_W::new(self)
    }
    #[doc = "Bit 30 - Temperature low Interrupt flag"]
    #[inline(always)]
    pub fn templow(&mut self) -> TEMPLOW_W {
        TEMPLOW_W::new(self)
    }
    #[doc = "Bit 31 - Temperature high Interrupt flag"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TEMPHIGH_W {
        TEMPHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}