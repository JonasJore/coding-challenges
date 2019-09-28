//returns the sum of positive numbers in an array

class Main {
  public static void main(String[] args) {
    int sumOfPositiveArr = new Main().sum(new int[]{1,-2,3,4,5});
    System.out.println(sumOfPositiveArr);
  }
  
  public static int sum(int[] arr){
    int sum = 0;
    for(int i = 0; i < arr.length; i++) {
      if(arr[i] >= 0) {
        sum += arr[i];
      }
    } 
    
    return sum;
  }
  
}
