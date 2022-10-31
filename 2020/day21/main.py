from dataclasses import dataclass

@dataclass
class Recipe:
    ingredients: set[str]
    allergens: list[str]

def load_file(file_name):
    total_allergens = set()
    total_ingredients = set()

    with open(file_name) as infile:
        recipies = []
        
        for line in infile:
            ingredients = [
                x.strip() 
                for x in line.split('(')[0].split(" ") 
                if len(x) > 0
            ]
                
            allergens = [
                x.strip().replace(")", "").replace(",", "")
                for x in line.split('(')[1].split(" ")
                if len(x) > 0 and x != "contains"
            ]

            for a in allergens:
                total_allergens.add(a)
            for i in ingredients:
                total_ingredients.add(i)
            
            recipies.append(Recipe(set(ingredients), allergens))
        
    return recipies, total_ingredients, total_allergens

def part_1():
    recipies, total_ingredients, total_allergens = load_file("input.txt")

    impossible = total_ingredients

    for allergen in total_allergens:
        possible = total_ingredients
        for recipe in recipies:
            if allergen in recipe.allergens:
                possible = possible.intersection(recipe.ingredients)

        impossible = impossible.difference(possible)

    count = 0
    for recipe in recipies:
        for ingredient in recipe.ingredients:
            if ingredient in impossible:
                count += 1

    return count

def part_2():
    recipies, total_ingredients, total_allergens = load_file("input.txt")

    impossible = total_ingredients

    allergens = {}

    for allergen in total_allergens:
        possible = total_ingredients
        for recipe in recipies:
            if allergen in recipe.allergens:
                possible = possible.intersection(recipe.ingredients)
        
        allergens[allergen] = possible

        impossible = impossible.difference(possible)


    while any(filter(lambda x: len(allergens[x]) > 1, allergens)):
        for a1 in allergens:
            if len(allergens[a1]) == 1:
                for a2 in allergens:
                    if a2 != a1:
                        allergens[a2] = allergens[a2].difference(allergens[a1])


    return ",".join(map(lambda x: next(iter(x[1])), sorted(allergens.items(), key=lambda x: x[0])))

    

def main():
    print("A:", part_1())
    print("B:", part_2())
    

   

if __name__ == "__main__":
    main()