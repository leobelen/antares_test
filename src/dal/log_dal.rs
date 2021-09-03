use crate::db::DbConnection;
use crate::models::Log;
use crate::schema::core_log::dsl::*;
use crate::traits::{BaseRepositoryTrait, BaseUpdateRepositoryTrait};
use diesel::prelude::*;

pub struct LogDal;

impl BaseRepositoryTrait<core_log, Log> for LogDal {};
impl BaseUpdateRepositoryTrait<core_log, Log> for LogDal {};
