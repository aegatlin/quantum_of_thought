defmodule Qot.Storage.Adapter do
  @moduledoc """
  Behaviour for note storage adapters.
  """

  @type note_id :: String.t()
  @type note_data :: binary()
  @type note :: %{
          id: note_id(),
          data: note_data()
        }

  @callback init() :: :ok | {:error, term()}
  @callback set(note_id(), note_data()) :: {:ok, note()} | {:error, term()}
  @callback get(note_id()) :: {:ok, note()} | {:error, :not_found}
  @callback list() :: {:ok, [note()]} | {:error, term()}
  @callback delete(note_id()) :: :ok | {:error, term()}
end
