use diesel::r2d2::Pool;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use crate::user::module::{UserModule, UserModuleImpl};
use crate::files::module::{AppFileModuleImpl, AppFileModule};
use crate::config::AppConfig;
use crate::question::module::{QuestionModule, QuestionModuleImpl};

pub struct AppModule {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
    pub config: Box<AppConfig>
}

impl Clone for AppModule{
    fn clone(&self) -> Self{
        return Self {
            pool: self.pool.clone(),
            config: self.config.clone()
        }
    }
}

impl AppModule {
    pub fn user_module(&self) -> Box<dyn UserModule>{
        let pool = self.pool.clone();
        return Box::new(
            UserModuleImpl::new(pool)
        );
    }

    pub fn file_module(&self) -> Box<dyn AppFileModule> {
        let pool = self.pool.clone();
        return Box::new(
            AppFileModuleImpl::new(pool)
        );
    }

    pub fn question_module(&self) -> Box<dyn QuestionModule> {
        return Box::new(
            QuestionModuleImpl {
                pool: self.pool.clone(),
                user_module: self.user_module(),
                config: self.config.clone(),
                file_module: self.file_module()
            }
        )
    }
}

