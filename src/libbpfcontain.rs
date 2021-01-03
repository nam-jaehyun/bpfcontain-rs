// SPDX-License-Identifier: GPL-2
//
// BPFContain - Container security with eBPF
// Copyright (C) 2020  William Findlay
//
// Dec. 29, 2020  William Findlay  Created this.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use anyhow::{bail, Result};
use bitflags::bitflags;
use plain::Plain;

/// Include bindings from [`bindings.rs`](lib/bindings.rs)
mod bindings {
    include!("lib/bindings.rs");
}

/// Place the current process into a container with ID `container_id`.
pub fn containerize(container_id: libc::c_ulong) -> Result<()> {
    let result = unsafe { bindings::containerize(container_id) };

    match result {
        0 => Ok(()),
        n if n == -libc::EAGAIN => bail! {"Failed to call into uprobe"},
        n if n == -libc::ENOENT => bail! {"No such container with ID {}", container_id},
        n if n == -libc::EINVAL => bail! {"Process is already containerized or no room in map"},
        n => bail! {"Unknown error {}", n},
    }
}

use bindings::capability_t;
use bindings::file_permission_t;
use bindings::net_category_t;
use bindings::net_operation_t;
use bindings::policy_decision_t;

bitflags! {
    /// Represents a policy decision from the BPF side.
    ///
    /// # Warning
    ///
    /// Keep this in sync with [structs.h](src/include/structs.h)
    #[derive(Default)]
    pub struct PolicyDecision :policy_decision_t {
        const NO_DECISION = 0x00;
        const ALLOW       = 0x01;
        const DENY        = 0x02;
    }
}

bitflags! {
    /// Represents the file permissions bitmask on the BPF side.
    ///
    /// # Warning
    ///
    /// Keep this in sync with [structs.h](src/include/structs.h)
    #[derive(Default)]
    pub struct FilePermission :file_permission_t {
        const MAY_EXEC      = 0x00000001;
        const MAY_WRITE     = 0x00000002;
        const MAY_READ      = 0x00000004;
        const MAY_APPEND    = 0x00000008;
        const MAY_CREATE    = 0x00000010;
        const MAY_DELETE    = 0x00000020;
        const MAY_RENAME    = 0x00000040;
        const MAY_SETATTR   = 0x00000080;
        const MAY_CHMOD     = 0x00000100;
        const MAY_CHOWN     = 0x00000200;
        const MAY_LINK      = 0x00000400;
        const MAY_EXEC_MMAP = 0x00000800;
        const MAY_CHDIR     = 0x00001000;
    }
}

bitflags! {
    /// Represents the capabilities bitmask on the BPF side.
    ///
    /// # Warning
    ///
    /// Keep this in sync with [structs.h](src/include/structs.h)
    #[derive(Default)]
    pub struct Capability :capability_t {
        const NET_BIND_SERVICE = 0x00000001;
        const NET_RAW          = 0x00000002;
        const NET_BROADCAST    = 0x00000004;
        const DAC_OVERRIDE     = 0x00000008;
        const DAC_READ_SEARCH  = 0x00000010;
    }
}

bitflags! {
    /// Represents the network categories bitmask on the BPF side.
    ///
    /// # Warning
    ///
    /// Keep this in sync with [structs.h](src/include/structs.h)
    #[derive(Default)]
    pub struct NetCategory :net_category_t {
        const WWW = 0x01;
        const IPC = 0x02;
    }
}

bitflags! {
    /// Represents the network operations bitmask on the BPF side.
    ///
    /// # Warning
    ///
    /// Keep this in sync with [structs.h](src/include/structs.h)
    #[derive(Default)]
    pub struct NetOperation :net_operation_t {
        const NET_CONNECT  = 0x00000001;
        const NET_BIND     = 0x00000002;
        const NET_ACCEPT   = 0x00000004;
        const NET_LISTEN   = 0x00000008;
        const NET_SEND     = 0x00000010;
        const NET_RECV     = 0x00000020;
        const NET_CREATE   = 0x00000040;
        const NET_SHUTDOWN = 0x00000080;
    }
}

/// Represents a container on the BPF side.
///
/// # Warning
///
/// Keep this in sync with [structs.h](src/include/structs.h)
pub use bindings::bpfcon_container;
unsafe impl Plain for bpfcon_container {}

/// Represents a process on the BPF side.
///
/// # Warning
///
/// Keep this in sync with [structs.h](src/include/structs.h)
pub use bindings::bpfcon_process;
unsafe impl Plain for bpfcon_process {}

/// Represents a per-filesystem policy key on the BPF side.
///
/// # Warning
///
/// Keep this in sync with [structs.h](src/include/structs.h)
pub use bindings::fs_policy_key;
unsafe impl Plain for fs_policy_key {}

/// Represents a per-file policy key on the BPF side.
///
/// # Warning
///
/// Keep this in sync with [structs.h](src/include/structs.h)
pub use bindings::file_policy_key;
unsafe impl Plain for file_policy_key {}

/// Represents a per-device policy key on the BPF side.
///
/// # Warning
///
/// Keep this in sync with [structs.h](src/include/structs.h)
pub use bindings::dev_policy_key;
unsafe impl Plain for dev_policy_key {}

/// Represents a capability policy key on the BPF side.
///
/// # Warning
///
/// Keep this in sync with [structs.h](src/include/structs.h)
pub use bindings::cap_policy_key;
unsafe impl Plain for cap_policy_key {}

/// Represents a network policy key on the BPF side.
///
/// # Warning
///
/// Keep this in sync with [structs.h](src/include/structs.h)
pub use bindings::net_policy_key;
unsafe impl Plain for net_policy_key {}

/// Represents a per-inode key on the BPF side.
///
/// # Warning
///
/// Keep this in sync with [structs.h](src/include/structs.h)
pub use bindings::inode_key;
unsafe impl Plain for inode_key {}