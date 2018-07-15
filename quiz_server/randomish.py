

primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]

def shuffle(lst, seed):
    lst = list(lst)
    length = len(lst)
    indices = [(1 + (seed % p)) for p in primes]
    for ind in indices:
        ind = ind % length
        a,b = lst[0:ind], lst[ind:]
        lst = list(reversed(a)) + b
        lst.reverse()
    return list(lst)

def randint(a, b, lim=None):
    r = (a + b + a*b + a // (b+1)) >> 4
    if lim:
        r = r % lim
    return r

def pick(lst, state, n=0):
    ind = randint(state, n, lim=len(lst))
    return lst[ind]
