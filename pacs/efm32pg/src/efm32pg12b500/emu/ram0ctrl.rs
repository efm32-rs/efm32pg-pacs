#[doc = "Register `RAM0CTRL` reader"]
pub struct R(crate::R<RAM0CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM0CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM0CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM0CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM0CTRL` writer"]
pub struct W(crate::W<RAM0CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM0CTRL_SPEC>;
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
impl From<crate::W<RAM0CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM0CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMPOWERDOWN` reader - RAM0 Blockset Power-down"]
pub type RAMPOWERDOWN_R = crate::FieldReader<u8, RAMPOWERDOWN_A>;
#[doc = "RAM0 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE = 0,
    #[doc = "8: Power down RAM blocks 4 and above"]
    BLK4 = 8,
    #[doc = "12: Power down RAM blocks 3 and above"]
    BLK3TO4 = 12,
    #[doc = "14: Power down RAM blocks 2 and above"]
    BLK2TO4 = 14,
    #[doc = "15: Power down RAM blocks 1 and above"]
    BLK1TO4 = 15,
}
impl From<RAMPOWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        variant as _
    }
}
impl RAMPOWERDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMPOWERDOWN_A> {
        match self.bits {
            0 => Some(RAMPOWERDOWN_A::NONE),
            8 => Some(RAMPOWERDOWN_A::BLK4),
            12 => Some(RAMPOWERDOWN_A::BLK3TO4),
            14 => Some(RAMPOWERDOWN_A::BLK2TO4),
            15 => Some(RAMPOWERDOWN_A::BLK1TO4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `BLK4`"]
    #[inline(always)]
    pub fn is_blk4(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK4
    }
    #[doc = "Checks if the value of the field is `BLK3TO4`"]
    #[inline(always)]
    pub fn is_blk3to4(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK3TO4
    }
    #[doc = "Checks if the value of the field is `BLK2TO4`"]
    #[inline(always)]
    pub fn is_blk2to4(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK2TO4
    }
    #[doc = "Checks if the value of the field is `BLK1TO4`"]
    #[inline(always)]
    pub fn is_blk1to4(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1TO4
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM0 Blockset Power-down"]
pub type RAMPOWERDOWN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RAM0CTRL_SPEC, u8, RAMPOWERDOWN_A, 4, O>;
impl<'a, const O: u8> RAMPOWERDOWN_W<'a, O> {
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline(always)]
    pub fn blk4(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK4)
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline(always)]
    pub fn blk3to4(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK3TO4)
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn blk2to4(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK2TO4)
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn blk1to4(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK1TO4)
    }
}
impl R {
    #[doc = "Bits 0:3 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM0 Blockset Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W<0> {
        RAMPOWERDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0ctrl](index.html) module"]
pub struct RAM0CTRL_SPEC;
impl crate::RegisterSpec for RAM0CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram0ctrl::R](R) reader structure"]
impl crate::Readable for RAM0CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram0ctrl::W](W) writer structure"]
impl crate::Writable for RAM0CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM0CTRL to value 0"]
impl crate::Resettable for RAM0CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
