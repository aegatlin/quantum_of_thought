defmodule Qot.Storage.ETSAdapter do
  @moduledoc """
  In-memory storage adapter using ETS.
  """

  @behaviour Qot.Storage.Adapter

  @table :qot_notes

  @impl true
  def init do
    :ets.new(@table, [:set, :public, :named_table])
    :ok
  catch
    :error, :badarg ->
      # Table already exists
      :ok
  end

  @impl true
  def set(user_id, id, data) when is_binary(user_id) and is_binary(id) and is_binary(data) do
    note = %{
      id: id,
      data: data,
      user_id: user_id
    }

    # Store with composite key: {user_id, note_id}
    :ets.insert(@table, {{user_id, id}, note})
    {:ok, note}
  end

  @impl true
  def get(user_id, id) when is_binary(user_id) and is_binary(id) do
    case :ets.lookup(@table, {user_id, id}) do
      [{{^user_id, ^id}, note}] -> {:ok, note}
      [] -> {:error, :not_found}
    end
  end

  @impl true
  def list(user_id) when is_binary(user_id) do
    notes =
      :ets.foldl(
        fn {{uid, _id}, note}, acc ->
          if uid == user_id do
            [note | acc]
          else
            acc
          end
        end,
        [],
        @table
      )

    {:ok, notes}
  end

  @impl true
  def delete(user_id, id) when is_binary(user_id) and is_binary(id) do
    :ets.delete(@table, {user_id, id})
    :ok
  end
end
