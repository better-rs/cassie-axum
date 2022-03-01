use rbatis::rbatis::Rbatis;

pub mod asi;
pub mod sys;

pub use sys::*;

use self::{
    asi::asi_service::AsiGroupService,
    cache_service::CacheService,
    sys_auth_service::SysAuthService,
    sys_dict_service::{SysDictDataService, SysDictTypeService},
    sys_menu_service::SysMenuService,
    sys_params_service::SysParamsService,
    sys_role_service::SysRoleService,
    sys_user_service::SysUserService,
};

pub struct ServiceContext {
    pub rbatis: Rbatis,
    pub cache_service: CacheService,
    /*权限服务 */
    pub sys_auth_service: SysAuthService,
    /*用户服务 */
    pub sys_user_service: SysUserService,
    /*角色服务 */
    pub sys_role_service: SysRoleService,

    pub sys_menu_service: SysMenuService,
    pub sys_params_service: SysParamsService,
    /*数据字典服务 */
    pub sys_dict_type_service: SysDictTypeService,
    pub sys_dict_value_service: SysDictDataService,

    pub asi_service: AsiGroupService,
}

impl ServiceContext {
    pub fn default() -> Self {
        Self {
            rbatis: async_std::task::block_on(async { crate::dao::init_rbatis().await }),
            cache_service: CacheService::new().unwrap(),
            sys_auth_service: Default::default(),
            sys_user_service: Default::default(),
            sys_role_service: Default::default(),
            sys_menu_service: Default::default(),
            sys_params_service: Default::default(),
            sys_dict_type_service: Default::default(),
            sys_dict_value_service: Default::default(),
            asi_service: Default::default(),
        }
    }
}
