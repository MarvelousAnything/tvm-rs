fun sieve(n) {
    var i, prime, p

    if .n <= 1 {
        return -1
    }
    prime : alloc(.n + 1)
    i : 0
    loop {
        until .i >= .n
        (.prime + .i) : 1
        i : .i + 1
    }

    p : 2
    loop {
        until .p * .p > .n
        if .(.prime + .p) == 1 {
            i : .p * .p
            loop {
                until .i > .n
                (.prime + .i) : 0
                i : .i + .p
            }
        }
        p : .p + 1
    }

    i : 0
    loop {
        until .i >= .n + 1
        if .(.prime + .i) == 1 {
            iprint(.i)
            sprint("\n")
        }
        i : .i + 1
    }
    return prime
}


fun init() {
    var max
    # max : iread("Max prime number.")
    max : 100
    sieve(.max)
}