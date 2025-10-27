defmodule Qot.Notes do
  @moduledoc """
  The Notes context.

  Publishes events to PubSub so WebSocket clients stay in sync
  regardless of whether changes come from HTTP, WebSocket, or other sources.
  """

  @storage Application.compile_env(:qot, :storage_adapter)
  @pubsub Qot.PubSub

  def set(user_id, id, data) when is_binary(user_id) and is_binary(id) and is_binary(data) do
    case @storage.set(user_id, id, data) do
      {:ok, note} = result ->
        Phoenix.PubSub.broadcast(@pubsub, "notes:user:#{user_id}", {:note_created, note})
        result

      error ->
        error
    end
  end

  def get(user_id, id) when is_binary(user_id) and is_binary(id) do
    @storage.get(user_id, id)
  end

  def list(user_id) when is_binary(user_id) do
    @storage.list(user_id)
  end

  def delete(user_id, id) when is_binary(user_id) and is_binary(id) do
    case @storage.delete(user_id, id) do
      :ok ->
        Phoenix.PubSub.broadcast(@pubsub, "notes:user:#{user_id}", {:note_deleted, id})
        :ok

      error ->
        error
    end
  end
end
