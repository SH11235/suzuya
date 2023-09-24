use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // itemテーブルにカラム追加
        let sql = r#"
            ALTER TABLE item
            ADD rough_coordinator_id uuid,
            ADD rough_check_person_id uuid,
            ADD line_art_coordinator_id uuid,
            ADD line_art_check_person_id uuid,
            ADD coloring_coordinator_id uuid,
            ADD coloring_check_person_id uuid,
            ADD design_coordinator_id uuid,
            ADD design_check_person_id uuid,
            ADD submission_data_coordinator_id uuid,
            ADD submission_data_check_person_id uuid,
            ADD announcement_materials_coordinator_id uuid,
            ADD announcement_materials_check_person_id uuid,
            ADD jan_coordinator_id uuid,
            ADD jan_check_person_id uuid;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())?;

        // itemテーブルに外部キー追加
        let sql = r#"
            ALTER TABLE item
            ADD CONSTRAINT item_rough_coordinator_id_fkey FOREIGN KEY (rough_coordinator_id) REFERENCES worker (id),
            ADD CONSTRAINT item_rough_check_person_id_fkey FOREIGN KEY (rough_check_person_id) REFERENCES worker (id),
            ADD CONSTRAINT item_line_art_coordinator_id_fkey FOREIGN KEY (line_art_coordinator_id) REFERENCES worker (id),
            ADD CONSTRAINT item_line_art_check_person_id_fkey FOREIGN KEY (line_art_check_person_id) REFERENCES worker (id),
            ADD CONSTRAINT item_coloring_coordinator_id_fkey FOREIGN KEY (coloring_coordinator_id) REFERENCES worker (id),
            ADD CONSTRAINT item_coloring_check_person_id_fkey FOREIGN KEY (coloring_check_person_id) REFERENCES worker (id),
            ADD CONSTRAINT item_design_coordinator_id_fkey FOREIGN KEY (design_coordinator_id) REFERENCES worker (id),
            ADD CONSTRAINT item_design_check_person_id_fkey FOREIGN KEY (design_check_person_id) REFERENCES worker (id),
            ADD CONSTRAINT item_submission_data_coordinator_id_fkey FOREIGN KEY (submission_data_coordinator_id) REFERENCES worker (id),
            ADD CONSTRAINT item_submission_data_check_person_id_fkey FOREIGN KEY (submission_data_check_person_id) REFERENCES worker (id),
            ADD CONSTRAINT item_announcement_materials_coordinator_id_fkey FOREIGN KEY (announcement_materials_coordinator_id) REFERENCES worker (id),
            ADD CONSTRAINT item_announcement_materials_check_person_id_fkey FOREIGN KEY (announcement_materials_check_person_id) REFERENCES worker (id),
            ADD CONSTRAINT item_jan_coordinator_id_fkey FOREIGN KEY (jan_coordinator_id) REFERENCES worker (id),
            ADD CONSTRAINT item_jan_check_person_id_fkey FOREIGN KEY (jan_check_person_id) REFERENCES worker (id);
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
