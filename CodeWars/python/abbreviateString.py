def abbrevName(name):
    split = name.split(' ')
    abbreviation = ''

    for i in range(0, len(split)):
        abbreviation += split[i][:1]

    return '.'.join(abbreviation).upper()
