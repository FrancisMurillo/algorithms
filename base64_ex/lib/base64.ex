defmodule Base64 do
  @moduledoc nil

  @padding_char 61

  @encoding_table %{
    0 => 65,
    1 => 66,
    2 => 67,
    3 => 68,
    4 => 69,
    5 => 70,
    6 => 71,
    7 => 72,
    8 => 73,
    9 => 74,
    10 => 75,
    11 => 76,
    12 => 77,
    13 => 78,
    14 => 79,
    15 => 80,
    16 => 81,
    17 => 82,
    18 => 83,
    19 => 84,
    20 => 85,
    21 => 86,
    22 => 87,
    23 => 88,
    24 => 89,
    25 => 90,
    26 => 97,
    27 => 98,
    28 => 99,
    29 => 100,
    30 => 101,
    31 => 102,
    32 => 103,
    33 => 104,
    34 => 105,
    35 => 106,
    36 => 107,
    37 => 108,
    38 => 109,
    39 => 110,
    40 => 111,
    41 => 112,
    42 => 113,
    43 => 114,
    44 => 115,
    45 => 116,
    46 => 117,
    47 => 118,
    48 => 119,
    49 => 120,
    50 => 121,
    51 => 122,
    52 => 48,
    53 => 49,
    54 => 50,
    55 => 51,
    56 => 52,
    57 => 53,
    58 => 54,
    59 => 55,
    60 => 56,
    61 => 57,
    62 => 43,
    63 => 47
  }

  @decoding_table %{
    65 => 0,
    66 => 1,
    67 => 2,
    68 => 3,
    69 => 4,
    70 => 5,
    71 => 6,
    72 => 7,
    73 => 8,
    74 => 9,
    75 => 10,
    76 => 11,
    77 => 12,
    78 => 13,
    79 => 14,
    80 => 15,
    81 => 16,
    82 => 17,
    83 => 18,
    84 => 19,
    85 => 20,
    86 => 21,
    87 => 22,
    88 => 23,
    89 => 24,
    90 => 25,
    97 => 26,
    98 => 27,
    99 => 28,
    100 => 29,
    101 => 30,
    102 => 31,
    103 => 32,
    104 => 33,
    105 => 34,
    106 => 35,
    107 => 36,
    108 => 37,
    109 => 38,
    110 => 39,
    111 => 40,
    112 => 41,
    113 => 42,
    114 => 43,
    115 => 44,
    116 => 45,
    117 => 46,
    118 => 47,
    119 => 48,
    120 => 49,
    121 => 50,
    122 => 51,
    48 => 52,
    49 => 53,
    50 => 54,
    51 => 55,
    52 => 56,
    53 => 57,
    54 => 58,
    55 => 59,
    56 => 60,
    57 => 61,
    43 => 62,
    47 => 63
  }

  def encode(data, opts \\ []) do
    unpadded = Keyword.get(opts, :unpadded, false)

    encode_binary(data, unpadded)
  end

  defp encode_binary(<<encoded_bits::binary-size(3), rest_bits::binary>>, unpadded) do
    <<
      first_bits::size(6),
      second_bits::size(6),
      third_bits::size(6),
      fourth_bits::size(6)
    >> = encoded_bits

    <<
      encode_bits(first_bits),
      encode_bits(second_bits),
      encode_bits(third_bits),
      encode_bits(fourth_bits),
      encode_binary(rest_bits, unpadded)::binary
    >>
  end

  defp encode_binary(<<data::binary-size(2)>>, unpadded) do
    <<
      first_bits::size(6),
      second_bits::size(6),
      third_bits::size(4)
    >> = data

    <<last_bits::size(6)>> = <<third_bits::size(4), 0::size(2)>>

    encoded_bits = <<
      encode_bits(first_bits),
      encode_bits(second_bits),
      encode_bits(last_bits)
    >>

    if unpadded do
      encoded_bits
    else
      encoded_bits <> <<@padding_char>>
    end
  end

  defp encode_binary(<<data::binary-size(1)>>, unpadded) do
    <<first_bits::size(6), second_bits::size(2)>> = data
    <<last_bits::size(6)>> = <<second_bits::size(2), 0::size(4)>>

    encoded_bits = <<
      encode_bits(first_bits),
      encode_bits(last_bits)
    >>

    if unpadded do
      encoded_bits
    else
      encoded_bits <> <<@padding_char, @padding_char>>
    end
  end

  defp encode_binary(<<>>, _unpadded),
    do: <<>>

  defp encode_bits(bits) do
    Map.get(@encoding_table, bits)
  end

  def decode(encoded_data, opts \\ []) do
    unpadded = Keyword.get(opts, :unpadded, false)

    decode_binary(encoded_data, unpadded)
  end

  defp decode_binary(
         <<trailing_bits::binary-size(2), @padding_char::size(8), @padding_char::size(8)>>,
         false
       ),
       do: decode_binary(trailing_bits, true)

  defp decode_binary(<<trailing_bits::binary-size(2)>>, true) do
    <<
      first_bits::size(8),
      second_bits::size(8)
    >> = trailing_bits

    with {:ok, first_value} <- Map.fetch(@decoding_table, first_bits),
         {:ok, second_value} <- Map.fetch(@decoding_table, second_bits),
         <<remaining_value::size(2), _excess_value::size(4)>> <- <<second_value::size(6)>> do
      {:ok,
       <<
         first_value::size(6),
         remaining_value::size(2)
       >>}
    else
      :error -> :error
    end
  end

  defp decode_binary(<<trailing_bits::binary-size(3), @padding_char::size(8)>>, false),
    do: decode_binary(trailing_bits, true)

  defp decode_binary(<<trailing_bits::binary-size(3)>>, true) do
    <<
      first_bits::size(8),
      second_bits::size(8),
      third_bits::size(8)
    >> = trailing_bits

    with {:ok, first_value} <- Map.fetch(@decoding_table, first_bits),
         {:ok, second_value} <- Map.fetch(@decoding_table, second_bits),
         {:ok, third_value} <- Map.fetch(@decoding_table, third_bits),
         <<remaining_value::size(4), _excess_value::size(2)>> <- <<third_value::size(6)>> do
      {:ok,
       <<
         first_value::size(6),
         second_value::size(6),
         remaining_value::size(4)
       >>}
    else
      :error -> :error
    end
  end

  defp decode_binary(<<leading_bits::binary-size(4), rest::binary>>, unpadded) do
    <<
      first_bits::size(8),
      second_bits::size(8),
      third_bits::size(8),
      fourth_bits::size(8)
    >> = leading_bits

    with {:ok, first_value} <- Map.fetch(@decoding_table, first_bits),
         {:ok, second_value} <- Map.fetch(@decoding_table, second_bits),
         {:ok, third_value} <- Map.fetch(@decoding_table, third_bits),
         {:ok, fourth_value} <- Map.fetch(@decoding_table, fourth_bits),
         {:ok, rest_binary} <- decode_binary(rest, unpadded) do
      {:ok,
       <<
         first_value::size(6),
         second_value::size(6),
         third_value::size(6),
         fourth_value::size(6),
         rest_binary::binary
       >>}
    else
      :error -> :error
    end
  end

  defp decode_binary(<<>>, _),
    do: {:ok, <<>>}

  defp decode_binary(_, _),
    do: :error
end
