import Data.List

compress :: Eq a => [a] -> [a]
compress = map head . group

main :: IO ()
main = do
  print $ compress "aaaabccaadeeee"
  print $ compress "jjjjoooonneeeerrrrssss"