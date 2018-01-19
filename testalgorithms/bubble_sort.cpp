/**
 *  Description: bubble sort algorithm from scratch
 *  Name: Jonas Jore
 *  Date: 19/01/18
*/

#include <iostream>

void swap(int *x, int *y){
  int tmp = *x;
  *x = *y;
  *y = tmp;
}

void bubble_sort(int arr[],size_t size){
  for(int i = 0; i < size-1; i++){
    for(int j = 0; j < size-i-1; j++){
      if(arr[j] > arr[j+1]){
        swap(&arr[j],&arr[j+1]);
      }
    }
  }
}

void print_arr(int arr[],size_t size){
  std::cout << "{ ";
  for(int i = 0; i < size; i++){
    std::cout << arr[i];
    if(i >= 0){
      std::cout << ",";
    }
    if(i == size-1){
      std::cout << " }";
    }
  }
  std::cout << std::endl;
}


int main() {
  int arr[] = {33,1,689,42,17,50,101,99};
  size_t size = 8;
  std::cout << "Unsorted: " << std::endl;
  print_arr(arr,size);
  bubble_sort(arr,size);
  std::cout << "Sorted: " << std::endl;
  print_arr(arr,size);
  
  return 0;
}