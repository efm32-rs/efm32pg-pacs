#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Register for status flags."]
    pub nsstatus: NSSTATUS,
    #[doc = "0x08 - Register used to lock/unlock access to the register file."]
    pub nslock: NSLOCK,
    #[doc = "0x0c - Register for interrupt status flags."]
    pub nsif: NSIF,
    #[doc = "0x10 - Register used for enabling/disabling interrupts."]
    pub nsien: NSIEN,
    _reserved4: [u8; 0x2c],
    #[doc = "0x40 - Set peripheral bits to 1 to mark as privileged access only."]
    pub ppunspatd0: PPUNSPATD0,
    #[doc = "0x44 - Set peripheral bits to 1 to mark as privileged access only."]
    pub ppunspatd1: PPUNSPATD1,
    _reserved6: [u8; 0xf8],
    #[doc = "0x140 - Read this register to query the fault status."]
    pub ppunsfs: PPUNSFS,
    _reserved7: [u8; 0x0c],
    #[doc = "0x150 - Write to set BMPU priveledged attributes."]
    pub bmpunspatd0: BMPUNSPATD0,
}
#[doc = "NSSTATUS (r) register accessor: an alias for `Reg<NSSTATUS_SPEC>`"]
pub type NSSTATUS = crate::Reg<nsstatus::NSSTATUS_SPEC>;
#[doc = "Register for status flags."]
pub mod nsstatus;
#[doc = "NSLOCK (w) register accessor: an alias for `Reg<NSLOCK_SPEC>`"]
pub type NSLOCK = crate::Reg<nslock::NSLOCK_SPEC>;
#[doc = "Register used to lock/unlock access to the register file."]
pub mod nslock;
#[doc = "NSIF (rw) register accessor: an alias for `Reg<NSIF_SPEC>`"]
pub type NSIF = crate::Reg<nsif::NSIF_SPEC>;
#[doc = "Register for interrupt status flags."]
pub mod nsif;
#[doc = "NSIEN (rw) register accessor: an alias for `Reg<NSIEN_SPEC>`"]
pub type NSIEN = crate::Reg<nsien::NSIEN_SPEC>;
#[doc = "Register used for enabling/disabling interrupts."]
pub mod nsien;
#[doc = "PPUNSPATD0 (rw) register accessor: an alias for `Reg<PPUNSPATD0_SPEC>`"]
pub type PPUNSPATD0 = crate::Reg<ppunspatd0::PPUNSPATD0_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only."]
pub mod ppunspatd0;
#[doc = "PPUNSPATD1 (rw) register accessor: an alias for `Reg<PPUNSPATD1_SPEC>`"]
pub type PPUNSPATD1 = crate::Reg<ppunspatd1::PPUNSPATD1_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only."]
pub mod ppunspatd1;
#[doc = "PPUNSFS (r) register accessor: an alias for `Reg<PPUNSFS_SPEC>`"]
pub type PPUNSFS = crate::Reg<ppunsfs::PPUNSFS_SPEC>;
#[doc = "Read this register to query the fault status."]
pub mod ppunsfs;
#[doc = "BMPUNSPATD0 (rw) register accessor: an alias for `Reg<BMPUNSPATD0_SPEC>`"]
pub type BMPUNSPATD0 = crate::Reg<bmpunspatd0::BMPUNSPATD0_SPEC>;
#[doc = "Write to set BMPU priveledged attributes."]
pub mod bmpunspatd0;
