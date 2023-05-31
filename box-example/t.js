// console.log();

function fib(n) {
    // if (n == 0) return;
    // n1 + n2
    if (n < 2) return 1
    // if (n == 0) {
    //     return 
    // }
    // n = n - 1;
    return  fib(n-1) + fib(n-2)
}
// 0 1 1 2 3 5 8 13 21
for (let i = 0; i < 10; i++) {
    console.log(fib(i));
}

function fib2(n) {
    // if (n == 0) return;
    // n1 + n2
    if (n < 2) return 1
    // if (n == 0) {
    //     return 
    // }
    // n = n - 1;
    return  fib(n-1) + fib(n-2)
}



