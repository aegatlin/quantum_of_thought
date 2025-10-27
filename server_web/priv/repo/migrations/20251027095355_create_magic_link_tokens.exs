defmodule Qot.Repo.Migrations.CreateMagicLinkTokens do
  use Ecto.Migration

  def change do
    create table(:magic_link_tokens) do
      add :token, :string, null: false
      add :email, :string, null: false
      add :expires_at, :utc_datetime, null: false

      timestamps(type: :utc_datetime, updated_at: false)
    end

    create unique_index(:magic_link_tokens, [:token])
    create index(:magic_link_tokens, [:email])
    create index(:magic_link_tokens, [:expires_at])
  end
end
