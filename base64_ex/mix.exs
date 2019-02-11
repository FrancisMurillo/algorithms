defmodule Base64.MixProject do
  use Mix.Project

  def project do
    [
      app: :base64,
      version: "0.1.0",
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:quixir, "~> 0.9.0", only: [:dev, :test]},
      {:bmark, "~> 1.0.0"}
    ]
  end
end
