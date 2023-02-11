#[doc = "Register `CONSUMER_VDAC0_SYNCTRIGCH0` reader"]
pub struct R(crate::R<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSUMER_VDAC0_SYNCTRIGCH0` writer"]
pub struct W(crate::W<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>;
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
impl From<crate::W<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPRSSEL` reader - SYNCTRIG sync channel select"]
pub type SPRSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPRSSEL` writer - SYNCTRIG sync channel select"]
pub type SPRSSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSUMER_VDAC0_SYNCTRIGCH0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 8:9 - SYNCTRIG sync channel select"]
    #[inline(always)]
    pub fn sprssel(&self) -> SPRSSEL_R {
        SPRSSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - SYNCTRIG sync channel select"]
    #[inline(always)]
    #[must_use]
    pub fn sprssel(&mut self) -> SPRSSEL_W<8> {
        SPRSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNCTRIG Consumer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [consumer_vdac0_synctrigch0](index.html) module"]
pub struct CONSUMER_VDAC0_SYNCTRIGCH0_SPEC;
impl crate::RegisterSpec for CONSUMER_VDAC0_SYNCTRIGCH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [consumer_vdac0_synctrigch0::R](R) reader structure"]
impl crate::Readable for CONSUMER_VDAC0_SYNCTRIGCH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [consumer_vdac0_synctrigch0::W](W) writer structure"]
impl crate::Writable for CONSUMER_VDAC0_SYNCTRIGCH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSUMER_VDAC0_SYNCTRIGCH0 to value 0"]
impl crate::Resettable for CONSUMER_VDAC0_SYNCTRIGCH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
