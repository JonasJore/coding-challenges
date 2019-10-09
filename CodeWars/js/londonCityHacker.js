const londonCityHacker = (journey) => {
  const busPrice = 1.50
  const busStop = Math.round((journey.filter(trip => typeof trip === 'number').length * busPrice) * 100) / 100
  const tubePrice = 2.40
  const tubeStop = Math.round((journey.filter(trip => typeof trip === 'string').length * tubePrice) * 100) / 100
  let sum = busStop + tubeStop

  for(let i = 0; i < journey.length; i++) {
    if(typeof journey[i] === 'number' && typeof journey[i+1] === 'number') {
      sum -= 1.50
      ++i
    }
  }

  return `£${sum.toFixed(2)}`
}
