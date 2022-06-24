#[doc = "Register `I2C1_ROUTEEN` reader"]
pub struct R(crate::R<I2C1_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C1_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C1_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C1_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C1_ROUTEEN` writer"]
pub struct W(crate::W<I2C1_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C1_ROUTEEN_SPEC>;
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
impl From<crate::W<I2C1_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C1_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLPEN` reader - SCL pin enable control bit"]
pub type SCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `SCLPEN` writer - SCL pin enable control bit"]
pub type SCLPEN_W<'a> = crate::BitWriter<'a, u32, I2C1_ROUTEEN_SPEC, bool, 0>;
#[doc = "Field `SDAPEN` reader - SDA pin enable control bit"]
pub type SDAPEN_R = crate::BitReader<bool>;
#[doc = "Field `SDAPEN` writer - SDA pin enable control bit"]
pub type SDAPEN_W<'a> = crate::BitWriter<'a, u32, I2C1_ROUTEEN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - SCL pin enable control bit"]
    #[inline(always)]
    pub fn sclpen(&self) -> SCLPEN_R {
        SCLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDA pin enable control bit"]
    #[inline(always)]
    pub fn sdapen(&self) -> SDAPEN_R {
        SDAPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCL pin enable control bit"]
    #[inline(always)]
    pub fn sclpen(&mut self) -> SCLPEN_W {
        SCLPEN_W::new(self)
    }
    #[doc = "Bit 1 - SDA pin enable control bit"]
    #[inline(always)]
    pub fn sdapen(&mut self) -> SDAPEN_W {
        SDAPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C1 pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c1_routeen](index.html) module"]
pub struct I2C1_ROUTEEN_SPEC;
impl crate::RegisterSpec for I2C1_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c1_routeen::R](R) reader structure"]
impl crate::Readable for I2C1_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c1_routeen::W](W) writer structure"]
impl crate::Writable for I2C1_ROUTEEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C1_ROUTEEN to value 0"]
impl crate::Resettable for I2C1_ROUTEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
