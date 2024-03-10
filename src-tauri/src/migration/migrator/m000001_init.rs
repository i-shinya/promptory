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
                    .col(ColumnDef::new(PromptManager::ActionType).string())
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
                            .name("fk-prompt_manager-tag-prompt_manager_id")
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

        // プロンプト比較管理 テーブル
        manager
            .create_table(
                Table::create()
                    .table(ComparingPromptManager::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComparingPromptManager::ManagerId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comparing_prompt_manager-prompt_manager_id")
                            .from(
                                ComparingPromptManager::Table,
                                ComparingPromptManager::ManagerId,
                            )
                            .to(PromptManager::Table, PromptManager::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // プロンプト比較設定テーブル
        manager
            .create_table(
                Table::create()
                    .table(ComparingPromptSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComparingPromptSettings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptSettings::ManagerId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comparing_prompt_settings-manager-id")
                            .from(
                                ComparingPromptSettings::Table,
                                ComparingPromptSettings::ManagerId,
                            )
                            .to(
                                ComparingPromptManager::Table,
                                ComparingPromptManager::ManagerId,
                            ),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptSettings::CurrentVersion)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ComparingPromptSettings::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // プロンプト比較設定のバージョンテーブル
        manager
            .create_table(
                Table::create()
                    .table(ComparingPromptSettingVersions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComparingPromptSettingVersions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptSettingVersions::Version)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptSettingVersions::SettingId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(
                                "fk-comparing_prompt_setting_versions-comparing_prompt_setting-id",
                            )
                            .from(
                                ComparingPromptSettingVersions::Table,
                                ComparingPromptSettingVersions::SettingId,
                            )
                            .to(ComparingPromptSettings::Table, ComparingPromptSettings::Id),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptSettingVersions::SystemPrompt)
                            .text()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // プロンプト比較設定バージョンテーブルのユニークインデックス
        manager
            .create_index(
                Index::create()
                    .name("unique-idx-comparing_prompt_setting_versions-setting_id-version")
                    .table(ComparingPromptSettingVersions::Table)
                    .if_not_exists()
                    .col(ComparingPromptSettingVersions::SettingId)
                    .col(ComparingPromptSettingVersions::Version)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // プロンプト比較Chat設定詳細 テーブル
        manager
            .create_table(
                Table::create()
                    .table(ComparingPromptChatSettingDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComparingPromptChatSettingDetails::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptChatSettingDetails::VersionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comparing_prompt_chat_setting_details-comparing_prompt__setting_version-id")
                            .from(
                                ComparingPromptChatSettingDetails::Table,
                                ComparingPromptChatSettingDetails::VersionId,
                            )
                            .to(
                                ComparingPromptSettingVersions::Table,
                                ComparingPromptSettingVersions::Id,
                            ),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptChatSettingDetails::ResponseFormat)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // プロンプト比較Vision設定詳細 テーブル
        manager
            .create_table(
                Table::create()
                    .table(ComparingPromptVisionSettingDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComparingPromptVisionSettingDetails::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptVisionSettingDetails::VersionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comparing_prompt_vision_setting-details-comparing_prompt_setting_versions-id")
                            .from(
                                ComparingPromptVisionSettingDetails::Table,
                                ComparingPromptVisionSettingDetails::VersionId,
                            )
                            .to(
                                ComparingPromptSettingVersions::Table,
                                ComparingPromptSettingVersions::Id,
                            ),
                    )
                    .to_owned(),
            )
            .await?;

        // プロンプト比較実行テーブル
        manager
            .create_table(
                Table::create()
                    .table(ComparingPromptRuns::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComparingPromptRuns::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptRuns::ManagerId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comparing_prompt_runs-comparing_prompt_manager-id")
                            .from(ComparingPromptRuns::Table, ComparingPromptRuns::ManagerId)
                            .to(
                                ComparingPromptManager::Table,
                                ComparingPromptManager::ManagerId,
                            ),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptRuns::ProviderType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptRuns::Model)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptRuns::UserPrompt)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptRuns::Temperature)
                            .float()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ComparingPromptRuns::MaxToken).integer())
                    .to_owned(),
            )
            .await?;

        // プロンプト比較実行履歴テーブル
        manager
            .create_table(
                Table::create()
                    .table(ComparingPromptRunHistories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComparingPromptRunHistories::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptRunHistories::RunId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comparing_prompt_run_histories-comparing_prompt_runs-id")
                            .from(
                                ComparingPromptRunHistories::Table,
                                ComparingPromptRunHistories::RunId,
                            )
                            .to(ComparingPromptRuns::Table, ComparingPromptRuns::Id),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptRunHistories::VersionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comparing_prompt_run_histories-comparing_prompt_setting_versions-id")
                            .from(
                                ComparingPromptRunHistories::Table,
                                ComparingPromptRunHistories::VersionId,
                            )
                            .to(
                                ComparingPromptSettingVersions::Table,
                                ComparingPromptSettingVersions::Id,
                            ),
                    )
                    .col(
                        ColumnDef::new(ComparingPromptRunHistories::Response)
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
                    .table(ComparingPromptRunHistories::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ComparingPromptRuns::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ComparingPromptVisionSettingDetails::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ComparingPromptChatSettingDetails::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ComparingPromptSettingVersions::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ComparingPromptSettings::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ComparingPromptManager::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PromptManagerTag::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Tag::Table).if_exists().to_owned())
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
    ActionType,
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
enum ComparingPromptManager {
    Table,
    ManagerId,
}

// TODO 関連テーブルを別PRで追加する
#[derive(DeriveIden)]
enum ComparingModelManager {
    Table,
    Id,
    ManagerId,
}

#[derive(DeriveIden)]
enum ComparingPromptSettings {
    Table,
    Id,
    ManagerId,
    CurrentVersion, // 現在のバージョン、PromptSettingVersionsのバージョンと同じ値を設定する
    DeletedAt,
}

#[derive(DeriveIden)]
enum ComparingPromptSettingVersions {
    Table,
    Id,
    Version,
    SettingId,
    SystemPrompt,
}

#[derive(DeriveIden)]
enum ComparingPromptChatSettingDetails {
    Table,
    Id,
    VersionId,
    ResponseFormat,
}

#[derive(DeriveIden)]
enum ComparingPromptVisionSettingDetails {
    Table,
    Id,
    VersionId,
}

#[derive(DeriveIden)]
enum ComparingPromptRuns {
    Table,
    Id,
    ProviderType,
    ManagerId,
    UserPrompt,
    Model,
    Temperature,
    MaxToken,
}

#[derive(DeriveIden)]
enum ComparingPromptRunHistories {
    Table,
    Id,
    RunId,
    VersionId,
    Response,
}
