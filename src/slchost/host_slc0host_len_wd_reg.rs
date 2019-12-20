#[doc = "Reader of register HOST_SLC0HOST_LEN_WD_REG"]
pub type R = crate::R<u32, super::HOST_SLC0HOST_LEN_WD_REG>;
#[doc = "Writer for register HOST_SLC0HOST_LEN_WD_REG"]
pub type W = crate::W<u32, super::HOST_SLC0HOST_LEN_WD_REG>;
#[doc = "Register HOST_SLC0HOST_LEN_WD_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLC0HOST_LEN_WD_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLC0HOST_LEN_WD`"]
pub type HOST_SLC0HOST_LEN_WD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_SLC0HOST_LEN_WD`"]
pub struct HOST_SLC0HOST_LEN_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_LEN_WD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc0host_len_wd(&self) -> HOST_SLC0HOST_LEN_WD_R {
        HOST_SLC0HOST_LEN_WD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc0host_len_wd(&mut self) -> HOST_SLC0HOST_LEN_WD_W {
        HOST_SLC0HOST_LEN_WD_W { w: self }
    }
}
