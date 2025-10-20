defmodule Qot.MixProject do
  use Mix.Project

  def project do
    [
      app: :qot,
      version: "0.1.0",
      elixir: "~> 1.14",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      mod: {Qot.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps do
    [
      {:phoenix, "~> 1.7.0"},
      {:bandit, "~> 1.0"},
      {:jason, "~> 1.2"},
      {:cors_plug, "~> 3.0"}
    ]
  end
end
