#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Read to get system status."]
    pub if_: IF,
    #[doc = "0x04 - Write to enable interrupts."]
    pub ien: IEN,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Read to get the hard-wired chip revision."]
    pub chiprevhw: CHIPREVHW,
    #[doc = "0x14 - Read to get the chip revision programmed by feature configuration."]
    pub chiprev: CHIPREV,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - Configure the source of the system tick for the M33."]
    pub cfgsystic: CFGSYSTIC,
    _reserved5: [u8; 0x01dc],
    #[doc = "0x200 - Configure to provide general RAM configuration."]
    pub ctrl: CTRL,
    _reserved6: [u8; 0x04],
    #[doc = "0x208 - Configure to provide general RAM retention configuration."]
    pub dmem0retnctrl: DMEM0RETNCTRL,
    _reserved7: [u8; 0x04],
    #[doc = "0x210 - Read to get status of the DMEM0 ECC error address."]
    pub dmem0eccaddr: DMEM0ECCADDR,
    #[doc = "0x214 - Configure to set RAM ECC control."]
    pub dmem0eccctrl: DMEM0ECCCTRL,
    _reserved9: [u8; 0x03e8],
    #[doc = "0x600 - Data in this register is passed to the trusted root firmware upon reset."]
    pub rootdata0: ROOTDATA0,
    #[doc = "0x604 - Data in this register is passed to the trusted root firmware upon reset."]
    pub rootdata1: ROOTDATA1,
    #[doc = "0x608 - This register returns the status of the SE managed locks."]
    pub rootlockstatus: ROOTLOCKSTATUS,
}
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Read to get system status."]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Write to enable interrupts."]
pub mod ien;
#[doc = "CHIPREVHW (rw) register accessor: an alias for `Reg<CHIPREVHW_SPEC>`"]
pub type CHIPREVHW = crate::Reg<chiprevhw::CHIPREVHW_SPEC>;
#[doc = "Read to get the hard-wired chip revision."]
pub mod chiprevhw;
#[doc = "CHIPREV (rw) register accessor: an alias for `Reg<CHIPREV_SPEC>`"]
pub type CHIPREV = crate::Reg<chiprev::CHIPREV_SPEC>;
#[doc = "Read to get the chip revision programmed by feature configuration."]
pub mod chiprev;
#[doc = "CFGSYSTIC (rw) register accessor: an alias for `Reg<CFGSYSTIC_SPEC>`"]
pub type CFGSYSTIC = crate::Reg<cfgsystic::CFGSYSTIC_SPEC>;
#[doc = "Configure the source of the system tick for the M33."]
pub mod cfgsystic;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Configure to provide general RAM configuration."]
pub mod ctrl;
#[doc = "DMEM0RETNCTRL (rw) register accessor: an alias for `Reg<DMEM0RETNCTRL_SPEC>`"]
pub type DMEM0RETNCTRL = crate::Reg<dmem0retnctrl::DMEM0RETNCTRL_SPEC>;
#[doc = "Configure to provide general RAM retention configuration."]
pub mod dmem0retnctrl;
#[doc = "DMEM0ECCADDR (r) register accessor: an alias for `Reg<DMEM0ECCADDR_SPEC>`"]
pub type DMEM0ECCADDR = crate::Reg<dmem0eccaddr::DMEM0ECCADDR_SPEC>;
#[doc = "Read to get status of the DMEM0 ECC error address."]
pub mod dmem0eccaddr;
#[doc = "DMEM0ECCCTRL (rw) register accessor: an alias for `Reg<DMEM0ECCCTRL_SPEC>`"]
pub type DMEM0ECCCTRL = crate::Reg<dmem0eccctrl::DMEM0ECCCTRL_SPEC>;
#[doc = "Configure to set RAM ECC control."]
pub mod dmem0eccctrl;
#[doc = "ROOTDATA0 (rw) register accessor: an alias for `Reg<ROOTDATA0_SPEC>`"]
pub type ROOTDATA0 = crate::Reg<rootdata0::ROOTDATA0_SPEC>;
#[doc = "Data in this register is passed to the trusted root firmware upon reset."]
pub mod rootdata0;
#[doc = "ROOTDATA1 (rw) register accessor: an alias for `Reg<ROOTDATA1_SPEC>`"]
pub type ROOTDATA1 = crate::Reg<rootdata1::ROOTDATA1_SPEC>;
#[doc = "Data in this register is passed to the trusted root firmware upon reset."]
pub mod rootdata1;
#[doc = "ROOTLOCKSTATUS (r) register accessor: an alias for `Reg<ROOTLOCKSTATUS_SPEC>`"]
pub type ROOTLOCKSTATUS = crate::Reg<rootlockstatus::ROOTLOCKSTATUS_SPEC>;
#[doc = "This register returns the status of the SE managed locks."]
pub mod rootlockstatus;
