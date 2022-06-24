#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - No Description"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - No Description"]
    pub cal: crate::Reg<cal::CAL_SPEC>,
    #[doc = "0x0c - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x10 - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x14 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - No Description"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CAL register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "No Description"]
pub mod cal;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
