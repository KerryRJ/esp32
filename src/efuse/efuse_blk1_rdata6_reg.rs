#[doc = "Reader of register EFUSE_BLK1_RDATA6_REG"]
pub type R = crate::R<u32, super::EFUSE_BLK1_RDATA6_REG>;
#[doc = "Writer for register EFUSE_BLK1_RDATA6_REG"]
pub type W = crate::W<u32, super::EFUSE_BLK1_RDATA6_REG>;
#[doc = "Register EFUSE_BLK1_RDATA6_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_BLK1_RDATA6_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_BLK1_DOUT6`"]
pub type EFUSE_BLK1_DOUT6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_BLK1_DOUT6`"]
pub struct EFUSE_BLK1_DOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_BLK1_DOUT6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK1"]
    #[inline(always)]
    pub fn efuse_blk1_dout6(&self) -> EFUSE_BLK1_DOUT6_R {
        EFUSE_BLK1_DOUT6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - read for BLOCK1"]
    #[inline(always)]
    pub fn efuse_blk1_dout6(&mut self) -> EFUSE_BLK1_DOUT6_W {
        EFUSE_BLK1_DOUT6_W { w: self }
    }
}
