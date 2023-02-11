#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVDDBOD` reader - AVDD BOD Interrupt enable"]
pub type AVDDBOD_R = crate::BitReader<bool>;
#[doc = "Field `AVDDBOD` writer - AVDD BOD Interrupt enable"]
pub type AVDDBOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `IOVDD0BOD` reader - VDDIO0 BOD Interrupt enable"]
pub type IOVDD0BOD_R = crate::BitReader<bool>;
#[doc = "Field `IOVDD0BOD` writer - VDDIO0 BOD Interrupt enable"]
pub type IOVDD0BOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM23WAKEUP` reader - EM23 Wake up Interrupt enable"]
pub type EM23WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `EM23WAKEUP` writer - EM23 Wake up Interrupt enable"]
pub type EM23WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VSCALEDONE` reader - Vscale done Interrupt enable"]
pub type VSCALEDONE_R = crate::BitReader<bool>;
#[doc = "Field `VSCALEDONE` writer - Vscale done Interrupt enable"]
pub type VSCALEDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TEMPAVG` reader - Temperature Interrupt enable"]
pub type TEMPAVG_R = crate::BitReader<bool>;
#[doc = "Field `TEMPAVG` writer - Temperature Interrupt enable"]
pub type TEMPAVG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TEMP` reader - Temperature Interrupt enable"]
pub type TEMP_R = crate::BitReader<bool>;
#[doc = "Field `TEMP` writer - Temperature Interrupt enable"]
pub type TEMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TEMPLOW` reader - Temperature low Interrupt enable"]
pub type TEMPLOW_R = crate::BitReader<bool>;
#[doc = "Field `TEMPLOW` writer - Temperature low Interrupt enable"]
pub type TEMPLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TEMPHIGH` reader - Temperature high Interrupt enable"]
pub type TEMPHIGH_R = crate::BitReader<bool>;
#[doc = "Field `TEMPHIGH` writer - Temperature high Interrupt enable"]
pub type TEMPHIGH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - AVDD BOD Interrupt enable"]
    #[inline(always)]
    pub fn avddbod(&self) -> AVDDBOD_R {
        AVDDBOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VDDIO0 BOD Interrupt enable"]
    #[inline(always)]
    pub fn iovdd0bod(&self) -> IOVDD0BOD_R {
        IOVDD0BOD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - EM23 Wake up Interrupt enable"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> EM23WAKEUP_R {
        EM23WAKEUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Vscale done Interrupt enable"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VSCALEDONE_R {
        VSCALEDONE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn tempavg(&self) -> TEMPAVG_R {
        TEMPAVG_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature low Interrupt enable"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature high Interrupt enable"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - AVDD BOD Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn avddbod(&mut self) -> AVDDBOD_W<16> {
        AVDDBOD_W::new(self)
    }
    #[doc = "Bit 17 - VDDIO0 BOD Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iovdd0bod(&mut self) -> IOVDD0BOD_W<17> {
        IOVDD0BOD_W::new(self)
    }
    #[doc = "Bit 24 - EM23 Wake up Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W<24> {
        EM23WAKEUP_W::new(self)
    }
    #[doc = "Bit 25 - Vscale done Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W<25> {
        VSCALEDONE_W::new(self)
    }
    #[doc = "Bit 27 - Temperature Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tempavg(&mut self) -> TEMPAVG_W<27> {
        TEMPAVG_W::new(self)
    }
    #[doc = "Bit 29 - Temperature Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TEMP_W<29> {
        TEMP_W::new(self)
    }
    #[doc = "Bit 30 - Temperature low Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<30> {
        TEMPLOW_W::new(self)
    }
    #[doc = "Bit 31 - Temperature high Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<31> {
        TEMPHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
