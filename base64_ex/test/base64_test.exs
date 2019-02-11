defmodule Base64Test do
  use ExUnit.Case
  use Quixir

  alias Base64, as: Core

  test "encode should work" do
    ptest data: string() do
      assert Base.encode64(data) == Core.encode(data),
             "should match core function"

      assert Base.encode64(data, padding: false) == Core.encode(data, unpadded: true),
             "padding option should match as well"
    end
  end

  test "decode should work" do
    ptest data: string() do
      encoded_data = Base.encode64(data)

      assert Base.decode64(encoded_data) == Core.decode(encoded_data),
             "should match core function"

      trimmed_data = String.replace_trailing(encoded_data, "=", "")

      assert Base.decode64(trimmed_data, padding: false) ==
               Core.decode(trimmed_data, unpadded: true),
             "padding option should match as well"
    end
  end

  test "decode should fail safely" do
    ptest data: string() do
      assert Base.decode64(data) == Core.decode(data),
             "should match core function error"

      assert Base.decode64(data) == Core.decode(data),
             "padding option should fail safely as well"
    end
  end

  test "encode and decode should return the same binary" do
    ptest data: string() do
      assert {:ok, ^data} =
        data
        |> Core.encode()
        |> Core.decode(),
        "encode and decode are inverse operations"
    end
  end
end
