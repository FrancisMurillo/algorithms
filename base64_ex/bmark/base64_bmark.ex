defmodule Base64Benchmark do
  use Bmark

  alias Pollution
  alias Pollution.{Generator, VG}

  alias Base64, as: Core

  @random_data VG.string()
  |> Generator.as_stream
  |> Enum.take(100)

  bmark :core_encode do
    @random_data
    |> Enum.each(&Core.encode/1)
  end

  bmark :system_encode do
    @random_data
    |> Enum.each(&Base.encode64/1)
  end

  bmark :core_decode do
    @random_data
    |> Enum.map(&Base.encode64/1)
    |> Enum.each(&Core.decode/1)
  end

  bmark :system_decode do
    @random_data
    |> Enum.map(&Base.encode64/1)
    |> Enum.each(&Base.decode64/1)
  end

end
