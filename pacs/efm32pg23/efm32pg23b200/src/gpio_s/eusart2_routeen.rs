#[doc = "Register `EUSART2_ROUTEEN` reader"]
pub struct R(crate::R<EUSART2_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUSART2_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUSART2_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUSART2_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EUSART2_ROUTEEN` writer"]
pub struct W(crate::W<EUSART2_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EUSART2_ROUTEEN_SPEC>;
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
impl From<crate::W<EUSART2_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EUSART2_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSPEN` reader - CS pin enable control bit"]
pub type CSPEN_R = crate::BitReader<bool>;
#[doc = "Field `CSPEN` writer - CS pin enable control bit"]
pub type CSPEN_W<'a> = crate::BitWriter<'a, u32, EUSART2_ROUTEEN_SPEC, bool, 0>;
#[doc = "Field `RTSPEN` reader - RTS pin enable control bit"]
pub type RTSPEN_R = crate::BitReader<bool>;
#[doc = "Field `RTSPEN` writer - RTS pin enable control bit"]
pub type RTSPEN_W<'a> = crate::BitWriter<'a, u32, EUSART2_ROUTEEN_SPEC, bool, 1>;
#[doc = "Field `RXPEN` reader - RX pin enable control bit"]
pub type RXPEN_R = crate::BitReader<bool>;
#[doc = "Field `RXPEN` writer - RX pin enable control bit"]
pub type RXPEN_W<'a> = crate::BitWriter<'a, u32, EUSART2_ROUTEEN_SPEC, bool, 2>;
#[doc = "Field `SCLKPEN` reader - SCLK pin enable control bit"]
pub type SCLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `SCLKPEN` writer - SCLK pin enable control bit"]
pub type SCLKPEN_W<'a> = crate::BitWriter<'a, u32, EUSART2_ROUTEEN_SPEC, bool, 3>;
#[doc = "Field `TXPEN` reader - TX pin enable control bit"]
pub type TXPEN_R = crate::BitReader<bool>;
#[doc = "Field `TXPEN` writer - TX pin enable control bit"]
pub type TXPEN_W<'a> = crate::BitWriter<'a, u32, EUSART2_ROUTEEN_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - CS pin enable control bit"]
    #[inline(always)]
    pub fn cspen(&self) -> CSPEN_R {
        CSPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&self) -> RTSPEN_R {
        RTSPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX pin enable control bit"]
    #[inline(always)]
    pub fn rxpen(&self) -> RXPEN_R {
        RXPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCLK pin enable control bit"]
    #[inline(always)]
    pub fn sclkpen(&self) -> SCLKPEN_R {
        SCLKPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&self) -> TXPEN_R {
        TXPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CS pin enable control bit"]
    #[inline(always)]
    pub fn cspen(&mut self) -> CSPEN_W {
        CSPEN_W::new(self)
    }
    #[doc = "Bit 1 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&mut self) -> RTSPEN_W {
        RTSPEN_W::new(self)
    }
    #[doc = "Bit 2 - RX pin enable control bit"]
    #[inline(always)]
    pub fn rxpen(&mut self) -> RXPEN_W {
        RXPEN_W::new(self)
    }
    #[doc = "Bit 3 - SCLK pin enable control bit"]
    #[inline(always)]
    pub fn sclkpen(&mut self) -> SCLKPEN_W {
        SCLKPEN_W::new(self)
    }
    #[doc = "Bit 4 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TXPEN_W {
        TXPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EUSART2 pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eusart2_routeen](index.html) module"]
pub struct EUSART2_ROUTEEN_SPEC;
impl crate::RegisterSpec for EUSART2_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eusart2_routeen::R](R) reader structure"]
impl crate::Readable for EUSART2_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eusart2_routeen::W](W) writer structure"]
impl crate::Writable for EUSART2_ROUTEEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EUSART2_ROUTEEN to value 0"]
impl crate::Resettable for EUSART2_ROUTEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
