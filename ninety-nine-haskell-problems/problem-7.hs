data NestedList a = Elem a | List [NestedList a]

flattenList :: NestedList a -> [a]
flattenList (Elem x) = [x]
flattenList (List (x:xs)) = flattenList x ++ flattenList (List xs)
flattenList (List []) = []

main :: IO ()
main = do
  print $ flattenList (List [Elem 1, List [Elem 2, List [Elem 3, Elem 4], Elem 5]])
  print $ flattenList (List [Elem "J", List [Elem "o", Elem "n", Elem "n"], Elem "e", Elem "r", Elem "s"])
  