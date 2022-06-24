#[doc = "Register `SCAN5` reader"]
pub struct R(crate::R<SCAN5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAN5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAN5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAN5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAN5` writer"]
pub struct W(crate::W<SCAN5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAN5_SPEC>;
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
impl From<crate::W<SCAN5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAN5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINNEG` reader - Negative Pin Select"]
pub type PINNEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINNEG` writer - Negative Pin Select"]
pub type PINNEG_W<'a> = crate::FieldWriter<'a, u32, SCAN5_SPEC, u8, u8, 4, 0>;
#[doc = "Negative Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORTNEG_A {
    #[doc = "0: Ground (single-ended)"]
    GND = 0,
    #[doc = "8: Port A - Select pin number using PINNEG"]
    PORTA = 8,
    #[doc = "9: Port B - Select pin number using PINNEG"]
    PORTB = 9,
    #[doc = "10: Port C - Select pin number using PINNEG"]
    PORTC = 10,
    #[doc = "11: Port D - Select pin number using PINNEG"]
    PORTD = 11,
}
impl From<PORTNEG_A> for u8 {
    #[inline(always)]
    fn from(variant: PORTNEG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PORTNEG` reader - Negative Port Select"]
pub type PORTNEG_R = crate::FieldReader<u8, PORTNEG_A>;
impl PORTNEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PORTNEG_A> {
        match self.bits {
            0 => Some(PORTNEG_A::GND),
            8 => Some(PORTNEG_A::PORTA),
            9 => Some(PORTNEG_A::PORTB),
            10 => Some(PORTNEG_A::PORTC),
            11 => Some(PORTNEG_A::PORTD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == PORTNEG_A::GND
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == PORTNEG_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == PORTNEG_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == PORTNEG_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == PORTNEG_A::PORTD
    }
}
#[doc = "Field `PORTNEG` writer - Negative Port Select"]
pub type PORTNEG_W<'a> = crate::FieldWriter<'a, u32, SCAN5_SPEC, u8, PORTNEG_A, 4, 4>;
impl<'a> PORTNEG_W<'a> {
    #[doc = "Ground (single-ended)"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(PORTNEG_A::GND)
    }
    #[doc = "Port A - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(PORTNEG_A::PORTA)
    }
    #[doc = "Port B - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(PORTNEG_A::PORTB)
    }
    #[doc = "Port C - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(PORTNEG_A::PORTC)
    }
    #[doc = "Port D - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(PORTNEG_A::PORTD)
    }
}
#[doc = "Field `PINPOS` reader - Positive Pin Select"]
pub type PINPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINPOS` writer - Positive Pin Select"]
pub type PINPOS_W<'a> = crate::FieldWriter<'a, u32, SCAN5_SPEC, u8, u8, 4, 8>;
#[doc = "Positive Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORTPOS_A {
    #[doc = "0: Ground"]
    GND = 0,
    #[doc = "1: Supply Pin - Select specific supply using PINPOS"]
    SUPPLY = 1,
    #[doc = "8: Port A - Select pin number using PINPOS"]
    PORTA = 8,
    #[doc = "9: Port B - Select pin number using PINPOS"]
    PORTB = 9,
    #[doc = "10: Port C - Select pin number using PINPOS"]
    PORTC = 10,
    #[doc = "11: Port D - Select pin number using PINPOS"]
    PORTD = 11,
}
impl From<PORTPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: PORTPOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PORTPOS` reader - Positive Port Select"]
pub type PORTPOS_R = crate::FieldReader<u8, PORTPOS_A>;
impl PORTPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PORTPOS_A> {
        match self.bits {
            0 => Some(PORTPOS_A::GND),
            1 => Some(PORTPOS_A::SUPPLY),
            8 => Some(PORTPOS_A::PORTA),
            9 => Some(PORTPOS_A::PORTB),
            10 => Some(PORTPOS_A::PORTC),
            11 => Some(PORTPOS_A::PORTD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == PORTPOS_A::GND
    }
    #[doc = "Checks if the value of the field is `SUPPLY`"]
    #[inline(always)]
    pub fn is_supply(&self) -> bool {
        *self == PORTPOS_A::SUPPLY
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == PORTPOS_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == PORTPOS_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == PORTPOS_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == PORTPOS_A::PORTD
    }
}
#[doc = "Field `PORTPOS` writer - Positive Port Select"]
pub type PORTPOS_W<'a> = crate::FieldWriter<'a, u32, SCAN5_SPEC, u8, PORTPOS_A, 4, 12>;
impl<'a> PORTPOS_W<'a> {
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(PORTPOS_A::GND)
    }
    #[doc = "Supply Pin - Select specific supply using PINPOS"]
    #[inline(always)]
    pub fn supply(self) -> &'a mut W {
        self.variant(PORTPOS_A::SUPPLY)
    }
    #[doc = "Port A - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(PORTPOS_A::PORTA)
    }
    #[doc = "Port B - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(PORTPOS_A::PORTB)
    }
    #[doc = "Port C - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(PORTPOS_A::PORTC)
    }
    #[doc = "Port D - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(PORTPOS_A::PORTD)
    }
}
#[doc = "Configuration Group Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG_A {
    #[doc = "0: Use configuration group 0"]
    CONFIG0 = 0,
    #[doc = "1: Use configuration group 1"]
    CONFIG1 = 1,
}
impl From<CFG_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFG` reader - Configuration Group Select"]
pub type CFG_R = crate::BitReader<CFG_A>;
impl CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG_A {
        match self.bits {
            false => CFG_A::CONFIG0,
            true => CFG_A::CONFIG1,
        }
    }
    #[doc = "Checks if the value of the field is `CONFIG0`"]
    #[inline(always)]
    pub fn is_config0(&self) -> bool {
        *self == CFG_A::CONFIG0
    }
    #[doc = "Checks if the value of the field is `CONFIG1`"]
    #[inline(always)]
    pub fn is_config1(&self) -> bool {
        *self == CFG_A::CONFIG1
    }
}
#[doc = "Field `CFG` writer - Configuration Group Select"]
pub type CFG_W<'a> = crate::BitWriter<'a, u32, SCAN5_SPEC, CFG_A, 16>;
impl<'a> CFG_W<'a> {
    #[doc = "Use configuration group 0"]
    #[inline(always)]
    pub fn config0(self) -> &'a mut W {
        self.variant(CFG_A::CONFIG0)
    }
    #[doc = "Use configuration group 1"]
    #[inline(always)]
    pub fn config1(self) -> &'a mut W {
        self.variant(CFG_A::CONFIG1)
    }
}
#[doc = "Field `CMP` reader - Comparison Enable"]
pub type CMP_R = crate::BitReader<bool>;
#[doc = "Field `CMP` writer - Comparison Enable"]
pub type CMP_W<'a> = crate::BitWriter<'a, u32, SCAN5_SPEC, bool, 17>;
impl R {
    #[doc = "Bits 0:3 - Negative Pin Select"]
    #[inline(always)]
    pub fn pinneg(&self) -> PINNEG_R {
        PINNEG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Negative Port Select"]
    #[inline(always)]
    pub fn portneg(&self) -> PORTNEG_R {
        PORTNEG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Positive Pin Select"]
    #[inline(always)]
    pub fn pinpos(&self) -> PINPOS_R {
        PINPOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Positive Port Select"]
    #[inline(always)]
    pub fn portpos(&self) -> PORTPOS_R {
        PORTPOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Configuration Group Select"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparison Enable"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Negative Pin Select"]
    #[inline(always)]
    pub fn pinneg(&mut self) -> PINNEG_W {
        PINNEG_W::new(self)
    }
    #[doc = "Bits 4:7 - Negative Port Select"]
    #[inline(always)]
    pub fn portneg(&mut self) -> PORTNEG_W {
        PORTNEG_W::new(self)
    }
    #[doc = "Bits 8:11 - Positive Pin Select"]
    #[inline(always)]
    pub fn pinpos(&mut self) -> PINPOS_W {
        PINPOS_W::new(self)
    }
    #[doc = "Bits 12:15 - Positive Port Select"]
    #[inline(always)]
    pub fn portpos(&mut self) -> PORTPOS_W {
        PORTPOS_W::new(self)
    }
    #[doc = "Bit 16 - Configuration Group Select"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W {
        CFG_W::new(self)
    }
    #[doc = "Bit 17 - Comparison Enable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan5](index.html) module"]
pub struct SCAN5_SPEC;
impl crate::RegisterSpec for SCAN5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scan5::R](R) reader structure"]
impl crate::Readable for SCAN5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scan5::W](W) writer structure"]
impl crate::Writable for SCAN5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAN5 to value 0"]
impl crate::Resettable for SCAN5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}