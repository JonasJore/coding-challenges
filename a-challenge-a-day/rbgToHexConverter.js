function rgb(r, g, b){
  return '#' + toHex(r) + toHex(g) + toHex(b);  
}

function toHex(colorCode) {
  let hex = colorCode.toString(16).toUpperCase()
  return hex.length == 1 ? '0' + hex : hex
}