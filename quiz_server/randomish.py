import hashlib


def hash(string):
    bytes = string.encode('utf-8')
    hasher = hashlib.sha256()
    hasher.update(bytes)
    digest = hasher.digest()
    num = int.from_bytes(digest[0:32], byteorder='big', signed=False)
    return num

def shuffle(lst, seed):
    lst = list(lst)
    length = len(lst)
    indices = [hash("{}.{}".format(seed,i)) for i in range(length)]
    for ind in indices:
        ind = ind % length
        a,b = lst[0:ind], lst[ind:]
        lst = list(reversed(a)) + b
        lst.reverse()
    return list(lst)

def randint(a, b, seeds):
    r = hash(";".join((str(s) for s in seeds)))
    diff = b - a
    r = r % diff
    r = r + a
    return r

def pick(lst, state, n=0):
    ind = randint(0, len(lst), [state, n])
    return lst[ind]
