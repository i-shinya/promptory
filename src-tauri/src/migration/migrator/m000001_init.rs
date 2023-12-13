use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // プロンプト管理テーブル
        manager
            .create_table(
                Table::create()
                    .table(PromptManager::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PromptManager::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PromptManager::Title).string().not_null())
                    .col(ColumnDef::new(PromptManager::ApiType).string())
                    .col(ColumnDef::new(PromptManager::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // Tag テーブル
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Value).string().not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        // PromptManagerTag 中間テーブル
        manager
            .create_table(
                Table::create()
                    .table(PromptManagerTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PromptManagerTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PromptManagerTag::PromptManagerId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-prompt_manager_tag-prompt_manager_id")
                            .from(PromptManagerTag::Table, PromptManagerTag::PromptManagerId)
                            .to(PromptManager::Table, PromptManager::Id),
                    )
                    .col(ColumnDef::new(PromptManagerTag::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-prompt_manager_tag-tag_id")
                            .from(PromptManagerTag::Table, PromptManagerTag::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // プロンプト設定テーブル
        manager
            .create_table(
                Table::create()
                    .table(PromptSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PromptSettings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PromptSettings::ManagerId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-setting-versions-setting_id")
                            .from(PromptSettings::Table, PromptSettings::ManagerId)
                            .to(PromptManager::Table, PromptManager::Id),
                    )
                    .col(
                        ColumnDef::new(PromptSettings::CurrentVersion)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // プロンプト設定のバージョンテーブル
        manager
            .create_table(
                Table::create()
                    .table(PromptSettingVersions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PromptSettingVersions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PromptSettingVersions::Version)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PromptSettingVersions::SettingId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-prompt-setting-versions-setting_id")
                            .from(
                                PromptSettingVersions::Table,
                                PromptSettingVersions::SettingId,
                            )
                            .to(PromptSettings::Table, PromptSettings::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // 設定バージョンテーブルのユニークインデックス
        manager
            .create_index(
                Index::create()
                    .name("unique-idx-setting-versions-setting_id")
                    .table(PromptSettingVersions::Table)
                    .if_not_exists()
                    .col(PromptSettingVersions::SettingId)
                    .col(PromptSettingVersions::Version)
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
                    .col(ColumnDef::new(Runs::ManagerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-runs-setting_id")
                            .from(Runs::Table, Runs::ManagerId)
                            .to(PromptManager::Table, PromptManager::Id),
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
                            .to(PromptSettingVersions::Table, PromptSettingVersions::Id),
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
                            .to(PromptSettings::Table, PromptSettings::Id),
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
                            .to(PromptSettings::Table, PromptSettings::Id),
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
                    .table(PromptSettings::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PromptManager::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PromptManager {
    Table,
    Id,
    Title,
    ApiType,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Value,
}

#[derive(DeriveIden)]
enum PromptManagerTag {
    Table,
    Id,
    PromptManagerId,
    TagId,
}

#[derive(DeriveIden)]
enum PromptSettings {
    Table,
    Id,
    ManagerId,
    CurrentVersion, // 現在のバージョン、PromptSettingVersionsのバージョンと同じ値を設定する
}

#[derive(DeriveIden)]
enum PromptSettingVersions {
    Table,
    Id,
    Version,
    SettingId,
}

#[derive(DeriveIden)]
enum Runs {
    Table,
    Id,
    ManagerId,
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
