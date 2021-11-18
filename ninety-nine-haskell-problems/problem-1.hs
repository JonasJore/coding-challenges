myLast :: [a] -> a
myLast [] = error "no last item"
myLast [a] = a
myLast (_:xs) = myLast xs

main = do
  print $ myLast [1..10]
  print $ myLast ['a'..'z']