defmodule Qot.Storage.Adapter do
  @moduledoc """
  Behaviour for note storage adapters.
  """

  @type user_id :: String.t()
  @type note_id :: String.t()
  @type note_data :: binary()
  @type note :: %{
          id: note_id(),
          data: note_data(),
          user_id: user_id()
        }

  @callback init() :: :ok | {:error, term()}
  @callback set(user_id(), note_id(), note_data()) :: {:ok, note()} | {:error, term()}
  @callback get(user_id(), note_id()) :: {:ok, note()} | {:error, :not_found}
  @callback list(user_id()) :: {:ok, [note()]} | {:error, term()}
  @callback delete(user_id(), note_id()) :: :ok | {:error, term()}
end
