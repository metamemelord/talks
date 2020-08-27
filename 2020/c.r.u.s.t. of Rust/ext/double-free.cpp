#include<iostream>

int main() {
    int *x = new int[30];
    std::cout << x[0];
    delete x;
    delete x;
}