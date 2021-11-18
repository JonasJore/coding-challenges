pack :: Eq a => [a] -> [[a]]
pack [] = []
pack (x:xs) = let (head,tail) = span (==x) xs
               in (x:head) : pack tail

main :: IO ()
main = do
  print $ pack ['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e']