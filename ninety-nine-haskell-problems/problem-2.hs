myButLast :: [a] -> a
myButLast [x,_] = x
myButLast (_:xs) = myButLast xs

main = do
  print $ myButLast [1,2,3]
  print $ myButLast ["jonas", "jonners", "jonjon"]
