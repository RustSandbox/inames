#!/usr/bin/env python3
import csv
import random

# Read existing adjectives and nouns
with open('data/adjectives.txt', 'r') as f:
    existing_adjectives = set(line.strip() for line in f if line.strip())

with open('data/nouns.txt', 'r') as f:
    existing_nouns = set(line.strip() for line in f if line.strip())

# Process Arabic names
arabic_names = set()

# Read male names
with open('data/arabic_males.csv', 'r') as f:
    reader = csv.reader(f)
    next(reader)  # Skip header
    for row in reader:
        if row and row[0]:
            name = row[0].strip().lower()
            if name and len(name) > 2 and name.isalpha():
                arabic_names.add(name)

# Read female names  
with open('data/arabic_females.csv', 'r') as f:
    reader = csv.reader(f)
    next(reader)  # Skip header
    for row in reader:
        if row and row[0]:
            name = row[0].strip().lower()
            if name and len(name) > 2 and name.isalpha():
                arabic_names.add(name)

# Select a subset of Arabic names to add (about 100-150 each)
arabic_names_list = list(arabic_names)
random.shuffle(arabic_names_list)

# Split names into potential adjectives and nouns
# Names with certain qualities can be adjectives
adjective_qualities = ['aziz', 'karim', 'rashid', 'jamil', 'salim', 'sharif', 
                      'latif', 'fadil', 'hakim', 'majid', 'nasir', 'qasim',
                      'saeed', 'wahid', 'zahir', 'amina', 'fatima', 'jamila',
                      'karima', 'latifa', 'nadira', 'rahima', 'salima', 'zahra']

new_adjectives = []
new_nouns = []

for name in arabic_names_list[:300]:  # Take first 300 names
    # Add names that sound like qualities as adjectives
    is_adjective = False
    for quality in adjective_qualities:
        if quality in name:
            is_adjective = True
            break
    
    if is_adjective and len(new_adjectives) < 100:
        if name not in existing_adjectives:
            new_adjectives.append(name)
    elif len(new_nouns) < 200:
        if name not in existing_nouns:
            new_nouns.append(name)

# Add the new names to our lists
print(f"Adding {len(new_adjectives)} new adjectives and {len(new_nouns)} new nouns")

# Append to adjectives file
with open('data/adjectives.txt', 'a') as f:
    for adj in sorted(new_adjectives):
        f.write(f"\n{adj}")

# Append to nouns file  
with open('data/nouns.txt', 'a') as f:
    for noun in sorted(new_nouns):
        f.write(f"\n{noun}")

print("Done! Names have been added to the word lists.")