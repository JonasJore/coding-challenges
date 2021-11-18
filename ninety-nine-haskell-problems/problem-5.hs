myReverse :: [a] -> [a]
myReverse = foldl (\x s -> s:x) []

main = do
  print $ myReverse "A man, a plan, a canal, panama!"
  print $ myReverse [1..10]