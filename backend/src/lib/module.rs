use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Module {}

impl Module {
    pub fn new(pool: PgPool) -> Self {
        Self {}
    }
}
