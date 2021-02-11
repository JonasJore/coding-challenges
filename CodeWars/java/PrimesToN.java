package no.jonjon;

// timecomplexity: O(n)
public class Preems {

    public void preemSolution() {
        int n = 1_000_000;
        for (int i = 0; i <= n; i++) {
            if (preemChecker(i)) {
                System.out.println(i);
            }
        }
    }

    public boolean preemChecker(int n) {
        if (n <= 1)
            return false;
        for (int i = 2; i <= n / 2; i++) {
            if (n % i == 0)
                return false;
        }
        return true;
    }
}