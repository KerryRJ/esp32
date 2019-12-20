#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SLC_CONF0_REG"]
    pub slc_conf0_reg: SLC_CONF0_REG,
    #[doc = "0x04 - SLC_0INT_RAW_REG"]
    pub slc_0int_raw_reg: SLC_0INT_RAW_REG,
    #[doc = "0x08 - SLC_0INT_ST_REG"]
    pub slc_0int_st_reg: SLC_0INT_ST_REG,
    #[doc = "0x0c - SLC_0INT_ENA_REG"]
    pub slc_0int_ena_reg: SLC_0INT_ENA_REG,
    #[doc = "0x10 - SLC_0INT_CLR_REG"]
    pub slc_0int_clr_reg: SLC_0INT_CLR_REG,
    #[doc = "0x14 - SLC_1INT_RAW_REG"]
    pub slc_1int_raw_reg: SLC_1INT_RAW_REG,
    #[doc = "0x18 - SLC_1INT_ST_REG"]
    pub slc_1int_st_reg: SLC_1INT_ST_REG,
    #[doc = "0x1c - SLC_1INT_ENA_REG"]
    pub slc_1int_ena_reg: SLC_1INT_ENA_REG,
    #[doc = "0x20 - SLC_1INT_CLR_REG"]
    pub slc_1int_clr_reg: SLC_1INT_CLR_REG,
    #[doc = "0x24 - SLC_RX_STATUS_REG"]
    pub slc_rx_status_reg: SLC_RX_STATUS_REG,
    #[doc = "0x28 - SLC_0RXFIFO_PUSH_REG"]
    pub slc_0rxfifo_push_reg: SLC_0RXFIFO_PUSH_REG,
    #[doc = "0x2c - SLC_1RXFIFO_PUSH_REG"]
    pub slc_1rxfifo_push_reg: SLC_1RXFIFO_PUSH_REG,
    #[doc = "0x30 - SLC_TX_STATUS_REG"]
    pub slc_tx_status_reg: SLC_TX_STATUS_REG,
    #[doc = "0x34 - SLC_0TXFIFO_POP_REG"]
    pub slc_0txfifo_pop_reg: SLC_0TXFIFO_POP_REG,
    #[doc = "0x38 - SLC_1TXFIFO_POP_REG"]
    pub slc_1txfifo_pop_reg: SLC_1TXFIFO_POP_REG,
    #[doc = "0x3c - SLC_0RX_LINK_REG"]
    pub slc_0rx_link_reg: SLC_0RX_LINK_REG,
    #[doc = "0x40 - SLC_0TX_LINK_REG"]
    pub slc_0tx_link_reg: SLC_0TX_LINK_REG,
    #[doc = "0x44 - SLC_1RX_LINK_REG"]
    pub slc_1rx_link_reg: SLC_1RX_LINK_REG,
    #[doc = "0x48 - SLC_1TX_LINK_REG"]
    pub slc_1tx_link_reg: SLC_1TX_LINK_REG,
    #[doc = "0x4c - SLC_INTVEC_TOHOST_REG"]
    pub slc_intvec_tohost_reg: SLC_INTVEC_TOHOST_REG,
    #[doc = "0x50 - SLC_0TOKEN0_REG"]
    pub slc_0token0_reg: SLC_0TOKEN0_REG,
    #[doc = "0x54 - SLC_0TOKEN1_REG"]
    pub slc_0token1_reg: SLC_0TOKEN1_REG,
    #[doc = "0x58 - SLC_1TOKEN0_REG"]
    pub slc_1token0_reg: SLC_1TOKEN0_REG,
    #[doc = "0x5c - SLC_1TOKEN1_REG"]
    pub slc_1token1_reg: SLC_1TOKEN1_REG,
    #[doc = "0x60 - SLC_CONF1_REG"]
    pub slc_conf1_reg: SLC_CONF1_REG,
    #[doc = "0x64 - SLC_0_STATE0_REG"]
    pub slc_0_state0_reg: SLC_0_STATE0_REG,
    #[doc = "0x68 - SLC_0_STATE1_REG"]
    pub slc_0_state1_reg: SLC_0_STATE1_REG,
    #[doc = "0x6c - SLC_1_STATE0_REG"]
    pub slc_1_state0_reg: SLC_1_STATE0_REG,
    #[doc = "0x70 - SLC_1_STATE1_REG"]
    pub slc_1_state1_reg: SLC_1_STATE1_REG,
    #[doc = "0x74 - SLC_BRIDGE_CONF_REG"]
    pub slc_bridge_conf_reg: SLC_BRIDGE_CONF_REG,
    #[doc = "0x78 - SLC_0_TO_EOF_DES_ADDR_REG"]
    pub slc_0_to_eof_des_addr_reg: SLC_0_TO_EOF_DES_ADDR_REG,
    #[doc = "0x7c - SLC_0_TX_EOF_DES_ADDR_REG"]
    pub slc_0_tx_eof_des_addr_reg: SLC_0_TX_EOF_DES_ADDR_REG,
    #[doc = "0x80 - SLC_0_TO_EOF_BFR_DES_ADDR_REG"]
    pub slc_0_to_eof_bfr_des_addr_reg: SLC_0_TO_EOF_BFR_DES_ADDR_REG,
    #[doc = "0x84 - SLC_1_TO_EOF_DES_ADDR_REG"]
    pub slc_1_to_eof_des_addr_reg: SLC_1_TO_EOF_DES_ADDR_REG,
    #[doc = "0x88 - SLC_1_TX_EOF_DES_ADDR_REG"]
    pub slc_1_tx_eof_des_addr_reg: SLC_1_TX_EOF_DES_ADDR_REG,
    #[doc = "0x8c - SLC_1_TO_EOF_BFR_DES_ADDR_REG"]
    pub slc_1_to_eof_bfr_des_addr_reg: SLC_1_TO_EOF_BFR_DES_ADDR_REG,
    #[doc = "0x90 - SLC_AHB_TEST_REG"]
    pub slc_ahb_test_reg: SLC_AHB_TEST_REG,
    #[doc = "0x94 - SLC_SDIO_ST_REG"]
    pub slc_sdio_st_reg: SLC_SDIO_ST_REG,
    #[doc = "0x98 - SLC_RX_DSCR_CONF_REG"]
    pub slc_rx_dscr_conf_reg: SLC_RX_DSCR_CONF_REG,
    #[doc = "0x9c - SLC_0_TXLINK_DSCR_REG"]
    pub slc_0_txlink_dscr_reg: SLC_0_TXLINK_DSCR_REG,
    #[doc = "0xa0 - SLC_0_TXLINK_DSCR_BF0_REG"]
    pub slc_0_txlink_dscr_bf0_reg: SLC_0_TXLINK_DSCR_BF0_REG,
    #[doc = "0xa4 - SLC_0_TXLINK_DSCR_BF1_REG"]
    pub slc_0_txlink_dscr_bf1_reg: SLC_0_TXLINK_DSCR_BF1_REG,
    #[doc = "0xa8 - SLC_0_RXLINK_DSCR_REG"]
    pub slc_0_rxlink_dscr_reg: SLC_0_RXLINK_DSCR_REG,
    #[doc = "0xac - SLC_0_RXLINK_DSCR_BF0_REG"]
    pub slc_0_rxlink_dscr_bf0_reg: SLC_0_RXLINK_DSCR_BF0_REG,
    #[doc = "0xb0 - SLC_0_RXLINK_DSCR_BF1_REG"]
    pub slc_0_rxlink_dscr_bf1_reg: SLC_0_RXLINK_DSCR_BF1_REG,
    #[doc = "0xb4 - SLC_1_TXLINK_DSCR_REG"]
    pub slc_1_txlink_dscr_reg: SLC_1_TXLINK_DSCR_REG,
    #[doc = "0xb8 - SLC_1_TXLINK_DSCR_BF0_REG"]
    pub slc_1_txlink_dscr_bf0_reg: SLC_1_TXLINK_DSCR_BF0_REG,
    #[doc = "0xbc - SLC_1_TXLINK_DSCR_BF1_REG"]
    pub slc_1_txlink_dscr_bf1_reg: SLC_1_TXLINK_DSCR_BF1_REG,
    #[doc = "0xc0 - SLC_1_RXLINK_DSCR_REG"]
    pub slc_1_rxlink_dscr_reg: SLC_1_RXLINK_DSCR_REG,
    #[doc = "0xc4 - SLC_1_RXLINK_DSCR_BF0_REG"]
    pub slc_1_rxlink_dscr_bf0_reg: SLC_1_RXLINK_DSCR_BF0_REG,
    #[doc = "0xc8 - SLC_1_RXLINK_DSCR_BF1_REG"]
    pub slc_1_rxlink_dscr_bf1_reg: SLC_1_RXLINK_DSCR_BF1_REG,
    #[doc = "0xcc - SLC_0_TX_ERREOF_DES_ADDR_REG"]
    pub slc_0_tx_erreof_des_addr_reg: SLC_0_TX_ERREOF_DES_ADDR_REG,
    #[doc = "0xd0 - SLC_1_TX_ERREOF_DES_ADDR_REG"]
    pub slc_1_tx_erreof_des_addr_reg: SLC_1_TX_ERREOF_DES_ADDR_REG,
    #[doc = "0xd4 - SLC_TOKEN_LAT_REG"]
    pub slc_token_lat_reg: SLC_TOKEN_LAT_REG,
    #[doc = "0xd8 - SLC_TX_DSCR_CONF_REG"]
    pub slc_tx_dscr_conf_reg: SLC_TX_DSCR_CONF_REG,
    #[doc = "0xdc - SLC_CMD_INFOR0_REG"]
    pub slc_cmd_infor0_reg: SLC_CMD_INFOR0_REG,
    #[doc = "0xe0 - SLC_CMD_INFOR1_REG"]
    pub slc_cmd_infor1_reg: SLC_CMD_INFOR1_REG,
    #[doc = "0xe4 - SLC_0_LEN_CONF_REG"]
    pub slc_0_len_conf_reg: SLC_0_LEN_CONF_REG,
    #[doc = "0xe8 - SLC_0_LENGTH_REG"]
    pub slc_0_length_reg: SLC_0_LENGTH_REG,
    #[doc = "0xec - SLC_0_TXPKT_H_DSCR_REG"]
    pub slc_0_txpkt_h_dscr_reg: SLC_0_TXPKT_H_DSCR_REG,
    #[doc = "0xf0 - SLC_0_TXPKT_E_DSCR_REG"]
    pub slc_0_txpkt_e_dscr_reg: SLC_0_TXPKT_E_DSCR_REG,
    #[doc = "0xf4 - SLC_0_RXPKT_H_DSCR_REG"]
    pub slc_0_rxpkt_h_dscr_reg: SLC_0_RXPKT_H_DSCR_REG,
    #[doc = "0xf8 - SLC_0_RXPKT_E_DSCR_REG"]
    pub slc_0_rxpkt_e_dscr_reg: SLC_0_RXPKT_E_DSCR_REG,
    #[doc = "0xfc - SLC_0_TXPKTU_H_DSCR_REG"]
    pub slc_0_txpktu_h_dscr_reg: SLC_0_TXPKTU_H_DSCR_REG,
    #[doc = "0x100 - SLC_0_TXPKTU_E_DSCR_REG"]
    pub slc_0_txpktu_e_dscr_reg: SLC_0_TXPKTU_E_DSCR_REG,
    #[doc = "0x104 - SLC_0_RXPKTU_H_DSCR_REG"]
    pub slc_0_rxpktu_h_dscr_reg: SLC_0_RXPKTU_H_DSCR_REG,
    #[doc = "0x108 - SLC_0_RXPKTU_E_DSCR_REG"]
    pub slc_0_rxpktu_e_dscr_reg: SLC_0_RXPKTU_E_DSCR_REG,
    _reserved67: [u8; 8usize],
    #[doc = "0x114 - SLC_SEQ_POSITION_REG"]
    pub slc_seq_position_reg: SLC_SEQ_POSITION_REG,
    #[doc = "0x118 - SLC_0_DSCR_REC_CONF_REG"]
    pub slc_0_dscr_rec_conf_reg: SLC_0_DSCR_REC_CONF_REG,
    #[doc = "0x11c - SLC_SDIO_CRC_ST0_REG"]
    pub slc_sdio_crc_st0_reg: SLC_SDIO_CRC_ST0_REG,
    #[doc = "0x120 - SLC_SDIO_CRC_ST1_REG"]
    pub slc_sdio_crc_st1_reg: SLC_SDIO_CRC_ST1_REG,
    #[doc = "0x124 - SLC_0_EOF_START_DES_REG"]
    pub slc_0_eof_start_des_reg: SLC_0_EOF_START_DES_REG,
    #[doc = "0x128 - SLC_0_PUSH_DSCR_ADDR_REG"]
    pub slc_0_push_dscr_addr_reg: SLC_0_PUSH_DSCR_ADDR_REG,
    #[doc = "0x12c - SLC_0_DONE_DSCR_ADDR_REG"]
    pub slc_0_done_dscr_addr_reg: SLC_0_DONE_DSCR_ADDR_REG,
    #[doc = "0x130 - SLC_0_SUB_START_DES_REG"]
    pub slc_0_sub_start_des_reg: SLC_0_SUB_START_DES_REG,
    #[doc = "0x134 - SLC_0_DSCR_CNT_REG"]
    pub slc_0_dscr_cnt_reg: SLC_0_DSCR_CNT_REG,
    #[doc = "0x138 - SLC_0_LEN_LIM_CONF_REG"]
    pub slc_0_len_lim_conf_reg: SLC_0_LEN_LIM_CONF_REG,
    #[doc = "0x13c - SLC_0INT_ST1_REG"]
    pub slc_0int_st1_reg: SLC_0INT_ST1_REG,
    #[doc = "0x140 - SLC_0INT_ENA1_REG"]
    pub slc_0int_ena1_reg: SLC_0INT_ENA1_REG,
    #[doc = "0x144 - SLC_1INT_ST1_REG"]
    pub slc_1int_st1_reg: SLC_1INT_ST1_REG,
    #[doc = "0x148 - SLC_1INT_ENA1_REG"]
    pub slc_1int_ena1_reg: SLC_1INT_ENA1_REG,
    _reserved81: [u8; 172usize],
    #[doc = "0x1f8 - SLC_DATE_REG"]
    pub slc_date_reg: SLC_DATE_REG,
    #[doc = "0x1fc - SLC_ID_REG"]
    pub slc_id_reg: SLC_ID_REG,
}
#[doc = "SLC_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_conf0_reg](slc_conf0_reg) module"]
pub type SLC_CONF0_REG = crate::Reg<u32, _SLC_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_CONF0_REG;
#[doc = "`read()` method returns [slc_conf0_reg::R](slc_conf0_reg::R) reader structure"]
impl crate::Readable for SLC_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_conf0_reg::W](slc_conf0_reg::W) writer structure"]
impl crate::Writable for SLC_CONF0_REG {}
#[doc = "SLC_CONF0_REG"]
pub mod slc_conf0_reg;
#[doc = "SLC_0INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0int_raw_reg](slc_0int_raw_reg) module"]
pub type SLC_0INT_RAW_REG = crate::Reg<u32, _SLC_0INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0INT_RAW_REG;
#[doc = "`read()` method returns [slc_0int_raw_reg::R](slc_0int_raw_reg::R) reader structure"]
impl crate::Readable for SLC_0INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0int_raw_reg::W](slc_0int_raw_reg::W) writer structure"]
impl crate::Writable for SLC_0INT_RAW_REG {}
#[doc = "SLC_0INT_RAW_REG"]
pub mod slc_0int_raw_reg;
#[doc = "SLC_0INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0int_st_reg](slc_0int_st_reg) module"]
pub type SLC_0INT_ST_REG = crate::Reg<u32, _SLC_0INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0INT_ST_REG;
#[doc = "`read()` method returns [slc_0int_st_reg::R](slc_0int_st_reg::R) reader structure"]
impl crate::Readable for SLC_0INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0int_st_reg::W](slc_0int_st_reg::W) writer structure"]
impl crate::Writable for SLC_0INT_ST_REG {}
#[doc = "SLC_0INT_ST_REG"]
pub mod slc_0int_st_reg;
#[doc = "SLC_0INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0int_ena_reg](slc_0int_ena_reg) module"]
pub type SLC_0INT_ENA_REG = crate::Reg<u32, _SLC_0INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0INT_ENA_REG;
#[doc = "`read()` method returns [slc_0int_ena_reg::R](slc_0int_ena_reg::R) reader structure"]
impl crate::Readable for SLC_0INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0int_ena_reg::W](slc_0int_ena_reg::W) writer structure"]
impl crate::Writable for SLC_0INT_ENA_REG {}
#[doc = "SLC_0INT_ENA_REG"]
pub mod slc_0int_ena_reg;
#[doc = "SLC_0INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0int_clr_reg](slc_0int_clr_reg) module"]
pub type SLC_0INT_CLR_REG = crate::Reg<u32, _SLC_0INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0INT_CLR_REG;
#[doc = "`read()` method returns [slc_0int_clr_reg::R](slc_0int_clr_reg::R) reader structure"]
impl crate::Readable for SLC_0INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0int_clr_reg::W](slc_0int_clr_reg::W) writer structure"]
impl crate::Writable for SLC_0INT_CLR_REG {}
#[doc = "SLC_0INT_CLR_REG"]
pub mod slc_0int_clr_reg;
#[doc = "SLC_1INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1int_raw_reg](slc_1int_raw_reg) module"]
pub type SLC_1INT_RAW_REG = crate::Reg<u32, _SLC_1INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1INT_RAW_REG;
#[doc = "`read()` method returns [slc_1int_raw_reg::R](slc_1int_raw_reg::R) reader structure"]
impl crate::Readable for SLC_1INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1int_raw_reg::W](slc_1int_raw_reg::W) writer structure"]
impl crate::Writable for SLC_1INT_RAW_REG {}
#[doc = "SLC_1INT_RAW_REG"]
pub mod slc_1int_raw_reg;
#[doc = "SLC_1INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1int_st_reg](slc_1int_st_reg) module"]
pub type SLC_1INT_ST_REG = crate::Reg<u32, _SLC_1INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1INT_ST_REG;
#[doc = "`read()` method returns [slc_1int_st_reg::R](slc_1int_st_reg::R) reader structure"]
impl crate::Readable for SLC_1INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1int_st_reg::W](slc_1int_st_reg::W) writer structure"]
impl crate::Writable for SLC_1INT_ST_REG {}
#[doc = "SLC_1INT_ST_REG"]
pub mod slc_1int_st_reg;
#[doc = "SLC_1INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1int_ena_reg](slc_1int_ena_reg) module"]
pub type SLC_1INT_ENA_REG = crate::Reg<u32, _SLC_1INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1INT_ENA_REG;
#[doc = "`read()` method returns [slc_1int_ena_reg::R](slc_1int_ena_reg::R) reader structure"]
impl crate::Readable for SLC_1INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1int_ena_reg::W](slc_1int_ena_reg::W) writer structure"]
impl crate::Writable for SLC_1INT_ENA_REG {}
#[doc = "SLC_1INT_ENA_REG"]
pub mod slc_1int_ena_reg;
#[doc = "SLC_1INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1int_clr_reg](slc_1int_clr_reg) module"]
pub type SLC_1INT_CLR_REG = crate::Reg<u32, _SLC_1INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1INT_CLR_REG;
#[doc = "`read()` method returns [slc_1int_clr_reg::R](slc_1int_clr_reg::R) reader structure"]
impl crate::Readable for SLC_1INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1int_clr_reg::W](slc_1int_clr_reg::W) writer structure"]
impl crate::Writable for SLC_1INT_CLR_REG {}
#[doc = "SLC_1INT_CLR_REG"]
pub mod slc_1int_clr_reg;
#[doc = "SLC_RX_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_rx_status_reg](slc_rx_status_reg) module"]
pub type SLC_RX_STATUS_REG = crate::Reg<u32, _SLC_RX_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_RX_STATUS_REG;
#[doc = "`read()` method returns [slc_rx_status_reg::R](slc_rx_status_reg::R) reader structure"]
impl crate::Readable for SLC_RX_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [slc_rx_status_reg::W](slc_rx_status_reg::W) writer structure"]
impl crate::Writable for SLC_RX_STATUS_REG {}
#[doc = "SLC_RX_STATUS_REG"]
pub mod slc_rx_status_reg;
#[doc = "SLC_0RXFIFO_PUSH_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0rxfifo_push_reg](slc_0rxfifo_push_reg) module"]
pub type SLC_0RXFIFO_PUSH_REG = crate::Reg<u32, _SLC_0RXFIFO_PUSH_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0RXFIFO_PUSH_REG;
#[doc = "`read()` method returns [slc_0rxfifo_push_reg::R](slc_0rxfifo_push_reg::R) reader structure"]
impl crate::Readable for SLC_0RXFIFO_PUSH_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0rxfifo_push_reg::W](slc_0rxfifo_push_reg::W) writer structure"]
impl crate::Writable for SLC_0RXFIFO_PUSH_REG {}
#[doc = "SLC_0RXFIFO_PUSH_REG"]
pub mod slc_0rxfifo_push_reg;
#[doc = "SLC_1RXFIFO_PUSH_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1rxfifo_push_reg](slc_1rxfifo_push_reg) module"]
pub type SLC_1RXFIFO_PUSH_REG = crate::Reg<u32, _SLC_1RXFIFO_PUSH_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1RXFIFO_PUSH_REG;
#[doc = "`read()` method returns [slc_1rxfifo_push_reg::R](slc_1rxfifo_push_reg::R) reader structure"]
impl crate::Readable for SLC_1RXFIFO_PUSH_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1rxfifo_push_reg::W](slc_1rxfifo_push_reg::W) writer structure"]
impl crate::Writable for SLC_1RXFIFO_PUSH_REG {}
#[doc = "SLC_1RXFIFO_PUSH_REG"]
pub mod slc_1rxfifo_push_reg;
#[doc = "SLC_TX_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_tx_status_reg](slc_tx_status_reg) module"]
pub type SLC_TX_STATUS_REG = crate::Reg<u32, _SLC_TX_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_TX_STATUS_REG;
#[doc = "`read()` method returns [slc_tx_status_reg::R](slc_tx_status_reg::R) reader structure"]
impl crate::Readable for SLC_TX_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [slc_tx_status_reg::W](slc_tx_status_reg::W) writer structure"]
impl crate::Writable for SLC_TX_STATUS_REG {}
#[doc = "SLC_TX_STATUS_REG"]
pub mod slc_tx_status_reg;
#[doc = "SLC_0TXFIFO_POP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0txfifo_pop_reg](slc_0txfifo_pop_reg) module"]
pub type SLC_0TXFIFO_POP_REG = crate::Reg<u32, _SLC_0TXFIFO_POP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0TXFIFO_POP_REG;
#[doc = "`read()` method returns [slc_0txfifo_pop_reg::R](slc_0txfifo_pop_reg::R) reader structure"]
impl crate::Readable for SLC_0TXFIFO_POP_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0txfifo_pop_reg::W](slc_0txfifo_pop_reg::W) writer structure"]
impl crate::Writable for SLC_0TXFIFO_POP_REG {}
#[doc = "SLC_0TXFIFO_POP_REG"]
pub mod slc_0txfifo_pop_reg;
#[doc = "SLC_1TXFIFO_POP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1txfifo_pop_reg](slc_1txfifo_pop_reg) module"]
pub type SLC_1TXFIFO_POP_REG = crate::Reg<u32, _SLC_1TXFIFO_POP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1TXFIFO_POP_REG;
#[doc = "`read()` method returns [slc_1txfifo_pop_reg::R](slc_1txfifo_pop_reg::R) reader structure"]
impl crate::Readable for SLC_1TXFIFO_POP_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1txfifo_pop_reg::W](slc_1txfifo_pop_reg::W) writer structure"]
impl crate::Writable for SLC_1TXFIFO_POP_REG {}
#[doc = "SLC_1TXFIFO_POP_REG"]
pub mod slc_1txfifo_pop_reg;
#[doc = "SLC_0RX_LINK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0rx_link_reg](slc_0rx_link_reg) module"]
pub type SLC_0RX_LINK_REG = crate::Reg<u32, _SLC_0RX_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0RX_LINK_REG;
#[doc = "`read()` method returns [slc_0rx_link_reg::R](slc_0rx_link_reg::R) reader structure"]
impl crate::Readable for SLC_0RX_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0rx_link_reg::W](slc_0rx_link_reg::W) writer structure"]
impl crate::Writable for SLC_0RX_LINK_REG {}
#[doc = "SLC_0RX_LINK_REG"]
pub mod slc_0rx_link_reg;
#[doc = "SLC_0TX_LINK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0tx_link_reg](slc_0tx_link_reg) module"]
pub type SLC_0TX_LINK_REG = crate::Reg<u32, _SLC_0TX_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0TX_LINK_REG;
#[doc = "`read()` method returns [slc_0tx_link_reg::R](slc_0tx_link_reg::R) reader structure"]
impl crate::Readable for SLC_0TX_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0tx_link_reg::W](slc_0tx_link_reg::W) writer structure"]
impl crate::Writable for SLC_0TX_LINK_REG {}
#[doc = "SLC_0TX_LINK_REG"]
pub mod slc_0tx_link_reg;
#[doc = "SLC_1RX_LINK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1rx_link_reg](slc_1rx_link_reg) module"]
pub type SLC_1RX_LINK_REG = crate::Reg<u32, _SLC_1RX_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1RX_LINK_REG;
#[doc = "`read()` method returns [slc_1rx_link_reg::R](slc_1rx_link_reg::R) reader structure"]
impl crate::Readable for SLC_1RX_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1rx_link_reg::W](slc_1rx_link_reg::W) writer structure"]
impl crate::Writable for SLC_1RX_LINK_REG {}
#[doc = "SLC_1RX_LINK_REG"]
pub mod slc_1rx_link_reg;
#[doc = "SLC_1TX_LINK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1tx_link_reg](slc_1tx_link_reg) module"]
pub type SLC_1TX_LINK_REG = crate::Reg<u32, _SLC_1TX_LINK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1TX_LINK_REG;
#[doc = "`read()` method returns [slc_1tx_link_reg::R](slc_1tx_link_reg::R) reader structure"]
impl crate::Readable for SLC_1TX_LINK_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1tx_link_reg::W](slc_1tx_link_reg::W) writer structure"]
impl crate::Writable for SLC_1TX_LINK_REG {}
#[doc = "SLC_1TX_LINK_REG"]
pub mod slc_1tx_link_reg;
#[doc = "SLC_INTVEC_TOHOST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_intvec_tohost_reg](slc_intvec_tohost_reg) module"]
pub type SLC_INTVEC_TOHOST_REG = crate::Reg<u32, _SLC_INTVEC_TOHOST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_INTVEC_TOHOST_REG;
#[doc = "`read()` method returns [slc_intvec_tohost_reg::R](slc_intvec_tohost_reg::R) reader structure"]
impl crate::Readable for SLC_INTVEC_TOHOST_REG {}
#[doc = "`write(|w| ..)` method takes [slc_intvec_tohost_reg::W](slc_intvec_tohost_reg::W) writer structure"]
impl crate::Writable for SLC_INTVEC_TOHOST_REG {}
#[doc = "SLC_INTVEC_TOHOST_REG"]
pub mod slc_intvec_tohost_reg;
#[doc = "SLC_0TOKEN0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0token0_reg](slc_0token0_reg) module"]
pub type SLC_0TOKEN0_REG = crate::Reg<u32, _SLC_0TOKEN0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0TOKEN0_REG;
#[doc = "`read()` method returns [slc_0token0_reg::R](slc_0token0_reg::R) reader structure"]
impl crate::Readable for SLC_0TOKEN0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0token0_reg::W](slc_0token0_reg::W) writer structure"]
impl crate::Writable for SLC_0TOKEN0_REG {}
#[doc = "SLC_0TOKEN0_REG"]
pub mod slc_0token0_reg;
#[doc = "SLC_0TOKEN1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0token1_reg](slc_0token1_reg) module"]
pub type SLC_0TOKEN1_REG = crate::Reg<u32, _SLC_0TOKEN1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0TOKEN1_REG;
#[doc = "`read()` method returns [slc_0token1_reg::R](slc_0token1_reg::R) reader structure"]
impl crate::Readable for SLC_0TOKEN1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0token1_reg::W](slc_0token1_reg::W) writer structure"]
impl crate::Writable for SLC_0TOKEN1_REG {}
#[doc = "SLC_0TOKEN1_REG"]
pub mod slc_0token1_reg;
#[doc = "SLC_1TOKEN0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1token0_reg](slc_1token0_reg) module"]
pub type SLC_1TOKEN0_REG = crate::Reg<u32, _SLC_1TOKEN0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1TOKEN0_REG;
#[doc = "`read()` method returns [slc_1token0_reg::R](slc_1token0_reg::R) reader structure"]
impl crate::Readable for SLC_1TOKEN0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1token0_reg::W](slc_1token0_reg::W) writer structure"]
impl crate::Writable for SLC_1TOKEN0_REG {}
#[doc = "SLC_1TOKEN0_REG"]
pub mod slc_1token0_reg;
#[doc = "SLC_1TOKEN1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1token1_reg](slc_1token1_reg) module"]
pub type SLC_1TOKEN1_REG = crate::Reg<u32, _SLC_1TOKEN1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1TOKEN1_REG;
#[doc = "`read()` method returns [slc_1token1_reg::R](slc_1token1_reg::R) reader structure"]
impl crate::Readable for SLC_1TOKEN1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1token1_reg::W](slc_1token1_reg::W) writer structure"]
impl crate::Writable for SLC_1TOKEN1_REG {}
#[doc = "SLC_1TOKEN1_REG"]
pub mod slc_1token1_reg;
#[doc = "SLC_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_conf1_reg](slc_conf1_reg) module"]
pub type SLC_CONF1_REG = crate::Reg<u32, _SLC_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_CONF1_REG;
#[doc = "`read()` method returns [slc_conf1_reg::R](slc_conf1_reg::R) reader structure"]
impl crate::Readable for SLC_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_conf1_reg::W](slc_conf1_reg::W) writer structure"]
impl crate::Writable for SLC_CONF1_REG {}
#[doc = "SLC_CONF1_REG"]
pub mod slc_conf1_reg;
#[doc = "SLC_0_STATE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_state0_reg](slc_0_state0_reg) module"]
pub type SLC_0_STATE0_REG = crate::Reg<u32, _SLC_0_STATE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_STATE0_REG;
#[doc = "`read()` method returns [slc_0_state0_reg::R](slc_0_state0_reg::R) reader structure"]
impl crate::Readable for SLC_0_STATE0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_state0_reg::W](slc_0_state0_reg::W) writer structure"]
impl crate::Writable for SLC_0_STATE0_REG {}
#[doc = "SLC_0_STATE0_REG"]
pub mod slc_0_state0_reg;
#[doc = "SLC_0_STATE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_state1_reg](slc_0_state1_reg) module"]
pub type SLC_0_STATE1_REG = crate::Reg<u32, _SLC_0_STATE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_STATE1_REG;
#[doc = "`read()` method returns [slc_0_state1_reg::R](slc_0_state1_reg::R) reader structure"]
impl crate::Readable for SLC_0_STATE1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_state1_reg::W](slc_0_state1_reg::W) writer structure"]
impl crate::Writable for SLC_0_STATE1_REG {}
#[doc = "SLC_0_STATE1_REG"]
pub mod slc_0_state1_reg;
#[doc = "SLC_1_STATE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_state0_reg](slc_1_state0_reg) module"]
pub type SLC_1_STATE0_REG = crate::Reg<u32, _SLC_1_STATE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_STATE0_REG;
#[doc = "`read()` method returns [slc_1_state0_reg::R](slc_1_state0_reg::R) reader structure"]
impl crate::Readable for SLC_1_STATE0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_state0_reg::W](slc_1_state0_reg::W) writer structure"]
impl crate::Writable for SLC_1_STATE0_REG {}
#[doc = "SLC_1_STATE0_REG"]
pub mod slc_1_state0_reg;
#[doc = "SLC_1_STATE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_state1_reg](slc_1_state1_reg) module"]
pub type SLC_1_STATE1_REG = crate::Reg<u32, _SLC_1_STATE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_STATE1_REG;
#[doc = "`read()` method returns [slc_1_state1_reg::R](slc_1_state1_reg::R) reader structure"]
impl crate::Readable for SLC_1_STATE1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_state1_reg::W](slc_1_state1_reg::W) writer structure"]
impl crate::Writable for SLC_1_STATE1_REG {}
#[doc = "SLC_1_STATE1_REG"]
pub mod slc_1_state1_reg;
#[doc = "SLC_BRIDGE_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_bridge_conf_reg](slc_bridge_conf_reg) module"]
pub type SLC_BRIDGE_CONF_REG = crate::Reg<u32, _SLC_BRIDGE_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_BRIDGE_CONF_REG;
#[doc = "`read()` method returns [slc_bridge_conf_reg::R](slc_bridge_conf_reg::R) reader structure"]
impl crate::Readable for SLC_BRIDGE_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [slc_bridge_conf_reg::W](slc_bridge_conf_reg::W) writer structure"]
impl crate::Writable for SLC_BRIDGE_CONF_REG {}
#[doc = "SLC_BRIDGE_CONF_REG"]
pub mod slc_bridge_conf_reg;
#[doc = "SLC_0_TO_EOF_DES_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_to_eof_des_addr_reg](slc_0_to_eof_des_addr_reg) module"]
pub type SLC_0_TO_EOF_DES_ADDR_REG = crate::Reg<u32, _SLC_0_TO_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TO_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [slc_0_to_eof_des_addr_reg::R](slc_0_to_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TO_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_to_eof_des_addr_reg::W](slc_0_to_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TO_EOF_DES_ADDR_REG {}
#[doc = "SLC_0_TO_EOF_DES_ADDR_REG"]
pub mod slc_0_to_eof_des_addr_reg;
#[doc = "SLC_0_TX_EOF_DES_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_tx_eof_des_addr_reg](slc_0_tx_eof_des_addr_reg) module"]
pub type SLC_0_TX_EOF_DES_ADDR_REG = crate::Reg<u32, _SLC_0_TX_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TX_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [slc_0_tx_eof_des_addr_reg::R](slc_0_tx_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TX_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_tx_eof_des_addr_reg::W](slc_0_tx_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TX_EOF_DES_ADDR_REG {}
#[doc = "SLC_0_TX_EOF_DES_ADDR_REG"]
pub mod slc_0_tx_eof_des_addr_reg;
#[doc = "SLC_0_TO_EOF_BFR_DES_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_to_eof_bfr_des_addr_reg](slc_0_to_eof_bfr_des_addr_reg) module"]
pub type SLC_0_TO_EOF_BFR_DES_ADDR_REG = crate::Reg<u32, _SLC_0_TO_EOF_BFR_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TO_EOF_BFR_DES_ADDR_REG;
#[doc = "`read()` method returns [slc_0_to_eof_bfr_des_addr_reg::R](slc_0_to_eof_bfr_des_addr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TO_EOF_BFR_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_to_eof_bfr_des_addr_reg::W](slc_0_to_eof_bfr_des_addr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TO_EOF_BFR_DES_ADDR_REG {}
#[doc = "SLC_0_TO_EOF_BFR_DES_ADDR_REG"]
pub mod slc_0_to_eof_bfr_des_addr_reg;
#[doc = "SLC_1_TO_EOF_DES_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_to_eof_des_addr_reg](slc_1_to_eof_des_addr_reg) module"]
pub type SLC_1_TO_EOF_DES_ADDR_REG = crate::Reg<u32, _SLC_1_TO_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_TO_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [slc_1_to_eof_des_addr_reg::R](slc_1_to_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for SLC_1_TO_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_to_eof_des_addr_reg::W](slc_1_to_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for SLC_1_TO_EOF_DES_ADDR_REG {}
#[doc = "SLC_1_TO_EOF_DES_ADDR_REG"]
pub mod slc_1_to_eof_des_addr_reg;
#[doc = "SLC_1_TX_EOF_DES_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_tx_eof_des_addr_reg](slc_1_tx_eof_des_addr_reg) module"]
pub type SLC_1_TX_EOF_DES_ADDR_REG = crate::Reg<u32, _SLC_1_TX_EOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_TX_EOF_DES_ADDR_REG;
#[doc = "`read()` method returns [slc_1_tx_eof_des_addr_reg::R](slc_1_tx_eof_des_addr_reg::R) reader structure"]
impl crate::Readable for SLC_1_TX_EOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_tx_eof_des_addr_reg::W](slc_1_tx_eof_des_addr_reg::W) writer structure"]
impl crate::Writable for SLC_1_TX_EOF_DES_ADDR_REG {}
#[doc = "SLC_1_TX_EOF_DES_ADDR_REG"]
pub mod slc_1_tx_eof_des_addr_reg;
#[doc = "SLC_1_TO_EOF_BFR_DES_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_to_eof_bfr_des_addr_reg](slc_1_to_eof_bfr_des_addr_reg) module"]
pub type SLC_1_TO_EOF_BFR_DES_ADDR_REG = crate::Reg<u32, _SLC_1_TO_EOF_BFR_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_TO_EOF_BFR_DES_ADDR_REG;
#[doc = "`read()` method returns [slc_1_to_eof_bfr_des_addr_reg::R](slc_1_to_eof_bfr_des_addr_reg::R) reader structure"]
impl crate::Readable for SLC_1_TO_EOF_BFR_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_to_eof_bfr_des_addr_reg::W](slc_1_to_eof_bfr_des_addr_reg::W) writer structure"]
impl crate::Writable for SLC_1_TO_EOF_BFR_DES_ADDR_REG {}
#[doc = "SLC_1_TO_EOF_BFR_DES_ADDR_REG"]
pub mod slc_1_to_eof_bfr_des_addr_reg;
#[doc = "SLC_AHB_TEST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_ahb_test_reg](slc_ahb_test_reg) module"]
pub type SLC_AHB_TEST_REG = crate::Reg<u32, _SLC_AHB_TEST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_AHB_TEST_REG;
#[doc = "`read()` method returns [slc_ahb_test_reg::R](slc_ahb_test_reg::R) reader structure"]
impl crate::Readable for SLC_AHB_TEST_REG {}
#[doc = "`write(|w| ..)` method takes [slc_ahb_test_reg::W](slc_ahb_test_reg::W) writer structure"]
impl crate::Writable for SLC_AHB_TEST_REG {}
#[doc = "SLC_AHB_TEST_REG"]
pub mod slc_ahb_test_reg;
#[doc = "SLC_SDIO_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_sdio_st_reg](slc_sdio_st_reg) module"]
pub type SLC_SDIO_ST_REG = crate::Reg<u32, _SLC_SDIO_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_SDIO_ST_REG;
#[doc = "`read()` method returns [slc_sdio_st_reg::R](slc_sdio_st_reg::R) reader structure"]
impl crate::Readable for SLC_SDIO_ST_REG {}
#[doc = "`write(|w| ..)` method takes [slc_sdio_st_reg::W](slc_sdio_st_reg::W) writer structure"]
impl crate::Writable for SLC_SDIO_ST_REG {}
#[doc = "SLC_SDIO_ST_REG"]
pub mod slc_sdio_st_reg;
#[doc = "SLC_RX_DSCR_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_rx_dscr_conf_reg](slc_rx_dscr_conf_reg) module"]
pub type SLC_RX_DSCR_CONF_REG = crate::Reg<u32, _SLC_RX_DSCR_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_RX_DSCR_CONF_REG;
#[doc = "`read()` method returns [slc_rx_dscr_conf_reg::R](slc_rx_dscr_conf_reg::R) reader structure"]
impl crate::Readable for SLC_RX_DSCR_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [slc_rx_dscr_conf_reg::W](slc_rx_dscr_conf_reg::W) writer structure"]
impl crate::Writable for SLC_RX_DSCR_CONF_REG {}
#[doc = "SLC_RX_DSCR_CONF_REG"]
pub mod slc_rx_dscr_conf_reg;
#[doc = "SLC_0_TXLINK_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_txlink_dscr_reg](slc_0_txlink_dscr_reg) module"]
pub type SLC_0_TXLINK_DSCR_REG = crate::Reg<u32, _SLC_0_TXLINK_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TXLINK_DSCR_REG;
#[doc = "`read()` method returns [slc_0_txlink_dscr_reg::R](slc_0_txlink_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TXLINK_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_txlink_dscr_reg::W](slc_0_txlink_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TXLINK_DSCR_REG {}
#[doc = "SLC_0_TXLINK_DSCR_REG"]
pub mod slc_0_txlink_dscr_reg;
#[doc = "SLC_0_TXLINK_DSCR_BF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_txlink_dscr_bf0_reg](slc_0_txlink_dscr_bf0_reg) module"]
pub type SLC_0_TXLINK_DSCR_BF0_REG = crate::Reg<u32, _SLC_0_TXLINK_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TXLINK_DSCR_BF0_REG;
#[doc = "`read()` method returns [slc_0_txlink_dscr_bf0_reg::R](slc_0_txlink_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for SLC_0_TXLINK_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_txlink_dscr_bf0_reg::W](slc_0_txlink_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for SLC_0_TXLINK_DSCR_BF0_REG {}
#[doc = "SLC_0_TXLINK_DSCR_BF0_REG"]
pub mod slc_0_txlink_dscr_bf0_reg;
#[doc = "SLC_0_TXLINK_DSCR_BF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_txlink_dscr_bf1_reg](slc_0_txlink_dscr_bf1_reg) module"]
pub type SLC_0_TXLINK_DSCR_BF1_REG = crate::Reg<u32, _SLC_0_TXLINK_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TXLINK_DSCR_BF1_REG;
#[doc = "`read()` method returns [slc_0_txlink_dscr_bf1_reg::R](slc_0_txlink_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for SLC_0_TXLINK_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_txlink_dscr_bf1_reg::W](slc_0_txlink_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for SLC_0_TXLINK_DSCR_BF1_REG {}
#[doc = "SLC_0_TXLINK_DSCR_BF1_REG"]
pub mod slc_0_txlink_dscr_bf1_reg;
#[doc = "SLC_0_RXLINK_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_rxlink_dscr_reg](slc_0_rxlink_dscr_reg) module"]
pub type SLC_0_RXLINK_DSCR_REG = crate::Reg<u32, _SLC_0_RXLINK_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_RXLINK_DSCR_REG;
#[doc = "`read()` method returns [slc_0_rxlink_dscr_reg::R](slc_0_rxlink_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_RXLINK_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_rxlink_dscr_reg::W](slc_0_rxlink_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_RXLINK_DSCR_REG {}
#[doc = "SLC_0_RXLINK_DSCR_REG"]
pub mod slc_0_rxlink_dscr_reg;
#[doc = "SLC_0_RXLINK_DSCR_BF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_rxlink_dscr_bf0_reg](slc_0_rxlink_dscr_bf0_reg) module"]
pub type SLC_0_RXLINK_DSCR_BF0_REG = crate::Reg<u32, _SLC_0_RXLINK_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_RXLINK_DSCR_BF0_REG;
#[doc = "`read()` method returns [slc_0_rxlink_dscr_bf0_reg::R](slc_0_rxlink_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for SLC_0_RXLINK_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_rxlink_dscr_bf0_reg::W](slc_0_rxlink_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for SLC_0_RXLINK_DSCR_BF0_REG {}
#[doc = "SLC_0_RXLINK_DSCR_BF0_REG"]
pub mod slc_0_rxlink_dscr_bf0_reg;
#[doc = "SLC_0_RXLINK_DSCR_BF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_rxlink_dscr_bf1_reg](slc_0_rxlink_dscr_bf1_reg) module"]
pub type SLC_0_RXLINK_DSCR_BF1_REG = crate::Reg<u32, _SLC_0_RXLINK_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_RXLINK_DSCR_BF1_REG;
#[doc = "`read()` method returns [slc_0_rxlink_dscr_bf1_reg::R](slc_0_rxlink_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for SLC_0_RXLINK_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_rxlink_dscr_bf1_reg::W](slc_0_rxlink_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for SLC_0_RXLINK_DSCR_BF1_REG {}
#[doc = "SLC_0_RXLINK_DSCR_BF1_REG"]
pub mod slc_0_rxlink_dscr_bf1_reg;
#[doc = "SLC_1_TXLINK_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_txlink_dscr_reg](slc_1_txlink_dscr_reg) module"]
pub type SLC_1_TXLINK_DSCR_REG = crate::Reg<u32, _SLC_1_TXLINK_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_TXLINK_DSCR_REG;
#[doc = "`read()` method returns [slc_1_txlink_dscr_reg::R](slc_1_txlink_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_1_TXLINK_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_txlink_dscr_reg::W](slc_1_txlink_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_1_TXLINK_DSCR_REG {}
#[doc = "SLC_1_TXLINK_DSCR_REG"]
pub mod slc_1_txlink_dscr_reg;
#[doc = "SLC_1_TXLINK_DSCR_BF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_txlink_dscr_bf0_reg](slc_1_txlink_dscr_bf0_reg) module"]
pub type SLC_1_TXLINK_DSCR_BF0_REG = crate::Reg<u32, _SLC_1_TXLINK_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_TXLINK_DSCR_BF0_REG;
#[doc = "`read()` method returns [slc_1_txlink_dscr_bf0_reg::R](slc_1_txlink_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for SLC_1_TXLINK_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_txlink_dscr_bf0_reg::W](slc_1_txlink_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for SLC_1_TXLINK_DSCR_BF0_REG {}
#[doc = "SLC_1_TXLINK_DSCR_BF0_REG"]
pub mod slc_1_txlink_dscr_bf0_reg;
#[doc = "SLC_1_TXLINK_DSCR_BF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_txlink_dscr_bf1_reg](slc_1_txlink_dscr_bf1_reg) module"]
pub type SLC_1_TXLINK_DSCR_BF1_REG = crate::Reg<u32, _SLC_1_TXLINK_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_TXLINK_DSCR_BF1_REG;
#[doc = "`read()` method returns [slc_1_txlink_dscr_bf1_reg::R](slc_1_txlink_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for SLC_1_TXLINK_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_txlink_dscr_bf1_reg::W](slc_1_txlink_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for SLC_1_TXLINK_DSCR_BF1_REG {}
#[doc = "SLC_1_TXLINK_DSCR_BF1_REG"]
pub mod slc_1_txlink_dscr_bf1_reg;
#[doc = "SLC_1_RXLINK_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_rxlink_dscr_reg](slc_1_rxlink_dscr_reg) module"]
pub type SLC_1_RXLINK_DSCR_REG = crate::Reg<u32, _SLC_1_RXLINK_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_RXLINK_DSCR_REG;
#[doc = "`read()` method returns [slc_1_rxlink_dscr_reg::R](slc_1_rxlink_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_1_RXLINK_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_rxlink_dscr_reg::W](slc_1_rxlink_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_1_RXLINK_DSCR_REG {}
#[doc = "SLC_1_RXLINK_DSCR_REG"]
pub mod slc_1_rxlink_dscr_reg;
#[doc = "SLC_1_RXLINK_DSCR_BF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_rxlink_dscr_bf0_reg](slc_1_rxlink_dscr_bf0_reg) module"]
pub type SLC_1_RXLINK_DSCR_BF0_REG = crate::Reg<u32, _SLC_1_RXLINK_DSCR_BF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_RXLINK_DSCR_BF0_REG;
#[doc = "`read()` method returns [slc_1_rxlink_dscr_bf0_reg::R](slc_1_rxlink_dscr_bf0_reg::R) reader structure"]
impl crate::Readable for SLC_1_RXLINK_DSCR_BF0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_rxlink_dscr_bf0_reg::W](slc_1_rxlink_dscr_bf0_reg::W) writer structure"]
impl crate::Writable for SLC_1_RXLINK_DSCR_BF0_REG {}
#[doc = "SLC_1_RXLINK_DSCR_BF0_REG"]
pub mod slc_1_rxlink_dscr_bf0_reg;
#[doc = "SLC_1_RXLINK_DSCR_BF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_rxlink_dscr_bf1_reg](slc_1_rxlink_dscr_bf1_reg) module"]
pub type SLC_1_RXLINK_DSCR_BF1_REG = crate::Reg<u32, _SLC_1_RXLINK_DSCR_BF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_RXLINK_DSCR_BF1_REG;
#[doc = "`read()` method returns [slc_1_rxlink_dscr_bf1_reg::R](slc_1_rxlink_dscr_bf1_reg::R) reader structure"]
impl crate::Readable for SLC_1_RXLINK_DSCR_BF1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_rxlink_dscr_bf1_reg::W](slc_1_rxlink_dscr_bf1_reg::W) writer structure"]
impl crate::Writable for SLC_1_RXLINK_DSCR_BF1_REG {}
#[doc = "SLC_1_RXLINK_DSCR_BF1_REG"]
pub mod slc_1_rxlink_dscr_bf1_reg;
#[doc = "SLC_0_TX_ERREOF_DES_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_tx_erreof_des_addr_reg](slc_0_tx_erreof_des_addr_reg) module"]
pub type SLC_0_TX_ERREOF_DES_ADDR_REG = crate::Reg<u32, _SLC_0_TX_ERREOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TX_ERREOF_DES_ADDR_REG;
#[doc = "`read()` method returns [slc_0_tx_erreof_des_addr_reg::R](slc_0_tx_erreof_des_addr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TX_ERREOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_tx_erreof_des_addr_reg::W](slc_0_tx_erreof_des_addr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TX_ERREOF_DES_ADDR_REG {}
#[doc = "SLC_0_TX_ERREOF_DES_ADDR_REG"]
pub mod slc_0_tx_erreof_des_addr_reg;
#[doc = "SLC_1_TX_ERREOF_DES_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1_tx_erreof_des_addr_reg](slc_1_tx_erreof_des_addr_reg) module"]
pub type SLC_1_TX_ERREOF_DES_ADDR_REG = crate::Reg<u32, _SLC_1_TX_ERREOF_DES_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1_TX_ERREOF_DES_ADDR_REG;
#[doc = "`read()` method returns [slc_1_tx_erreof_des_addr_reg::R](slc_1_tx_erreof_des_addr_reg::R) reader structure"]
impl crate::Readable for SLC_1_TX_ERREOF_DES_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1_tx_erreof_des_addr_reg::W](slc_1_tx_erreof_des_addr_reg::W) writer structure"]
impl crate::Writable for SLC_1_TX_ERREOF_DES_ADDR_REG {}
#[doc = "SLC_1_TX_ERREOF_DES_ADDR_REG"]
pub mod slc_1_tx_erreof_des_addr_reg;
#[doc = "SLC_TOKEN_LAT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_token_lat_reg](slc_token_lat_reg) module"]
pub type SLC_TOKEN_LAT_REG = crate::Reg<u32, _SLC_TOKEN_LAT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_TOKEN_LAT_REG;
#[doc = "`read()` method returns [slc_token_lat_reg::R](slc_token_lat_reg::R) reader structure"]
impl crate::Readable for SLC_TOKEN_LAT_REG {}
#[doc = "`write(|w| ..)` method takes [slc_token_lat_reg::W](slc_token_lat_reg::W) writer structure"]
impl crate::Writable for SLC_TOKEN_LAT_REG {}
#[doc = "SLC_TOKEN_LAT_REG"]
pub mod slc_token_lat_reg;
#[doc = "SLC_TX_DSCR_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_tx_dscr_conf_reg](slc_tx_dscr_conf_reg) module"]
pub type SLC_TX_DSCR_CONF_REG = crate::Reg<u32, _SLC_TX_DSCR_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_TX_DSCR_CONF_REG;
#[doc = "`read()` method returns [slc_tx_dscr_conf_reg::R](slc_tx_dscr_conf_reg::R) reader structure"]
impl crate::Readable for SLC_TX_DSCR_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [slc_tx_dscr_conf_reg::W](slc_tx_dscr_conf_reg::W) writer structure"]
impl crate::Writable for SLC_TX_DSCR_CONF_REG {}
#[doc = "SLC_TX_DSCR_CONF_REG"]
pub mod slc_tx_dscr_conf_reg;
#[doc = "SLC_CMD_INFOR0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_cmd_infor0_reg](slc_cmd_infor0_reg) module"]
pub type SLC_CMD_INFOR0_REG = crate::Reg<u32, _SLC_CMD_INFOR0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_CMD_INFOR0_REG;
#[doc = "`read()` method returns [slc_cmd_infor0_reg::R](slc_cmd_infor0_reg::R) reader structure"]
impl crate::Readable for SLC_CMD_INFOR0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_cmd_infor0_reg::W](slc_cmd_infor0_reg::W) writer structure"]
impl crate::Writable for SLC_CMD_INFOR0_REG {}
#[doc = "SLC_CMD_INFOR0_REG"]
pub mod slc_cmd_infor0_reg;
#[doc = "SLC_CMD_INFOR1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_cmd_infor1_reg](slc_cmd_infor1_reg) module"]
pub type SLC_CMD_INFOR1_REG = crate::Reg<u32, _SLC_CMD_INFOR1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_CMD_INFOR1_REG;
#[doc = "`read()` method returns [slc_cmd_infor1_reg::R](slc_cmd_infor1_reg::R) reader structure"]
impl crate::Readable for SLC_CMD_INFOR1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_cmd_infor1_reg::W](slc_cmd_infor1_reg::W) writer structure"]
impl crate::Writable for SLC_CMD_INFOR1_REG {}
#[doc = "SLC_CMD_INFOR1_REG"]
pub mod slc_cmd_infor1_reg;
#[doc = "SLC_0_LEN_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_len_conf_reg](slc_0_len_conf_reg) module"]
pub type SLC_0_LEN_CONF_REG = crate::Reg<u32, _SLC_0_LEN_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_LEN_CONF_REG;
#[doc = "`read()` method returns [slc_0_len_conf_reg::R](slc_0_len_conf_reg::R) reader structure"]
impl crate::Readable for SLC_0_LEN_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_len_conf_reg::W](slc_0_len_conf_reg::W) writer structure"]
impl crate::Writable for SLC_0_LEN_CONF_REG {}
#[doc = "SLC_0_LEN_CONF_REG"]
pub mod slc_0_len_conf_reg;
#[doc = "SLC_0_LENGTH_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_length_reg](slc_0_length_reg) module"]
pub type SLC_0_LENGTH_REG = crate::Reg<u32, _SLC_0_LENGTH_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_LENGTH_REG;
#[doc = "`read()` method returns [slc_0_length_reg::R](slc_0_length_reg::R) reader structure"]
impl crate::Readable for SLC_0_LENGTH_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_length_reg::W](slc_0_length_reg::W) writer structure"]
impl crate::Writable for SLC_0_LENGTH_REG {}
#[doc = "SLC_0_LENGTH_REG"]
pub mod slc_0_length_reg;
#[doc = "SLC_0_TXPKT_H_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_txpkt_h_dscr_reg](slc_0_txpkt_h_dscr_reg) module"]
pub type SLC_0_TXPKT_H_DSCR_REG = crate::Reg<u32, _SLC_0_TXPKT_H_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TXPKT_H_DSCR_REG;
#[doc = "`read()` method returns [slc_0_txpkt_h_dscr_reg::R](slc_0_txpkt_h_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TXPKT_H_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_txpkt_h_dscr_reg::W](slc_0_txpkt_h_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TXPKT_H_DSCR_REG {}
#[doc = "SLC_0_TXPKT_H_DSCR_REG"]
pub mod slc_0_txpkt_h_dscr_reg;
#[doc = "SLC_0_TXPKT_E_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_txpkt_e_dscr_reg](slc_0_txpkt_e_dscr_reg) module"]
pub type SLC_0_TXPKT_E_DSCR_REG = crate::Reg<u32, _SLC_0_TXPKT_E_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TXPKT_E_DSCR_REG;
#[doc = "`read()` method returns [slc_0_txpkt_e_dscr_reg::R](slc_0_txpkt_e_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TXPKT_E_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_txpkt_e_dscr_reg::W](slc_0_txpkt_e_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TXPKT_E_DSCR_REG {}
#[doc = "SLC_0_TXPKT_E_DSCR_REG"]
pub mod slc_0_txpkt_e_dscr_reg;
#[doc = "SLC_0_RXPKT_H_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_rxpkt_h_dscr_reg](slc_0_rxpkt_h_dscr_reg) module"]
pub type SLC_0_RXPKT_H_DSCR_REG = crate::Reg<u32, _SLC_0_RXPKT_H_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_RXPKT_H_DSCR_REG;
#[doc = "`read()` method returns [slc_0_rxpkt_h_dscr_reg::R](slc_0_rxpkt_h_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_RXPKT_H_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_rxpkt_h_dscr_reg::W](slc_0_rxpkt_h_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_RXPKT_H_DSCR_REG {}
#[doc = "SLC_0_RXPKT_H_DSCR_REG"]
pub mod slc_0_rxpkt_h_dscr_reg;
#[doc = "SLC_0_RXPKT_E_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_rxpkt_e_dscr_reg](slc_0_rxpkt_e_dscr_reg) module"]
pub type SLC_0_RXPKT_E_DSCR_REG = crate::Reg<u32, _SLC_0_RXPKT_E_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_RXPKT_E_DSCR_REG;
#[doc = "`read()` method returns [slc_0_rxpkt_e_dscr_reg::R](slc_0_rxpkt_e_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_RXPKT_E_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_rxpkt_e_dscr_reg::W](slc_0_rxpkt_e_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_RXPKT_E_DSCR_REG {}
#[doc = "SLC_0_RXPKT_E_DSCR_REG"]
pub mod slc_0_rxpkt_e_dscr_reg;
#[doc = "SLC_0_TXPKTU_H_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_txpktu_h_dscr_reg](slc_0_txpktu_h_dscr_reg) module"]
pub type SLC_0_TXPKTU_H_DSCR_REG = crate::Reg<u32, _SLC_0_TXPKTU_H_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TXPKTU_H_DSCR_REG;
#[doc = "`read()` method returns [slc_0_txpktu_h_dscr_reg::R](slc_0_txpktu_h_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TXPKTU_H_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_txpktu_h_dscr_reg::W](slc_0_txpktu_h_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TXPKTU_H_DSCR_REG {}
#[doc = "SLC_0_TXPKTU_H_DSCR_REG"]
pub mod slc_0_txpktu_h_dscr_reg;
#[doc = "SLC_0_TXPKTU_E_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_txpktu_e_dscr_reg](slc_0_txpktu_e_dscr_reg) module"]
pub type SLC_0_TXPKTU_E_DSCR_REG = crate::Reg<u32, _SLC_0_TXPKTU_E_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_TXPKTU_E_DSCR_REG;
#[doc = "`read()` method returns [slc_0_txpktu_e_dscr_reg::R](slc_0_txpktu_e_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_TXPKTU_E_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_txpktu_e_dscr_reg::W](slc_0_txpktu_e_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_TXPKTU_E_DSCR_REG {}
#[doc = "SLC_0_TXPKTU_E_DSCR_REG"]
pub mod slc_0_txpktu_e_dscr_reg;
#[doc = "SLC_0_RXPKTU_H_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_rxpktu_h_dscr_reg](slc_0_rxpktu_h_dscr_reg) module"]
pub type SLC_0_RXPKTU_H_DSCR_REG = crate::Reg<u32, _SLC_0_RXPKTU_H_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_RXPKTU_H_DSCR_REG;
#[doc = "`read()` method returns [slc_0_rxpktu_h_dscr_reg::R](slc_0_rxpktu_h_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_RXPKTU_H_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_rxpktu_h_dscr_reg::W](slc_0_rxpktu_h_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_RXPKTU_H_DSCR_REG {}
#[doc = "SLC_0_RXPKTU_H_DSCR_REG"]
pub mod slc_0_rxpktu_h_dscr_reg;
#[doc = "SLC_0_RXPKTU_E_DSCR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_rxpktu_e_dscr_reg](slc_0_rxpktu_e_dscr_reg) module"]
pub type SLC_0_RXPKTU_E_DSCR_REG = crate::Reg<u32, _SLC_0_RXPKTU_E_DSCR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_RXPKTU_E_DSCR_REG;
#[doc = "`read()` method returns [slc_0_rxpktu_e_dscr_reg::R](slc_0_rxpktu_e_dscr_reg::R) reader structure"]
impl crate::Readable for SLC_0_RXPKTU_E_DSCR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_rxpktu_e_dscr_reg::W](slc_0_rxpktu_e_dscr_reg::W) writer structure"]
impl crate::Writable for SLC_0_RXPKTU_E_DSCR_REG {}
#[doc = "SLC_0_RXPKTU_E_DSCR_REG"]
pub mod slc_0_rxpktu_e_dscr_reg;
#[doc = "SLC_SEQ_POSITION_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_seq_position_reg](slc_seq_position_reg) module"]
pub type SLC_SEQ_POSITION_REG = crate::Reg<u32, _SLC_SEQ_POSITION_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_SEQ_POSITION_REG;
#[doc = "`read()` method returns [slc_seq_position_reg::R](slc_seq_position_reg::R) reader structure"]
impl crate::Readable for SLC_SEQ_POSITION_REG {}
#[doc = "`write(|w| ..)` method takes [slc_seq_position_reg::W](slc_seq_position_reg::W) writer structure"]
impl crate::Writable for SLC_SEQ_POSITION_REG {}
#[doc = "SLC_SEQ_POSITION_REG"]
pub mod slc_seq_position_reg;
#[doc = "SLC_0_DSCR_REC_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_dscr_rec_conf_reg](slc_0_dscr_rec_conf_reg) module"]
pub type SLC_0_DSCR_REC_CONF_REG = crate::Reg<u32, _SLC_0_DSCR_REC_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_DSCR_REC_CONF_REG;
#[doc = "`read()` method returns [slc_0_dscr_rec_conf_reg::R](slc_0_dscr_rec_conf_reg::R) reader structure"]
impl crate::Readable for SLC_0_DSCR_REC_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_dscr_rec_conf_reg::W](slc_0_dscr_rec_conf_reg::W) writer structure"]
impl crate::Writable for SLC_0_DSCR_REC_CONF_REG {}
#[doc = "SLC_0_DSCR_REC_CONF_REG"]
pub mod slc_0_dscr_rec_conf_reg;
#[doc = "SLC_SDIO_CRC_ST0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_sdio_crc_st0_reg](slc_sdio_crc_st0_reg) module"]
pub type SLC_SDIO_CRC_ST0_REG = crate::Reg<u32, _SLC_SDIO_CRC_ST0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_SDIO_CRC_ST0_REG;
#[doc = "`read()` method returns [slc_sdio_crc_st0_reg::R](slc_sdio_crc_st0_reg::R) reader structure"]
impl crate::Readable for SLC_SDIO_CRC_ST0_REG {}
#[doc = "`write(|w| ..)` method takes [slc_sdio_crc_st0_reg::W](slc_sdio_crc_st0_reg::W) writer structure"]
impl crate::Writable for SLC_SDIO_CRC_ST0_REG {}
#[doc = "SLC_SDIO_CRC_ST0_REG"]
pub mod slc_sdio_crc_st0_reg;
#[doc = "SLC_SDIO_CRC_ST1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_sdio_crc_st1_reg](slc_sdio_crc_st1_reg) module"]
pub type SLC_SDIO_CRC_ST1_REG = crate::Reg<u32, _SLC_SDIO_CRC_ST1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_SDIO_CRC_ST1_REG;
#[doc = "`read()` method returns [slc_sdio_crc_st1_reg::R](slc_sdio_crc_st1_reg::R) reader structure"]
impl crate::Readable for SLC_SDIO_CRC_ST1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_sdio_crc_st1_reg::W](slc_sdio_crc_st1_reg::W) writer structure"]
impl crate::Writable for SLC_SDIO_CRC_ST1_REG {}
#[doc = "SLC_SDIO_CRC_ST1_REG"]
pub mod slc_sdio_crc_st1_reg;
#[doc = "SLC_0_EOF_START_DES_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_eof_start_des_reg](slc_0_eof_start_des_reg) module"]
pub type SLC_0_EOF_START_DES_REG = crate::Reg<u32, _SLC_0_EOF_START_DES_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_EOF_START_DES_REG;
#[doc = "`read()` method returns [slc_0_eof_start_des_reg::R](slc_0_eof_start_des_reg::R) reader structure"]
impl crate::Readable for SLC_0_EOF_START_DES_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_eof_start_des_reg::W](slc_0_eof_start_des_reg::W) writer structure"]
impl crate::Writable for SLC_0_EOF_START_DES_REG {}
#[doc = "SLC_0_EOF_START_DES_REG"]
pub mod slc_0_eof_start_des_reg;
#[doc = "SLC_0_PUSH_DSCR_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_push_dscr_addr_reg](slc_0_push_dscr_addr_reg) module"]
pub type SLC_0_PUSH_DSCR_ADDR_REG = crate::Reg<u32, _SLC_0_PUSH_DSCR_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_PUSH_DSCR_ADDR_REG;
#[doc = "`read()` method returns [slc_0_push_dscr_addr_reg::R](slc_0_push_dscr_addr_reg::R) reader structure"]
impl crate::Readable for SLC_0_PUSH_DSCR_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_push_dscr_addr_reg::W](slc_0_push_dscr_addr_reg::W) writer structure"]
impl crate::Writable for SLC_0_PUSH_DSCR_ADDR_REG {}
#[doc = "SLC_0_PUSH_DSCR_ADDR_REG"]
pub mod slc_0_push_dscr_addr_reg;
#[doc = "SLC_0_DONE_DSCR_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_done_dscr_addr_reg](slc_0_done_dscr_addr_reg) module"]
pub type SLC_0_DONE_DSCR_ADDR_REG = crate::Reg<u32, _SLC_0_DONE_DSCR_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_DONE_DSCR_ADDR_REG;
#[doc = "`read()` method returns [slc_0_done_dscr_addr_reg::R](slc_0_done_dscr_addr_reg::R) reader structure"]
impl crate::Readable for SLC_0_DONE_DSCR_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_done_dscr_addr_reg::W](slc_0_done_dscr_addr_reg::W) writer structure"]
impl crate::Writable for SLC_0_DONE_DSCR_ADDR_REG {}
#[doc = "SLC_0_DONE_DSCR_ADDR_REG"]
pub mod slc_0_done_dscr_addr_reg;
#[doc = "SLC_0_SUB_START_DES_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_sub_start_des_reg](slc_0_sub_start_des_reg) module"]
pub type SLC_0_SUB_START_DES_REG = crate::Reg<u32, _SLC_0_SUB_START_DES_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_SUB_START_DES_REG;
#[doc = "`read()` method returns [slc_0_sub_start_des_reg::R](slc_0_sub_start_des_reg::R) reader structure"]
impl crate::Readable for SLC_0_SUB_START_DES_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_sub_start_des_reg::W](slc_0_sub_start_des_reg::W) writer structure"]
impl crate::Writable for SLC_0_SUB_START_DES_REG {}
#[doc = "SLC_0_SUB_START_DES_REG"]
pub mod slc_0_sub_start_des_reg;
#[doc = "SLC_0_DSCR_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_dscr_cnt_reg](slc_0_dscr_cnt_reg) module"]
pub type SLC_0_DSCR_CNT_REG = crate::Reg<u32, _SLC_0_DSCR_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_DSCR_CNT_REG;
#[doc = "`read()` method returns [slc_0_dscr_cnt_reg::R](slc_0_dscr_cnt_reg::R) reader structure"]
impl crate::Readable for SLC_0_DSCR_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_dscr_cnt_reg::W](slc_0_dscr_cnt_reg::W) writer structure"]
impl crate::Writable for SLC_0_DSCR_CNT_REG {}
#[doc = "SLC_0_DSCR_CNT_REG"]
pub mod slc_0_dscr_cnt_reg;
#[doc = "SLC_0_LEN_LIM_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0_len_lim_conf_reg](slc_0_len_lim_conf_reg) module"]
pub type SLC_0_LEN_LIM_CONF_REG = crate::Reg<u32, _SLC_0_LEN_LIM_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0_LEN_LIM_CONF_REG;
#[doc = "`read()` method returns [slc_0_len_lim_conf_reg::R](slc_0_len_lim_conf_reg::R) reader structure"]
impl crate::Readable for SLC_0_LEN_LIM_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0_len_lim_conf_reg::W](slc_0_len_lim_conf_reg::W) writer structure"]
impl crate::Writable for SLC_0_LEN_LIM_CONF_REG {}
#[doc = "SLC_0_LEN_LIM_CONF_REG"]
pub mod slc_0_len_lim_conf_reg;
#[doc = "SLC_0INT_ST1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0int_st1_reg](slc_0int_st1_reg) module"]
pub type SLC_0INT_ST1_REG = crate::Reg<u32, _SLC_0INT_ST1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0INT_ST1_REG;
#[doc = "`read()` method returns [slc_0int_st1_reg::R](slc_0int_st1_reg::R) reader structure"]
impl crate::Readable for SLC_0INT_ST1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0int_st1_reg::W](slc_0int_st1_reg::W) writer structure"]
impl crate::Writable for SLC_0INT_ST1_REG {}
#[doc = "SLC_0INT_ST1_REG"]
pub mod slc_0int_st1_reg;
#[doc = "SLC_0INT_ENA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_0int_ena1_reg](slc_0int_ena1_reg) module"]
pub type SLC_0INT_ENA1_REG = crate::Reg<u32, _SLC_0INT_ENA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_0INT_ENA1_REG;
#[doc = "`read()` method returns [slc_0int_ena1_reg::R](slc_0int_ena1_reg::R) reader structure"]
impl crate::Readable for SLC_0INT_ENA1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_0int_ena1_reg::W](slc_0int_ena1_reg::W) writer structure"]
impl crate::Writable for SLC_0INT_ENA1_REG {}
#[doc = "SLC_0INT_ENA1_REG"]
pub mod slc_0int_ena1_reg;
#[doc = "SLC_1INT_ST1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1int_st1_reg](slc_1int_st1_reg) module"]
pub type SLC_1INT_ST1_REG = crate::Reg<u32, _SLC_1INT_ST1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1INT_ST1_REG;
#[doc = "`read()` method returns [slc_1int_st1_reg::R](slc_1int_st1_reg::R) reader structure"]
impl crate::Readable for SLC_1INT_ST1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1int_st1_reg::W](slc_1int_st1_reg::W) writer structure"]
impl crate::Writable for SLC_1INT_ST1_REG {}
#[doc = "SLC_1INT_ST1_REG"]
pub mod slc_1int_st1_reg;
#[doc = "SLC_1INT_ENA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_1int_ena1_reg](slc_1int_ena1_reg) module"]
pub type SLC_1INT_ENA1_REG = crate::Reg<u32, _SLC_1INT_ENA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_1INT_ENA1_REG;
#[doc = "`read()` method returns [slc_1int_ena1_reg::R](slc_1int_ena1_reg::R) reader structure"]
impl crate::Readable for SLC_1INT_ENA1_REG {}
#[doc = "`write(|w| ..)` method takes [slc_1int_ena1_reg::W](slc_1int_ena1_reg::W) writer structure"]
impl crate::Writable for SLC_1INT_ENA1_REG {}
#[doc = "SLC_1INT_ENA1_REG"]
pub mod slc_1int_ena1_reg;
#[doc = "SLC_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_date_reg](slc_date_reg) module"]
pub type SLC_DATE_REG = crate::Reg<u32, _SLC_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_DATE_REG;
#[doc = "`read()` method returns [slc_date_reg::R](slc_date_reg::R) reader structure"]
impl crate::Readable for SLC_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [slc_date_reg::W](slc_date_reg::W) writer structure"]
impl crate::Writable for SLC_DATE_REG {}
#[doc = "SLC_DATE_REG"]
pub mod slc_date_reg;
#[doc = "SLC_ID_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slc_id_reg](slc_id_reg) module"]
pub type SLC_ID_REG = crate::Reg<u32, _SLC_ID_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLC_ID_REG;
#[doc = "`read()` method returns [slc_id_reg::R](slc_id_reg::R) reader structure"]
impl crate::Readable for SLC_ID_REG {}
#[doc = "`write(|w| ..)` method takes [slc_id_reg::W](slc_id_reg::W) writer structure"]
impl crate::Writable for SLC_ID_REG {}
#[doc = "SLC_ID_REG"]
pub mod slc_id_reg;
