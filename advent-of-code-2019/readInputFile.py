def readInput(filePath):
  with open(filePath) as line:
    fileContentList = line.read().splitlines()
  
  return fileContentList