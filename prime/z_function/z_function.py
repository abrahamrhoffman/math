def Z(s):
    '''
    Z(s) = (1/(n**s))
    n = infinity
    s = chosen number
    '''

    # Since we run out of memory, we need to pick a discrete number.
    # This means our 'n' (infinity) actually equals a quantifiable number
    # and is chosen and stored in zeta_range
    zeta_range = int(10)

    # An object to store the results of our operations
    result = int()

    # Iterate over all Natural Numbers >= zeta_range
    for n in range(1, zeta_range):
        # Each operation's result will be added to the 'result' object
        result += (1.0/(n**s))

    # Once all operations have completed, we return the result
    return result

# 1 returns ~ 2.82896825397, which is correct
s = 1
print(Z(s))

# 2 returns ~ 1.53976773117. Result should be ~ 
s = 2
print(Z(s))