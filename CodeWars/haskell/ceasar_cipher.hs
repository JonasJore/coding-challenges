import Data.Char

shouldCipher :: Char -> Bool
shouldCipher c = isLetter(c) && isAscii(c)

cipherChar :: Int -> Char -> Char
cipherChar shift c
  | shouldCipher c = chr(ord(c) + shift)
  | otherwise      = c 

cipher :: Int -> [Char] -> [Char]
cipher shift plainText = map (cipherChar shift) plainText

decipher :: Int -> [Char] -> [Char]
decipher shift cipherText = cipher (-shift) cipherText

main :: IO()
main = do
  print $ cipher 1 "jonas jore" == "kpobt kpsf"
  print $ decipher 1 "kpobt kpsf" == "jonas jore"