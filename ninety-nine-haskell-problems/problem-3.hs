elementAt :: [a] -> Int -> Maybe a
elementAt x index = if (index-1) < (length x) 
                      then Just(x!!(index-1))
                    else Nothing

main = do
  print $ elementAt [1,2,3] 1
  print $ elementAt "jonas" 666
