myReverse :: [a] -> [a]
myReverse = foldl (\x s -> s:x) []

isPalindrome :: (Eq a) => [a] -> Bool
isPalindrome xs = xs == (myReverse xs)

main :: IO ()
main = do
  print $ isPalindrome "racecar"
  print $ isPalindrome "jonners"
  