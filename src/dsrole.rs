// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Contains public interfaces to query the network roles of workstations, servers, and DCs
ENUM!{enum DSROLE_MACHINE_ROLE {
    DsRole_RoleStandaloneWorkstation,
    DsRole_RoleMemberWorkstation,
    DsRole_RoleStandaloneServer,
    DsRole_RoleMemberServer,
    DsRole_RoleBackupDomainController,
    DsRole_RolePrimaryDomainController,
}}
ENUM!{enum DSROLE_SERVER_STATE {
    DsRoleServerUnknown = 0,
    DsRoleServerPrimary,
    DsRoleServerBackup,
}}
pub type PDSROLE_SERVER_STATE = *mut DSROLE_SERVER_STATE;
ENUM!{enum DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    DsRolePrimaryDomainInfoBasic = 1,
    DsRoleUpgradeStatus,
    DsRoleOperationState,
}}
pub const DSROLE_PRIMARY_DS_RUNNING: ::ULONG = 0x00000001;
pub const DSROLE_PRIMARY_DS_MIXED_MODE: ::ULONG = 0x00000002;
pub const DSROLE_UPGRADE_IN_PROGRESS: ::ULONG = 0x00000004;
pub const DSROLE_PRIMARY_DS_READONLY: ::ULONG = 0x00000008;
pub const DSROLE_PRIMARY_DOMAIN_GUID_PRESENT: ::ULONG = 0x01000000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    pub MachineRole: DSROLE_MACHINE_ROLE,
    pub Flags: ::ULONG,
    pub DomainNameFlat: ::LPWSTR,
    pub DomainNameDns: ::LPWSTR,
    pub DomainForestName: ::LPWSTR,
    pub DomainGuid: ::GUID,
}
pub type PDSROLE_PRIMARY_DOMAIN_INFO_BASIC = *mut DSROLE_PRIMARY_DOMAIN_INFO_BASIC;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSROLE_UPGRADE_STATUS_INFO {
    pub OperationState: ::ULONG,
    pub PreviousServerState: DSROLE_SERVER_STATE,
}
pub type PDSROLE_UPGRADE_STATUS_INFO = *mut DSROLE_UPGRADE_STATUS_INFO;
ENUM!{enum DSROLE_OPERATION_STATE {
    DsRoleOperationIdle = 0,
    DsRoleOperationActive,
    DsRoleOperationNeedReboot,
}}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSROLE_OPERATION_STATE_INFO {
    pub OperationState: DSROLE_OPERATION_STATE,
}
pub type PDSROLE_OPERATION_STATE_INFO = *mut DSROLE_OPERATION_STATE_INFO;
