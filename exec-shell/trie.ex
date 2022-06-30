defmodule Trie do
  defstruct [:is_end, children: %{}]

  def search(_upper, [], acc, _upper_acc), do: acc

  def search(upper, [current | neibours], acc, upper_acc) do
    current_node = upper.children[current]

    if current_node.is_end && Map.keys(current_node.children) == [] do
      search(upper, neibours, [upper_acc ++ current | acc], upper_acc)
    else
      current_acc = search(current_node,
        Map.keys(current_node.children),
        acc,
        upper_acc ++ current
        )
      if current_node.is_end do
        search(upper, neibours, [upper_acc ++ current | current_acc], upper_acc)
      else
        search(upper, neibours, current_acc, upper_acc)
      end
    end
  end



  def end_node(), do: %Trie{ is_end: true, children: %{} }

  def test() do
    testing_trie = %Trie{
      is_end: false,
      children: %{
        'a' => %Trie{
          is_end: false,
          children: %{
            'b' => end_node(),
            'c' => %Trie{
              is_end: true,
              children: %{
                'd' => end_node()
              }
            }
          }
        },
        'x' => %Trie{
          is_end: false,
          children: %{
            'y' => end_node()
          }
        }
      }
    }

    search(testing_trie, Map.keys(testing_trie.children), [], [])
    |> IO.inspect()
  end
end

Trie.test()
        
