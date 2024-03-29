#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - No Description"]
    pub xtalcfg: XTALCFG,
    _reserved2: [u8; 0x04],
    #[doc = "0x18 - No Description"]
    pub xtalctrl: XTALCTRL,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - No Description"]
    pub cfg: CFG,
    _reserved4: [u8; 0x04],
    #[doc = "0x28 - No Description"]
    pub ctrl: CTRL,
    _reserved5: [u8; 0x24],
    #[doc = "0x50 - No Description"]
    pub cmd: CMD,
    _reserved6: [u8; 0x04],
    #[doc = "0x58 - No Description"]
    pub status: STATUS,
    _reserved7: [u8; 0x14],
    #[doc = "0x70 - No Description"]
    pub if_: IF,
    #[doc = "0x74 - No Description"]
    pub ien: IEN,
    _reserved9: [u8; 0x08],
    #[doc = "0x80 - No Description"]
    pub lock: LOCK,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "XTALCFG (rw) register accessor: an alias for `Reg<XTALCFG_SPEC>`"]
pub type XTALCFG = crate::Reg<xtalcfg::XTALCFG_SPEC>;
#[doc = "No Description"]
pub mod xtalcfg;
#[doc = "XTALCTRL (rw) register accessor: an alias for `Reg<XTALCTRL_SPEC>`"]
pub type XTALCTRL = crate::Reg<xtalctrl::XTALCTRL_SPEC>;
#[doc = "No Description"]
pub mod xtalctrl;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
