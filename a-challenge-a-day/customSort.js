/**
 * the challenge was to implement a sorting algorithm without the use of Arrays.sort()
 * ended up using a bubble sort technique because it was the fastest for me to write
 *
 * @link: https://www.codewars.com/kata/custom-sort-function
 * date: 12.02.19
 */
let sort = (items) => {
  for(let i = 0; i < items.length; i++) {
    for(let j = 0; j < items.length - i - 1; j++) {
      if(items[j] >= items[j+1]) {
        let temp = items[j]
        items[j] = items[j+1]
        items[j+1] = temp
      }
    }
  }
  
  return items
}

sort([1,3,2,3,4,1])
