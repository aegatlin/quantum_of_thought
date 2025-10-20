defmodule Qot.Notes do
  @moduledoc """
  The Notes context.
  """

  @storage Application.compile_env(:qot, :storage_adapter)

  def put_note(id, data) when is_binary(id) and is_binary(data) do
    @storage.set(id, data)
  end

  def get_note(id) when is_binary(id) do
    @storage.get(id)
  end

  def list_notes do
    @storage.list()
  end

  def delete_note(id) when is_binary(id) do
    @storage.delete(id)
  end
end
