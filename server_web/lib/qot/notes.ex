defmodule Qot.Notes do
  @moduledoc """
  The Notes context.

  Publishes events to PubSub so WebSocket clients stay in sync
  regardless of whether changes come from HTTP, WebSocket, or other sources.
  """

  @storage Application.compile_env(:qot, :storage_adapter)
  @pubsub Qot.PubSub

  def set(id, data) when is_binary(id) and is_binary(data) do
    case @storage.set(id, data) do
      {:ok, note} = result ->
        # Broadcast to all WebSocket clients
        Phoenix.PubSub.broadcast(@pubsub, "notes:lobby", {:note_created, note})
        result

      error ->
        error
    end
  end

  def get(id) when is_binary(id) do
    @storage.get(id)
  end

  def list do
    @storage.list()
  end

  def delete(id) when is_binary(id) do
    case @storage.delete(id) do
      :ok ->
        # Broadcast to all WebSocket clients
        Phoenix.PubSub.broadcast(@pubsub, "notes:lobby", {:note_deleted, id})
        :ok

      error ->
        error
    end
  end
end
