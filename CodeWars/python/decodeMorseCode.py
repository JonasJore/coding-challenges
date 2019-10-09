#https://www.codewars.com/kata/decode-the-morse-code/train/python
#TODO: adds space for each character, need to just add a space after each word

morseAlphabetDict = {
  '.-': 'A',
  '-...': 'B',
  '-.-.': 'C', 
  '-..': 'D',
  '.': 'E',
  '..-.': 'F',
  '--.': 'G',
  '....': 'H',
  '..': 'I',
  '.---': 'J',
  '-.-': 'K',
  '.-..': 'L',
  '--': 'M',
  '-.': 'N',
  '---': 'O',
  '.--.': 'P',
  '--.-': 'Q',
  '.-.': 'R',
  '...': 'S',
  '-': 'T',
  '..-': 'U',
  '.--': 'W',
  '-..-': 'X',
  '-.--': 'Y',
  '--..': 'Z',
  '.-.-': 'Æ',
  '---.': 'Ø',
  '.--.-': 'Å',
}

def decodeEachCharacter(word):
  return morseAlphabetDict[word]
  

def decodeMorse(morse_code):
  decodedWord = ''

  for word in morse_code.split(' '):
    if word:
      decodedWord += ' '
    decodedWord += decodeEachCharacter(word)

  return decodedWord


print(decodeMorse('.... . -.-- .--- ..- -.. .'))
