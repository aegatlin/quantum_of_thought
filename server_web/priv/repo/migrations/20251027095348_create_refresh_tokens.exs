defmodule Qot.Repo.Migrations.CreateRefreshTokens do
  use Ecto.Migration

  def change do
    create table(:refresh_tokens) do
      add :token, :string, null: false
      add :user_id, references(:users, type: :uuid, on_delete: :delete_all), null: false
      add :expires_at, :utc_datetime, null: false

      timestamps(type: :utc_datetime, updated_at: false)
    end

    create unique_index(:refresh_tokens, [:token])
    create index(:refresh_tokens, [:user_id])
    create index(:refresh_tokens, [:expires_at])
  end
end
