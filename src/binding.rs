/* automatically generated by rust-bindgen */

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused)]use libc::*;type __be16 = u16;type __be32 = u32;type __be64 = u64;

pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const NF_NETLINK_CONNTRACK_NEW: u32 = 1;
pub const NF_NETLINK_CONNTRACK_UPDATE: u32 = 2;
pub const NF_NETLINK_CONNTRACK_DESTROY: u32 = 4;
pub const NF_NETLINK_CONNTRACK_EXP_NEW: u32 = 8;
pub const NF_NETLINK_CONNTRACK_EXP_UPDATE: u32 = 16;
pub const NF_NETLINK_CONNTRACK_EXP_DESTROY: u32 = 32;
pub const NFNL_NFA_NEST: u32 = 32768;
pub const NFA_ALIGNTO: u32 = 4;
pub const NFNETLINK_V0: u32 = 0;
pub const NFNL_SUBSYS_NONE: u32 = 0;
pub const NFNL_SUBSYS_CTNETLINK: u32 = 1;
pub const NFNL_SUBSYS_CTNETLINK_EXP: u32 = 2;
pub const NFNL_SUBSYS_QUEUE: u32 = 3;
pub const NFNL_SUBSYS_ULOG: u32 = 4;
pub const NFNL_SUBSYS_OSF: u32 = 5;
pub const NFNL_SUBSYS_IPSET: u32 = 6;
pub const NFNL_SUBSYS_ACCT: u32 = 7;
pub const NFNL_SUBSYS_CTNETLINK_TIMEOUT: u32 = 8;
pub const NFNL_SUBSYS_CTHELPER: u32 = 9;
pub const NFNL_SUBSYS_NFTABLES: u32 = 10;
pub const NFNL_SUBSYS_NFT_COMPAT: u32 = 11;
pub const NFNL_SUBSYS_COUNT: u32 = 12;
pub const NFQA_CFG_F_FAIL_OPEN: u32 = 1;
pub const NFQA_CFG_F_CONNTRACK: u32 = 2;
pub const NFQA_CFG_F_GSO: u32 = 4;
pub const NFQA_CFG_F_UID_GID: u32 = 8;
pub const NFQA_CFG_F_SECCTX: u32 = 16;
pub const NFQA_CFG_F_MAX: u32 = 32;
pub const NFQA_SKB_CSUMNOTREADY: u32 = 1;
pub const NFQA_SKB_GSO: u32 = 2;
pub const NFQA_SKB_CSUM_NOTVERIFIED: u32 = 4;
#[repr(C)]
pub struct nfattr {
    pub nfa_len: __u16,
    pub nfa_type: __u16,
}
#[test]
fn bindgen_test_layout_nfattr() {
    assert_eq!(
        ::core::mem::size_of::<nfattr>(),
        4usize,
        concat!("Size of: ", stringify!(nfattr))
    );
    assert_eq!(
        ::core::mem::align_of::<nfattr>(),
        2usize,
        concat!("Alignment of ", stringify!(nfattr))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfattr>())).nfa_len as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfattr),
            "::",
            stringify!(nfa_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfattr>())).nfa_type as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(nfattr),
            "::",
            stringify!(nfa_type)
        )
    );
}
pub const NFNLGRP_NONE: nfnetlink_groups = 0;
pub const NFNLGRP_CONNTRACK_NEW: nfnetlink_groups = 1;
pub const NFNLGRP_CONNTRACK_UPDATE: nfnetlink_groups = 2;
pub const NFNLGRP_CONNTRACK_DESTROY: nfnetlink_groups = 3;
pub const NFNLGRP_CONNTRACK_EXP_NEW: nfnetlink_groups = 4;
pub const NFNLGRP_CONNTRACK_EXP_UPDATE: nfnetlink_groups = 5;
pub const NFNLGRP_CONNTRACK_EXP_DESTROY: nfnetlink_groups = 6;
pub const NFNLGRP_NFTABLES: nfnetlink_groups = 7;
pub const NFNLGRP_ACCT_QUOTA: nfnetlink_groups = 8;
pub const NFNLGRP_NFTRACE: nfnetlink_groups = 9;
pub const __NFNLGRP_MAX: nfnetlink_groups = 10;
pub type nfnetlink_groups = u32;
#[repr(C)]
pub struct nfgenmsg {
    pub nfgen_family: __u8,
    pub version: __u8,
    pub res_id: __be16,
}
#[test]
fn bindgen_test_layout_nfgenmsg() {
    assert_eq!(
        ::core::mem::size_of::<nfgenmsg>(),
        4usize,
        concat!("Size of: ", stringify!(nfgenmsg))
    );
    assert_eq!(
        ::core::mem::align_of::<nfgenmsg>(),
        2usize,
        concat!("Alignment of ", stringify!(nfgenmsg))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfgenmsg>())).nfgen_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfgenmsg),
            "::",
            stringify!(nfgen_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfgenmsg>())).version as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(nfgenmsg),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfgenmsg>())).res_id as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(nfgenmsg),
            "::",
            stringify!(res_id)
        )
    );
}
pub const NFNL_BATCH_UNSPEC: nfnl_batch_attributes = 0;
pub const NFNL_BATCH_GENID: nfnl_batch_attributes = 1;
pub const __NFNL_BATCH_MAX: nfnl_batch_attributes = 2;
pub type nfnl_batch_attributes = u32;
pub const NFQNL_MSG_PACKET: nfqnl_msg_types = 0;
pub const NFQNL_MSG_VERDICT: nfqnl_msg_types = 1;
pub const NFQNL_MSG_CONFIG: nfqnl_msg_types = 2;
pub const NFQNL_MSG_VERDICT_BATCH: nfqnl_msg_types = 3;
pub const NFQNL_MSG_MAX: nfqnl_msg_types = 4;
pub type nfqnl_msg_types = u32;
#[repr(C, packed)]
pub struct nfqnl_msg_packet_hdr {
    pub packet_id: __be32,
    pub hw_protocol: __be16,
    pub hook: __u8,
}
#[test]
fn bindgen_test_layout_nfqnl_msg_packet_hdr() {
    assert_eq!(
        ::core::mem::size_of::<nfqnl_msg_packet_hdr>(),
        7usize,
        concat!("Size of: ", stringify!(nfqnl_msg_packet_hdr))
    );
    assert_eq!(
        ::core::mem::align_of::<nfqnl_msg_packet_hdr>(),
        1usize,
        concat!("Alignment of ", stringify!(nfqnl_msg_packet_hdr))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_packet_hdr>())).packet_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_packet_hdr),
            "::",
            stringify!(packet_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<nfqnl_msg_packet_hdr>())).hw_protocol as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_packet_hdr),
            "::",
            stringify!(hw_protocol)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_packet_hdr>())).hook as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_packet_hdr),
            "::",
            stringify!(hook)
        )
    );
}
#[repr(C)]
pub struct nfqnl_msg_packet_hw {
    pub hw_addrlen: __be16,
    pub _pad: __u16,
    pub hw_addr: [__u8; 8usize],
}
#[test]
fn bindgen_test_layout_nfqnl_msg_packet_hw() {
    assert_eq!(
        ::core::mem::size_of::<nfqnl_msg_packet_hw>(),
        12usize,
        concat!("Size of: ", stringify!(nfqnl_msg_packet_hw))
    );
    assert_eq!(
        ::core::mem::align_of::<nfqnl_msg_packet_hw>(),
        2usize,
        concat!("Alignment of ", stringify!(nfqnl_msg_packet_hw))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_packet_hw>())).hw_addrlen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_packet_hw),
            "::",
            stringify!(hw_addrlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_packet_hw>()))._pad as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_packet_hw),
            "::",
            stringify!(_pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_packet_hw>())).hw_addr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_packet_hw),
            "::",
            stringify!(hw_addr)
        )
    );
}
#[repr(C)]
pub struct nfqnl_msg_packet_timestamp {
    pub sec: __be64,
    pub usec: __be64,
}
#[test]
fn bindgen_test_layout_nfqnl_msg_packet_timestamp() {
    assert_eq!(
        ::core::mem::size_of::<nfqnl_msg_packet_timestamp>(),
        16usize,
        concat!("Size of: ", stringify!(nfqnl_msg_packet_timestamp))
    );
    assert_eq!(
        ::core::mem::align_of::<nfqnl_msg_packet_timestamp>(),
        8usize,
        concat!("Alignment of ", stringify!(nfqnl_msg_packet_timestamp))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_packet_timestamp>())).sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_packet_timestamp),
            "::",
            stringify!(sec)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<nfqnl_msg_packet_timestamp>())).usec as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_packet_timestamp),
            "::",
            stringify!(usec)
        )
    );
}
pub const NFQA_VLAN_UNSPEC: nfqnl_vlan_attr = 0;
pub const NFQA_VLAN_PROTO: nfqnl_vlan_attr = 1;
pub const NFQA_VLAN_TCI: nfqnl_vlan_attr = 2;
pub const __NFQA_VLAN_MAX: nfqnl_vlan_attr = 3;
pub type nfqnl_vlan_attr = u32;
pub const NFQA_UNSPEC: nfqnl_attr_type = 0;
pub const NFQA_PACKET_HDR: nfqnl_attr_type = 1;
pub const NFQA_VERDICT_HDR: nfqnl_attr_type = 2;
pub const NFQA_MARK: nfqnl_attr_type = 3;
pub const NFQA_TIMESTAMP: nfqnl_attr_type = 4;
pub const NFQA_IFINDEX_INDEV: nfqnl_attr_type = 5;
pub const NFQA_IFINDEX_OUTDEV: nfqnl_attr_type = 6;
pub const NFQA_IFINDEX_PHYSINDEV: nfqnl_attr_type = 7;
pub const NFQA_IFINDEX_PHYSOUTDEV: nfqnl_attr_type = 8;
pub const NFQA_HWADDR: nfqnl_attr_type = 9;
pub const NFQA_PAYLOAD: nfqnl_attr_type = 10;
pub const NFQA_CT: nfqnl_attr_type = 11;
pub const NFQA_CT_INFO: nfqnl_attr_type = 12;
pub const NFQA_CAP_LEN: nfqnl_attr_type = 13;
pub const NFQA_SKB_INFO: nfqnl_attr_type = 14;
pub const NFQA_EXP: nfqnl_attr_type = 15;
pub const NFQA_UID: nfqnl_attr_type = 16;
pub const NFQA_GID: nfqnl_attr_type = 17;
pub const NFQA_SECCTX: nfqnl_attr_type = 18;
pub const NFQA_VLAN: nfqnl_attr_type = 19;
pub const NFQA_L2HDR: nfqnl_attr_type = 20;
pub const __NFQA_MAX: nfqnl_attr_type = 21;
pub type nfqnl_attr_type = u32;
#[repr(C)]
pub struct nfqnl_msg_verdict_hdr {
    pub verdict: __be32,
    pub id: __be32,
}
#[test]
fn bindgen_test_layout_nfqnl_msg_verdict_hdr() {
    assert_eq!(
        ::core::mem::size_of::<nfqnl_msg_verdict_hdr>(),
        8usize,
        concat!("Size of: ", stringify!(nfqnl_msg_verdict_hdr))
    );
    assert_eq!(
        ::core::mem::align_of::<nfqnl_msg_verdict_hdr>(),
        4usize,
        concat!("Alignment of ", stringify!(nfqnl_msg_verdict_hdr))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_verdict_hdr>())).verdict as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_verdict_hdr),
            "::",
            stringify!(verdict)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_verdict_hdr>())).id as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_verdict_hdr),
            "::",
            stringify!(id)
        )
    );
}
pub const NFQNL_CFG_CMD_NONE: nfqnl_msg_config_cmds = 0;
pub const NFQNL_CFG_CMD_BIND: nfqnl_msg_config_cmds = 1;
pub const NFQNL_CFG_CMD_UNBIND: nfqnl_msg_config_cmds = 2;
pub const NFQNL_CFG_CMD_PF_BIND: nfqnl_msg_config_cmds = 3;
pub const NFQNL_CFG_CMD_PF_UNBIND: nfqnl_msg_config_cmds = 4;
pub type nfqnl_msg_config_cmds = u32;
#[repr(C)]
pub struct nfqnl_msg_config_cmd {
    pub command: __u8,
    pub _pad: __u8,
    pub pf: __be16,
}
#[test]
fn bindgen_test_layout_nfqnl_msg_config_cmd() {
    assert_eq!(
        ::core::mem::size_of::<nfqnl_msg_config_cmd>(),
        4usize,
        concat!("Size of: ", stringify!(nfqnl_msg_config_cmd))
    );
    assert_eq!(
        ::core::mem::align_of::<nfqnl_msg_config_cmd>(),
        2usize,
        concat!("Alignment of ", stringify!(nfqnl_msg_config_cmd))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_config_cmd>())).command as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_config_cmd),
            "::",
            stringify!(command)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_config_cmd>()))._pad as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_config_cmd),
            "::",
            stringify!(_pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<nfqnl_msg_config_cmd>())).pf as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_config_cmd),
            "::",
            stringify!(pf)
        )
    );
}
pub const NFQNL_COPY_NONE: nfqnl_config_mode = 0;
pub const NFQNL_COPY_META: nfqnl_config_mode = 1;
pub const NFQNL_COPY_PACKET: nfqnl_config_mode = 2;
pub type nfqnl_config_mode = u32;
#[repr(C, packed)]
pub struct nfqnl_msg_config_params {
    pub copy_range: __be32,
    pub copy_mode: __u8,
}
#[test]
fn bindgen_test_layout_nfqnl_msg_config_params() {
    assert_eq!(
        ::core::mem::size_of::<nfqnl_msg_config_params>(),
        5usize,
        concat!("Size of: ", stringify!(nfqnl_msg_config_params))
    );
    assert_eq!(
        ::core::mem::align_of::<nfqnl_msg_config_params>(),
        1usize,
        concat!("Alignment of ", stringify!(nfqnl_msg_config_params))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<nfqnl_msg_config_params>())).copy_range as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_config_params),
            "::",
            stringify!(copy_range)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<nfqnl_msg_config_params>())).copy_mode as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nfqnl_msg_config_params),
            "::",
            stringify!(copy_mode)
        )
    );
}
pub const NFQA_CFG_UNSPEC: nfqnl_attr_config = 0;
pub const NFQA_CFG_CMD: nfqnl_attr_config = 1;
pub const NFQA_CFG_PARAMS: nfqnl_attr_config = 2;
pub const NFQA_CFG_QUEUE_MAXLEN: nfqnl_attr_config = 3;
pub const NFQA_CFG_MASK: nfqnl_attr_config = 4;
pub const NFQA_CFG_FLAGS: nfqnl_attr_config = 5;
pub const __NFQA_CFG_MAX: nfqnl_attr_config = 6;
pub type nfqnl_attr_config = u32;
