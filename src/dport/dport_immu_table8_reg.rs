#[doc = "Reader of register DPORT_IMMU_TABLE8_REG"]
pub type R = crate::R<u32, super::DPORT_IMMU_TABLE8_REG>;
#[doc = "Writer for register DPORT_IMMU_TABLE8_REG"]
pub type W = crate::W<u32, super::DPORT_IMMU_TABLE8_REG>;
#[doc = "Register DPORT_IMMU_TABLE8_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_IMMU_TABLE8_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_IMMU_TABLE8`"]
pub type DPORT_IMMU_TABLE8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_IMMU_TABLE8`"]
pub struct DPORT_IMMU_TABLE8_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_IMMU_TABLE8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dport_immu_table8(&self) -> DPORT_IMMU_TABLE8_R {
        DPORT_IMMU_TABLE8_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dport_immu_table8(&mut self) -> DPORT_IMMU_TABLE8_W {
        DPORT_IMMU_TABLE8_W { w: self }
    }
}
