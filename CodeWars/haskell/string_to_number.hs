module Main where

stringToNumber :: String -> Integer
stringToNumber numberString = read numberString :: Integer

main :: IO()
main = do
  let number = "9"
  let result = stringToNumber number
  print result
  