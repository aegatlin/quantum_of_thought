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
  def set(id, data) when is_binary(id) and is_binary(data) do
    note = %{
      id: id,
      data: data
    }

    :ets.insert(@table, {id, note})
    {:ok, note}
  end

  @impl true
  def get(id) when is_binary(id) do
    case :ets.lookup(@table, id) do
      [{^id, note}] -> {:ok, note}
      [] -> {:error, :not_found}
    end
  end

  @impl true
  def list do
    notes =
      :ets.foldl(
        fn {_id, note}, acc -> [note | acc] end,
        [],
        @table
      )

    {:ok, notes}
  end

  @impl true
  def delete(id) when is_binary(id) do
    :ets.delete(@table, id)
    :ok
  end
end
