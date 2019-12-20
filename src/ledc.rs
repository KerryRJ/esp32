#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LEDC_HSCH0_CONF0_REG"]
    pub ledc_hsch0_conf0_reg: LEDC_HSCH0_CONF0_REG,
    #[doc = "0x04 - LEDC_HSCH0_HPOINT_REG"]
    pub ledc_hsch0_hpoint_reg: LEDC_HSCH0_HPOINT_REG,
    #[doc = "0x08 - LEDC_HSCH0_DUTY_REG"]
    pub ledc_hsch0_duty_reg: LEDC_HSCH0_DUTY_REG,
    #[doc = "0x0c - LEDC_HSCH0_CONF1_REG"]
    pub ledc_hsch0_conf1_reg: LEDC_HSCH0_CONF1_REG,
    #[doc = "0x10 - LEDC_HSCH0_DUTY_R_REG"]
    pub ledc_hsch0_duty_r_reg: LEDC_HSCH0_DUTY_R_REG,
    #[doc = "0x14 - LEDC_HSCH1_CONF0_REG"]
    pub ledc_hsch1_conf0_reg: LEDC_HSCH1_CONF0_REG,
    #[doc = "0x18 - LEDC_HSCH1_HPOINT_REG"]
    pub ledc_hsch1_hpoint_reg: LEDC_HSCH1_HPOINT_REG,
    #[doc = "0x1c - LEDC_HSCH1_DUTY_REG"]
    pub ledc_hsch1_duty_reg: LEDC_HSCH1_DUTY_REG,
    #[doc = "0x20 - LEDC_HSCH1_CONF1_REG"]
    pub ledc_hsch1_conf1_reg: LEDC_HSCH1_CONF1_REG,
    #[doc = "0x24 - LEDC_HSCH1_DUTY_R_REG"]
    pub ledc_hsch1_duty_r_reg: LEDC_HSCH1_DUTY_R_REG,
    #[doc = "0x28 - LEDC_HSCH2_CONF0_REG"]
    pub ledc_hsch2_conf0_reg: LEDC_HSCH2_CONF0_REG,
    #[doc = "0x2c - LEDC_HSCH2_HPOINT_REG"]
    pub ledc_hsch2_hpoint_reg: LEDC_HSCH2_HPOINT_REG,
    #[doc = "0x30 - LEDC_HSCH2_DUTY_REG"]
    pub ledc_hsch2_duty_reg: LEDC_HSCH2_DUTY_REG,
    #[doc = "0x34 - LEDC_HSCH2_CONF1_REG"]
    pub ledc_hsch2_conf1_reg: LEDC_HSCH2_CONF1_REG,
    #[doc = "0x38 - LEDC_HSCH2_DUTY_R_REG"]
    pub ledc_hsch2_duty_r_reg: LEDC_HSCH2_DUTY_R_REG,
    #[doc = "0x3c - LEDC_HSCH3_CONF0_REG"]
    pub ledc_hsch3_conf0_reg: LEDC_HSCH3_CONF0_REG,
    #[doc = "0x40 - LEDC_HSCH3_HPOINT_REG"]
    pub ledc_hsch3_hpoint_reg: LEDC_HSCH3_HPOINT_REG,
    #[doc = "0x44 - LEDC_HSCH3_DUTY_REG"]
    pub ledc_hsch3_duty_reg: LEDC_HSCH3_DUTY_REG,
    #[doc = "0x48 - LEDC_HSCH3_CONF1_REG"]
    pub ledc_hsch3_conf1_reg: LEDC_HSCH3_CONF1_REG,
    #[doc = "0x4c - LEDC_HSCH3_DUTY_R_REG"]
    pub ledc_hsch3_duty_r_reg: LEDC_HSCH3_DUTY_R_REG,
    #[doc = "0x50 - LEDC_HSCH4_CONF0_REG"]
    pub ledc_hsch4_conf0_reg: LEDC_HSCH4_CONF0_REG,
    #[doc = "0x54 - LEDC_HSCH4_HPOINT_REG"]
    pub ledc_hsch4_hpoint_reg: LEDC_HSCH4_HPOINT_REG,
    #[doc = "0x58 - LEDC_HSCH4_DUTY_REG"]
    pub ledc_hsch4_duty_reg: LEDC_HSCH4_DUTY_REG,
    #[doc = "0x5c - LEDC_HSCH4_CONF1_REG"]
    pub ledc_hsch4_conf1_reg: LEDC_HSCH4_CONF1_REG,
    #[doc = "0x60 - LEDC_HSCH4_DUTY_R_REG"]
    pub ledc_hsch4_duty_r_reg: LEDC_HSCH4_DUTY_R_REG,
    #[doc = "0x64 - LEDC_HSCH5_CONF0_REG"]
    pub ledc_hsch5_conf0_reg: LEDC_HSCH5_CONF0_REG,
    #[doc = "0x68 - LEDC_HSCH5_HPOINT_REG"]
    pub ledc_hsch5_hpoint_reg: LEDC_HSCH5_HPOINT_REG,
    #[doc = "0x6c - LEDC_HSCH5_DUTY_REG"]
    pub ledc_hsch5_duty_reg: LEDC_HSCH5_DUTY_REG,
    #[doc = "0x70 - LEDC_HSCH5_CONF1_REG"]
    pub ledc_hsch5_conf1_reg: LEDC_HSCH5_CONF1_REG,
    #[doc = "0x74 - LEDC_HSCH5_DUTY_R_REG"]
    pub ledc_hsch5_duty_r_reg: LEDC_HSCH5_DUTY_R_REG,
    #[doc = "0x78 - LEDC_HSCH6_CONF0_REG"]
    pub ledc_hsch6_conf0_reg: LEDC_HSCH6_CONF0_REG,
    #[doc = "0x7c - LEDC_HSCH6_HPOINT_REG"]
    pub ledc_hsch6_hpoint_reg: LEDC_HSCH6_HPOINT_REG,
    #[doc = "0x80 - LEDC_HSCH6_DUTY_REG"]
    pub ledc_hsch6_duty_reg: LEDC_HSCH6_DUTY_REG,
    #[doc = "0x84 - LEDC_HSCH6_CONF1_REG"]
    pub ledc_hsch6_conf1_reg: LEDC_HSCH6_CONF1_REG,
    #[doc = "0x88 - LEDC_HSCH6_DUTY_R_REG"]
    pub ledc_hsch6_duty_r_reg: LEDC_HSCH6_DUTY_R_REG,
    #[doc = "0x8c - LEDC_HSCH7_CONF0_REG"]
    pub ledc_hsch7_conf0_reg: LEDC_HSCH7_CONF0_REG,
    #[doc = "0x90 - LEDC_HSCH7_HPOINT_REG"]
    pub ledc_hsch7_hpoint_reg: LEDC_HSCH7_HPOINT_REG,
    #[doc = "0x94 - LEDC_HSCH7_DUTY_REG"]
    pub ledc_hsch7_duty_reg: LEDC_HSCH7_DUTY_REG,
    #[doc = "0x98 - LEDC_HSCH7_CONF1_REG"]
    pub ledc_hsch7_conf1_reg: LEDC_HSCH7_CONF1_REG,
    #[doc = "0x9c - LEDC_HSCH7_DUTY_R_REG"]
    pub ledc_hsch7_duty_r_reg: LEDC_HSCH7_DUTY_R_REG,
    #[doc = "0xa0 - LEDC_LSCH0_CONF0_REG"]
    pub ledc_lsch0_conf0_reg: LEDC_LSCH0_CONF0_REG,
    #[doc = "0xa4 - LEDC_LSCH0_HPOINT_REG"]
    pub ledc_lsch0_hpoint_reg: LEDC_LSCH0_HPOINT_REG,
    #[doc = "0xa8 - LEDC_LSCH0_DUTY_REG"]
    pub ledc_lsch0_duty_reg: LEDC_LSCH0_DUTY_REG,
    #[doc = "0xac - LEDC_LSCH0_CONF1_REG"]
    pub ledc_lsch0_conf1_reg: LEDC_LSCH0_CONF1_REG,
    #[doc = "0xb0 - LEDC_LSCH0_DUTY_R_REG"]
    pub ledc_lsch0_duty_r_reg: LEDC_LSCH0_DUTY_R_REG,
    #[doc = "0xb4 - LEDC_LSCH1_CONF0_REG"]
    pub ledc_lsch1_conf0_reg: LEDC_LSCH1_CONF0_REG,
    #[doc = "0xb8 - LEDC_LSCH1_HPOINT_REG"]
    pub ledc_lsch1_hpoint_reg: LEDC_LSCH1_HPOINT_REG,
    #[doc = "0xbc - LEDC_LSCH1_DUTY_REG"]
    pub ledc_lsch1_duty_reg: LEDC_LSCH1_DUTY_REG,
    #[doc = "0xc0 - LEDC_LSCH1_CONF1_REG"]
    pub ledc_lsch1_conf1_reg: LEDC_LSCH1_CONF1_REG,
    #[doc = "0xc4 - LEDC_LSCH1_DUTY_R_REG"]
    pub ledc_lsch1_duty_r_reg: LEDC_LSCH1_DUTY_R_REG,
    #[doc = "0xc8 - LEDC_LSCH2_CONF0_REG"]
    pub ledc_lsch2_conf0_reg: LEDC_LSCH2_CONF0_REG,
    #[doc = "0xcc - LEDC_LSCH2_HPOINT_REG"]
    pub ledc_lsch2_hpoint_reg: LEDC_LSCH2_HPOINT_REG,
    #[doc = "0xd0 - LEDC_LSCH2_DUTY_REG"]
    pub ledc_lsch2_duty_reg: LEDC_LSCH2_DUTY_REG,
    #[doc = "0xd4 - LEDC_LSCH2_CONF1_REG"]
    pub ledc_lsch2_conf1_reg: LEDC_LSCH2_CONF1_REG,
    #[doc = "0xd8 - LEDC_LSCH2_DUTY_R_REG"]
    pub ledc_lsch2_duty_r_reg: LEDC_LSCH2_DUTY_R_REG,
    #[doc = "0xdc - LEDC_LSCH3_CONF0_REG"]
    pub ledc_lsch3_conf0_reg: LEDC_LSCH3_CONF0_REG,
    #[doc = "0xe0 - LEDC_LSCH3_HPOINT_REG"]
    pub ledc_lsch3_hpoint_reg: LEDC_LSCH3_HPOINT_REG,
    #[doc = "0xe4 - LEDC_LSCH3_DUTY_REG"]
    pub ledc_lsch3_duty_reg: LEDC_LSCH3_DUTY_REG,
    #[doc = "0xe8 - LEDC_LSCH3_CONF1_REG"]
    pub ledc_lsch3_conf1_reg: LEDC_LSCH3_CONF1_REG,
    #[doc = "0xec - LEDC_LSCH3_DUTY_R_REG"]
    pub ledc_lsch3_duty_r_reg: LEDC_LSCH3_DUTY_R_REG,
    #[doc = "0xf0 - LEDC_LSCH4_CONF0_REG"]
    pub ledc_lsch4_conf0_reg: LEDC_LSCH4_CONF0_REG,
    #[doc = "0xf4 - LEDC_LSCH4_HPOINT_REG"]
    pub ledc_lsch4_hpoint_reg: LEDC_LSCH4_HPOINT_REG,
    #[doc = "0xf8 - LEDC_LSCH4_DUTY_REG"]
    pub ledc_lsch4_duty_reg: LEDC_LSCH4_DUTY_REG,
    #[doc = "0xfc - LEDC_LSCH4_CONF1_REG"]
    pub ledc_lsch4_conf1_reg: LEDC_LSCH4_CONF1_REG,
    #[doc = "0x100 - LEDC_LSCH4_DUTY_R_REG"]
    pub ledc_lsch4_duty_r_reg: LEDC_LSCH4_DUTY_R_REG,
    #[doc = "0x104 - LEDC_LSCH5_CONF0_REG"]
    pub ledc_lsch5_conf0_reg: LEDC_LSCH5_CONF0_REG,
    #[doc = "0x108 - LEDC_LSCH5_HPOINT_REG"]
    pub ledc_lsch5_hpoint_reg: LEDC_LSCH5_HPOINT_REG,
    #[doc = "0x10c - LEDC_LSCH5_DUTY_REG"]
    pub ledc_lsch5_duty_reg: LEDC_LSCH5_DUTY_REG,
    #[doc = "0x110 - LEDC_LSCH5_CONF1_REG"]
    pub ledc_lsch5_conf1_reg: LEDC_LSCH5_CONF1_REG,
    #[doc = "0x114 - LEDC_LSCH5_DUTY_R_REG"]
    pub ledc_lsch5_duty_r_reg: LEDC_LSCH5_DUTY_R_REG,
    #[doc = "0x118 - LEDC_LSCH6_CONF0_REG"]
    pub ledc_lsch6_conf0_reg: LEDC_LSCH6_CONF0_REG,
    #[doc = "0x11c - LEDC_LSCH6_HPOINT_REG"]
    pub ledc_lsch6_hpoint_reg: LEDC_LSCH6_HPOINT_REG,
    #[doc = "0x120 - LEDC_LSCH6_DUTY_REG"]
    pub ledc_lsch6_duty_reg: LEDC_LSCH6_DUTY_REG,
    #[doc = "0x124 - LEDC_LSCH6_CONF1_REG"]
    pub ledc_lsch6_conf1_reg: LEDC_LSCH6_CONF1_REG,
    #[doc = "0x128 - LEDC_LSCH6_DUTY_R_REG"]
    pub ledc_lsch6_duty_r_reg: LEDC_LSCH6_DUTY_R_REG,
    #[doc = "0x12c - LEDC_LSCH7_CONF0_REG"]
    pub ledc_lsch7_conf0_reg: LEDC_LSCH7_CONF0_REG,
    #[doc = "0x130 - LEDC_LSCH7_HPOINT_REG"]
    pub ledc_lsch7_hpoint_reg: LEDC_LSCH7_HPOINT_REG,
    #[doc = "0x134 - LEDC_LSCH7_DUTY_REG"]
    pub ledc_lsch7_duty_reg: LEDC_LSCH7_DUTY_REG,
    #[doc = "0x138 - LEDC_LSCH7_CONF1_REG"]
    pub ledc_lsch7_conf1_reg: LEDC_LSCH7_CONF1_REG,
    #[doc = "0x13c - LEDC_LSCH7_DUTY_R_REG"]
    pub ledc_lsch7_duty_r_reg: LEDC_LSCH7_DUTY_R_REG,
    #[doc = "0x140 - LEDC_HSTIMER0_CONF_REG"]
    pub ledc_hstimer0_conf_reg: LEDC_HSTIMER0_CONF_REG,
    #[doc = "0x144 - LEDC_HSTIMER0_VALUE_REG"]
    pub ledc_hstimer0_value_reg: LEDC_HSTIMER0_VALUE_REG,
    #[doc = "0x148 - LEDC_HSTIMER1_CONF_REG"]
    pub ledc_hstimer1_conf_reg: LEDC_HSTIMER1_CONF_REG,
    #[doc = "0x14c - LEDC_HSTIMER1_VALUE_REG"]
    pub ledc_hstimer1_value_reg: LEDC_HSTIMER1_VALUE_REG,
    #[doc = "0x150 - LEDC_HSTIMER2_CONF_REG"]
    pub ledc_hstimer2_conf_reg: LEDC_HSTIMER2_CONF_REG,
    #[doc = "0x154 - LEDC_HSTIMER2_VALUE_REG"]
    pub ledc_hstimer2_value_reg: LEDC_HSTIMER2_VALUE_REG,
    #[doc = "0x158 - LEDC_HSTIMER3_CONF_REG"]
    pub ledc_hstimer3_conf_reg: LEDC_HSTIMER3_CONF_REG,
    #[doc = "0x15c - LEDC_HSTIMER3_VALUE_REG"]
    pub ledc_hstimer3_value_reg: LEDC_HSTIMER3_VALUE_REG,
    #[doc = "0x160 - LEDC_LSTIMER0_CONF_REG"]
    pub ledc_lstimer0_conf_reg: LEDC_LSTIMER0_CONF_REG,
    #[doc = "0x164 - LEDC_LSTIMER0_VALUE_REG"]
    pub ledc_lstimer0_value_reg: LEDC_LSTIMER0_VALUE_REG,
    #[doc = "0x168 - LEDC_LSTIMER1_CONF_REG"]
    pub ledc_lstimer1_conf_reg: LEDC_LSTIMER1_CONF_REG,
    #[doc = "0x16c - LEDC_LSTIMER1_VALUE_REG"]
    pub ledc_lstimer1_value_reg: LEDC_LSTIMER1_VALUE_REG,
    #[doc = "0x170 - LEDC_LSTIMER2_CONF_REG"]
    pub ledc_lstimer2_conf_reg: LEDC_LSTIMER2_CONF_REG,
    #[doc = "0x174 - LEDC_LSTIMER2_VALUE_REG"]
    pub ledc_lstimer2_value_reg: LEDC_LSTIMER2_VALUE_REG,
    #[doc = "0x178 - LEDC_LSTIMER3_CONF_REG"]
    pub ledc_lstimer3_conf_reg: LEDC_LSTIMER3_CONF_REG,
    #[doc = "0x17c - LEDC_LSTIMER3_VALUE_REG"]
    pub ledc_lstimer3_value_reg: LEDC_LSTIMER3_VALUE_REG,
    #[doc = "0x180 - LEDC_INT_RAW_REG"]
    pub ledc_int_raw_reg: LEDC_INT_RAW_REG,
    #[doc = "0x184 - LEDC_INT_ST_REG"]
    pub ledc_int_st_reg: LEDC_INT_ST_REG,
    #[doc = "0x188 - LEDC_INT_ENA_REG"]
    pub ledc_int_ena_reg: LEDC_INT_ENA_REG,
    #[doc = "0x18c - LEDC_INT_CLR_REG"]
    pub ledc_int_clr_reg: LEDC_INT_CLR_REG,
    #[doc = "0x190 - LEDC_CONF_REG"]
    pub ledc_conf_reg: LEDC_CONF_REG,
    _reserved101: [u8; 104usize],
    #[doc = "0x1fc - LEDC_DATE_REG"]
    pub ledc_date_reg: LEDC_DATE_REG,
}
#[doc = "LEDC_HSCH0_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch0_conf0_reg](ledc_hsch0_conf0_reg) module"]
pub type LEDC_HSCH0_CONF0_REG = crate::Reg<u32, _LEDC_HSCH0_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH0_CONF0_REG;
#[doc = "`read()` method returns [ledc_hsch0_conf0_reg::R](ledc_hsch0_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH0_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch0_conf0_reg::W](ledc_hsch0_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH0_CONF0_REG {}
#[doc = "LEDC_HSCH0_CONF0_REG"]
pub mod ledc_hsch0_conf0_reg;
#[doc = "LEDC_HSCH0_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch0_hpoint_reg](ledc_hsch0_hpoint_reg) module"]
pub type LEDC_HSCH0_HPOINT_REG = crate::Reg<u32, _LEDC_HSCH0_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH0_HPOINT_REG;
#[doc = "`read()` method returns [ledc_hsch0_hpoint_reg::R](ledc_hsch0_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH0_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch0_hpoint_reg::W](ledc_hsch0_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH0_HPOINT_REG {}
#[doc = "LEDC_HSCH0_HPOINT_REG"]
pub mod ledc_hsch0_hpoint_reg;
#[doc = "LEDC_HSCH0_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch0_duty_reg](ledc_hsch0_duty_reg) module"]
pub type LEDC_HSCH0_DUTY_REG = crate::Reg<u32, _LEDC_HSCH0_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH0_DUTY_REG;
#[doc = "`read()` method returns [ledc_hsch0_duty_reg::R](ledc_hsch0_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH0_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch0_duty_reg::W](ledc_hsch0_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH0_DUTY_REG {}
#[doc = "LEDC_HSCH0_DUTY_REG"]
pub mod ledc_hsch0_duty_reg;
#[doc = "LEDC_HSCH0_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch0_conf1_reg](ledc_hsch0_conf1_reg) module"]
pub type LEDC_HSCH0_CONF1_REG = crate::Reg<u32, _LEDC_HSCH0_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH0_CONF1_REG;
#[doc = "`read()` method returns [ledc_hsch0_conf1_reg::R](ledc_hsch0_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH0_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch0_conf1_reg::W](ledc_hsch0_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH0_CONF1_REG {}
#[doc = "LEDC_HSCH0_CONF1_REG"]
pub mod ledc_hsch0_conf1_reg;
#[doc = "LEDC_HSCH0_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch0_duty_r_reg](ledc_hsch0_duty_r_reg) module"]
pub type LEDC_HSCH0_DUTY_R_REG = crate::Reg<u32, _LEDC_HSCH0_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH0_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_hsch0_duty_r_reg::R](ledc_hsch0_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH0_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch0_duty_r_reg::W](ledc_hsch0_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH0_DUTY_R_REG {}
#[doc = "LEDC_HSCH0_DUTY_R_REG"]
pub mod ledc_hsch0_duty_r_reg;
#[doc = "LEDC_HSCH1_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch1_conf0_reg](ledc_hsch1_conf0_reg) module"]
pub type LEDC_HSCH1_CONF0_REG = crate::Reg<u32, _LEDC_HSCH1_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH1_CONF0_REG;
#[doc = "`read()` method returns [ledc_hsch1_conf0_reg::R](ledc_hsch1_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH1_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch1_conf0_reg::W](ledc_hsch1_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH1_CONF0_REG {}
#[doc = "LEDC_HSCH1_CONF0_REG"]
pub mod ledc_hsch1_conf0_reg;
#[doc = "LEDC_HSCH1_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch1_hpoint_reg](ledc_hsch1_hpoint_reg) module"]
pub type LEDC_HSCH1_HPOINT_REG = crate::Reg<u32, _LEDC_HSCH1_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH1_HPOINT_REG;
#[doc = "`read()` method returns [ledc_hsch1_hpoint_reg::R](ledc_hsch1_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH1_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch1_hpoint_reg::W](ledc_hsch1_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH1_HPOINT_REG {}
#[doc = "LEDC_HSCH1_HPOINT_REG"]
pub mod ledc_hsch1_hpoint_reg;
#[doc = "LEDC_HSCH1_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch1_duty_reg](ledc_hsch1_duty_reg) module"]
pub type LEDC_HSCH1_DUTY_REG = crate::Reg<u32, _LEDC_HSCH1_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH1_DUTY_REG;
#[doc = "`read()` method returns [ledc_hsch1_duty_reg::R](ledc_hsch1_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH1_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch1_duty_reg::W](ledc_hsch1_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH1_DUTY_REG {}
#[doc = "LEDC_HSCH1_DUTY_REG"]
pub mod ledc_hsch1_duty_reg;
#[doc = "LEDC_HSCH1_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch1_conf1_reg](ledc_hsch1_conf1_reg) module"]
pub type LEDC_HSCH1_CONF1_REG = crate::Reg<u32, _LEDC_HSCH1_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH1_CONF1_REG;
#[doc = "`read()` method returns [ledc_hsch1_conf1_reg::R](ledc_hsch1_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH1_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch1_conf1_reg::W](ledc_hsch1_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH1_CONF1_REG {}
#[doc = "LEDC_HSCH1_CONF1_REG"]
pub mod ledc_hsch1_conf1_reg;
#[doc = "LEDC_HSCH1_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch1_duty_r_reg](ledc_hsch1_duty_r_reg) module"]
pub type LEDC_HSCH1_DUTY_R_REG = crate::Reg<u32, _LEDC_HSCH1_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH1_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_hsch1_duty_r_reg::R](ledc_hsch1_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH1_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch1_duty_r_reg::W](ledc_hsch1_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH1_DUTY_R_REG {}
#[doc = "LEDC_HSCH1_DUTY_R_REG"]
pub mod ledc_hsch1_duty_r_reg;
#[doc = "LEDC_HSCH2_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch2_conf0_reg](ledc_hsch2_conf0_reg) module"]
pub type LEDC_HSCH2_CONF0_REG = crate::Reg<u32, _LEDC_HSCH2_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH2_CONF0_REG;
#[doc = "`read()` method returns [ledc_hsch2_conf0_reg::R](ledc_hsch2_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH2_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch2_conf0_reg::W](ledc_hsch2_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH2_CONF0_REG {}
#[doc = "LEDC_HSCH2_CONF0_REG"]
pub mod ledc_hsch2_conf0_reg;
#[doc = "LEDC_HSCH2_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch2_hpoint_reg](ledc_hsch2_hpoint_reg) module"]
pub type LEDC_HSCH2_HPOINT_REG = crate::Reg<u32, _LEDC_HSCH2_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH2_HPOINT_REG;
#[doc = "`read()` method returns [ledc_hsch2_hpoint_reg::R](ledc_hsch2_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH2_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch2_hpoint_reg::W](ledc_hsch2_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH2_HPOINT_REG {}
#[doc = "LEDC_HSCH2_HPOINT_REG"]
pub mod ledc_hsch2_hpoint_reg;
#[doc = "LEDC_HSCH2_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch2_duty_reg](ledc_hsch2_duty_reg) module"]
pub type LEDC_HSCH2_DUTY_REG = crate::Reg<u32, _LEDC_HSCH2_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH2_DUTY_REG;
#[doc = "`read()` method returns [ledc_hsch2_duty_reg::R](ledc_hsch2_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH2_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch2_duty_reg::W](ledc_hsch2_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH2_DUTY_REG {}
#[doc = "LEDC_HSCH2_DUTY_REG"]
pub mod ledc_hsch2_duty_reg;
#[doc = "LEDC_HSCH2_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch2_conf1_reg](ledc_hsch2_conf1_reg) module"]
pub type LEDC_HSCH2_CONF1_REG = crate::Reg<u32, _LEDC_HSCH2_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH2_CONF1_REG;
#[doc = "`read()` method returns [ledc_hsch2_conf1_reg::R](ledc_hsch2_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH2_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch2_conf1_reg::W](ledc_hsch2_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH2_CONF1_REG {}
#[doc = "LEDC_HSCH2_CONF1_REG"]
pub mod ledc_hsch2_conf1_reg;
#[doc = "LEDC_HSCH2_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch2_duty_r_reg](ledc_hsch2_duty_r_reg) module"]
pub type LEDC_HSCH2_DUTY_R_REG = crate::Reg<u32, _LEDC_HSCH2_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH2_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_hsch2_duty_r_reg::R](ledc_hsch2_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH2_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch2_duty_r_reg::W](ledc_hsch2_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH2_DUTY_R_REG {}
#[doc = "LEDC_HSCH2_DUTY_R_REG"]
pub mod ledc_hsch2_duty_r_reg;
#[doc = "LEDC_HSCH3_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch3_conf0_reg](ledc_hsch3_conf0_reg) module"]
pub type LEDC_HSCH3_CONF0_REG = crate::Reg<u32, _LEDC_HSCH3_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH3_CONF0_REG;
#[doc = "`read()` method returns [ledc_hsch3_conf0_reg::R](ledc_hsch3_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH3_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch3_conf0_reg::W](ledc_hsch3_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH3_CONF0_REG {}
#[doc = "LEDC_HSCH3_CONF0_REG"]
pub mod ledc_hsch3_conf0_reg;
#[doc = "LEDC_HSCH3_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch3_hpoint_reg](ledc_hsch3_hpoint_reg) module"]
pub type LEDC_HSCH3_HPOINT_REG = crate::Reg<u32, _LEDC_HSCH3_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH3_HPOINT_REG;
#[doc = "`read()` method returns [ledc_hsch3_hpoint_reg::R](ledc_hsch3_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH3_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch3_hpoint_reg::W](ledc_hsch3_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH3_HPOINT_REG {}
#[doc = "LEDC_HSCH3_HPOINT_REG"]
pub mod ledc_hsch3_hpoint_reg;
#[doc = "LEDC_HSCH3_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch3_duty_reg](ledc_hsch3_duty_reg) module"]
pub type LEDC_HSCH3_DUTY_REG = crate::Reg<u32, _LEDC_HSCH3_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH3_DUTY_REG;
#[doc = "`read()` method returns [ledc_hsch3_duty_reg::R](ledc_hsch3_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH3_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch3_duty_reg::W](ledc_hsch3_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH3_DUTY_REG {}
#[doc = "LEDC_HSCH3_DUTY_REG"]
pub mod ledc_hsch3_duty_reg;
#[doc = "LEDC_HSCH3_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch3_conf1_reg](ledc_hsch3_conf1_reg) module"]
pub type LEDC_HSCH3_CONF1_REG = crate::Reg<u32, _LEDC_HSCH3_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH3_CONF1_REG;
#[doc = "`read()` method returns [ledc_hsch3_conf1_reg::R](ledc_hsch3_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH3_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch3_conf1_reg::W](ledc_hsch3_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH3_CONF1_REG {}
#[doc = "LEDC_HSCH3_CONF1_REG"]
pub mod ledc_hsch3_conf1_reg;
#[doc = "LEDC_HSCH3_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch3_duty_r_reg](ledc_hsch3_duty_r_reg) module"]
pub type LEDC_HSCH3_DUTY_R_REG = crate::Reg<u32, _LEDC_HSCH3_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH3_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_hsch3_duty_r_reg::R](ledc_hsch3_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH3_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch3_duty_r_reg::W](ledc_hsch3_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH3_DUTY_R_REG {}
#[doc = "LEDC_HSCH3_DUTY_R_REG"]
pub mod ledc_hsch3_duty_r_reg;
#[doc = "LEDC_HSCH4_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch4_conf0_reg](ledc_hsch4_conf0_reg) module"]
pub type LEDC_HSCH4_CONF0_REG = crate::Reg<u32, _LEDC_HSCH4_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH4_CONF0_REG;
#[doc = "`read()` method returns [ledc_hsch4_conf0_reg::R](ledc_hsch4_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH4_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch4_conf0_reg::W](ledc_hsch4_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH4_CONF0_REG {}
#[doc = "LEDC_HSCH4_CONF0_REG"]
pub mod ledc_hsch4_conf0_reg;
#[doc = "LEDC_HSCH4_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch4_hpoint_reg](ledc_hsch4_hpoint_reg) module"]
pub type LEDC_HSCH4_HPOINT_REG = crate::Reg<u32, _LEDC_HSCH4_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH4_HPOINT_REG;
#[doc = "`read()` method returns [ledc_hsch4_hpoint_reg::R](ledc_hsch4_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH4_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch4_hpoint_reg::W](ledc_hsch4_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH4_HPOINT_REG {}
#[doc = "LEDC_HSCH4_HPOINT_REG"]
pub mod ledc_hsch4_hpoint_reg;
#[doc = "LEDC_HSCH4_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch4_duty_reg](ledc_hsch4_duty_reg) module"]
pub type LEDC_HSCH4_DUTY_REG = crate::Reg<u32, _LEDC_HSCH4_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH4_DUTY_REG;
#[doc = "`read()` method returns [ledc_hsch4_duty_reg::R](ledc_hsch4_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH4_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch4_duty_reg::W](ledc_hsch4_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH4_DUTY_REG {}
#[doc = "LEDC_HSCH4_DUTY_REG"]
pub mod ledc_hsch4_duty_reg;
#[doc = "LEDC_HSCH4_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch4_conf1_reg](ledc_hsch4_conf1_reg) module"]
pub type LEDC_HSCH4_CONF1_REG = crate::Reg<u32, _LEDC_HSCH4_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH4_CONF1_REG;
#[doc = "`read()` method returns [ledc_hsch4_conf1_reg::R](ledc_hsch4_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH4_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch4_conf1_reg::W](ledc_hsch4_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH4_CONF1_REG {}
#[doc = "LEDC_HSCH4_CONF1_REG"]
pub mod ledc_hsch4_conf1_reg;
#[doc = "LEDC_HSCH4_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch4_duty_r_reg](ledc_hsch4_duty_r_reg) module"]
pub type LEDC_HSCH4_DUTY_R_REG = crate::Reg<u32, _LEDC_HSCH4_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH4_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_hsch4_duty_r_reg::R](ledc_hsch4_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH4_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch4_duty_r_reg::W](ledc_hsch4_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH4_DUTY_R_REG {}
#[doc = "LEDC_HSCH4_DUTY_R_REG"]
pub mod ledc_hsch4_duty_r_reg;
#[doc = "LEDC_HSCH5_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch5_conf0_reg](ledc_hsch5_conf0_reg) module"]
pub type LEDC_HSCH5_CONF0_REG = crate::Reg<u32, _LEDC_HSCH5_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH5_CONF0_REG;
#[doc = "`read()` method returns [ledc_hsch5_conf0_reg::R](ledc_hsch5_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH5_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch5_conf0_reg::W](ledc_hsch5_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH5_CONF0_REG {}
#[doc = "LEDC_HSCH5_CONF0_REG"]
pub mod ledc_hsch5_conf0_reg;
#[doc = "LEDC_HSCH5_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch5_hpoint_reg](ledc_hsch5_hpoint_reg) module"]
pub type LEDC_HSCH5_HPOINT_REG = crate::Reg<u32, _LEDC_HSCH5_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH5_HPOINT_REG;
#[doc = "`read()` method returns [ledc_hsch5_hpoint_reg::R](ledc_hsch5_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH5_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch5_hpoint_reg::W](ledc_hsch5_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH5_HPOINT_REG {}
#[doc = "LEDC_HSCH5_HPOINT_REG"]
pub mod ledc_hsch5_hpoint_reg;
#[doc = "LEDC_HSCH5_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch5_duty_reg](ledc_hsch5_duty_reg) module"]
pub type LEDC_HSCH5_DUTY_REG = crate::Reg<u32, _LEDC_HSCH5_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH5_DUTY_REG;
#[doc = "`read()` method returns [ledc_hsch5_duty_reg::R](ledc_hsch5_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH5_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch5_duty_reg::W](ledc_hsch5_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH5_DUTY_REG {}
#[doc = "LEDC_HSCH5_DUTY_REG"]
pub mod ledc_hsch5_duty_reg;
#[doc = "LEDC_HSCH5_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch5_conf1_reg](ledc_hsch5_conf1_reg) module"]
pub type LEDC_HSCH5_CONF1_REG = crate::Reg<u32, _LEDC_HSCH5_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH5_CONF1_REG;
#[doc = "`read()` method returns [ledc_hsch5_conf1_reg::R](ledc_hsch5_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH5_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch5_conf1_reg::W](ledc_hsch5_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH5_CONF1_REG {}
#[doc = "LEDC_HSCH5_CONF1_REG"]
pub mod ledc_hsch5_conf1_reg;
#[doc = "LEDC_HSCH5_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch5_duty_r_reg](ledc_hsch5_duty_r_reg) module"]
pub type LEDC_HSCH5_DUTY_R_REG = crate::Reg<u32, _LEDC_HSCH5_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH5_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_hsch5_duty_r_reg::R](ledc_hsch5_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH5_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch5_duty_r_reg::W](ledc_hsch5_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH5_DUTY_R_REG {}
#[doc = "LEDC_HSCH5_DUTY_R_REG"]
pub mod ledc_hsch5_duty_r_reg;
#[doc = "LEDC_HSCH6_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch6_conf0_reg](ledc_hsch6_conf0_reg) module"]
pub type LEDC_HSCH6_CONF0_REG = crate::Reg<u32, _LEDC_HSCH6_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH6_CONF0_REG;
#[doc = "`read()` method returns [ledc_hsch6_conf0_reg::R](ledc_hsch6_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH6_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch6_conf0_reg::W](ledc_hsch6_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH6_CONF0_REG {}
#[doc = "LEDC_HSCH6_CONF0_REG"]
pub mod ledc_hsch6_conf0_reg;
#[doc = "LEDC_HSCH6_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch6_hpoint_reg](ledc_hsch6_hpoint_reg) module"]
pub type LEDC_HSCH6_HPOINT_REG = crate::Reg<u32, _LEDC_HSCH6_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH6_HPOINT_REG;
#[doc = "`read()` method returns [ledc_hsch6_hpoint_reg::R](ledc_hsch6_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH6_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch6_hpoint_reg::W](ledc_hsch6_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH6_HPOINT_REG {}
#[doc = "LEDC_HSCH6_HPOINT_REG"]
pub mod ledc_hsch6_hpoint_reg;
#[doc = "LEDC_HSCH6_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch6_duty_reg](ledc_hsch6_duty_reg) module"]
pub type LEDC_HSCH6_DUTY_REG = crate::Reg<u32, _LEDC_HSCH6_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH6_DUTY_REG;
#[doc = "`read()` method returns [ledc_hsch6_duty_reg::R](ledc_hsch6_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH6_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch6_duty_reg::W](ledc_hsch6_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH6_DUTY_REG {}
#[doc = "LEDC_HSCH6_DUTY_REG"]
pub mod ledc_hsch6_duty_reg;
#[doc = "LEDC_HSCH6_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch6_conf1_reg](ledc_hsch6_conf1_reg) module"]
pub type LEDC_HSCH6_CONF1_REG = crate::Reg<u32, _LEDC_HSCH6_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH6_CONF1_REG;
#[doc = "`read()` method returns [ledc_hsch6_conf1_reg::R](ledc_hsch6_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH6_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch6_conf1_reg::W](ledc_hsch6_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH6_CONF1_REG {}
#[doc = "LEDC_HSCH6_CONF1_REG"]
pub mod ledc_hsch6_conf1_reg;
#[doc = "LEDC_HSCH6_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch6_duty_r_reg](ledc_hsch6_duty_r_reg) module"]
pub type LEDC_HSCH6_DUTY_R_REG = crate::Reg<u32, _LEDC_HSCH6_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH6_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_hsch6_duty_r_reg::R](ledc_hsch6_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH6_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch6_duty_r_reg::W](ledc_hsch6_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH6_DUTY_R_REG {}
#[doc = "LEDC_HSCH6_DUTY_R_REG"]
pub mod ledc_hsch6_duty_r_reg;
#[doc = "LEDC_HSCH7_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch7_conf0_reg](ledc_hsch7_conf0_reg) module"]
pub type LEDC_HSCH7_CONF0_REG = crate::Reg<u32, _LEDC_HSCH7_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH7_CONF0_REG;
#[doc = "`read()` method returns [ledc_hsch7_conf0_reg::R](ledc_hsch7_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH7_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch7_conf0_reg::W](ledc_hsch7_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH7_CONF0_REG {}
#[doc = "LEDC_HSCH7_CONF0_REG"]
pub mod ledc_hsch7_conf0_reg;
#[doc = "LEDC_HSCH7_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch7_hpoint_reg](ledc_hsch7_hpoint_reg) module"]
pub type LEDC_HSCH7_HPOINT_REG = crate::Reg<u32, _LEDC_HSCH7_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH7_HPOINT_REG;
#[doc = "`read()` method returns [ledc_hsch7_hpoint_reg::R](ledc_hsch7_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH7_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch7_hpoint_reg::W](ledc_hsch7_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH7_HPOINT_REG {}
#[doc = "LEDC_HSCH7_HPOINT_REG"]
pub mod ledc_hsch7_hpoint_reg;
#[doc = "LEDC_HSCH7_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch7_duty_reg](ledc_hsch7_duty_reg) module"]
pub type LEDC_HSCH7_DUTY_REG = crate::Reg<u32, _LEDC_HSCH7_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH7_DUTY_REG;
#[doc = "`read()` method returns [ledc_hsch7_duty_reg::R](ledc_hsch7_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH7_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch7_duty_reg::W](ledc_hsch7_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH7_DUTY_REG {}
#[doc = "LEDC_HSCH7_DUTY_REG"]
pub mod ledc_hsch7_duty_reg;
#[doc = "LEDC_HSCH7_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch7_conf1_reg](ledc_hsch7_conf1_reg) module"]
pub type LEDC_HSCH7_CONF1_REG = crate::Reg<u32, _LEDC_HSCH7_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH7_CONF1_REG;
#[doc = "`read()` method returns [ledc_hsch7_conf1_reg::R](ledc_hsch7_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH7_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch7_conf1_reg::W](ledc_hsch7_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH7_CONF1_REG {}
#[doc = "LEDC_HSCH7_CONF1_REG"]
pub mod ledc_hsch7_conf1_reg;
#[doc = "LEDC_HSCH7_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hsch7_duty_r_reg](ledc_hsch7_duty_r_reg) module"]
pub type LEDC_HSCH7_DUTY_R_REG = crate::Reg<u32, _LEDC_HSCH7_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSCH7_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_hsch7_duty_r_reg::R](ledc_hsch7_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_HSCH7_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hsch7_duty_r_reg::W](ledc_hsch7_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_HSCH7_DUTY_R_REG {}
#[doc = "LEDC_HSCH7_DUTY_R_REG"]
pub mod ledc_hsch7_duty_r_reg;
#[doc = "LEDC_LSCH0_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch0_conf0_reg](ledc_lsch0_conf0_reg) module"]
pub type LEDC_LSCH0_CONF0_REG = crate::Reg<u32, _LEDC_LSCH0_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_CONF0_REG;
#[doc = "`read()` method returns [ledc_lsch0_conf0_reg::R](ledc_lsch0_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_conf0_reg::W](ledc_lsch0_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_CONF0_REG {}
#[doc = "LEDC_LSCH0_CONF0_REG"]
pub mod ledc_lsch0_conf0_reg;
#[doc = "LEDC_LSCH0_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch0_hpoint_reg](ledc_lsch0_hpoint_reg) module"]
pub type LEDC_LSCH0_HPOINT_REG = crate::Reg<u32, _LEDC_LSCH0_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_HPOINT_REG;
#[doc = "`read()` method returns [ledc_lsch0_hpoint_reg::R](ledc_lsch0_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_hpoint_reg::W](ledc_lsch0_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_HPOINT_REG {}
#[doc = "LEDC_LSCH0_HPOINT_REG"]
pub mod ledc_lsch0_hpoint_reg;
#[doc = "LEDC_LSCH0_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch0_duty_reg](ledc_lsch0_duty_reg) module"]
pub type LEDC_LSCH0_DUTY_REG = crate::Reg<u32, _LEDC_LSCH0_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_DUTY_REG;
#[doc = "`read()` method returns [ledc_lsch0_duty_reg::R](ledc_lsch0_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_duty_reg::W](ledc_lsch0_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_DUTY_REG {}
#[doc = "LEDC_LSCH0_DUTY_REG"]
pub mod ledc_lsch0_duty_reg;
#[doc = "LEDC_LSCH0_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch0_conf1_reg](ledc_lsch0_conf1_reg) module"]
pub type LEDC_LSCH0_CONF1_REG = crate::Reg<u32, _LEDC_LSCH0_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_CONF1_REG;
#[doc = "`read()` method returns [ledc_lsch0_conf1_reg::R](ledc_lsch0_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_conf1_reg::W](ledc_lsch0_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_CONF1_REG {}
#[doc = "LEDC_LSCH0_CONF1_REG"]
pub mod ledc_lsch0_conf1_reg;
#[doc = "LEDC_LSCH0_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch0_duty_r_reg](ledc_lsch0_duty_r_reg) module"]
pub type LEDC_LSCH0_DUTY_R_REG = crate::Reg<u32, _LEDC_LSCH0_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_lsch0_duty_r_reg::R](ledc_lsch0_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_duty_r_reg::W](ledc_lsch0_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_DUTY_R_REG {}
#[doc = "LEDC_LSCH0_DUTY_R_REG"]
pub mod ledc_lsch0_duty_r_reg;
#[doc = "LEDC_LSCH1_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch1_conf0_reg](ledc_lsch1_conf0_reg) module"]
pub type LEDC_LSCH1_CONF0_REG = crate::Reg<u32, _LEDC_LSCH1_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_CONF0_REG;
#[doc = "`read()` method returns [ledc_lsch1_conf0_reg::R](ledc_lsch1_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_conf0_reg::W](ledc_lsch1_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_CONF0_REG {}
#[doc = "LEDC_LSCH1_CONF0_REG"]
pub mod ledc_lsch1_conf0_reg;
#[doc = "LEDC_LSCH1_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch1_hpoint_reg](ledc_lsch1_hpoint_reg) module"]
pub type LEDC_LSCH1_HPOINT_REG = crate::Reg<u32, _LEDC_LSCH1_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_HPOINT_REG;
#[doc = "`read()` method returns [ledc_lsch1_hpoint_reg::R](ledc_lsch1_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_hpoint_reg::W](ledc_lsch1_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_HPOINT_REG {}
#[doc = "LEDC_LSCH1_HPOINT_REG"]
pub mod ledc_lsch1_hpoint_reg;
#[doc = "LEDC_LSCH1_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch1_duty_reg](ledc_lsch1_duty_reg) module"]
pub type LEDC_LSCH1_DUTY_REG = crate::Reg<u32, _LEDC_LSCH1_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_DUTY_REG;
#[doc = "`read()` method returns [ledc_lsch1_duty_reg::R](ledc_lsch1_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_duty_reg::W](ledc_lsch1_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_DUTY_REG {}
#[doc = "LEDC_LSCH1_DUTY_REG"]
pub mod ledc_lsch1_duty_reg;
#[doc = "LEDC_LSCH1_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch1_conf1_reg](ledc_lsch1_conf1_reg) module"]
pub type LEDC_LSCH1_CONF1_REG = crate::Reg<u32, _LEDC_LSCH1_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_CONF1_REG;
#[doc = "`read()` method returns [ledc_lsch1_conf1_reg::R](ledc_lsch1_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_conf1_reg::W](ledc_lsch1_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_CONF1_REG {}
#[doc = "LEDC_LSCH1_CONF1_REG"]
pub mod ledc_lsch1_conf1_reg;
#[doc = "LEDC_LSCH1_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch1_duty_r_reg](ledc_lsch1_duty_r_reg) module"]
pub type LEDC_LSCH1_DUTY_R_REG = crate::Reg<u32, _LEDC_LSCH1_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_lsch1_duty_r_reg::R](ledc_lsch1_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_duty_r_reg::W](ledc_lsch1_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_DUTY_R_REG {}
#[doc = "LEDC_LSCH1_DUTY_R_REG"]
pub mod ledc_lsch1_duty_r_reg;
#[doc = "LEDC_LSCH2_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch2_conf0_reg](ledc_lsch2_conf0_reg) module"]
pub type LEDC_LSCH2_CONF0_REG = crate::Reg<u32, _LEDC_LSCH2_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_CONF0_REG;
#[doc = "`read()` method returns [ledc_lsch2_conf0_reg::R](ledc_lsch2_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_conf0_reg::W](ledc_lsch2_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_CONF0_REG {}
#[doc = "LEDC_LSCH2_CONF0_REG"]
pub mod ledc_lsch2_conf0_reg;
#[doc = "LEDC_LSCH2_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch2_hpoint_reg](ledc_lsch2_hpoint_reg) module"]
pub type LEDC_LSCH2_HPOINT_REG = crate::Reg<u32, _LEDC_LSCH2_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_HPOINT_REG;
#[doc = "`read()` method returns [ledc_lsch2_hpoint_reg::R](ledc_lsch2_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_hpoint_reg::W](ledc_lsch2_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_HPOINT_REG {}
#[doc = "LEDC_LSCH2_HPOINT_REG"]
pub mod ledc_lsch2_hpoint_reg;
#[doc = "LEDC_LSCH2_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch2_duty_reg](ledc_lsch2_duty_reg) module"]
pub type LEDC_LSCH2_DUTY_REG = crate::Reg<u32, _LEDC_LSCH2_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_DUTY_REG;
#[doc = "`read()` method returns [ledc_lsch2_duty_reg::R](ledc_lsch2_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_duty_reg::W](ledc_lsch2_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_DUTY_REG {}
#[doc = "LEDC_LSCH2_DUTY_REG"]
pub mod ledc_lsch2_duty_reg;
#[doc = "LEDC_LSCH2_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch2_conf1_reg](ledc_lsch2_conf1_reg) module"]
pub type LEDC_LSCH2_CONF1_REG = crate::Reg<u32, _LEDC_LSCH2_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_CONF1_REG;
#[doc = "`read()` method returns [ledc_lsch2_conf1_reg::R](ledc_lsch2_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_conf1_reg::W](ledc_lsch2_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_CONF1_REG {}
#[doc = "LEDC_LSCH2_CONF1_REG"]
pub mod ledc_lsch2_conf1_reg;
#[doc = "LEDC_LSCH2_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch2_duty_r_reg](ledc_lsch2_duty_r_reg) module"]
pub type LEDC_LSCH2_DUTY_R_REG = crate::Reg<u32, _LEDC_LSCH2_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_lsch2_duty_r_reg::R](ledc_lsch2_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_duty_r_reg::W](ledc_lsch2_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_DUTY_R_REG {}
#[doc = "LEDC_LSCH2_DUTY_R_REG"]
pub mod ledc_lsch2_duty_r_reg;
#[doc = "LEDC_LSCH3_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch3_conf0_reg](ledc_lsch3_conf0_reg) module"]
pub type LEDC_LSCH3_CONF0_REG = crate::Reg<u32, _LEDC_LSCH3_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_CONF0_REG;
#[doc = "`read()` method returns [ledc_lsch3_conf0_reg::R](ledc_lsch3_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_conf0_reg::W](ledc_lsch3_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_CONF0_REG {}
#[doc = "LEDC_LSCH3_CONF0_REG"]
pub mod ledc_lsch3_conf0_reg;
#[doc = "LEDC_LSCH3_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch3_hpoint_reg](ledc_lsch3_hpoint_reg) module"]
pub type LEDC_LSCH3_HPOINT_REG = crate::Reg<u32, _LEDC_LSCH3_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_HPOINT_REG;
#[doc = "`read()` method returns [ledc_lsch3_hpoint_reg::R](ledc_lsch3_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_hpoint_reg::W](ledc_lsch3_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_HPOINT_REG {}
#[doc = "LEDC_LSCH3_HPOINT_REG"]
pub mod ledc_lsch3_hpoint_reg;
#[doc = "LEDC_LSCH3_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch3_duty_reg](ledc_lsch3_duty_reg) module"]
pub type LEDC_LSCH3_DUTY_REG = crate::Reg<u32, _LEDC_LSCH3_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_DUTY_REG;
#[doc = "`read()` method returns [ledc_lsch3_duty_reg::R](ledc_lsch3_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_duty_reg::W](ledc_lsch3_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_DUTY_REG {}
#[doc = "LEDC_LSCH3_DUTY_REG"]
pub mod ledc_lsch3_duty_reg;
#[doc = "LEDC_LSCH3_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch3_conf1_reg](ledc_lsch3_conf1_reg) module"]
pub type LEDC_LSCH3_CONF1_REG = crate::Reg<u32, _LEDC_LSCH3_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_CONF1_REG;
#[doc = "`read()` method returns [ledc_lsch3_conf1_reg::R](ledc_lsch3_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_conf1_reg::W](ledc_lsch3_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_CONF1_REG {}
#[doc = "LEDC_LSCH3_CONF1_REG"]
pub mod ledc_lsch3_conf1_reg;
#[doc = "LEDC_LSCH3_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch3_duty_r_reg](ledc_lsch3_duty_r_reg) module"]
pub type LEDC_LSCH3_DUTY_R_REG = crate::Reg<u32, _LEDC_LSCH3_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_lsch3_duty_r_reg::R](ledc_lsch3_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_duty_r_reg::W](ledc_lsch3_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_DUTY_R_REG {}
#[doc = "LEDC_LSCH3_DUTY_R_REG"]
pub mod ledc_lsch3_duty_r_reg;
#[doc = "LEDC_LSCH4_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch4_conf0_reg](ledc_lsch4_conf0_reg) module"]
pub type LEDC_LSCH4_CONF0_REG = crate::Reg<u32, _LEDC_LSCH4_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_CONF0_REG;
#[doc = "`read()` method returns [ledc_lsch4_conf0_reg::R](ledc_lsch4_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_conf0_reg::W](ledc_lsch4_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_CONF0_REG {}
#[doc = "LEDC_LSCH4_CONF0_REG"]
pub mod ledc_lsch4_conf0_reg;
#[doc = "LEDC_LSCH4_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch4_hpoint_reg](ledc_lsch4_hpoint_reg) module"]
pub type LEDC_LSCH4_HPOINT_REG = crate::Reg<u32, _LEDC_LSCH4_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_HPOINT_REG;
#[doc = "`read()` method returns [ledc_lsch4_hpoint_reg::R](ledc_lsch4_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_hpoint_reg::W](ledc_lsch4_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_HPOINT_REG {}
#[doc = "LEDC_LSCH4_HPOINT_REG"]
pub mod ledc_lsch4_hpoint_reg;
#[doc = "LEDC_LSCH4_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch4_duty_reg](ledc_lsch4_duty_reg) module"]
pub type LEDC_LSCH4_DUTY_REG = crate::Reg<u32, _LEDC_LSCH4_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_DUTY_REG;
#[doc = "`read()` method returns [ledc_lsch4_duty_reg::R](ledc_lsch4_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_duty_reg::W](ledc_lsch4_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_DUTY_REG {}
#[doc = "LEDC_LSCH4_DUTY_REG"]
pub mod ledc_lsch4_duty_reg;
#[doc = "LEDC_LSCH4_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch4_conf1_reg](ledc_lsch4_conf1_reg) module"]
pub type LEDC_LSCH4_CONF1_REG = crate::Reg<u32, _LEDC_LSCH4_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_CONF1_REG;
#[doc = "`read()` method returns [ledc_lsch4_conf1_reg::R](ledc_lsch4_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_conf1_reg::W](ledc_lsch4_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_CONF1_REG {}
#[doc = "LEDC_LSCH4_CONF1_REG"]
pub mod ledc_lsch4_conf1_reg;
#[doc = "LEDC_LSCH4_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch4_duty_r_reg](ledc_lsch4_duty_r_reg) module"]
pub type LEDC_LSCH4_DUTY_R_REG = crate::Reg<u32, _LEDC_LSCH4_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_lsch4_duty_r_reg::R](ledc_lsch4_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_duty_r_reg::W](ledc_lsch4_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_DUTY_R_REG {}
#[doc = "LEDC_LSCH4_DUTY_R_REG"]
pub mod ledc_lsch4_duty_r_reg;
#[doc = "LEDC_LSCH5_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch5_conf0_reg](ledc_lsch5_conf0_reg) module"]
pub type LEDC_LSCH5_CONF0_REG = crate::Reg<u32, _LEDC_LSCH5_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_CONF0_REG;
#[doc = "`read()` method returns [ledc_lsch5_conf0_reg::R](ledc_lsch5_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_conf0_reg::W](ledc_lsch5_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_CONF0_REG {}
#[doc = "LEDC_LSCH5_CONF0_REG"]
pub mod ledc_lsch5_conf0_reg;
#[doc = "LEDC_LSCH5_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch5_hpoint_reg](ledc_lsch5_hpoint_reg) module"]
pub type LEDC_LSCH5_HPOINT_REG = crate::Reg<u32, _LEDC_LSCH5_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_HPOINT_REG;
#[doc = "`read()` method returns [ledc_lsch5_hpoint_reg::R](ledc_lsch5_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_hpoint_reg::W](ledc_lsch5_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_HPOINT_REG {}
#[doc = "LEDC_LSCH5_HPOINT_REG"]
pub mod ledc_lsch5_hpoint_reg;
#[doc = "LEDC_LSCH5_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch5_duty_reg](ledc_lsch5_duty_reg) module"]
pub type LEDC_LSCH5_DUTY_REG = crate::Reg<u32, _LEDC_LSCH5_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_DUTY_REG;
#[doc = "`read()` method returns [ledc_lsch5_duty_reg::R](ledc_lsch5_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_duty_reg::W](ledc_lsch5_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_DUTY_REG {}
#[doc = "LEDC_LSCH5_DUTY_REG"]
pub mod ledc_lsch5_duty_reg;
#[doc = "LEDC_LSCH5_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch5_conf1_reg](ledc_lsch5_conf1_reg) module"]
pub type LEDC_LSCH5_CONF1_REG = crate::Reg<u32, _LEDC_LSCH5_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_CONF1_REG;
#[doc = "`read()` method returns [ledc_lsch5_conf1_reg::R](ledc_lsch5_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_conf1_reg::W](ledc_lsch5_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_CONF1_REG {}
#[doc = "LEDC_LSCH5_CONF1_REG"]
pub mod ledc_lsch5_conf1_reg;
#[doc = "LEDC_LSCH5_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch5_duty_r_reg](ledc_lsch5_duty_r_reg) module"]
pub type LEDC_LSCH5_DUTY_R_REG = crate::Reg<u32, _LEDC_LSCH5_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_lsch5_duty_r_reg::R](ledc_lsch5_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_duty_r_reg::W](ledc_lsch5_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_DUTY_R_REG {}
#[doc = "LEDC_LSCH5_DUTY_R_REG"]
pub mod ledc_lsch5_duty_r_reg;
#[doc = "LEDC_LSCH6_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch6_conf0_reg](ledc_lsch6_conf0_reg) module"]
pub type LEDC_LSCH6_CONF0_REG = crate::Reg<u32, _LEDC_LSCH6_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH6_CONF0_REG;
#[doc = "`read()` method returns [ledc_lsch6_conf0_reg::R](ledc_lsch6_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH6_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch6_conf0_reg::W](ledc_lsch6_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH6_CONF0_REG {}
#[doc = "LEDC_LSCH6_CONF0_REG"]
pub mod ledc_lsch6_conf0_reg;
#[doc = "LEDC_LSCH6_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch6_hpoint_reg](ledc_lsch6_hpoint_reg) module"]
pub type LEDC_LSCH6_HPOINT_REG = crate::Reg<u32, _LEDC_LSCH6_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH6_HPOINT_REG;
#[doc = "`read()` method returns [ledc_lsch6_hpoint_reg::R](ledc_lsch6_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH6_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch6_hpoint_reg::W](ledc_lsch6_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH6_HPOINT_REG {}
#[doc = "LEDC_LSCH6_HPOINT_REG"]
pub mod ledc_lsch6_hpoint_reg;
#[doc = "LEDC_LSCH6_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch6_duty_reg](ledc_lsch6_duty_reg) module"]
pub type LEDC_LSCH6_DUTY_REG = crate::Reg<u32, _LEDC_LSCH6_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH6_DUTY_REG;
#[doc = "`read()` method returns [ledc_lsch6_duty_reg::R](ledc_lsch6_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH6_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch6_duty_reg::W](ledc_lsch6_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH6_DUTY_REG {}
#[doc = "LEDC_LSCH6_DUTY_REG"]
pub mod ledc_lsch6_duty_reg;
#[doc = "LEDC_LSCH6_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch6_conf1_reg](ledc_lsch6_conf1_reg) module"]
pub type LEDC_LSCH6_CONF1_REG = crate::Reg<u32, _LEDC_LSCH6_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH6_CONF1_REG;
#[doc = "`read()` method returns [ledc_lsch6_conf1_reg::R](ledc_lsch6_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH6_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch6_conf1_reg::W](ledc_lsch6_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH6_CONF1_REG {}
#[doc = "LEDC_LSCH6_CONF1_REG"]
pub mod ledc_lsch6_conf1_reg;
#[doc = "LEDC_LSCH6_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch6_duty_r_reg](ledc_lsch6_duty_r_reg) module"]
pub type LEDC_LSCH6_DUTY_R_REG = crate::Reg<u32, _LEDC_LSCH6_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH6_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_lsch6_duty_r_reg::R](ledc_lsch6_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH6_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch6_duty_r_reg::W](ledc_lsch6_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH6_DUTY_R_REG {}
#[doc = "LEDC_LSCH6_DUTY_R_REG"]
pub mod ledc_lsch6_duty_r_reg;
#[doc = "LEDC_LSCH7_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch7_conf0_reg](ledc_lsch7_conf0_reg) module"]
pub type LEDC_LSCH7_CONF0_REG = crate::Reg<u32, _LEDC_LSCH7_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH7_CONF0_REG;
#[doc = "`read()` method returns [ledc_lsch7_conf0_reg::R](ledc_lsch7_conf0_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH7_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch7_conf0_reg::W](ledc_lsch7_conf0_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH7_CONF0_REG {}
#[doc = "LEDC_LSCH7_CONF0_REG"]
pub mod ledc_lsch7_conf0_reg;
#[doc = "LEDC_LSCH7_HPOINT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch7_hpoint_reg](ledc_lsch7_hpoint_reg) module"]
pub type LEDC_LSCH7_HPOINT_REG = crate::Reg<u32, _LEDC_LSCH7_HPOINT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH7_HPOINT_REG;
#[doc = "`read()` method returns [ledc_lsch7_hpoint_reg::R](ledc_lsch7_hpoint_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH7_HPOINT_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch7_hpoint_reg::W](ledc_lsch7_hpoint_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH7_HPOINT_REG {}
#[doc = "LEDC_LSCH7_HPOINT_REG"]
pub mod ledc_lsch7_hpoint_reg;
#[doc = "LEDC_LSCH7_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch7_duty_reg](ledc_lsch7_duty_reg) module"]
pub type LEDC_LSCH7_DUTY_REG = crate::Reg<u32, _LEDC_LSCH7_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH7_DUTY_REG;
#[doc = "`read()` method returns [ledc_lsch7_duty_reg::R](ledc_lsch7_duty_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH7_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch7_duty_reg::W](ledc_lsch7_duty_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH7_DUTY_REG {}
#[doc = "LEDC_LSCH7_DUTY_REG"]
pub mod ledc_lsch7_duty_reg;
#[doc = "LEDC_LSCH7_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch7_conf1_reg](ledc_lsch7_conf1_reg) module"]
pub type LEDC_LSCH7_CONF1_REG = crate::Reg<u32, _LEDC_LSCH7_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH7_CONF1_REG;
#[doc = "`read()` method returns [ledc_lsch7_conf1_reg::R](ledc_lsch7_conf1_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH7_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch7_conf1_reg::W](ledc_lsch7_conf1_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH7_CONF1_REG {}
#[doc = "LEDC_LSCH7_CONF1_REG"]
pub mod ledc_lsch7_conf1_reg;
#[doc = "LEDC_LSCH7_DUTY_R_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lsch7_duty_r_reg](ledc_lsch7_duty_r_reg) module"]
pub type LEDC_LSCH7_DUTY_R_REG = crate::Reg<u32, _LEDC_LSCH7_DUTY_R_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH7_DUTY_R_REG;
#[doc = "`read()` method returns [ledc_lsch7_duty_r_reg::R](ledc_lsch7_duty_r_reg::R) reader structure"]
impl crate::Readable for LEDC_LSCH7_DUTY_R_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch7_duty_r_reg::W](ledc_lsch7_duty_r_reg::W) writer structure"]
impl crate::Writable for LEDC_LSCH7_DUTY_R_REG {}
#[doc = "LEDC_LSCH7_DUTY_R_REG"]
pub mod ledc_lsch7_duty_r_reg;
#[doc = "LEDC_HSTIMER0_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hstimer0_conf_reg](ledc_hstimer0_conf_reg) module"]
pub type LEDC_HSTIMER0_CONF_REG = crate::Reg<u32, _LEDC_HSTIMER0_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSTIMER0_CONF_REG;
#[doc = "`read()` method returns [ledc_hstimer0_conf_reg::R](ledc_hstimer0_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_HSTIMER0_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hstimer0_conf_reg::W](ledc_hstimer0_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_HSTIMER0_CONF_REG {}
#[doc = "LEDC_HSTIMER0_CONF_REG"]
pub mod ledc_hstimer0_conf_reg;
#[doc = "LEDC_HSTIMER0_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hstimer0_value_reg](ledc_hstimer0_value_reg) module"]
pub type LEDC_HSTIMER0_VALUE_REG = crate::Reg<u32, _LEDC_HSTIMER0_VALUE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSTIMER0_VALUE_REG;
#[doc = "`read()` method returns [ledc_hstimer0_value_reg::R](ledc_hstimer0_value_reg::R) reader structure"]
impl crate::Readable for LEDC_HSTIMER0_VALUE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hstimer0_value_reg::W](ledc_hstimer0_value_reg::W) writer structure"]
impl crate::Writable for LEDC_HSTIMER0_VALUE_REG {}
#[doc = "LEDC_HSTIMER0_VALUE_REG"]
pub mod ledc_hstimer0_value_reg;
#[doc = "LEDC_HSTIMER1_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hstimer1_conf_reg](ledc_hstimer1_conf_reg) module"]
pub type LEDC_HSTIMER1_CONF_REG = crate::Reg<u32, _LEDC_HSTIMER1_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSTIMER1_CONF_REG;
#[doc = "`read()` method returns [ledc_hstimer1_conf_reg::R](ledc_hstimer1_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_HSTIMER1_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hstimer1_conf_reg::W](ledc_hstimer1_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_HSTIMER1_CONF_REG {}
#[doc = "LEDC_HSTIMER1_CONF_REG"]
pub mod ledc_hstimer1_conf_reg;
#[doc = "LEDC_HSTIMER1_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hstimer1_value_reg](ledc_hstimer1_value_reg) module"]
pub type LEDC_HSTIMER1_VALUE_REG = crate::Reg<u32, _LEDC_HSTIMER1_VALUE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSTIMER1_VALUE_REG;
#[doc = "`read()` method returns [ledc_hstimer1_value_reg::R](ledc_hstimer1_value_reg::R) reader structure"]
impl crate::Readable for LEDC_HSTIMER1_VALUE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hstimer1_value_reg::W](ledc_hstimer1_value_reg::W) writer structure"]
impl crate::Writable for LEDC_HSTIMER1_VALUE_REG {}
#[doc = "LEDC_HSTIMER1_VALUE_REG"]
pub mod ledc_hstimer1_value_reg;
#[doc = "LEDC_HSTIMER2_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hstimer2_conf_reg](ledc_hstimer2_conf_reg) module"]
pub type LEDC_HSTIMER2_CONF_REG = crate::Reg<u32, _LEDC_HSTIMER2_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSTIMER2_CONF_REG;
#[doc = "`read()` method returns [ledc_hstimer2_conf_reg::R](ledc_hstimer2_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_HSTIMER2_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hstimer2_conf_reg::W](ledc_hstimer2_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_HSTIMER2_CONF_REG {}
#[doc = "LEDC_HSTIMER2_CONF_REG"]
pub mod ledc_hstimer2_conf_reg;
#[doc = "LEDC_HSTIMER2_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hstimer2_value_reg](ledc_hstimer2_value_reg) module"]
pub type LEDC_HSTIMER2_VALUE_REG = crate::Reg<u32, _LEDC_HSTIMER2_VALUE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSTIMER2_VALUE_REG;
#[doc = "`read()` method returns [ledc_hstimer2_value_reg::R](ledc_hstimer2_value_reg::R) reader structure"]
impl crate::Readable for LEDC_HSTIMER2_VALUE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hstimer2_value_reg::W](ledc_hstimer2_value_reg::W) writer structure"]
impl crate::Writable for LEDC_HSTIMER2_VALUE_REG {}
#[doc = "LEDC_HSTIMER2_VALUE_REG"]
pub mod ledc_hstimer2_value_reg;
#[doc = "LEDC_HSTIMER3_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hstimer3_conf_reg](ledc_hstimer3_conf_reg) module"]
pub type LEDC_HSTIMER3_CONF_REG = crate::Reg<u32, _LEDC_HSTIMER3_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSTIMER3_CONF_REG;
#[doc = "`read()` method returns [ledc_hstimer3_conf_reg::R](ledc_hstimer3_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_HSTIMER3_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hstimer3_conf_reg::W](ledc_hstimer3_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_HSTIMER3_CONF_REG {}
#[doc = "LEDC_HSTIMER3_CONF_REG"]
pub mod ledc_hstimer3_conf_reg;
#[doc = "LEDC_HSTIMER3_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_hstimer3_value_reg](ledc_hstimer3_value_reg) module"]
pub type LEDC_HSTIMER3_VALUE_REG = crate::Reg<u32, _LEDC_HSTIMER3_VALUE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_HSTIMER3_VALUE_REG;
#[doc = "`read()` method returns [ledc_hstimer3_value_reg::R](ledc_hstimer3_value_reg::R) reader structure"]
impl crate::Readable for LEDC_HSTIMER3_VALUE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_hstimer3_value_reg::W](ledc_hstimer3_value_reg::W) writer structure"]
impl crate::Writable for LEDC_HSTIMER3_VALUE_REG {}
#[doc = "LEDC_HSTIMER3_VALUE_REG"]
pub mod ledc_hstimer3_value_reg;
#[doc = "LEDC_LSTIMER0_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lstimer0_conf_reg](ledc_lstimer0_conf_reg) module"]
pub type LEDC_LSTIMER0_CONF_REG = crate::Reg<u32, _LEDC_LSTIMER0_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER0_CONF_REG;
#[doc = "`read()` method returns [ledc_lstimer0_conf_reg::R](ledc_lstimer0_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER0_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer0_conf_reg::W](ledc_lstimer0_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER0_CONF_REG {}
#[doc = "LEDC_LSTIMER0_CONF_REG"]
pub mod ledc_lstimer0_conf_reg;
#[doc = "LEDC_LSTIMER0_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lstimer0_value_reg](ledc_lstimer0_value_reg) module"]
pub type LEDC_LSTIMER0_VALUE_REG = crate::Reg<u32, _LEDC_LSTIMER0_VALUE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER0_VALUE_REG;
#[doc = "`read()` method returns [ledc_lstimer0_value_reg::R](ledc_lstimer0_value_reg::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER0_VALUE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer0_value_reg::W](ledc_lstimer0_value_reg::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER0_VALUE_REG {}
#[doc = "LEDC_LSTIMER0_VALUE_REG"]
pub mod ledc_lstimer0_value_reg;
#[doc = "LEDC_LSTIMER1_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lstimer1_conf_reg](ledc_lstimer1_conf_reg) module"]
pub type LEDC_LSTIMER1_CONF_REG = crate::Reg<u32, _LEDC_LSTIMER1_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER1_CONF_REG;
#[doc = "`read()` method returns [ledc_lstimer1_conf_reg::R](ledc_lstimer1_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER1_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer1_conf_reg::W](ledc_lstimer1_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER1_CONF_REG {}
#[doc = "LEDC_LSTIMER1_CONF_REG"]
pub mod ledc_lstimer1_conf_reg;
#[doc = "LEDC_LSTIMER1_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lstimer1_value_reg](ledc_lstimer1_value_reg) module"]
pub type LEDC_LSTIMER1_VALUE_REG = crate::Reg<u32, _LEDC_LSTIMER1_VALUE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER1_VALUE_REG;
#[doc = "`read()` method returns [ledc_lstimer1_value_reg::R](ledc_lstimer1_value_reg::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER1_VALUE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer1_value_reg::W](ledc_lstimer1_value_reg::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER1_VALUE_REG {}
#[doc = "LEDC_LSTIMER1_VALUE_REG"]
pub mod ledc_lstimer1_value_reg;
#[doc = "LEDC_LSTIMER2_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lstimer2_conf_reg](ledc_lstimer2_conf_reg) module"]
pub type LEDC_LSTIMER2_CONF_REG = crate::Reg<u32, _LEDC_LSTIMER2_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER2_CONF_REG;
#[doc = "`read()` method returns [ledc_lstimer2_conf_reg::R](ledc_lstimer2_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER2_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer2_conf_reg::W](ledc_lstimer2_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER2_CONF_REG {}
#[doc = "LEDC_LSTIMER2_CONF_REG"]
pub mod ledc_lstimer2_conf_reg;
#[doc = "LEDC_LSTIMER2_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lstimer2_value_reg](ledc_lstimer2_value_reg) module"]
pub type LEDC_LSTIMER2_VALUE_REG = crate::Reg<u32, _LEDC_LSTIMER2_VALUE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER2_VALUE_REG;
#[doc = "`read()` method returns [ledc_lstimer2_value_reg::R](ledc_lstimer2_value_reg::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER2_VALUE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer2_value_reg::W](ledc_lstimer2_value_reg::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER2_VALUE_REG {}
#[doc = "LEDC_LSTIMER2_VALUE_REG"]
pub mod ledc_lstimer2_value_reg;
#[doc = "LEDC_LSTIMER3_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lstimer3_conf_reg](ledc_lstimer3_conf_reg) module"]
pub type LEDC_LSTIMER3_CONF_REG = crate::Reg<u32, _LEDC_LSTIMER3_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER3_CONF_REG;
#[doc = "`read()` method returns [ledc_lstimer3_conf_reg::R](ledc_lstimer3_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER3_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer3_conf_reg::W](ledc_lstimer3_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER3_CONF_REG {}
#[doc = "LEDC_LSTIMER3_CONF_REG"]
pub mod ledc_lstimer3_conf_reg;
#[doc = "LEDC_LSTIMER3_VALUE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_lstimer3_value_reg](ledc_lstimer3_value_reg) module"]
pub type LEDC_LSTIMER3_VALUE_REG = crate::Reg<u32, _LEDC_LSTIMER3_VALUE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER3_VALUE_REG;
#[doc = "`read()` method returns [ledc_lstimer3_value_reg::R](ledc_lstimer3_value_reg::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER3_VALUE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer3_value_reg::W](ledc_lstimer3_value_reg::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER3_VALUE_REG {}
#[doc = "LEDC_LSTIMER3_VALUE_REG"]
pub mod ledc_lstimer3_value_reg;
#[doc = "LEDC_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_int_raw_reg](ledc_int_raw_reg) module"]
pub type LEDC_INT_RAW_REG = crate::Reg<u32, _LEDC_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_INT_RAW_REG;
#[doc = "`read()` method returns [ledc_int_raw_reg::R](ledc_int_raw_reg::R) reader structure"]
impl crate::Readable for LEDC_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_int_raw_reg::W](ledc_int_raw_reg::W) writer structure"]
impl crate::Writable for LEDC_INT_RAW_REG {}
#[doc = "LEDC_INT_RAW_REG"]
pub mod ledc_int_raw_reg;
#[doc = "LEDC_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_int_st_reg](ledc_int_st_reg) module"]
pub type LEDC_INT_ST_REG = crate::Reg<u32, _LEDC_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_INT_ST_REG;
#[doc = "`read()` method returns [ledc_int_st_reg::R](ledc_int_st_reg::R) reader structure"]
impl crate::Readable for LEDC_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_int_st_reg::W](ledc_int_st_reg::W) writer structure"]
impl crate::Writable for LEDC_INT_ST_REG {}
#[doc = "LEDC_INT_ST_REG"]
pub mod ledc_int_st_reg;
#[doc = "LEDC_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_int_ena_reg](ledc_int_ena_reg) module"]
pub type LEDC_INT_ENA_REG = crate::Reg<u32, _LEDC_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_INT_ENA_REG;
#[doc = "`read()` method returns [ledc_int_ena_reg::R](ledc_int_ena_reg::R) reader structure"]
impl crate::Readable for LEDC_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_int_ena_reg::W](ledc_int_ena_reg::W) writer structure"]
impl crate::Writable for LEDC_INT_ENA_REG {}
#[doc = "LEDC_INT_ENA_REG"]
pub mod ledc_int_ena_reg;
#[doc = "LEDC_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_int_clr_reg](ledc_int_clr_reg) module"]
pub type LEDC_INT_CLR_REG = crate::Reg<u32, _LEDC_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_INT_CLR_REG;
#[doc = "`read()` method returns [ledc_int_clr_reg::R](ledc_int_clr_reg::R) reader structure"]
impl crate::Readable for LEDC_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_int_clr_reg::W](ledc_int_clr_reg::W) writer structure"]
impl crate::Writable for LEDC_INT_CLR_REG {}
#[doc = "LEDC_INT_CLR_REG"]
pub mod ledc_int_clr_reg;
#[doc = "LEDC_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_conf_reg](ledc_conf_reg) module"]
pub type LEDC_CONF_REG = crate::Reg<u32, _LEDC_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_CONF_REG;
#[doc = "`read()` method returns [ledc_conf_reg::R](ledc_conf_reg::R) reader structure"]
impl crate::Readable for LEDC_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_conf_reg::W](ledc_conf_reg::W) writer structure"]
impl crate::Writable for LEDC_CONF_REG {}
#[doc = "LEDC_CONF_REG"]
pub mod ledc_conf_reg;
#[doc = "LEDC_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ledc_date_reg](ledc_date_reg) module"]
pub type LEDC_DATE_REG = crate::Reg<u32, _LEDC_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_DATE_REG;
#[doc = "`read()` method returns [ledc_date_reg::R](ledc_date_reg::R) reader structure"]
impl crate::Readable for LEDC_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [ledc_date_reg::W](ledc_date_reg::W) writer structure"]
impl crate::Writable for LEDC_DATE_REG {}
#[doc = "LEDC_DATE_REG"]
pub mod ledc_date_reg;
