#!/usr/bin/env python3

import random
import string
import re
import time
import sys

def get_word_of_length(str_length):
    return ''.join(random.choice(string.ascii_lowercase) for _ in range(str_length))

all_words = [get_word_of_length(random.choice([3,4,5,6,7,8])) for i in range(5000000)]
keys = set(random.sample(all_words,int(len(all_words)/100)))
replacements = [get_word_of_length(random.choice([3,4,5,6,7,8])) for i in range(len(keys))]

print('\n'.join(all_words))
print('\n'.join(map('\t'.join,zip(keys,replacements))),file=sys.stderr)

