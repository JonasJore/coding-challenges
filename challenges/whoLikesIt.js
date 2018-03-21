// function that mimics the like-function from facebook.
function likesFunction(people) {
    if(people.length === 0) {
      return 'no one likes this';
    } else if(people.length === 1) {
      return people[0] + ' likes this';
    } else if(people.length === 2) {
      return people[0] + ' and ' + people[1] + ' likes this';
    } else if(people.length === 3) {
      return people[0] + ', ' + people[1] + ' and ' + people[2] + ' likes this';
    } else if(people.length > 3) {
      return people[0] + ', ' + people[1] + ' and ' + (people.length-2) + ' others like this';
    }
  }
  
  function test() {
    return likesFunction(['Ola','Kari','Trond','Sigrid']);
  }
  
  test();