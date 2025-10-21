defmodule QotWeb.ChannelCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      @endpoint QotWeb.Endpoint

      import Phoenix.ChannelTest
      import QotWeb.ChannelCase
    end
  end

  setup _tags do
    :ok
  end
end
