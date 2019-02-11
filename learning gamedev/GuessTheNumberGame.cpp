#include <iostream>
#include <random>

typedef std::mt19937 rng_type;
rng_type rng;

int randomNumber() {
  rng.seed(std::random_device()());
  std::uniform_int_distribution<rng_type::result_type> dist(1, 100);
  
  return dist(rng);
}

void guessTheNumberGame() {
  int numberOfTries = 0;
  int guess = 0;
  int ballValue = randomNumber();
  
  std::cout << "Welcome! Try guessing how many balls i have in my basket" << std::endl;

  while(true) {
    std::cin >> guess;
    if(guess > 0 && guess <= 100) {
      if(guess < ballValue) {
        std::cout << "Your guess is too low " << guess << std::endl;
        std::cout << numberOfTries << std::endl;
        ++numberOfTries;
      } else if(guess > ballValue) {
        std::cout << "Your guess is too high " << guess << std::endl;
        std::cout << numberOfTries << std::endl;
        ++numberOfTries;
      } else {
        std::cout << "Correct!" << std::endl;
        std::cout << "...and it only took " << numberOfTries << " guesses." << std::endl;
        ++numberOfTries;
        
        if(numberOfTries > 5) {
          std::cout << "but i think you can do a lot better than " << numberOfTries << " tries" << std::endl;
        }

        break;
      }  
    } else {
      std::cout << "invalid guess" << std::endl;
      std::cout << "you only have to guess between 1 and 100 balls" << std::endl;
    }
  }
}

int main() {
  guessTheNumberGame();
}
