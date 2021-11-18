myLength :: [a] -> Int
myLength = foldl (\n _-> n + 1) 0

main = do
  print $ myLength [1,2,3]
  print $ myLength "jonners"
  print $ myLength []