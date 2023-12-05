use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 設定テーブル
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Settings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Settings::Title).string().not_null())
                    .col(ColumnDef::new(Settings::ApiType).string().not_null())
                    .to_owned(),
            )
            .await?;

        // 設定バージョンテーブル
        manager
            .create_table(
                Table::create()
                    .table(SettingVersions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SettingVersions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SettingVersions::Version)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingVersions::SettingId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-setting-versions-setting_id")
                            .from(SettingVersions::Table, SettingVersions::SettingId)
                            .to(Settings::Table, Settings::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // 設定バージョンテーブルのユニークインデックス
        manager
            .create_index(
                Index::create()
                    .name("idx-setting-versions-setting_id")
                    .table(SettingVersions::Table)
                    .col(SettingVersions::SettingId)
                    .col(SettingVersions::Version)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Run テーブル
        manager
            .create_table(
                Table::create()
                    .table(Runs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Runs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Runs::SettingId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-runs-setting_id")
                            .from(Runs::Table, Runs::SettingId)
                            .to(Settings::Table, Settings::Id),
                    )
                    .col(ColumnDef::new(Runs::UserPrompt).text().not_null())
                    .col(ColumnDef::new(Runs::Model).string().not_null())
                    .col(ColumnDef::new(Runs::Temperature).float().not_null())
                    .to_owned(),
            )
            .await?;

        // RunHistory テーブル
        manager
            .create_table(
                Table::create()
                    .table(RunHistories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RunHistories::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RunHistories::RunId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-run-histories-run_id")
                            .from(RunHistories::Table, RunHistories::RunId)
                            .to(Runs::Table, Runs::Id),
                    )
                    .col(ColumnDef::new(RunHistories::VersionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-run-histories-version_id")
                            .from(RunHistories::Table, RunHistories::VersionId)
                            .to(SettingVersions::Table, SettingVersions::Id),
                    )
                    .col(ColumnDef::new(RunHistories::Response).text().not_null())
                    .to_owned(),
            )
            .await?;

        // ChatSetting テーブル
        manager
            .create_table(
                Table::create()
                    .table(ChatSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatSettings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ChatSettings::VersionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat-settings-version_id")
                            .from(ChatSettings::Table, ChatSettings::VersionId)
                            .to(SettingVersions::Table, SettingVersions::Id),
                    )
                    .col(ColumnDef::new(ChatSettings::SystemPrompt).text().not_null())
                    .col(
                        ColumnDef::new(ChatSettings::ResponseFormat)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // AssistantSetting テーブル
        manager
            .create_table(
                Table::create()
                    .table(AssistantSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AssistantSettings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AssistantSettings::VersionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-assistant-settings-version_id")
                            .from(AssistantSettings::Table, AssistantSettings::VersionId)
                            .to(SettingVersions::Table, SettingVersions::Id),
                    )
                    .col(
                        ColumnDef::new(AssistantSettings::SystemPrompt)
                            .text()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(AssistantSettings::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ChatSettings::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(RunHistories::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Runs::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(SettingVersions::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Settings::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Settings {
    Table,
    Id,
    Title,
    ApiType,
}

#[derive(DeriveIden)]
enum SettingVersions {
    Table,
    Id,
    Version,
    SettingId,
}

#[derive(DeriveIden)]
enum Runs {
    Table,
    Id,
    SettingId,
    UserPrompt,
    Model,
    Temperature,
}

#[derive(DeriveIden)]
enum RunHistories {
    Table,
    Id,
    RunId,
    VersionId,
    Response,
}

#[derive(DeriveIden)]
enum ChatSettings {
    Table,
    Id,
    VersionId,
    SystemPrompt,
    ResponseFormat,
}

#[derive(DeriveIden)]
enum AssistantSettings {
    Table,
    Id,
    VersionId,
    SystemPrompt,
}
