#[doc = "Register `RET2_REG` reader"]
pub struct R(crate::R<RET2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RET2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RET2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RET2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RET2_REG` writer"]
pub struct W(crate::W<RET2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RET2_REG_SPEC>;
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
impl From<crate::W<RET2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RET2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETREG` reader - Latch based Retention register"]
pub type RETREG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RETREG` writer - Latch based Retention register"]
pub type RETREG_W<'a> = crate::FieldWriter<'a, u32, RET2_REG_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Latch based Retention register"]
    #[inline(always)]
    pub fn retreg(&self) -> RETREG_R {
        RETREG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Latch based Retention register"]
    #[inline(always)]
    pub fn retreg(&mut self) -> RETREG_W {
        RETREG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ret2_reg](index.html) module"]
pub struct RET2_REG_SPEC;
impl crate::RegisterSpec for RET2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ret2_reg::R](R) reader structure"]
impl crate::Readable for RET2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ret2_reg::W](W) writer structure"]
impl crate::Writable for RET2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RET2_REG to value 0"]
impl crate::Resettable for RET2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
