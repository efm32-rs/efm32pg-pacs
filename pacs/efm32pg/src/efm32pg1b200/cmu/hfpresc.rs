#[doc = "Register `HFPRESC` reader"]
pub struct R(crate::R<HFPRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFPRESC` writer"]
pub struct W(crate::W<HFPRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPRESC_SPEC>;
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
impl From<crate::W<HFPRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - HFCLK Prescaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
#[doc = "HFCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Field `PRESC` writer - HFCLK Prescaler"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFPRESC_SPEC, u8, PRESC_A, 5, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
}
#[doc = "Field `HFCLKLEPRESC` reader - HFCLKLE Prescaler"]
pub type HFCLKLEPRESC_R = crate::BitReader<HFCLKLEPRESC_A>;
#[doc = "HFCLKLE Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFCLKLEPRESC_A {
    #[doc = "0: HFCLKLE is HFBUSCLKLE divided by 2."]
    DIV2 = 0,
    #[doc = "1: HFCLKLE is HFBUSCLKLE divided by 4."]
    DIV4 = 1,
}
impl From<HFCLKLEPRESC_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKLEPRESC_A) -> Self {
        variant as u8 != 0
    }
}
impl HFCLKLEPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKLEPRESC_A {
        match self.bits {
            false => HFCLKLEPRESC_A::DIV2,
            true => HFCLKLEPRESC_A::DIV4,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV4
    }
}
#[doc = "Field `HFCLKLEPRESC` writer - HFCLKLE Prescaler"]
pub type HFCLKLEPRESC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HFPRESC_SPEC, HFCLKLEPRESC_A, O>;
impl<'a, const O: u8> HFCLKLEPRESC_W<'a, O> {
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HFCLKLEPRESC_A::DIV2)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HFCLKLEPRESC_A::DIV4)
    }
}
impl R {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&self) -> HFCLKLEPRESC_R {
        HFCLKLEPRESC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<8> {
        PRESC_W::new(self)
    }
    #[doc = "Bit 24 - HFCLKLE Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hfclklepresc(&mut self) -> HFCLKLEPRESC_W<24> {
        HFCLKLEPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfpresc](index.html) module"]
pub struct HFPRESC_SPEC;
impl crate::RegisterSpec for HFPRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfpresc::R](R) reader structure"]
impl crate::Readable for HFPRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfpresc::W](W) writer structure"]
impl crate::Writable for HFPRESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFPRESC to value 0"]
impl crate::Resettable for HFPRESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
